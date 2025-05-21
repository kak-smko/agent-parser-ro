# User Agent Parser

[![Crates.io](https://img.shields.io/crates/v/agent-parser-ro)](https://crates.io/crates/agent-parser-ro)
[![Documentation](https://docs.rs/agent-parser-ro/badge.svg)](https://docs.rs/agent-parser-ro)
[![License](https://img.shields.io/crates/l/agent-parser-ro)](LICENSE)


A fast and comprehensive user agent string parser for Rust that detects:
- **Browsers** (Chrome, Safari, Firefox, etc.)
- **Operating Systems** (Windows, macOS, Android, etc.)
- **Device Types** (Mobile, Tablet, Desktop, etc.)

## Features

- 🚀 **Fast parsing** using optimized regular expressions
- 📦 **No external network requests** - works offline
- 🧩 **Comprehensive coverage** of browsers, OSes, and devices
- 🔍 **Accurate detection** of modern and legacy user agents
- 🛠 **Serde support** for easy serialization/deserialization
- 📚 **Well-documented** with examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
agent-parser-ro = "0.1"
```

## Usage

```rust
use agent_parser_ro::{UserAgentParser, Browser, OperatingSystem, DeviceType};

let info = UserAgentParser::parse("Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1");

assert_eq!(info.os, OperatingSystem::IOS);
assert_eq!(info.browser, Browser::Safari);
assert_eq!(info.device_type, DeviceType::Mobile);
```



## Contributing

Contributions are welcome! Please open an issue or submit a PR for:
- New browser/OS/device detection
- Performance improvements
- Bug fixes


## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.