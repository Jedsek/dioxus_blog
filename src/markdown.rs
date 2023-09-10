use pulldown_cmark::{html::push_html, Options, Parser};
use std::{fs, path::Path};

pub fn _to_html<T: AsRef<Path>>(markdown_path: T) -> String {
    let markdown_path: &Path = markdown_path.as_ref();
    let markdown_text = fs::read_to_string(markdown_path).unwrap();
    let parser = Parser::new_ext(&markdown_text, Options::all());

    let mut html_text = String::new();
    push_html(&mut html_text, parser);

    html_text
}
