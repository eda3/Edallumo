//! # register.rs
//!  
//! アプリケーションコマンド登録・解除モジュール  
//! Discordコマンド /register 実装モジュール  
//! ギルドまたはグローバルでアプリケーションコマンドを登録・解除する処理を提供  
//! このコマンドは、Botの所有者のみ実行可能

use crate::{error::Result, Context};

/// アプリケーションコマンドの登録・解除処理  
#[poise::command(prefix_command, slash_command, hide_in_help, owners_only)]
pub async fn register(ctx: Context<'_>) -> Result<()> {
    // アプリケーションコマンドの登録・解除用ボタンを表示する処理
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
