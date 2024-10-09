use anyhow::Error;
use vergen::*;

fn main() -> Result<(), Error> {
    let build = BuildBuilder::all_build()?;
    let cargo = CargoBuilder::all_cargo()?;
    let rustc = RustcBuilder::all_rustc()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .emit()?;

    let git_commit_id = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .output()?;
    let git_commit_date = std::process::Command::new("git")
        .arg("show")
        .arg("-s")
        .arg("--format=%ci")
        .output()?;
    let git_commit_message = std::process::Command::new("git")
        .arg("show")
        .arg("-s")
        .arg("--format=%s")
        .output()?;
    let git_commit_author_name = std::process::Command::new("git")
        .arg("show")
        .arg("-s")
        .arg("--format=%an")
        .output()?;
    let git_commit_author_email = std::process::Command::new("git")
        .arg("show")
        .arg("-s")
        .arg("--format=%ae")
        .output()?;

    println!(
        "cargo:rustc-env=GIT_COMMIT_ID={}",
        String::from_utf8(git_commit_id.stdout)?
    );
    println!(
        "cargo:rustc-env=GIT_COMMIT_DATE={}",
        String::from_utf8(git_commit_date.stdout)?
    );
    println!(
        "cargo:rustc-env=GIT_COMMIT_MSG={}",
        String::from_utf8(git_commit_message.stdout)?
    );
    println!(
        "cargo:rustc-env=GIT_COMMIT_AUTHOR_NAME={}",
        String::from_utf8(git_commit_author_name.stdout)?
    );
    println!(
        "cargo:rustc-env=GIT_COMMIT_AUTHOR_EMAIL={}",
        String::from_utf8(git_commit_author_email.stdout)?
    );

    Ok(())
}
