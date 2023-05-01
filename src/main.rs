mod packet;
mod weather;

use packet::Packet;
use serde_json;
use std::net::UdpSocket;
use time::{OffsetDateTime, UtcOffset};
use weather::Weather;

use crate::weather::IntoWeather;

fn main() {
    let local_offset = UtcOffset::current_local_offset().unwrap();

    let socket = UdpSocket::bind("0.0.0.0:50222").expect("Failed to bind socket");
    let mut weather: Option<Weather> = None;

    loop {
        let mut buf = [0u8; 65507];
        let result = socket.recv(&mut buf);

        if let Some(Weather { time_epoch, .. }) = weather {
            if time_epoch > 0 {
                let date = OffsetDateTime::from_unix_timestamp(time_epoch)
                    .unwrap()
                    .to_offset(local_offset);

                println!("time: {}", date);

                println!("weather: {:?}", weather);
            }
        }

        match result {
            Ok(num_bytes) => {
                let payload = &buf[0..num_bytes];

                match serde_json::from_slice::<Packet>(payload) {
                    Ok(packet) => {
                        println!("PACKET: {}", serde_json::to_string_pretty(&packet).unwrap());
                        weather = packet.into_weather().or(weather);
                    }
                    Err(err) => {
                        println!("Unable to deserialize: {}", err);
                    }
                }
            }

            Err(ref err) => {
                println!("Network error: {}", err)
            }
        }
    }
}
