/*
 * Created on Wed Dec 08 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::error::Error;

use talk_loco_client::{
    client::booking::BookingClient,
    command::{manager::BsonCommandManager, session::BsonCommandSession},
    request,
    stream::ChunkedWriteStream,
};
use tokio::net::TcpStream;
use tokio_native_tls::native_tls;
use tokio_util::compat::TokioAsyncReadCompatExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connector = tokio_native_tls::TlsConnector::from(native_tls::TlsConnector::new().unwrap());

    let stream = ChunkedWriteStream::new(
        connector
            .connect(
                "booking-loco.kakao.com",
                TcpStream::connect("booking-loco.kakao.com:443")
                    .await
                    .unwrap(),
            )
            .await
            .unwrap()
            .compat(),
        2048,
    );

    let mut booking_conn = BsonCommandSession::new(BsonCommandManager::new(stream));
    let mut booking_client = BookingClient(&mut booking_conn);

    let booking_res = booking_client
        .get_conf(&request::booking::GetConf {
            os: "win32".into(),
            mccmnc: "999".into(),
            model: "".into(),
        })
        .await?;

    println!("GETCONF response: {:?}", booking_res);

    Ok(())
}
