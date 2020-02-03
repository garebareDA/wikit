use super::boxs;

pub fn wiki_text(text: &str) -> Vec<String> {
    let mut text = text.to_string();
    let mut text_results:Vec<String> = Vec::new();

    loop{
        if text.len() < 1 {
            return text_results;
        }

        let one_char = text.chars().nth(0).unwrap();

        if one_char == '|'{
            text.remove(0);
            let (text_continue, text_result) = boxs::box_view(&text);
            text = text_continue;
            text_results.push(text_result);
            continue;
        }

        text.remove(0);
    }
}