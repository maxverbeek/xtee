use std::io::{BufRead, Write};

use regex::Regex;
use tokio::sync::mpsc::Sender;

pub(crate) async fn filter(
    mut buf: impl BufRead,
    mut out: impl Write,
    pattern: &str,
    tx: Sender<String>,
) {
    let regex = Regex::new(pattern).unwrap();
    let mut linebuf = vec![];

    while buf.read_until(b'\n', &mut linebuf).unwrap() != 0 {
        out.write(&linebuf).unwrap();
        out.flush().unwrap();

        let line = String::from_utf8_lossy(&linebuf);
        let mut matches = regex.find_iter(&line);

        while let Some(m) = matches.next() {
            tx.send(m.as_str().to_owned()).await.unwrap();
        }

        linebuf.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn output_is_input() {
        let bytes = r"
Enumerating objects: 12, done.
Counting objects: 100% (12/12), done.
Delta compression using up to 4 threads
Compressing objects: 100% (10/10), done.
Writing objects: 100% (12/12), 6.93 KiB | 1.15 MiB/s, done.
Total 12 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
To github.com:maxverbeek/xtee.git
 * [new branch]      master -> master
branch 'master' set up to track 'origin/master'.
        "
        .trim();

        let mut output = vec![];

        let reader = Cursor::new(bytes);
        let writer = Cursor::new(&mut output);

        let (tx, mut rx) = tokio::sync::mpsc::channel(10);

        // look for 100% in the above output
        filter(reader, writer, "\\d{2,3}%", tx).await;

        assert_eq!(rx.recv().await, Some("100%".to_owned()));
        assert_eq!(rx.recv().await, Some("100%".to_owned()));
        assert_eq!(rx.recv().await, Some("100%".to_owned()));
        assert_eq!(rx.recv().await, None);
        assert_eq!(bytes, String::from_utf8(output).unwrap())
    }
}
