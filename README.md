
# home-config

[![Crates.io](https://img.shields.io/crates/v/home-config.svg?style=flat-square)](https://crates.io/crates/home-config)
[![docs.rs](https://img.shields.io/badge/docs-rs-informational.svg?style=flat-square)](https://docs.rs/home-config)
[![LICENSE](https://img.shields.io/crates/l/home-config.svg?style=flat-square)](https://crates.io/crates/home-config)
 
Use configuration file in the HOME directory
 
## Usage

```rust
use home_config::HomeConfig;

let config = HomeConfig::new("app", "config");
// Linux: /home/name/.config/app/config
// macOS: /Users/name/.config/app/config
// Windows: C:\Users\name\.config\app\config

// Write
config.save("123456789").unwrap();

// Read
let data = config.read_to_string().unwrap();
// 123456789
```

### json / yaml

Cargo.toml

```toml
home-config = { version = "*", features = ["json", "yaml"] }
```

```rust
use home_config::HomeConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
struct Options {
    name: String,
    age: u32,
}

let config = HomeConfig::new("app", "config.json");

// Parse
let options = config.json::<Options>().unwrap_or_default();
// options.name == "XiaoMing";
// options.age == 18;

// Save to file
config.save_json(&options).unwrap();
```

