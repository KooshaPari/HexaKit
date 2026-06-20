# HexaKit Configuration Reference

The `HexaKitSettings` struct consolidates all hardcoded configuration values
from across the HexaKit workspace into a single, deserializable settings object.

## Loading

Settings are loaded via `figment` with the following priority order (highest
wins):

1. **Environment variables** — prefix `HEXAKIT_`, dot-separated path segments
   split by `_`.  
   Example: `HEXAKIT_BULKHEAD_MAX_CONCURRENT=20` overrides
   `settings.bulkhead.max_concurrent`.

2. **Config file** — loaded from `./hexakit.toml` by default, or from the path
   pointed to by the `HEXAKIT_CONFIG` environment variable.

3. **Built-in defaults** — defined in `HexaKitSettings::default()`.

## Config Keys

### `[bulkhead]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `max_concurrent` | `usize` | `10` | Maximum concurrent executions. |
| `max_queue` | `usize` | `20` | Maximum queued waiters. |
| `queue_timeout_ms` | `u64` | `1000` | Queue timeout in milliseconds. |

### `[circuit]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `failure_threshold` | `u32` | `5` | Failure count before circuit opens. |
| `success_threshold` | `u32` | `3` | Success count in half-open before closing. |
| `open_duration_secs` | `u64` | `30` | Seconds to wait before half-open. |
| `window_size_secs` | `u64` | `60` | Sliding window for counting failures. |

### `[rate_limit]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `capacity` | `u32` | `100` | Token bucket capacity. |
| `refill_rate` | `u32` | `10` | Token refill rate (tokens/second). |
| `refill_interval_ms` | `u64` | `1000` | Refill interval in milliseconds. |

### `[pool]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `max_size` | `usize` | `10` | Maximum connections in the pool. |
| `min_idle` | `usize` | `1` | Minimum idle connections to retain. |
| `max_lifetime_secs` | `u64` | `1800` | Max connection lifetime (30 min). |
| `idle_timeout_secs` | `u64` | `600` | Idle timeout (10 min). |
| `connection_timeout_secs` | `u64` | `30` | Connection acquisition timeout. |

### `[paths]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `system_config_path` | `String` | `/etc/{app_name}` | System-wide config directory template. `{app_name}` is replaced at runtime. |
| `config_dir_env_var` | `String` | `CONFIG_DIR` | Environment variable name overriding config directory. |
| `default_config_dir` | `String` | `./config` | Fallback config directory. |

### `[security]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `github_api_base_url` | `String` | `https://api.github.com` | Base URL for GitHub API. |
| `critical_deduction` | `f32` | `25.0` | Score deduction per critical alert. |
| `high_deduction` | `f32` | `10.0` | Score deduction per high alert. |
| `medium_deduction` | `f32` | `2.0` | Score deduction per medium alert. |
| `max_score` | `f32` | `100.0` | Maximum possible security score. |

### `[ttl]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `one_minute_secs` | `u64` | `60` | 1-minute TTL. |
| `five_minutes_secs` | `u64` | `300` | 5-minute TTL. |
| `fifteen_minutes_secs` | `u64` | `900` | 15-minute TTL. |
| `thirty_minutes_secs` | `u64` | `1800` | 30-minute TTL. |
| `one_hour_secs` | `u64` | `3600` | 1-hour TTL. |
| `one_day_secs` | `u64` | `86400` | 1-day TTL. |
| `one_week_secs` | `u64` | `604800` | 1-week TTL. |

## Load API

```rust
use phenotype_shared_config::settings::HexaKitSettings;

// Load with default chain (env → file → built-in defaults)
let settings = HexaKitSettings::load().expect("valid settings");

// Load from a specific file path
let settings = HexaKitSettings::load_from("/path/to/config.toml").expect("valid settings");

// Use defaults directly
let settings = HexaKitSettings::default();
println!("{}", settings.bulkhead.max_concurrent);
```

## Environment Variables

Every key can be overridden via environment variables with the `HEXAKIT_`
prefix and `_` separators for nesting:

| Env Variable | Overrides |
|---|---|
| `HEXAKIT_BULKHEAD_MAX_CONCURRENT` | `[bulkhead] max_concurrent` |
| `HEXAKIT_CIRCUIT_FAILURE_THRESHOLD` | `[circuit] failure_threshold` |
| `HEXAKIT_RATE_LIMIT_CAPACITY` | `[rate_limit] capacity` |
| `HEXAKIT_POOL_MAX_SIZE` | `[pool] max_size` |
| `HEXAKIT_SECURITY_MAX_SCORE` | `[security] max_score` |
| `HEXAKIT_TTL_ONE_HOUR_SECS` | `[ttl] one_hour_secs` |
| `HEXAKIT_CONFIG` | Config file path (not a settings key) |
