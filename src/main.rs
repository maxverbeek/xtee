use std::io::{stdin, stdout, BufReader};

use clap::Parser;
use filterer::filter;
use tokio::sync::mpsc;

mod executor;
mod filterer;

#[derive(Parser, Debug)]
#[command(name = "xtee")]
#[command(
    about = "Filter STDIN for text that matches a regular expression, and pass this matched input as STDIN to an executable."
)]
struct Args {
    #[arg(
        short,
        long,
        help = "(Can apear multiple times) execute these with the contents of the regex"
    )]
    exec: Vec<String>,

    #[arg(short, long, help = "Match this regex on the STDIN")]
    pattern: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Args::parse();
    let (tx, mut rx) = mpsc::channel(2);

    let bufstdin = BufReader::new(stdin());
    let output = stdout();

    tokio::spawn(async move { filter(bufstdin, output, &cli.pattern, tx).await });

    while let Some(msg) = rx.recv().await {
        println!("received msg: {}", msg)
    }

    println!("hello");

    Ok(())
}
