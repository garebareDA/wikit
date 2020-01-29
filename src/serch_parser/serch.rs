pub struct Serch {
    pub title: String,
    pub text: String,
}

impl Serch {
    pub fn new(title: &str, text: &str) -> Serch {
        let title = title.to_string();
        let text = text.to_string();

        Serch {title, text}
    }
}