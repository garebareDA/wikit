extern crate ureq;
extern crate xml;

use std::env;

use xml::reader::{EventReader, XmlEvent};

use wikit::serch_parser;

fn main() {
    let args:Vec<String> = env::args().collect();
    let command_line = &args[1];

    if command_line == "serch" {
        let command_line_serch = &args[2];
        let uri_template = "https://ja.wikipedia.org/w/api.php?format=xml&action=query&list=search&srsearch=";
        let uri = format!("{}{}", uri_template, command_line_serch);
        let response = get_request(&uri);
        let parser = EventReader::from_str(&response);

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                    if name.to_string() == "p"{
                        let title = &attributes[1].value;
                        let value = &attributes[5].value;
                        let text = serch_parser::parser::serch_parse(value);

                    }
                }

                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }else if command_line == "open"{

    }else{

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
