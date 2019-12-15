use failure::Fallible;
use std::{env, fs, thread, time};

use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};

fn main() -> Fallible<()> {
    let options = LaunchOptionsBuilder::default()
        .headless(false)
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;

    let username = "osaremochi";
    let password = &env::var("TWITTER_PASSWORD")?;

    tab.navigate_to("https://twitter.com/home?lang=ja")?
        .wait_until_navigated()?
        .wait_for_element(".js-username-field.email-input.js-initial-focus")?
        .click()?;
    tab.type_str(username)?
        .wait_for_element(".js-password-field")?
        .click()?;
    tab.type_str(password)?
        .wait_for_element("button.submit.EdgeButton.EdgeButton--primary.EdgeButtom--medium")?
        .click()?;

    let five_seconds = time::Duration::new(5, 0);
    thread::sleep(five_seconds);

    let timeline = tab.capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("./screenshots/timeline.jpg", &timeline)?;

    println!("Screenshots successfully created.");
    Ok(())
}
