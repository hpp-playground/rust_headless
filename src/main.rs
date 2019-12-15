use std::fs;

use failure::Fallible;

use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};

fn main() -> Fallible<()> {
    let options = LaunchOptionsBuilder::default()
        .headless(false)
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://hpprc.com")?
        .wait_until_navigated()?
        .wait_for_element("div.css-rm8i1b")?
        .click()?;
    tab.wait_until_navigated()?
        .wait_for_element("div.css-1cngdp0")?
        .click()?;
    let hpp = tab.wait_until_navigated()?.capture_screenshot(
        ScreenshotFormat::JPEG(Some(75)),
        None,
        true,
    )?;
    fs::write("./screenshots/hpp.jpg", &hpp)?;

    let twitter = tab
        .navigate_to("https://twitter.com/home?lang=ja")?
        .wait_until_navigated()?
        .capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("./screenshots/twitter.jpg", &twitter)?;

    println!("Screenshots successfully created.");
    Ok(())
}
