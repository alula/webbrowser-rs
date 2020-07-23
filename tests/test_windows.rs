#[cfg(target_os = "windows")]
mod common;
#[cfg(target_os = "windows")]
use common::*;

#[cfg(all(unix, not(target_os = "macos")))]
mod tests {
    const TEST_PLATFORM: &str = "windows";

    use super::check_browser;
    use webbrowser::Browser;

    #[actix_rt::test]
    async fn test_open_default() {
        check_browser(Browser::Default, TEST_PLATFORM).await;
    }

    #[actix_rt::test]
    #[ignore]
    async fn test_open_internet_explorer() {
        check_browser(Browser::InternetExplorer, TEST_PLATFORM).await;
    }
}
