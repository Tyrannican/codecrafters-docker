use crate::image::ImageService;
use std::io::Write;
use std::os::unix::fs;

use anyhow::{Context, Result};
use tempfile::TempDir;

fn setup_chroot(dir: &str, image: &str, command: &str) -> Result<()> {
    let tmp_dir = TempDir::new()
        .context("creating temporary dir")?
        .into_path();

    let root = tmp_dir.join(dir);
    let dev = root.join("dev");
    let bin = root.join("usr/local/bin");
    let proc = root.join("proc");

    std::fs::create_dir_all(&root).context("creating root directory")?;
    std::fs::create_dir_all(&dev).context("creating dev directory")?;
    std::fs::create_dir_all(&bin).context("creating bin directory")?;
    std::fs::create_dir_all(proc).context("creating proc directory")?;

    let Some(bin_name) = command.split('/').last() else {
        anyhow::bail!("need a binary name");
    };
    let mut null = std::fs::File::create(dev.join("null")).context("creating /dev/null")?;
    null.write_all(b"nothing").context("filling /dev/null")?;

    std::fs::copy(command, bin.join(bin_name)).context("copying binary over")?;

    let is = ImageService::new(image);
    is.download_image(&root)
        .context("attempting to download image")?;
    fs::chroot(&root).context("chrooting")?;
    std::env::set_current_dir("/").context("setting current directory to chroot")?;

    // Safety: Executed in isolated env...?
    // Tbh, i haven't a clue how this works but it does
    unsafe {
        libc::unshare(libc::CLONE_NEWPID);
    }

    Ok(())
}

pub(crate) fn run_command(image: &str, command: &str, command_args: &[String]) -> Result<()> {
    setup_chroot("dockersandbox", image, command)?;
    let exec = std::process::Command::new(command)
        .args(command_args)
        .output()
        .with_context(|| {
            format!(
                "Running command {:?} with arguments {:?}",
                &command, &command_args
            )
        })?;

    let status = exec.status;
    let exit_code = status.code().unwrap_or(1);

    let mut stdout = std::io::stdout();
    stdout
        .write_all(&exec.stdout)
        .context("command stdout response")?;

    let mut stderr = std::io::stderr();
    stderr
        .write(&exec.stderr)
        .context("command stderr output")?;

    std::process::exit(exit_code);
}
