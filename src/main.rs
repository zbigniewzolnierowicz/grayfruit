use std::path::PathBuf;

use clap::Parser;
use lopdf::Document;

#[derive(Parser)]
struct Cli {
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let doc = Document::load(cli.file).unwrap();
    let pages = doc.get_pages();
    let pages_keys = pages.keys();
    let page_numbers: Vec<u32> = pages_keys.into_iter().copied().collect();

    let contents = doc.extract_text(&page_numbers).unwrap();

    let poem: Vec<String> = contents
        .lines()
        .map(|line| line.split_whitespace())
        .map(|line| {
            line.map(|word| word.chars().next().unwrap())
                .collect::<String>()
        })
        .collect();

    println!("{}", poem.join("\n"));
}
