use glob::glob;
use pulldown_cmark::{html::push_html, Options, Parser};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

fn main() {
    for input_path in glob("source/**/*.md").unwrap().filter_map(|f| f.ok()) {
        println!("{}", input_path.display());
        let markdown_text = fs::read_to_string(&input_path).unwrap();
        let parser = Parser::new_ext(&markdown_text, Options::all());

        let mut html_text = String::new();
        push_html(&mut html_text, parser);

        let output_path = {
            let dist_path = Path::new("dist");
            let post_path = input_path.strip_prefix("source").unwrap();
            dist_path.join(post_path)
        };
        fs::create_dir_all(output_path.parent().unwrap()).unwrap();

        println!("{}", output_path.display());
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(output_path)
            .unwrap()
            .write_all(html_text.as_bytes())
            .unwrap()
    }
}
