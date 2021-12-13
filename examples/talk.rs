/*
 * Created on Wed Dec 08 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{borrow::Cow, env, error::Error, thread, time::Duration};

use futures::{AsyncRead, AsyncWrite};
use loco_protocol::secure::{
    crypto::CryptoStore, session::SecureClientSession, stream::SecureStream,
};
use rsa::{pkcs8::FromPublicKey, RsaPublicKey};
use talk_api_client::{
    agent::TalkApiAgent,
    auth::{
        resources::LoginData, xvc::default::Win32XVCHasher, AccountLoginForm, AuthClientConfig,
        AuthDeviceConfig, LoginMethod, TalkAuthClient,
    },
    response::TalkStatusResponse,
    ApiRequestError,
};
use talk_loco_client::{
    client::{checkin::CheckinClient, talk::TalkClient, RequestResult},
    command::{manager::BsonCommandManager, session::BsonCommandSession},
    request::{self, chat::{LoginListReq, LChatListReq}}, response,
    stream::ChunkedWriteStream,
    structs::client::ClientInfo,
};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

pub const CONFIG: AuthClientConfig = AuthClientConfig::new_const(
    AuthDeviceConfig::new_const_pc(
        "TEST_DEVICE",
        "",
    ),
    "ko",
    "3.2.8",
    TalkApiAgent::Win32(Cow::Borrowed("10.0")),
);

pub const HASHER: Win32XVCHasher = Win32XVCHasher::new_const("JAYDEN", "JAYMOND");

pub static KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIDANBgkqhkiG9w0BAQEFAAOCAQ0AMIIBCAKCAQEApElgRBx+g7sniYFW7LE8ivrwXShKTRFV8lXNItMXbN5QSC8vJ/cTSOTS619Xv5Zx7xXJIk4EKxtWesEGbgZpEUP2xQ+IeH9oz0JxayEMvvD1nVNAWgpWE4pociEoArsK7qY3YwXb1CiDHo9hojLv7djbo3cwXvlyMh4TUrX2RjCZPlVJxk/LVjzcl9ohJLkl3eoSrf0AE4kQ9mk3+raEhq5Dv+IDxKYX+fIytUWKmrQJusjtre9oVUX5sBOYZ0dzez/XapusEhUWImmB6mciVXfRXQ8IK4IH6vfNyxMSOTfLEhRYN2SMLzplAYFiMV536tLS3VmG5GJRdkpDubqPeQIBAw==
-----END PUBLIC KEY-----";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!(
            "Usage: {} <email> <password> <device_uuid>",
            args.get(0).unwrap_or(&String::new())
        );
        return Ok(());
    }

    let method = LoginMethod::Account(AccountLoginForm {
        email: Cow::Borrowed(&args[1]),
        password: Cow::Borrowed(&args[2]),
    });

    // Do auth using provided information
    let auth_res = do_login(&args[3], &method).await?;
    if auth_res.data.is_none() {
        println!("Auth failed with status: {}", auth_res.status);
        return Ok(());
    }

    let auth_data = auth_res.data.unwrap();

    let loco_session = SecureClientSession::new(
        RsaPublicKey::from_public_key_der(&pem::parse(KEY)?.contents).unwrap(),
    );

    let client = ClientInfo {
        os: "win32".into(),
        net_type: 0,
        app_version: "3.2.8".into(),
        mccmnc: "999".into(),
    };

    // Skipped booking process

    // Checkin start
    let checkin_data = {
        let mut checkin_stream = SecureStream::new(
            CryptoStore::new(),
            ChunkedWriteStream::new(
                TcpStream::connect("ticket-loco.kakao.com:443")
                    .await
                    .unwrap()
                    .compat(),
                2048,
            ),
        );
    
        loco_session.handshake_async(&mut checkin_stream).await?;
        
        let checkin_res = do_checkin(checkin_stream, client.clone()).await?;
    
        if checkin_res.data.data.is_none() {
            println!("CHECKIN failed with status: {}", checkin_res.data.status);
            return Ok(());
        }
        checkin_res.data.data.unwrap()
    };

    println!("CHECKIN response: {:?}", checkin_data);
    // Checkin end

    // Login start
    let mut talk_stream = SecureStream::new(
        CryptoStore::new(),
        ChunkedWriteStream::new(
            TcpStream::connect(&format!("{}:{}", checkin_data.host, checkin_data.port))
                .await
                .unwrap()
                .compat(),
            2048,
        ),
    );

    loco_session.handshake_async(&mut talk_stream).await?;

    let mut talk_conn = BsonCommandSession::new(BsonCommandManager::new(talk_stream));

    let login_res_data = {
        let mut talk_client = TalkClient(&mut talk_conn);

        let login_res = talk_client.login(&LoginListReq {
            client,
            protocol_version: "1".into(),
            device_uuid: args[3].clone(),
            oauth_token: auth_data.credential.access_token,
            language: "ko".into(),
            device_type: 2,
            revision: 0,
            rp: (),
            chat_list: LChatListReq {
                chat_ids: Vec::new(),
                max_ids: Vec::new(),
                last_token_id: 0,
                last_chat_id: None,
            },
            last_block_token: 0,
            background: false,
        }).await?;
        
        if login_res.data.data.is_none() {
            println!("LOGINLIST failed with status: {}", login_res.data.status);
            return Ok(());
        }

        login_res.data.data.unwrap()
    };

    println!("LOGINLIST response: {:?}", login_res_data);
    // Login end

    loop {
        // Read incoming broadcast commands
        let (read_id, read) = talk_conn.read_async().await?;

        println!("READ {}: {:?}", read_id, read);

        thread::sleep(Duration::from_millis(1));
    }
}

pub async fn do_login(
    uuid: &str,
    method: &LoginMethod<'_>,
) -> Result<TalkStatusResponse<LoginData>, ApiRequestError> {
    let mut config = CONFIG;
    config.device.uuid = Cow::Owned(uuid.to_string());

    let auth_client = TalkAuthClient::new(config, HASHER);

    Ok(auth_client.login(method, true).await?)
}

pub async fn do_checkin(
    stream: impl AsyncRead + AsyncWrite + Unpin,
    client_info: ClientInfo,
) -> RequestResult<response::checkin::CheckinRes> {
    let mut conn = BsonCommandSession::new(BsonCommandManager::new(stream));
    let mut client = CheckinClient(&mut conn);

    Ok(client
        .checkin(&request::checkin::CheckinReq {
            // Should be actual user id. but any numbers work
            user_id: 1,
            client: client_info,
            language: "ko".into(),
            country_iso: "KR".into(),
            use_usb: true,
        })
        .await?)
}
