mod advanced;
mod meter;
mod simple;
use crate::{Context, Error};
use advanced::advanced;
use meter::meter;
use simple::simple;
use std::string::String;

/// Display a move's frame data.
#[poise::command(
    prefix_command,
    slash_command,
    subcommands("simple", "advanced", "meter"),
    subcommand_required
)]
pub async fn frames(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
