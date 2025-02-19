use crate::{Context, Error};
use colored::Colorize;
use std::{fs::OpenOptions, io::Write};

/// フィードバック送信処理
/// 開発者宛フィードバック登録
#[poise::command(prefix_command, slash_command, aliases("r"))]
pub async fn feedback(
    ctx: Context<'_>,
    #[description = "Message for the dev."] text: String,
) -> Result<(), Error> {
    // ファイルオープン処理
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("request.txt")
        .expect("\nFailed to open 'request.txt' file.");

    // テキスト整形処理
    let new_text = text.to_owned() + "\n\n";

    // ファイル書き込み処理
    write!(file, "{}", new_text).expect("\nFailed to write to 'request.txt'");

    // ログ出力処理
    println!("{}", "Done writting to 'request.txt'".yellow());
    // 送信メッセージ出力処理
    ctx.say("Submitted successfully!").await?;

    Ok(())
}
