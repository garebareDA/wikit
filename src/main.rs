extern crate ureq;
extern crate xml;

use std::env;

use xml::reader::{EventReader, XmlEvent};

use wikit::serch_parser::{parser, serch::Serch};
use wikit::terminal::serch_view;
use wikit::wiki_text_parser;

fn main() {
    let args:Vec<String> = env::args().collect();
    let command_line = &args[1];

    if command_line == "serch" {
        let command_line_serch = &args[2];
        if command_line_serch.len() < 1 {
            return;
        }

        let uri_template = "https://ja.wikipedia.org/w/api.php?format=xml&action=query&list=search&srsearch=";
        let uri = format!("{}{}", uri_template, command_line_serch);
        let response = get_request(&uri);
        let parser = EventReader::from_str(&response);
        let mut serch_result:Vec<Serch> = Vec::new();

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                    if name.to_string() == "p"{
                        let title = &attributes[1].value;
                        let value = &attributes[5].value;
                        let text = parser::serch_parse(value);

                        let result = Serch::new(title, &text);
                        serch_result.push(result);
                    }
                }

                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        serch_view::views(&serch_result);
    }else if command_line == "open"{
        let command_line_open = &args[2];
        if command_line_open.len() < 1 {
            return;
        }

        let mut  wiki_text = String::new();
        let uri_template = "http://ja.wikipedia.org/w/api.php?format=xml&action=query&prop=revisions&titles=";
        let uri = format!("{}{}&rvprop=content", uri_template, command_line_open);
        let response = get_request(&uri);
        let parser = EventReader::from_str(&response);
        for e in parser {
            match e {
                Ok(XmlEvent::Characters(chars)) => {
                    wiki_text = chars;
                }

                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }

                _ => {}
            }
        }

        wiki_text_parser::parser::wiki_text(&wiki_text);
    }else{
        println!("<command> serch [word] or open [word]");
    }
}

fn get_request(uri: &str) -> String {
    let response = ureq::get(uri).call();
    if response.ok() {
        let xml = response.into_string().unwrap();
        return xml;
    }else{
        println!("error");
        println!("レスポンスが有りません");
        return "".to_string();
    }
}
