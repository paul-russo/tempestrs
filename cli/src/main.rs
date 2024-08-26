use db::{self, GetObservations};

fn main() {
    let conn = db::connect().unwrap();

    let weather = conn
        .get_latest_observation()
        .expect("unable to get latest weather observation");

    println!("{}", weather);
}
