#[derive(Debug)]
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
#[derive(Debug)]
pub struct Weather {
    pub time_epoch: i64,
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

pub trait IntoWeather {
    fn into_weather(&self) -> Option<Weather>;
}
