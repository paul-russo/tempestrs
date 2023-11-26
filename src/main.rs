mod db;
mod packet;
mod weather;

use packet::Packet;
use serde_json;
use std::net::UdpSocket;
use weather::Weather;

use crate::{db::InsertObservation, weather::IntoWeather};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:50222").expect("Port 50222 should be bindable");

    let conn = db::connect().unwrap();

    loop {
        let mut buf = [0u8; 65507];
        let result = socket.recv(&mut buf);

        match result {
            Ok(num_bytes) => {
                let payload = &buf[0..num_bytes];

                match serde_json::from_slice::<Packet>(payload) {
                    Ok(packet) => {
                        println!("PACKET: {}", serde_json::to_string_pretty(&packet).unwrap());

                        if let Some(weather) = packet.into_weather() {
                            match conn.insert_observation(weather) {
                                Err(error) => println!("DB ERROR: {:?}", error),
                                _ => (),
                            }
                        }
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
