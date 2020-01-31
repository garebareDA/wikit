pub fn box_view(text: &str) -> String {
    let mut text = text.to_string();
    let mut text_result = String::new();

    loop{
        if text.len() < 1 {
            return "".to_string();
        }

        let one_char = text.chars().nth(0).unwrap();

        if one_char == '{' || one_char == '}'{
            text.remove(0);
            continue;
        }


        if one_char == '[' || one_char == ']' {
            text.remove(0);
            continue;
        }

        if one_char == '<'{
            let mut tag_name = String::new();
            loop {
                let tag = text.chars().nth(0).unwrap();
                if tag == '>'{
                    text.remove(0);
                    break;
                }

                tag_name = format!("{}{}", tag_name, tag);
                text.remove(0);
            }
            tag_name.retain(|c| c != ' ');

            if tag_name == "ref"{
                loop {
                    let is_tag = text.chars().nth(0).unwrap();
                    let is_close = text.chars().nth(1).unwrap();
                    if is_tag == '<' && is_close =='/'{
                        loop {
                            text.remove(0);
                            if text.chars().nth(0).unwrap() == '>'{
                                text.remove(0);
                                break;
                            }
                        }
                    }
                    text.remove(0);
                }
            }

            if tag_name == "br/"{
                println!("");
            }
        }

        if one_char == '|'{
            if text.len() < 0 {
                return text;
            }
            println!("{}", text_result);
            return text;
        }

        text_result = format!("{}{}", text_result, one_char);
        text.remove(0);
    }
}