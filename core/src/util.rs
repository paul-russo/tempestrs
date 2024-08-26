use chrono::Duration;
use core::fmt::Display;
use num_traits::int::PrimInt;

pub trait Counted {
    fn counted(&self, singular: &str) -> String;
}

impl<T: Display + PrimInt> Counted for T {
    fn counted(&self, singular: &str) -> String {
        if *self == T::one() {
            format!("{} {}", self, singular)
        } else {
            format!("{} {}s", self, singular)
        }
    }
}

pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.num_seconds() % 60;
    let minutes = (duration.num_seconds() / 60) % 60;
    let hours = duration.num_seconds() / 3600;

    let mut pieces: Vec<String> = Vec::new();

    if hours > 0 {
        pieces.push(hours.counted("hour"));
    }

    if hours > 0 || minutes > 0 {
        pieces.push(minutes.counted("minute"));
    }

    pieces.push(seconds.counted("second"));

    pieces.join(", ")
}
