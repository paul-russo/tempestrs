# Tempest Weather Station

Rust workspace for collecting and serving WeatherFlow Tempest weather station data.

## Cloud-specific instructions

### Services

| Service | Command | Notes |
|---------|---------|-------|
| **Worker** (Cloudflare Worker API) | `npx wrangler dev --local --port 8787` | Local D1 database; create `observation` table first (see below) |
| **CLI** | `cargo run -p cli` | Reads from local SQLite at `~/.tempestrs/weather.db3` |
| **Listener** | `cargo run -p listener` | Binds UDP:50222; requires physical WeatherFlow hub on network |

### Build / Lint / Test

- **Build:** `cargo build --workspace`
- **Lint:** `cargo clippy --workspace`
- **Test:** `cargo test --workspace` (doc-tests in `core` fail due to crate name shadowing `std::core`; unit tests pass)
- **Worker build:** `cd worker && worker-build --release` (requires `worker-build@0.1.2`)

### Key caveats

- The `wrangler.toml` build command pins `worker-build@0.1.2` because the project uses `worker` crate v0.2.0 and `wasm-bindgen` v0.2.93. Newer `worker-build` versions are incompatible.
- Before running `wrangler dev --local`, create the D1 observation table:
  ```
  npx wrangler d1 execute tempest --local --command "CREATE TABLE IF NOT EXISTS observation (id INTEGER PRIMARY KEY, time_epoch INTEGER, wind_lull REAL, wind_avg REAL, wind_gust REAL, wind_direction INTEGER, wind_sample_interval INTEGER, station_pressure REAL, air_temp REAL, relative_humidity REAL, illuminance INTEGER, uv_index REAL, solar_radiation INTEGER, rain_over_prev_minute REAL, precip_type INTEGER, lightning_avg_distance INTEGER, lightning_strike_count INTEGER, battery_voltage REAL, report_interval INTEGER)"
  ```
- The `POST /weather` endpoint hits a D1 BigInt error because `time_epoch` is `u64`. Use `wrangler d1 execute --local` to insert data directly, then `GET /weather/latest` works.
- The `listener` binary requires a physical WeatherFlow Tempest hub broadcasting on the local network; it cannot be tested without one.
