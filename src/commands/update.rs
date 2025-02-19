//! # update.rs
//!
//! フレームデータおよび画像データの更新処理を提供するモジュール
//! Dustloop API からデータを取得し、ローカルのデータを更新する
//! このコマンドはボットの所有者のみ実行可能

mod framedata;
mod framedata_json;
mod images;
mod images_json;

use crate::serenity::futures::{self, Stream, StreamExt};
use crate::{check, find, Context, Error, CHARS};
use colored::Colorize;

/// 更新オプションのオートコンプリートを提供する関数
/// ユーザーの入力に基づき、"all"、"frames"、"images" のいずれかを提案する
/// ユーザーが入力した部分文字列にマッチする候補を返す
async fn autocomplete_option<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&["all", "frames", "images"])
        .filter(move |name| {
            futures::future::ready(name.to_lowercase().contains(&partial.to_lowercase()))
        })
        .map(|name| name.to_string())
}

/// フレームデータおよび画像データを更新するコマンド
#[poise::command(prefix_command, slash_command, aliases("u"), owners_only)]
pub async fn update(
    ctx: Context<'_>,
    // キャラクター名、ニックネーム、または 'all' を指定する
    #[description = "キャラクター名、ニックネーム、または 'all'"] character: String,
    // 更新対象のデータ ('frames', 'images', 'all') を指定する
    #[description = "更新対象のデータ ('frames', 'images', 'all')"]
    #[autocomplete = "autocomplete_option"]
    option: String,
) -> Result<(), Error> {
    let option = option.trim().to_lowercase();

    // キャラクター名の妥当性をチェック
    if (check::adaptive_check(
        ctx,
        (true, &character),
        (false, &String::new()),
        true,
        true,
        true,
        false,
        false,
    )
    .await)
        .is_err()
    {
        return Ok(());
    }

    // キャラクターの正式名称を検索し、存在しない場合はエラーメッセージを返す
    let character_arg_altered = match find::find_character(&character).await {
        Ok(character_arg_altered) => character_arg_altered,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            println!("{}", ("エラー: ".to_owned() + &err.to_string()).red());
            return Ok(());
        }
    };

    // フレームデータの更新処理
    if option == "frames" {
        if character.trim().to_lowercase() == "all" {
            ctx.say("更新開始").await?;
            framedata::get_char_data(CHARS, "all").await;
        } else {
            ctx.say("更新開始").await?;
            framedata::get_char_data(CHARS, &character_arg_altered).await;
        }
    } else if option == "images" {
        // 画像データの更新処理
        if character.trim().to_lowercase() == "all" {
            ctx.say("更新開始").await?;
            images::get_char_data(CHARS, "all").await;
        } else {
            ctx.say("更新開始").await?;
            images::get_char_data(CHARS, &character_arg_altered).await;
        }
    } else if option == "all" {
        // 両方のデータを更新
        if character.trim().to_lowercase() == "all" {
            ctx.say("更新開始").await?;
            framedata::get_char_data(CHARS, "all").await;
            images::get_char_data(CHARS, "all").await;
        } else {
            ctx.say("更新開始").await?;
            framedata::get_char_data(CHARS, &character_arg_altered).await;
            images::get_char_data(CHARS, &character_arg_altered).await;
        }
    } else {
        // 無効なオプションの場合、エラーメッセージを返す
        let error_msg = format!("選択 `{}` は無効", option);
        ctx.say(&error_msg).await?;
        println!("{}", format!("エラー: 選択 `{}` は無効", option).red());
        return Ok(());
    }

    ctx.channel_id().say(ctx, "更新完了").await?;

    Ok(())
}
