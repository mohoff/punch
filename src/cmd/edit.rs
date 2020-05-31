use std::process::{Command, ExitStatus};
use std::env;

use crate::card::Card;
use crate::err::*;

pub fn run() -> Result<ExitStatus> {
    let card: Card = Default::default();

    let env_editor = "EDITOR";
    match env::var_os(env_editor) {
        None => Err(ErrorKind::EnvVarNotFound(env_editor.into()).into()),
        Some(editor) => Command::new(editor)
            .arg(card.path())
            .spawn()
            .chain_err(|| "Failed to open editor")?
            .wait()
            .chain_err(|| "Editor returned non-zero exit code")
    }
}
