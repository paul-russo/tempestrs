use std::fmt::Display;

#[derive(Debug)]
pub enum TempUnit {
    F,
    C,
}

impl Display for TempUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TempUnit::C => write!(f, "°C"),
            TempUnit::F => write!(f, "°F"),
        }
    }
}

pub struct Temperature {
    value: f32,
    unit: TempUnit,
}

impl Temperature {
    pub fn new(temp: f32, unit: TempUnit) -> Self {
        Self { value: temp, unit }
    }

    pub fn into_f(&self) -> Temperature {
        Temperature::new(
            match self.unit {
                TempUnit::C => self.value * (9.0 / 5.0) + 32.0,
                TempUnit::F => self.value,
            },
            TempUnit::F,
        )
    }

    pub fn into_c(&self) -> Temperature {
        Temperature::new(
            match self.unit {
                TempUnit::C => self.value,
                TempUnit::F => (self.value - 32.0) * (5.0 / 9.0),
            },
            TempUnit::C,
        )
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

#[derive(Debug)]
pub enum SpeedUnit {
    MetersPerSecond,
    MilesPerHour,
}

impl Display for SpeedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpeedUnit::MetersPerSecond => write!(f, "m/s"),
            SpeedUnit::MilesPerHour => write!(f, "mph"),
        }
    }
}

pub struct Speed {
    value: f32,
    unit: SpeedUnit,
}

impl Speed {
    pub fn new(value: f32, unit: SpeedUnit) -> Self {
        Self { value, unit }
    }

    pub fn into_meters_per_second(&self) -> Speed {
        Speed::new(
            match self.unit {
                SpeedUnit::MetersPerSecond => self.value,
                SpeedUnit::MilesPerHour => self.value * 0.44704, // 1 mph = 0.44704 m/s
            },
            SpeedUnit::MetersPerSecond,
        )
    }

    pub fn into_miles_per_hour(&self) -> Speed {
        Speed::new(
            match self.unit {
                SpeedUnit::MetersPerSecond => self.value / 0.44704, // 1 m/s = 2.23694 mph
                SpeedUnit::MilesPerHour => self.value,
            },
            SpeedUnit::MilesPerHour,
        )
    }
}

impl Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}
