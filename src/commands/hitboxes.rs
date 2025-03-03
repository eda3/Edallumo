//! # hitboxes.rs
//!
//! ヒットボックス画像表示モジュール。
//! キャラクターの技のヒットボックス画像を表示するためのコマンドを提供する。
//! 指定されたキャラクターと技に対応するヒットボックス画像をDiscord上に埋め込み表示する。

// 必要なインポート
use crate::{check, error::AppError, find, Context, ImageLinks, MoveInfo, EMBED_COLOR}; // 各種機能とデータ型
use colored::Colorize; // ターミナル出力の色付け
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter}; // Discord埋め込み作成
use std::{fs, string::String}; // ファイル操作と文字列型

/// デフォルトヒットボックス画像URL
const HITBOX_DEFAULT: &str = "https://www.dustloop.com/wiki/images/5/54/GGST_Logo_Sparkly.png";

/// ヒットボックス画像表示コマンド
///
/// # 概要
/// 指定されたキャラクターの技のヒットボックス画像を表示する。
/// キャラクター名（またはニックネーム）と技名（または入力コマンド、エイリアス）を
/// 指定することで、対応するヒットボックス画像をDiscord上に表示する。
///
/// # 引数
/// * `ctx` - コマンドコンテキスト
/// * `character` - キャラクター名またはニックネーム（最低2文字以上）
/// * `character_move` - 技名、入力コマンド、またはエイリアス（最低2文字以上）
///
/// # 戻り値
/// 成功時は `Ok(())`, エラー時は `Err(Error)` を返す
#[poise::command(prefix_command, slash_command)]
pub async fn hitboxes(
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = "Character name or nickname."]
    character: String,
    #[min_length = 2]
    #[rename = "move"]
    #[description = "Move name, input or alias."]
    character_move: String,
) -> Result<(), AppError> {
    // コマンド引数のログ出力
    println!(
        "{}",
        ("Command Args: '".to_owned() + &character + ", " + &character_move + "'").purple()
    );

    // 各種チェック実行（データフォルダ、JSONファイル等の存在確認）
    if (check::adaptive_check(
        ctx,
        check::CheckOptions {
            data_folder: true,
            nicknames_json: true,
            character_folders: true,
            character_jsons: true,
            character_images: true,
        },
    )
    .await)
        .is_err()
    {
        return Ok(());
    }

    // キャラクター名検索
    // ユーザー入力がエイリアスの場合、正式なキャラクター名を取得
    let character_arg_altered = match find::find_character(&character).await {
        Ok(character_arg_altered) => character_arg_altered,
        Err(err) => {
            // キャラクター未検出時のエラーメッセージ送信
            ctx.say(err.to_string()).await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(());
        }
    };

    // キャラクターJSONファイル読み込み
    let char_file_path =
        "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character_arg_altered + ".json" + "' file."));

    // キャラクターJSONデシリアライズ
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    // 読み込み成功ログ出力
    println!(
        "{}",
        ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green()
    );

    // 技インデックス検索
    let index =
        match find::find_move_index(&character_arg_altered, character_move, &moves_info).await {
            Ok(index) => index,
            Err(err) => {
                // 技未検出時のエラーメッセージ送信
                ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.")
                    .await?;
                println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
                return Ok(());
            }
        };

    // 画像JSONファイル読み込み
    let image_links = fs::read_to_string(
        "data/".to_owned() + &character_arg_altered + "/images.json",
    )
    .expect(
        &("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."),
    );

    // 画像JSONデシリアライズ
    let image_links = serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();

    // 技情報取得と埋め込み準備
    let move_info = &moves_info[index];
    let mut vec_embeds = Vec::new();

    // 埋め込みタイトルとURL設定
    let embed_title = "__**".to_owned() + &move_info.input + "**__";
    let embed_url =
        "https://dustloop.com/w/GGST/".to_owned() + &character_arg_altered + "#Overview";

    // 画像リンク走査
    for img_links in image_links {
        // 対象技の画像リンク検索
        if move_info.input == img_links.input {
            // 技情報読み込み成功ログ出力
            println!(
                "{}",
                ("Successfully read move '".to_owned()
                    + &move_info.input.to_string()
                    + "' in '"
                    + &character_arg_altered
                    + ".json' file.")
                    .green()
            );

            // ヒットボックス画像なしの場合
            if img_links.hitbox_img.is_empty() {
                // デフォルト画像で埋め込み作成
                let empty_embed = CreateEmbed::new()
                    .color(EMBED_COLOR)
                    .title(&embed_title)
                    .url(&embed_url)
                    .image(HITBOX_DEFAULT);

                vec_embeds.push(empty_embed);
            }
            // ヒットボックス画像が1枚の場合
            else if img_links.hitbox_img.len() == 1 {
                // 単一画像で埋め込み作成
                let embed = CreateEmbed::new()
                    .color(EMBED_COLOR)
                    .title(&embed_title)
                    .url(&embed_url)
                    .image(&img_links.hitbox_img[0]);

                vec_embeds.push(embed);
            }
            // ヒットボックス画像が複数の場合
            else {
                // 各画像ごとに埋め込み作成
                for htbx_img in &img_links.hitbox_img {
                    // フッター情報（画像枚数）設定
                    let embed_footer = CreateEmbedFooter::new(
                        "Move has ".to_owned()
                            + &img_links.hitbox_img.len().to_string()
                            + " hitbox images.",
                    );

                    // 埋め込み作成
                    let embed = CreateEmbed::new()
                        .color(EMBED_COLOR)
                        .title(&embed_title)
                        .url(&embed_url)
                        .image(htbx_img)
                        .footer(embed_footer);

                    vec_embeds.push(embed);
                }
            }
        }
    }

    // 返信作成と送信
    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds);
    ctx.send(reply).await?;

    Ok(())
}
