extern crate ureq;
extern crate xml;

use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    let command_line = &args[1];

    if command_line == "serch" {
        let command_line_serch = &args[2];
        let uri_template = "https://ja.wikipedia.org/w/api.php?format=xml&action=query&list=search&srsearch=";
        let uri = format!("{}{}", uri_template, command_line_serch);
        let response = get_request(&uri);
        println!("{}", response);
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
