extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

pub fn views() {
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    write!(
        stdout,
        "{}",
        termion::clear::All,
    )
    .unwrap();
    stdout.flush().unwrap();

    for evt in stdin.events() {
        if evt.unwrap() == Event::Key(Key::Ctrl('c')) {
            return;
        }
    }
}