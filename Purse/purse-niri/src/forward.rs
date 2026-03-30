use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;

use crate::resolution::purse_socket_path;

pub fn forward(pid: u32, paths: &[PathBuf]) -> anyhow::Result<()> {
    let socket_path = purse_socket_path(pid);
    let mut stream = std::os::unix::net::UnixStream::connect(&socket_path)
        .with_context(|| format!("failed to connect to purse socket: {socket_path:?}"))?;

    for path in paths {
        writeln!(stream, "{}", path.display())
            .context("failed to write path to purse socket")?;
    }

    stream.flush()?;
    Ok(())
}
