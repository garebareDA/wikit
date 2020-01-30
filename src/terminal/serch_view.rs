extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use super::super::serch_parser::serch;
use super::draw;

pub fn views(result:&Vec<serch::Serch>) {
    let stdin = stdin();
    let mut cols = 0;
    let mut stdout = stdout();

    write!(
        stdout,
        "{}",
        termion::clear::All,
    )
    .unwrap();
    stdout.flush().unwrap();

    draw::serch_view(&mut stdout ,result, cols);

    for evt in stdin.events() {
        match evt.unwrap(){
            Event::Key(Key::Ctrl('c')) => {
                return;
            }

            Event::Key(Key::Up) => {
                cols += 1;
            }

            Event::Key(Key::Down) => {
                cols -= 1;
            }

            _ => {}
        }

        draw::serch_view(&mut stdout ,result, cols);
        stdout.flush().unwrap();
    }
}