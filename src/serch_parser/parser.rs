pub fn serch_parse(text: &str) -> String {
    let mut parsed_text = String::new();
    let mut text = text.to_string();

    loop {
        if text.len() < 1{
            break;
        }

        let one_char = text.chars().nth(0).unwrap();

        if one_char == '<'{
            loop{
                text.remove(0);
                let next_remove_text = text.chars().nth(0).unwrap();
                if next_remove_text == '>' {
                    text.remove(0);
                    break;
                }
            }
            continue
        }

        if one_char == 'a'{
            let tow_char = text.chars().nth(1).unwrap();
            let three_char = text.chars().nth(2).unwrap();
            let four_char = text.chars().nth(3).unwrap();

            if tow_char == 'm' && three_char == 'p' && four_char == ';'{
                loop{
                    text.remove(0);
                    if(text.chars().nth(0).unwrap() == ';'){
                        text.remove(0);
                        break;
                    }
                }

                continue;
            }
        }

        parsed_text = format!("{}{}", parsed_text, one_char);
        text.remove(0);
    }

    return parsed_text;
}