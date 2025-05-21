use agent_parser_ro::{Browser, DeviceType, OperatingSystem, UserAgentParser};

    fn assert_ua(
        ua: &str,
        expected_os: OperatingSystem,
        expected_browser: Browser,
        expected_device: DeviceType,
    ) {
        let result = UserAgentParser::parse(ua);
        assert_eq!(result.os, expected_os, "OS mismatch for UA: {}", ua);
        assert_eq!(
            result.browser, expected_browser,
            "Browser mismatch for UA: {}",
            ua
        );
        assert_eq!(
            result.device_type, expected_device,
            "Device mismatch for UA: {}",
            ua
        );
    }

    // Desktop Browsers
    #[test]
    fn test_desktop_browsers() {
        // Windows
        assert_ua(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            OperatingSystem::Windows,
            Browser::Chrome,
            DeviceType::Desktop,
        );

        assert_ua(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/115.0",
            OperatingSystem::Windows,
            Browser::Firefox,
            DeviceType::Desktop,
        );

        assert_ua(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
            OperatingSystem::Windows,
            Browser::Edge,
            DeviceType::Desktop,
        );

        // macOS
        assert_ua(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15",
            OperatingSystem::MacOS,
            Browser::Safari,
            DeviceType::Desktop,
        );

        // Linux
        assert_ua(
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            OperatingSystem::Linux,
            Browser::Chrome,
            DeviceType::Desktop,
        );

        assert_ua(
            "Mozilla/5.0 (X11; Linux i686; rv:109.0) Gecko/20100101 Firefox/115.0",
            OperatingSystem::Linux,
            Browser::Firefox,
            DeviceType::Desktop,
        );
    }

    // Mobile Browsers
    #[test]
    fn test_mobile_browsers() {
        // Android

        assert_ua(
            "Mozilla/5.0 (Android 13; Mobile; rv:109.0) Gecko/109.0 Firefox/115.0",
            OperatingSystem::Android,
            Browser::Firefox,
            DeviceType::Mobile,
        );

        assert_ua(
            "Mozilla/5.0 (Linux; Android 13; SM-A536B) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/21.0 Chrome/110.0.5481.154 Mobile Safari/537.36",
            OperatingSystem::Android,
            Browser::SamsungBrowser,
            DeviceType::Mobile,
        );

        assert_ua(
            "Mozilla/5.0 (Linux; U; Android 10; en-US; RMX2061 Build/QKQ1.200428.002) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 UCBrowser/13.0.0.1308 Mobile Safari/537.36",
            OperatingSystem::Android,
            Browser::UCBrowser,
            DeviceType::Mobile,
        );

        // iOS
        assert_ua(
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Mobile/15E148 Safari/604.1",
            OperatingSystem::IOS,
            Browser::Safari,
            DeviceType::Mobile,
        );

        assert_ua(
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/120.0.6099.119 Mobile/15E148 Safari/604.1",
            OperatingSystem::IOS,
            Browser::Chrome,
            DeviceType::Mobile,
        );

        assert_ua(
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/115.0 Mobile/15E148 Safari/605.1.15",
            OperatingSystem::IOS,
            Browser::Firefox,
            DeviceType::Mobile,
        );
    }

    // Tablet Browsers
    #[test]
    fn test_tablet_browsers() {
        // iPad
        assert_ua(
            "Mozilla/5.0 (iPad; CPU OS 16_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Mobile/15E148 Safari/604.1",
            OperatingSystem::IPadOS,
            Browser::Safari,
            DeviceType::Tablet,
        );

        // Android Tablets
        assert_ua(
            "Mozilla/5.0 (Linux; Android 13; SM-X700) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.6099.43 Safari/537.36",
            OperatingSystem::Android,
            Browser::Chrome,
            DeviceType::Tablet,
        );

        assert_ua(
            "Mozilla/5.0 (Linux; Android 13; SM-T870) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/21.0 Chrome/110.0.5481.154 Safari/537.36",
            OperatingSystem::Android,
            Browser::SamsungBrowser,
            DeviceType::Tablet,
        );

        // Kindle Fire
        assert_ua(
            "Mozilla/5.0 (Linux; Android 9; KFMAWI) AppleWebKit/537.36 (KHTML, like Gecko) Silk/86.3.13 like Chrome/86.0.4240.198 Safari/537.36",
            OperatingSystem::Android,
            Browser::Silk,
            DeviceType::Tablet,
        );
    }

    // Game Consoles
    #[test]
    fn test_game_consoles() {
        assert_ua(
            "Mozilla/5.0 (PlayStation 5 8.00) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Safari/605.1.15",
            OperatingSystem::PlayStation,
            Browser::Safari,
            DeviceType::Game,
        );

        assert_ua(
            "Mozilla/5.0 (Nintendo Switch; WifiWebAuthApplet) AppleWebKit/609.4 (KHTML, like Gecko) NF/6.0.2.20.2 NintendoBrowser/5.1.0.22401",
            OperatingSystem::Nintendo,
            Browser::Unknown,
            DeviceType::Game,
        );

        assert_ua(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; Xbox; Xbox One) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edge/44.18363.8131",
            OperatingSystem::Xbox,
            Browser::Edge,
            DeviceType::Game,
        );
    }

    // Smart TVs and Streaming Devices
    #[test]
    fn test_tv_devices() {
        assert_ua(
            "Mozilla/5.0 (SMART-TV; Linux; Tizen 6.5) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/5.2 Chrome/92.0.4515.166 TV Safari/537.36",
            OperatingSystem::Tizen,
            Browser::SamsungBrowser,
            DeviceType::TV,
        );

        assert_ua(
            "Mozilla/5.0 (X11; Linux armv7l) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.182 Safari/537.36 CrKey/1.54.250320",
            OperatingSystem::ChromeOS,
            Browser::Chrome,
            DeviceType::TV,
        );

        assert_ua(
            "Mozilla/5.0 (DTV) AppleWebKit/531.2 (KHTML, like Gecko) NX/3.0.0.9.12 (PhilipsTV; 65OLED706/12; TPM211CE_R.101.002.178.222;) Capella/1.0 WebKit/531.2",
            OperatingSystem::Unknown,
            Browser::Unknown,
            DeviceType::TV,
        );
    }

    // Bots and Crawlers
    #[test]
    fn test_bots() {
        assert_ua(
            "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
            OperatingSystem::Unknown,
            Browser::Googlebot,
            DeviceType::Bot,
        );

        assert_ua(
            "Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)",
            OperatingSystem::Unknown,
            Browser::Bingbot,
            DeviceType::Bot,
        );

        assert_ua(
            "Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com/bots)",
            OperatingSystem::Unknown,
            Browser::Unknown,
            DeviceType::Bot,
        );
    }

    // Special Cases
    #[test]
    fn test_special_cases() {
        // Facebook in-app browser
        assert_ua(
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 [FBAN/FBIOS;FBDV/iPhone14,3;FBMD/iPhone;FBSN/iOS;FBSV/16.6;FBSS/3;FBID/phone;FBLC/en_US;FBOP/5]",
            OperatingSystem::IOS,
            Browser::Facebook,
            DeviceType::Mobile,
        );

        // Electron Apps
        assert_ua(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) myapp/1.0.0 Chrome/120.0.6099.109 Electron/28.1.0 Safari/537.36",
            OperatingSystem::Windows,
            Browser::Electron,
            DeviceType::Desktop,
        );

        // Tesla Browser
        assert_ua(
            "Mozilla/5.0 (X11; GNU/Linux) AppleWebKit/537.36 (KHTML, like Gecko) Tesla/2023.44.30.1 Chrome/120.0.6099.109 Safari/537.36",
            OperatingSystem::Linux,
            Browser::Chrome,
            DeviceType::CarSystem,
        );

        // VR Headsets
        assert_ua(
            "Mozilla/5.0 (Linux; Android 10; Quest 2) AppleWebKit/537.36 (KHTML, like Gecko) OculusBrowser/20.0.0.6.74.348674094 SamsungBrowser/4.0 Chrome/99.0.4844.88 Mobile VR Safari/537.36",
            OperatingSystem::Android,
            Browser::OculusBrowser,
            DeviceType::VRHeadset,
        );
    }

    // Edge Cases
    #[test]
    fn test_edge_cases() {
        // Empty User Agent
        assert_ua(
            "",
            OperatingSystem::Unknown,
            Browser::Unknown,
            DeviceType::Unknown,
        );

        // Malformed User Agent
        assert_ua(
            "This is not a real user agent",
            OperatingSystem::Unknown,
            Browser::Unknown,
            DeviceType::Unknown,
        );

        // Legacy Internet Explorer
        assert_ua(
            "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; AS; rv:11.0) like Gecko",
            OperatingSystem::Windows,
            Browser::InternetExplorer,
            DeviceType::Desktop,
        );

        // Opera Mini
        assert_ua(
            "Opera/9.80 (Android; Opera Mini/8.0.1807/36.1609; U; en) Presto/2.12.423 Version/12.16",
            OperatingSystem::Android,
            Browser::Opera,
            DeviceType::Mobile,
        );
    }
