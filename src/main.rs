use clap::Parser;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// String to dookize
    #[clap(short, long, value_parser)]
    sentence: String,
}

fn dookize(s: &str) -> String {
    let mut new_str = "".to_owned();
    for g in s.graphemes(true) {
        if g == "o" {
            new_str.push_str("oo")
        } else {
            new_str.push_str(g)
        }
    }
    new_str
}

fn main() {
    let args = Args::parse();
    println!("{}", &dookize(&args.sentence))
}
