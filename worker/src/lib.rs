use core::{queries::QUERY_INSERT_OBSERVATION, weather::Weather};
use worker::*;

async fn handle_get_weather_latest(db: &D1Database) -> Result<Response> {
    let weather_result = db
        .prepare("SELECT * FROM observation ORDER BY id DESC LIMIT 1")
        .first::<Weather>(None)
        .await?;

    match weather_result {
        Some(weather) => Response::from_json(&weather),
        None => Response::error("No weather observations found.", 404),
    }
}

async fn handle_post_weather(mut req: Request, db: &D1Database) -> Result<Response> {
    let weather: Weather = req.json().await?;

    db.prepare(QUERY_INSERT_OBSERVATION)
        .bind(&[
            (weather.time_epoch as f64).into(), // wasm-bindgen doesn't support i64
            weather.wind_lull.into(),
            weather.wind_avg.into(),
            weather.wind_gust.into(),
            weather.wind_direction.into(),
            weather.wind_sample_interval.into(),
            weather.station_pressure.into(),
            weather.air_temp.into(),
            weather.relative_humidity.into(),
            weather.illuminance.into(),
            weather.uv_index.into(),
            weather.solar_radiation.into(),
            weather.rain_over_prev_minute.into(),
            (weather.precip_type as u8).into(),
            weather.lightning_avg_distance.into(),
            weather.lightning_strike_count.into(),
            weather.battery_voltage.into(),
            weather.report_interval.into(),
        ])?
        .run()
        .await?;

    Response::ok("")
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let db = env.d1("DB")?;

    match (req.method(), &*req.path()) {
        (Method::Get, "/weather/latest") => handle_get_weather_latest(&db).await,
        (Method::Post, "/weather") => handle_post_weather(req, &db).await,
        _ => Response::error("Not found", 404),
    }
}
