use failure::Fallible;
use std::{env, io, thread, time};

use headless_chrome::{Browser, LaunchOptionsBuilder};

fn main() -> Fallible<()> {
    let timeout = time::Duration::new(1000, 0);
    let seconds = time::Duration::new(1, 0);
    let longtime = time::Duration::new(3, 0);

    let options = LaunchOptionsBuilder::default()
        .headless(false)
        .idle_browser_timeout(timeout)
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;

    let username = "hpp.ricecake@gmail.com";
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

    tab.wait_for_element("#react-root > div > div > div > header > div > div > div > div > div > nav > a:nth-child(2)")?.click()?;
    tab.wait_for_element("#react-root > div > div > div > main > div > div > div > div > div > div > div > div > div > div > div > div > div > div > div > div > form > div > div > div > div > input")?.click()?;
    tab.type_str("from:osaremochi since:2018-10-1 until:2019-2-31")?;
    tab.press_key("Enter")?;

    thread::sleep(seconds);
    tab.wait_for_element("div[role='tablist'] > div:nth-child(3)")?
        .click()?;
    thread::sleep(seconds);

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s);
        tab.wait_for_element("div[aria-label='もっと見る']")?
            .click()?;
        tab.wait_for_element("div[role='menu'] > div > div > div > div:nth-child(1)")?
            .click()?;
        tab.wait_for_element("div[data-testid='confirmationSheetConfirm']")?
            .click()?;
    }
}
