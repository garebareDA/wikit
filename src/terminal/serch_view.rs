extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;

use super::super::serch_parser::serch;
use super::draw;

pub fn views(result:&Vec<serch::Serch>) {
    let stdin = stdin();
    let mut stdout = stdout();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    draw::serch_draw(&mut stdout ,result);

    for evt in stdin.events() {
        match evt.unwrap(){
            Event::Key(Key::Ctrl('c')) => {
                return;
            }

            _ => {}
        }
    }
}