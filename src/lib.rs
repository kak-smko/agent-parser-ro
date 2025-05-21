//! A comprehensive user agent string parser.
//!
//! This crate provides functionality to parse user agent strings and extract information about:
//! - Browser/Client (Chrome, Safari, Firefox, etc.)
//! - Operating System (Windows, macOS, Android, etc.)
//! - Device Type (Mobile, Tablet, Desktop, etc.)
//!
//! # Examples
//!
//! ```
//! use agent_parser_ro::{UserAgentParser, Browser, OperatingSystem, DeviceType};
//!
//! let info = UserAgentParser::parse("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36");
//!
//! assert_eq!(info.os, OperatingSystem::Windows);
//! assert_eq!(info.browser, Browser::Chrome);
//! assert_eq!(info.device_type, DeviceType::Desktop);
//! ```

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum Browser {
    Chrome,
    Safari,
    Firefox,
    Edge,
    InternetExplorer,
    Opera,
    Dolphin,
    Brave,
    Puffin,
    Maxthon,
    Mercury,
    Silk,
    Vivaldi,
    Yandex,
    DuckDuckGo,
    Tor,
    Electron,
    PhantomJS,
    WebView,
    Facebook,
    Instagram,
    Twitter,
    Snapchat,
    Googlebot,
    Bingbot,
    Yahoo,
    Baidu,
    UCBrowser,
    SamsungBrowser,
    OculusBrowser,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum OperatingSystem {
    Windows,
    WindowsPhone,
    MacOS,
    IOS,
    IPadOS,
    Android,
    Linux,
    Ubuntu,
    Fedora,
    Debian,
    ChromeOS,
    BlackBerry,
    Symbian,
    WebOS,
    Bada,
    Tizen,
    Nintendo,
    PlayStation,
    Xbox,
    Wii,
    FreeBSD,
    OpenBSD,
    Solaris,
    AIX,
    HPUX,
    HarmonyOS,
    KaiOS,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum DeviceType {
    Mobile,
    Tablet,
    Desktop,
    Game,
    TV,
    Smartwatch,
    VRHeadset,
    CarSystem,
    Bot,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAgentInfo {
    pub os: OperatingSystem,
    pub browser: Browser,
    pub device_type: DeviceType,
}

pub struct UserAgentParser;

impl UserAgentParser {
    /// Parses a user agent string and returns detected information
    ///
    /// # Arguments
    ///
    /// * `ua` - The user agent string to parse
    ///
    /// # Returns
    ///
    /// A `UserAgentInfo` struct containing the parsed information
    ///
    /// # Example
    ///
    /// ```
    /// use agent_parser_ro::UserAgentParser;
    ///
    /// let info = UserAgentParser::parse("Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1");
    /// ```
    pub fn parse(ua: &str) -> UserAgentInfo {
        lazy_static! {
            // Updated OS regex to better handle Android and other mobile OS patterns
            static ref OS_REGEX: [Regex; 2] =[
                Regex::new(
                r"(?i)(windows phone|mac os x|iphone os|ipad; cpu os|android|ubuntu|fedora|debian|cros|crkey|chrome os|blackberry|symbian|webos|bada|tizen|nintendo|playstation|xbox|wii|freebsd|openbsd|solaris|aix|hp-ux|harmonyos|kaios)"
            ).unwrap(),
                Regex::new(
                 r"(?i)(windows|linux)"
            ).unwrap()
            ];

            static ref BROWSER_REGEX: [Regex; 2] = [
                Regex::new(
                 r"(?i)(ucbrowser|samsungbrowser|oculusbrowser|ucweb|crios|headlesschrome|mobile safari|fxios|edge|edg|edga|edgios|msie|trident|opera|opr|dolphin|brave|puffin|maxthon|mercury|nokiabrowser|silk|vivaldi|yabrowser|duckduckgo|tor|electron|phantomjs|wv|fban|fbav|instagram|twitter|snapchat|googlebot|bingbot|yahoo! slurp|baiduspider)"
            ).unwrap(),
                Regex::new(
                 r"(?i)(chrome|safari|firefox)"
            ).unwrap()];

            static ref DEVICE_REGEX: [Regex; 2] =[
                 Regex::new(
                r"(?i)(kfmawi|ipod|windows phone|blackberry|symbian|ipad|tablet|kindle|playbook|nexus|sm-t|sm-x|sm-s|gt-p|playstation|ps4|ps5|xbox|nintendo|wii|smart-tv|tv|appletv|roku|chromecast|crkey|fire tv|watch|apple watch|vive|oculus|tesla|android auto|carplay|googlebot|bingbot|slurp|baiduspider|facebookexternalhit|twitterbot|monitoring|scraper|yandexbot)"
            ).unwrap(),
                Regex::new(
                r"(?i)(android|iphone|x11|x86_64)"
            ).unwrap()
            ];
        }

        // Default values
        let mut os = OperatingSystem::Unknown;
        let mut browser = Browser::Unknown;
        let mut device_type = DeviceType::Unknown;
        // Detect OS - now handles Android better
        for reg in OS_REGEX.iter() {
            if let Some(caps) = reg.captures(ua) {
                let matched_os = caps.get(1).unwrap().as_str().to_lowercase();
                os = match matched_os.as_str() {
                    "windows" => OperatingSystem::Windows,
                    "windows phone" => OperatingSystem::WindowsPhone,
                    "mac os x" => OperatingSystem::MacOS,
                    "iphone os" => OperatingSystem::IOS,
                    "ipad; cpu os" => OperatingSystem::IPadOS,
                    "android" => OperatingSystem::Android,
                    "linux" => OperatingSystem::Linux,
                    "ubuntu" => OperatingSystem::Ubuntu,
                    "fedora" => OperatingSystem::Fedora,
                    "debian" => OperatingSystem::Debian,
                    "chrome os" | "cros" | "crkey" => OperatingSystem::ChromeOS,
                    "blackberry" => OperatingSystem::BlackBerry,
                    "symbian" => OperatingSystem::Symbian,
                    "webos" => OperatingSystem::WebOS,
                    "bada" => OperatingSystem::Bada,
                    "tizen" => OperatingSystem::Tizen,
                    "nintendo" => OperatingSystem::Nintendo,
                    "playstation" => OperatingSystem::PlayStation,
                    "xbox" => OperatingSystem::Xbox,
                    "wii" => OperatingSystem::Wii,
                    "freebsd" => OperatingSystem::FreeBSD,
                    "openbsd" => OperatingSystem::OpenBSD,
                    "solaris" => OperatingSystem::Solaris,
                    "aix" => OperatingSystem::AIX,
                    "hp-ux" => OperatingSystem::HPUX,
                    "harmonyos" => OperatingSystem::HarmonyOS,
                    "kaios" => OperatingSystem::KaiOS,
                    _ => OperatingSystem::Unknown,
                };
            }
            if os != OperatingSystem::Unknown {
                break;
            }
        }

        // Detect Browser
        for reg in BROWSER_REGEX.iter() {
            if let Some(caps) = reg.captures(ua) {
                let matched_browser = caps.get(1).unwrap().as_str().to_lowercase();
                browser = match matched_browser.as_str() {
                    "chrome" | "headlesschrome" | "crios" => Browser::Chrome,
                    "safari" | "mobile safari" => Browser::Safari,
                    "firefox" | "fxios" => Browser::Firefox,
                    "edge" | "edg" | "edga" | "edgios" => Browser::Edge,
                    "msie" | "trident" => Browser::InternetExplorer,
                    "opera" | "opr" => Browser::Opera,
                    "ucbrowser" | "ucweb" => Browser::UCBrowser,
                    "samsungbrowser" => Browser::SamsungBrowser,
                    "oculusbrowser" => Browser::OculusBrowser,
                    "dolphin" => Browser::Dolphin,
                    "brave" => Browser::Brave,
                    "puffin" => Browser::Puffin,
                    "maxthon" => Browser::Maxthon,
                    "mercury" => Browser::Mercury,
                    "silk" => Browser::Silk,
                    "vivaldi" => Browser::Vivaldi,
                    "yabrowser" => Browser::Yandex,
                    "duckduckgo" => Browser::DuckDuckGo,
                    "tor" => Browser::Tor,
                    "electron" => Browser::Electron,
                    "phantomjs" => Browser::PhantomJS,
                    "wv" => Browser::WebView,
                    "fban" | "fbav" => Browser::Facebook,
                    "instagram" => Browser::Instagram,
                    "twitter" => Browser::Twitter,
                    "snapchat" => Browser::Snapchat,
                    "googlebot" => Browser::Googlebot,
                    "bingbot" => Browser::Bingbot,
                    "yahoo! slurp" => Browser::Yahoo,
                    "baiduspider" => Browser::Baidu,
                    _ => Browser::Unknown,
                };
                if browser != Browser::Unknown {
                    break;
                }
            }
        }

        for reg in DEVICE_REGEX.iter() {
            if let Some(caps) = reg.captures(ua) {
                let device = caps.get(1).unwrap().as_str().to_lowercase();
                device_type = match device.as_str() {
                    "x11" | "x86_64" => DeviceType::Desktop,
                    "iphone" | "ipod" | "android" | "windows phone" | "blackberry"|"sm-s" | "symbian" => {
                        DeviceType::Mobile
                    }
                    "tablet" | "kindle" | "playbook" | "nexus" | "gt-p" | "sm-t"| "sm-x" | "ipad"|"kfmawi" => {
                        DeviceType::Tablet
                    }
                    "playstation" | "ps4" | "ps5" | "xbox" | "nintendo" | "wii" => DeviceType::Game,
                    "smart-tv" | "tv" | "appletv" | "roku" | "chromecast"| "crkey" | "fire tv" => {
                        DeviceType::TV
                    }
                    "watch" | "apple watch" => DeviceType::Smartwatch,
                    "vive" | "oculus" => DeviceType::VRHeadset,
                    "tesla" | "android auto" | "carplay" => DeviceType::CarSystem,
                    "googlebot"
                    | "bingbot"
                    | "slurp"
                    | "baiduspider"
                    | "facebookexternalhit"
                    | "twitterbot"
                    | "yandexbot"
                    | "monitoring"
                    | "scraper" => DeviceType::Bot,
                    _ => DeviceType::Unknown,
                };
                if device_type != DeviceType::Unknown {
                    break;
                }
            }
        }
        if device_type == DeviceType::Unknown {
            if ua.contains("Windows") || ua.contains("Macintosh") || ua.contains("Linux") {
                device_type = DeviceType::Desktop;
            }
        };
        UserAgentInfo {
            os,
            browser,
            device_type,
        }
    }
}

