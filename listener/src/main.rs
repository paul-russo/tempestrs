use core::packet::Packet;
use core::weather::IntoWeather;
use db;
use db::InsertObservation;
use serde_json;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:50222").expect("unable to bind to port 50222");
    let conn = db::connect().unwrap();

    loop {
        let mut buf = [0u8; 64000];
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

            Err(err) => {
                println!("Network error: {}", err)
            }
        }
    }
}
