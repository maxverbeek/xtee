use std::process::Stdio;

use tokio::{io::AsyncWriteExt, process::Command};

pub(crate) async fn runcommand(command: &str, stdinbuf: &[u8]) {
    let mut cmd = Command::new(command);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::null());

    let mut child = cmd.spawn().unwrap();

    let mut stdin = child.stdin.take().unwrap();

    stdin.write(stdinbuf).await.unwrap();

    // remove the buffer, which sends an EOF
    drop(stdin);

    child.wait().await.unwrap();
}
