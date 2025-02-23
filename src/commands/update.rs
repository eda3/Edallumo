//! update.rs
//!
//! このファイルは、Dustloop Wiki のデータ更新機能を提供する。
//! フレームデータ、画像データの更新処理を実行する関数群を実装する。
//! オーナー専用コマンドとして利用可能。

mod framedata; // framedata.rs モジュール　フレームデータ更新処理群
mod framedata_json; // framedata_json.rs モジュール　フレームデータJSON変換処理群
mod images; // images.rs モジュール　画像データ更新処理群
mod images_json; // images_json.rs モジュール　画像データJSON変換処理群

use crate::{check, find, Context, Error, CHARS}; // 共通チェック関数、検索関数、型定義群
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

/// Dustloop のデータ更新コマンド（オーナー専用）
///
/// # 概要
/// 指定キャラクターまたは "all" により、フレームデータおよび画像データの更新処理を実行する。
/// キャラクター名またはニックネーム、または "all" を指定することで更新対象を決定する。
///
/// # 引数
/// * `ctx` - コマンド実行コンテキスト
/// * `character` - キャラクター名、ニックネーム、または "all"（2文字以上）
/// * `option` - 更新対象選択（"frames", "images", "all"）
///
/// # 戻り値
/// `Result<(), Error>` を返す
#[poise::command(prefix_command, slash_command, owners_only, ephemeral)]
pub async fn update(
    ctx: Context<'_>, // コマンドコンテキスト
    #[min_length = 2]
    #[description = r#"Character name, nickname or "all"."#]
    // キャラクター名、ニックネーム、または "all"
    character: String, // キャラクター指定文字列
    #[description = r#"Select "frames", "images" or "all"."#] // 更新対象選択用文字列
    option: UpdateChoice, // 更新対象選択値
) -> Result<(), Error> {
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
    match option {
        UpdateChoice::All => {
            // 全更新の場合
            // "all" が指定された場合：全キャラクター更新
            if character.trim().to_lowercase() == "all" {
                ctx.say("Update started!").await?; // 更新開始通知
                update_all_char_data().await; // 全キャラクター更新実行
            } else {
                // 特定キャラクター更新の場合
                ctx.say("Update started!").await?; // 更新開始通知
                framedata::get_char_data(CHARS, &character_arg_altered).await; // フレームデータ更新
                images::get_char_images(CHARS, &character_arg_altered).await; // 画像データ更新
            }
        }
        UpdateChoice::Frames => {
            // フレーム更新の場合
            if character.trim().to_lowercase() == "all" {
                ctx.say("Update started!").await?; // 更新開始通知
                framedata::get_char_data(CHARS, "all").await; // 全キャラクターのフレームデータ更新
            } else {
                ctx.say("Update started!").await?; // 更新開始通知
                framedata::get_char_data(CHARS, &character_arg_altered).await; // 特定キャラクターのフレームデータ更新
            }
        }
        UpdateChoice::Images => {
            // 画像更新の場合
            if character.trim().to_lowercase() == "all" {
                ctx.say("Update started!").await?; // 更新開始通知
                images::get_char_images(CHARS, "all").await; // 全キャラクターの画像データ更新
            } else {
                ctx.say("Update started!").await?; // 更新開始通知
                images::get_char_images(CHARS, &character_arg_altered).await; // 特定キャラクターの画像データ更新
            }
        }
    }

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
