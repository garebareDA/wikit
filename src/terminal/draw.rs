extern crate termion;

use super::super::serch_parser::serch;
use std::io::Write;

pub fn serch_draw(stdout: &mut std::io::Stdout, results: &Vec<serch::Serch>,) {
    write!(
        stdout,
        "{}",
        termion::clear::All,
    )
    .unwrap();

    for result in results{
        let title = result.title.clone();
        let text = result.text.clone();

        write!(stdout, "{}\r\n{}...\r\n", title, text);
        write!(stdout, "\r\n");
    }
}