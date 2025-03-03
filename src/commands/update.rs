//! update.rs
//!
//! このファイルは、Dustloop Wiki のデータ更新機能を提供する。
//! フレームデータ、画像データの更新処理を実行する関数群を実装する。
//! オーナー専用コマンドとして利用可能。

mod framedata; // framedata.rs モジュール　フレームデータ更新処理群
mod framedata_json; // framedata_json.rs モジュール　フレームデータJSON変換処理群
mod images; // images.rs モジュール　画像データ更新処理群
mod images_json; // images_json.rs モジュール　画像データJSON変換処理群

use crate::{check, error::Result, find, Context, CHARS}; // 共通チェック関数、検索関数、型定義群
use colored::Colorize; // 文字色変換機能

/// 更新対象選択列挙体
#[derive(Debug, poise::ChoiceParameter)]
pub enum UpdateChoice {
    #[name = "all"]
    All, // 全更新選択
    #[name = "frames"]
    Frames, // フレーム更新選択
    #[name = "images"]
    Images, // 画像更新選択
}

/// アップデートコマンド本体  
/// ロール名：BotOwner が必須
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn update(ctx: Context<'_>) -> Result<()> {
    // 入力チェック実施　条件確認
    if (check::adaptive_check(ctx, true, true, true, true, false).await).is_err() {
        return Ok(()); // チェック失敗時終了
    }

    ctx.say("Update started!").await?; // 更新開始通知

    // 全キャラクター情報更新
    framedata::get_char_data(CHARS, "all").await; // フレームデータ更新
    images::get_char_images(CHARS, "all").await; // 画像データ更新

    ctx.say("Update succesful!").await?; // 更新完了通知

    Ok(()) // 正常終了
}

/// 全キャラクター更新処理　自動更新用関数
///
/// 24時間ごとに全キャラクターのフレームデータおよび画像データを更新する。
pub async fn update_all_char_data() {
    // 全キャラクターのフレームデータ更新
    framedata::get_char_data(CHARS, "all").await;
    // 全キャラクターの画像データ更新
    images::get_char_images(CHARS, "all").await;
}

/// キャラクター別アップデートサブコマンド  
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn character(
    ctx: Context<'_>,
    #[description = "対象キャラクター名"] character: String,
) -> Result<()> {
    // 入力チェック実施　条件確認
    if (check::adaptive_check(ctx, true, true, true, true, false).await).is_err() {
        return Ok(()); // チェック失敗時終了
    }

    // キャラクター探索処理　find関数呼出
    let character_arg_altered = match find::find_character(&character).await {
        Ok(character_arg_altered) => character_arg_altered, // キャラクター名称取得
        Err(err) => {
            ctx.say(err.to_string()).await?; // エラーメッセージ送信
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red()); // エラー出力
            return Ok(()); // エラー時終了
        }
    };

    // 更新対象分岐処理
    ctx.say("Update started!").await?; // 更新開始通知
    framedata::get_char_data(CHARS, &character_arg_altered).await; // フレームデータ更新
    images::get_char_images(CHARS, &character_arg_altered).await; // 画像データ更新

    ctx.say("Update succesful!").await?; // 更新完了通知

    Ok(()) // 正常終了
}
