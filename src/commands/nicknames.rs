//! # nicknames.rs
//!
//! キャラクターのニックネーム一覧表示モジュール  
//! Discordコマンド /nicknames 実装モジュール  
//! JSONファイルからキャラクターごとのニックネームを取得し、整形後に送信する処理を提供

use crate::{check, error::AppError, Context, Nicknames};
use std::{fs, string::String};

/// キャラクターごとのニックネーム一覧を表示する処理  
#[poise::command(prefix_command, slash_command, aliases("n"))]
pub async fn nicknames(ctx: Context<'_>) -> Result<(), AppError> {
    // 入力検証および必要なファイルの存在確認処理
    // adaptive_check: ファイルの整合性確認（失敗時は処理を中断）
    if (check::adaptive_check(
        ctx,
        check::CheckOptions::CHARACTER_FOLDERS | check::CheckOptions::CHARACTER_JSONS,
    )
    .await)
        .is_err()
    {
        return Ok(());
    }

    // ニックネーム情報JSONファイルの読み込み処理
    let data_from_file =
        fs::read_to_string("data/nicknames.json").expect("\nFailed to read 'nicknames.json' file.");

    // JSON文字列を Nicknames 型の Vec にデシリアライズ
    let vec_nicknames = serde_json::from_str::<Vec<Nicknames>>(&data_from_file).unwrap();

    // Discord 送信用のメッセージ文字列の初期化
    let mut nicks_as_msg = "__**Character Nicknames**__\n```diff".to_string();

    // ニックネーム情報を整形し、メッセージ文字列に追加
    for nicknames in vec_nicknames {
        // キャラクター名の追加
        nicks_as_msg = nicks_as_msg.clone() + "\n* Character: " + &nicknames.character.to_string();

        // ニックネームの追加
        nicks_as_msg += "\n+ Nicknames: ";

        for x in 0..nicknames.nicknames.len() {
            if x == nicknames.nicknames.len() - 1 {
                nicks_as_msg.push_str(&nicknames.nicknames[x]);
            } else {
                // 空のニックネームが含まれている場合を考慮し、カンマ区切りで追加
                if nicknames.nicknames[x].is_empty() {
                    nicks_as_msg.push_str(&nicknames.nicknames[x]);
                } else {
                    nicks_as_msg.push_str(&nicknames.nicknames[x]);
                    nicks_as_msg.push_str(", ");
                }
            }
        }
        // 各キャラクターのニックネーム情報の終了マーク
        nicks_as_msg = nicks_as_msg.clone() + ".\n";
    }

    // メッセージ終端のマーク
    nicks_as_msg += "```";

    // Discordへメッセージを送信
    ctx.say(&nicks_as_msg).await?;

    Ok(())
}
