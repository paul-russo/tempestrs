use db::{self, GetObservations};

fn main() {
    let conn = db::connect().unwrap();

    let weather = conn
        .get_latest_observation()
        .expect("unable to get latest weather observation");

    println!("FORMATTED WEATHER OBSERVATION:");
    println!("{}", weather);

    println!("JSON WEATHER OBSERVATION:");
    println!("{}", serde_json::to_string_pretty(&weather).unwrap());
}
