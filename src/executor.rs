use std::process::Stdio;

use tokio::{io::AsyncWriteExt, process::Command};

fn make_command(command: &str) -> Command {
    let mut words = command.split(' ');
    let mut cmd = Command::new(words.next().unwrap());

    while let Some(arg) = words.next() {
        cmd.arg(arg);
    }

    cmd
}

pub(crate) async fn runcommand(command: &str, stdinbuf: &[u8], asarg: bool) {
    let mut cmd = make_command(command);

    if asarg {
        cmd.arg(String::from_utf8(stdinbuf.to_vec()).unwrap());
        cmd.stdin(Stdio::null());
    } else {
        cmd.stdin(Stdio::piped());
    }

    cmd.stdout(Stdio::null());

    let mut child = cmd.spawn().unwrap();

    if !asarg {
        let mut stdin = child.stdin.take().unwrap();

        stdin.write(stdinbuf).await.unwrap();

        // remove the buffer, which sends an EOF
        drop(stdin);
    }

    child.wait().await.unwrap();
}
