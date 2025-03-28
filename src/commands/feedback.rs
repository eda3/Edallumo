//! `feedback.rs`
//!
//! フィードバック送信モジュール  
//! Discordコマンド /feedback 実装モジュール  
//! ユーザーから受け取ったフィードバックメッセージをファイルに保存し、送信完了を通知する処理を提供

// crate 内の共通型（Context, AppError）をインポート
use crate::{error::AppError, Context};
// colored クレートを利用して、コンソール出力文字列に色を付けるための拡張メソッドを使用
use colored::Colorize;
// ファイル操作および書き込み操作に必要な標準ライブラリのモジュールをインポート
use std::{
    fs::OpenOptions, // ファイルオープンのためのオプション指定をサポート
    io::Write,       // ファイル書き込み操作のためのトレイトをインポート
};

/// フィードバック送信処理
/// 開発者宛フィードバック登録
///
/// ユーザーから受け取ったテキストを 'request.txt' ファイルに追記し、
/// 送信完了メッセージを返す処理。
#[poise::command(prefix_command, slash_command, aliases("r"))]
pub async fn feedback(
    ctx: Context<'_>, // Discord コマンド実行時のコンテキスト（ユーザー情報、チャンネル情報等）を保持
    #[description = "Message for the dev."] text: String, // 開発者へ送るフィードバックのメッセージ
) -> Result<(), AppError> {
    // 正常終了時は Ok(())、エラー時は Error を返す非同期関数
    // ファイルオープン処理
    // 'request.txt' を新規作成（存在しなければ）し、既存の場合は末尾に追記するモードでオープンする
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./request.txt")
        .expect("\nFailed to open 'request.txt' file."); // オープン失敗時はエラーメッセージを出力してパニック

    // テキスト整形処理
    // ユーザーから送られたフィードバックテキストに改行を追加し、各メッセージ間に空行を挿入する
    let new_text = text.clone() + "\n\n";

    // ファイル書き込み処理
    // 整形済みのテキストを 'request.txt' に書き込む
    write!(file, "{new_text}").expect("\nFailed to write to 'request.txt'");

    // ログ出力処理
    // コンソールに書き込み完了のログを出力し、黄色で強調表示する
    println!("{}", "Done writting to 'request.txt'".yellow());

    // 送信メッセージ出力処理
    // Discord のチャネルに成功メッセージを送信する
    ctx.say("Submitted successfully!").await?;

    Ok(()) // 正常終了
}
