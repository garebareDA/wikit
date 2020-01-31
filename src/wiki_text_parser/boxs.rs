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
            text.remove(0);

            for i in text.as_str().chars(){
                if i == '>'{
                    break;
                }

                tag_name = format!("{}{}", tag_name, i);
            }

            if tag_name.starts_with("ref") {
                println!("\n");
            }

            if tag_name.starts_with("br") {
                loop{
                    let is_close = text.chars().nth(0).unwrap();
                    if is_close == '>'{
                        text.remove(0);
                        println!("\n");
                        break;
                    }
                    text.remove(0);
                }
            }

            continue;
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