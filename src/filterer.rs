use std::io::{BufRead, Write};

use regex::Regex;
use tokio::sync::mpsc::Sender;

pub(crate) async fn filter(
    buf: impl BufRead,
    mut out: impl Write,
    pattern: &str,
    tx: Sender<String>,
) {
    let regex = Regex::new(pattern).unwrap();

    for line in buf.lines() {
        match line {
            Ok(line) => {
                writeln!(out, "{}", line).unwrap();
                out.flush().unwrap();

                let mut matches = regex.find_iter(&line);

                while let Some(m) = matches.next() {
                    tx.send(m.as_str().to_owned()).await.unwrap();
                }
            }
            Err(err) => eprintln!("err: {}", err),
        };
    }
}

mod test {
    #[tokio::test]
    async fn output_is_input() {
        let bytes = r"";
    }
}
