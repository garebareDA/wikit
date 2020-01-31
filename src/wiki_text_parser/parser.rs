use super::boxs;

pub fn wiki_text(text: &str) {
    let mut text = text.to_string();

    loop{
        if text.len() < 1 {
            break;
        }

        let one_char = text.chars().nth(0).unwrap();

        if one_char == '|'{
            text.remove(0);
            text = boxs::box_view(&text);
            continue;
        }

        text.remove(0);
    }
}