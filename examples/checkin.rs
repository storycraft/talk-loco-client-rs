/*
 * Created on Wed Dec 08 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::error::Error;

use loco_protocol::secure::{
    crypto::CryptoStore, session::SecureClientSession, stream::SecureStream,
};
use rsa::{pkcs8::FromPublicKey, RsaPublicKey};
use talk_loco_client::{
    client::checkin::CheckinClient, command::session::BsonCommandSession, request,
    stream::ChunkedWriteStream, structs::client::ClientInfo,
};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

pub static KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIDANBgkqhkiG9w0BAQEFAAOCAQ0AMIIBCAKCAQEApElgRBx+g7sniYFW7LE8ivrwXShKTRFV8lXNItMXbN5QSC8vJ/cTSOTS619Xv5Zx7xXJIk4EKxtWesEGbgZpEUP2xQ+IeH9oz0JxayEMvvD1nVNAWgpWE4pociEoArsK7qY3YwXb1CiDHo9hojLv7djbo3cwXvlyMh4TUrX2RjCZPlVJxk/LVjzcl9ohJLkl3eoSrf0AE4kQ9mk3+raEhq5Dv+IDxKYX+fIytUWKmrQJusjtre9oVUX5sBOYZ0dzez/XapusEhUWImmB6mciVXfRXQ8IK4IH6vfNyxMSOTfLEhRYN2SMLzplAYFiMV536tLS3VmG5GJRdkpDubqPeQIBAw==
-----END PUBLIC KEY-----";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let loco_session = SecureClientSession::new(
        RsaPublicKey::from_public_key_der(&pem::parse(KEY)?.contents).unwrap(),
    );

    let mut stream = SecureStream::new(
        CryptoStore::new(),
        ChunkedWriteStream::new(
            TcpStream::connect("ticket-loco.kakao.com:443")
                .await
                .unwrap()
                .compat(),
            2048,
        ),
    );

    loco_session.handshake_async(&mut stream).await?;

    let mut checkin_conn = BsonCommandSession::new(stream);
    let mut checkin_client = CheckinClient(&mut checkin_conn);

    let checkin_res = checkin_client
        .checkin(&request::checkin::CheckinReq {
            user_id: 1,
            client: ClientInfo {
                os: "win32".into(),
                net_type: 0,
                app_version: "3.2.8".into(),
                mccmnc: "999".into(),
            },
            language: "ko".into(),
            country_iso: "KR".into(),
            use_sub: true,
        })
        .await?;

    println!("CHECKIN response: {:?}", checkin_res);

    Ok(())
}
