use core::{
    queries::{QUERY_CREATE_TABLE_OBSERVATION, QUERY_INSERT_OBSERVATION},
    weather::Weather,
};
use dirs::home_dir;
use rusqlite::{params, Connection};
use std::fs::create_dir;

pub fn connect() -> rusqlite::Result<Connection> {
    let dir = home_dir().unwrap().join(".tempestrs");

    if !dir.exists() {
        create_dir(&dir).expect("unable to create directory");
    }

    let conn = Connection::open(dir.join("weather.db3"))?;

    conn.execute(QUERY_CREATE_TABLE_OBSERVATION, ())?;

    Ok(conn)
}

pub trait InsertObservation {
    fn insert_observation(&self, obs: Weather) -> rusqlite::Result<()>;
}

impl InsertObservation for Connection {
    fn insert_observation(&self, obs: Weather) -> rusqlite::Result<()> {
        self.execute(
            QUERY_INSERT_OBSERVATION,
            params!(
                obs.time_epoch,
                obs.wind_lull,
                obs.wind_avg,
                obs.wind_gust,
                obs.wind_direction,
                obs.wind_sample_interval,
                obs.station_pressure,
                obs.air_temp,
                obs.relative_humidity,
                obs.illuminance,
                obs.uv_index,
                obs.solar_radiation,
                obs.rain_over_prev_minute,
                obs.precip_type as u8,
                obs.lightning_avg_distance,
                obs.lightning_strike_count,
                obs.battery_voltage,
                obs.report_interval,
            ),
        )?;

        Ok(())
    }
}

pub trait GetObservations {
    fn get_observations(&self, limit: usize) -> rusqlite::Result<Vec<Weather>>;
    fn get_latest_observation(&self) -> Option<Weather>;
}

impl GetObservations for Connection {
    fn get_observations(&self, limit: usize) -> rusqlite::Result<Vec<Weather>> {
        let mut stmt = self.prepare("SELECT * FROM observation ORDER BY id DESC LIMIT ?1")?;
        let weather_rows = stmt.query_map(params!(limit), |row| {
            Ok(Weather {
                time_epoch: row.get(1)?,
                wind_lull: row.get(2)?,
                wind_avg: row.get(3)?,
                wind_gust: row.get(4)?,
                wind_direction: row.get(5)?,
                wind_sample_interval: row.get(6)?,
                station_pressure: row.get(7)?,
                air_temp: row.get(8)?,
                relative_humidity: row.get(9)?,
                illuminance: row.get(10)?,
                uv_index: row.get(11)?,
                solar_radiation: row.get(12)?,
                rain_over_prev_minute: row.get(13)?,
                precip_type: row.get::<_, f64>(14)?.into(),
                lightning_avg_distance: row.get(15)?,
                lightning_strike_count: row.get(16)?,
                battery_voltage: row.get(17)?,
                report_interval: row.get(18)?,
            })
        })?;

        weather_rows.collect()
    }

    fn get_latest_observation(&self) -> Option<Weather> {
        self.get_observations(1).ok()?.first().map(|w| *w)
    }
}
