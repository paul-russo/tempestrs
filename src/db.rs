use rusqlite::{params, Connection, Result};

use crate::weather::Weather;

pub fn connect() -> Result<Connection> {
    let conn = Connection::open("./weather.db3")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS observation (
            id INTEGER PRIMARY KEY,
            time_epoch INTEGER,
            wind_lull REAL,
            wind_avg REAL,
            wind_gust REAL,
            wind_direction INTEGER,
            wind_sample_interval INTEGER,
            station_pressure REAL,
            air_temp REAL,
            relative_humidity REAL,
            illuminance INTEGER,
            uv_index REAL,
            solar_radiation INTEGER,
            rain_over_prev_minute REAL,
            precip_type INTEGER,
            lightning_avg_distance INTEGER,
            lightning_strike_count INTEGER,
            battery_voltage REAL,
            report_interval INTEGER
        )",
        (),
    )?;

    Ok(conn)
}

pub trait InsertObservation {
    fn insert_observation(&self, obs: Weather) -> Result<()>;
}

impl InsertObservation for Connection {
    fn insert_observation(&self, obs: Weather) -> Result<()> {
        self.execute(
            "INSERT INTO observation (
                time_epoch,
                wind_lull,
                wind_avg,
                wind_gust,
                wind_direction,
                wind_sample_interval,
                station_pressure,
                air_temp,
                relative_humidity,
                illuminance,
                uv_index,
                solar_radiation,
                rain_over_prev_minute,
                precip_type,
                lightning_avg_distance,
                lightning_strike_count,
                battery_voltage,
                report_interval
              )
            VALUES (
                ?1,
                ?2,
                ?3,
                ?4,
                ?5,
                ?6,
                ?7,
                ?8,
                ?9,
                ?10,
                ?11,
                ?12,
                ?13,
                ?14,
                ?15,
                ?16,
                ?17,
                ?18
              )",
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
