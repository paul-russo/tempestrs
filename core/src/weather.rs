use chrono::{DateTime, Local, LocalResult, TimeZone};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use wasm_bindgen::JsValue;

use crate::{
    units::{Speed, SpeedUnit, TempUnit, Temperature},
    util::format_duration,
};

#[derive(Debug, Clone, Copy)]
pub enum PrecipitationType {
    None,
    Rain,
    Hail,
    RainAndHail,
}

impl From<f64> for PrecipitationType {
    fn from(item: f64) -> PrecipitationType {
        match item as u64 {
            1 => PrecipitationType::Rain,
            2 => PrecipitationType::Hail,
            3 => PrecipitationType::RainAndHail,
            _ => PrecipitationType::None,
        }
    }
}

impl Into<JsValue> for PrecipitationType {
    fn into(self) -> JsValue {
        match self {
            PrecipitationType::None => JsValue::from(0),
            PrecipitationType::Rain => JsValue::from(1),
            PrecipitationType::Hail => JsValue::from(2),
            PrecipitationType::RainAndHail => JsValue::from(3),
        }
    }
}

impl Serialize for PrecipitationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            PrecipitationType::None => 0,
            PrecipitationType::Rain => 1,
            PrecipitationType::Hail => 2,
            PrecipitationType::RainAndHail => 3,
        };
        serializer.serialize_u8(value)
    }
}

impl<'de> Deserialize<'de> for PrecipitationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(PrecipitationType::None),
            1 => Ok(PrecipitationType::Rain),
            2 => Ok(PrecipitationType::Hail),
            3 => Ok(PrecipitationType::RainAndHail),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid value for PrecipitationType: {}",
                value
            ))),
        }
    }
}

/**
0: Time Epoch, Seconds
1: Wind Lull (minimum 3 second sample), m/s
2: Wind Avg (average over report interval), m/s
3: Wind Gust (maximum 3 second sample), m/s
4: Wind Direction, Degrees
5: Wind Sample Interval, seconds
6: Station Pressure, mbar
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
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Weather {
    pub time_epoch: u64,
    pub wind_lull: f32,
    pub wind_avg: f32,
    pub wind_gust: f32,
    pub wind_direction: u16,
    pub wind_sample_interval: u16,
    pub station_pressure: f32,
    pub air_temp: f32,
    pub relative_humidity: f32,
    pub illuminance: u32,
    pub uv_index: f32,
    pub solar_radiation: u32,
    pub rain_over_prev_minute: f32,
    pub precip_type: PrecipitationType,
    pub lightning_avg_distance: u32,
    pub lightning_strike_count: u32,
    pub battery_voltage: f32,
    pub report_interval: u16,
}

impl Weather {
    pub fn get_time(&self) -> LocalResult<DateTime<Local>> {
        Local.timestamp_opt(self.time_epoch as i64, 0)
    }

    pub fn get_air_temp(&self) -> Temperature {
        Temperature::new(self.air_temp, TempUnit::C)
    }

    pub fn get_wind_lull(&self) -> Speed {
        Speed::new(self.wind_lull, SpeedUnit::MetersPerSecond)
    }

    pub fn get_wind_avg(&self) -> Speed {
        Speed::new(self.wind_avg, SpeedUnit::MetersPerSecond)
    }

    pub fn get_wind_gust(&self) -> Speed {
        Speed::new(self.wind_gust, SpeedUnit::MetersPerSecond)
    }
}

pub trait IntoWeather {
    fn into_weather(&self) -> Option<Weather>;
}

impl Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let obs_time = self.get_time().unwrap();
        let elapsed = Local::now().signed_duration_since(obs_time);
        let display_time = obs_time.format("%B %-d, %Y at %-I:%M %p");

        writeln!(f, "{} ({} ago)", display_time, format_duration(elapsed))?;
        writeln!(f, "Air Temperature: {}", self.get_air_temp().into_f())?;
        writeln!(
            f,
            "Wind Lull: {}",
            self.get_wind_lull().into_miles_per_hour()
        )?;
        writeln!(f, "Wind Avg: {}", self.get_wind_avg().into_miles_per_hour())?;
        writeln!(
            f,
            "Wind Gust: {}",
            self.get_wind_gust().into_miles_per_hour()
        )?;
        writeln!(f, "Wind Direction: {}°", self.wind_direction)?;
        writeln!(
            f,
            "Wind Sample Interval: {} seconds",
            self.wind_sample_interval
        )?;
        writeln!(f, "Station Pressure: {} mbar", self.station_pressure)?;
        writeln!(f, "Relative Humidity: {}%", self.relative_humidity)?;
        writeln!(f, "Illuminance: {} Lux", self.illuminance)?;
        writeln!(f, "UV Index: {}", self.uv_index)?;
        writeln!(f, "Solar Radiation: {} W/m^2", self.solar_radiation)?;
        writeln!(
            f,
            "Rain over Previous Minute: {} mm",
            self.rain_over_prev_minute
        )?;
        writeln!(f, "Precipitation Type: {:?}", self.precip_type)?;
        writeln!(
            f,
            "Lightning Average Distance: {} km",
            self.lightning_avg_distance
        )?;
        writeln!(f, "Lightning Strike Count: {}", self.lightning_strike_count)?;
        writeln!(f, "Battery Voltage: {} Volts", self.battery_voltage)?;
        writeln!(f, "Report Interval: {} Minutes", self.report_interval)?;
        Ok(())
    }
}
