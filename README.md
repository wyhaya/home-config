
# home-config
[![Crates.io](https://img.shields.io/crates/v/home-config.svg?style=flat-square)](https://crates.io/crates/home-config)
[![LICENSE](https://img.shields.io/crates/l/home-config.svg?style=flat-square)](https://crates.io/crates/home-config)
 
Use configuration file in the HOME directory
 
## Usage
 
```rust
use home_config::HomeConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Options {
    name: String,
    age: u32,
}

let config = HomeConfig::new("your_name", "config.json");
// /Users/name/your_name/.config/config.json

// Parse
let options = config.parse::<Options>().unwrap();
// options.name == "XiaoMing";
// options.age == 18;

// Save to file
config.save(&options).unwrap();
```

