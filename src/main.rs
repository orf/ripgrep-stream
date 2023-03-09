use clap::{Parser};
use grep::cli;
use grep::printer::{ColorSpecs, StandardBuilder};
use grep::regex::RegexMatcherBuilder;
use grep::searcher::{BinaryDetection, SearcherBuilder};
use serde_json::Value;
use std::io;
use std::io::BufRead;
use termcolor::ColorChoice;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    pattern: String,
    // #[command(subcommand)]
    // format: Format,
}

// #[derive(Subcommand, Debug)]
// enum Format {
//     Lines {},
// }

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let matcher = RegexMatcherBuilder::new()
        .multi_line(true)
        .build(&args.pattern)?;
    let mut searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .line_number(false)
        .multi_line(true)
        .build();

    let mut printer = StandardBuilder::new()
        .color_specs(ColorSpecs::default_with_color())
        .build(cli::stdout(if cli::is_tty_stdout() {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        }));

    // let mut printer = JSONBuilder::new()
    //     .pretty(false)
    //     .always_begin_end(false)
    //     .build(cli::stdout(ColorChoice::Never));

    for line in io::stdin().lock().lines().flatten() {
        // Lines are like so:
        // [name] [json encoded line]
        let (name, rest) = line.split_once(' ').unwrap();
        let item: Value = serde_json::from_reader(rest.as_bytes())?;

        if let Value::String(s) = item {
            searcher.search_reader(
                &matcher,
                s.as_bytes(),
                printer.sink_with_path(&matcher, name),
            )?;
        }
    }

    Ok(())
}
