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

pub fn draw_wikit(stdout: &mut std::io::Stdout, results: &mut Vec<String>){
    write!(
        stdout,
        "{}",
        termion::clear::All,
    ).unwrap();

    for result in results{
        write!(stdout, "{}\r\n", result);
        write!(stdout, "\r\n");
    }
}