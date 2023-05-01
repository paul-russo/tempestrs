use crate::weather::{IntoWeather, Weather};
use serde::de::{self, Deserializer, Unexpected};
use serde::{Deserialize, Serialize};

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Packet {
    #[serde(rename = "obs_st")]
    Observation {
        serial_number: String,
        hub_sn: String,
        firmware_revision: u64,
        /**
        0: Time Epoch, Seconds
        1: Wind Lull (minimum 3 second sample), m/s
        2: Wind Avg (average over report interval), m/s
        3: Wind Gust (maximum 3 second sample), m/s
        4: Wind Direction, Degrees
        5: Wind Sample Interval, seconds
        6: Station Pressure, MB
        7: Air Temperature, C
        8: Relative Humidity, %
        9: Illuminance, Lux
        10: UV, Index
        11: Solar Radiation, W/m^2
        12: Rain amount over previous minute, mm
        13: Precipitation Type, 0 = none; 1 = rain; 2 = hail; 3 = rain + hail
        14: Lightning Strike Avg Distance, km
        15: Lightning Strike Count, count
        16: Battery, Volts
        17: Report Interval, Minutes
        */
        obs: [[f64; 18]; 1],
    },

    #[serde(rename = "rapid_wind")]
    RapidWind {
        serial_number: String,
        hub_sn: String,
        /**
        0: Time Epoch, Seconds
        1: Wind Speed, m/s
        2: Wind Direction, Degrees
        */
        ob: (u64, f64, u64),
    },

    #[serde(rename = "evt_precip")]
    EventRainStart {
        serial_number: String,
        hub_sn: String,
        // Time Epoch (Seconds)
        evt: [u64; 1],
    },

    #[serde(rename = "evt_strike")]
    EventLightningStrike {
        serial_number: String,
        hub_sn: String,
        /**
         * 0: Time Epoch (Seconds)
         * 1: Distance (km)
         * 2: Energy (unitless)
         */
        evt: [u64; 3],
    },

    #[serde(rename = "device_status")]
    DeviceStatus {
        serial_number: String,
        hub_sn: String,
        timestamp: u64,
        uptime: u64,
        voltage: f64,
        firmware_revision: u64,
        rssi: i64,
        hub_rssi: i64,
        /**
         * 0b000000000	Sensors OK
         * 0b000000001	lightning failed
         * 0b000000010	lightning noise
         * 0b000000100	lightning disturber
         * 0b000001000	pressure failed
         * 0b000010000	temperature failed
         * 0b000100000	rh failed
         * 0b001000000	wind failed
         * 0b010000000	precip failed
         * 0b100000000	light/uv failed
         */
        sensor_status: u64,
        #[serde(deserialize_with = "bool_from_int")]
        debug: bool,
    },

    #[serde(rename = "hub_status")]
    HubStatus {
        serial_number: String,
        // Yes, this is a stringified number, as opposed to the numeric value used
        // elsewhere.
        firmware_revision: String,
        uptime: u64,
        rssi: i64,
        timestamp: u64,
        reset_flags: String,
        seq: u64,
        radio_stats: [u64; 5],
        mqtt_stats: [u64; 2],
    },

    #[serde(other)]
    Other,
}

impl IntoWeather for Packet {
    fn into_weather(&self) -> Option<Weather> {
        match self {
            Packet::Observation { obs, .. } => {
                let obs = obs[0];
                Some(Weather {
                    time_epoch: obs[0] as i64,
                    wind_lull: obs[1] as f32,
                    wind_avg: obs[2] as f32,
                    wind_gust: obs[3] as f32,
                    wind_direction: obs[4] as u16,
                    wind_sample_interval: obs[5] as u16,
                    station_pressure: obs[6] as f32,
                    air_temp: obs[7] as f32,
                    relative_humidity: obs[8] as f32,
                    illuminance: obs[9] as u32,
                    uv_index: obs[10] as f32,
                    solar_radiation: obs[11] as u32,
                    rain_over_prev_minute: obs[12] as f32,
                    precip_type: obs[13].into(),
                    lightning_avg_distance: obs[14] as u32,
                    lightning_strike_count: obs[15] as u32,
                    battery_voltage: obs[16] as f32,
                    report_interval: obs[17] as u16,
                })
            }
            _ => None,
        }
    }
}
