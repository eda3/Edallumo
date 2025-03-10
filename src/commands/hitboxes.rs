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
const HITBOX_DEFAULT: &str =
    "https://raw.githubusercontent.com/eda3/Edallumo/main/data/images/no_hitbox.png";

/// キャラクターデータを読み込む関数
///
/// # 引数
/// * `character` - ユーザーが入力したキャラクター名
/// * `ctx` - コマンドコンテキスト
///
/// # 戻り値
/// 成功時は正式なキャラクター名、失敗時はエラー
async fn load_character_data(character: &str, ctx: &Context<'_>) -> Result<String, AppError> {
    // キャラクター名検索
    // ユーザー入力がエイリアスの場合、正式なキャラクター名を取得
    let character_arg_altered = match find::find_character(&character.to_string()).await {
        Ok(character_arg_altered) => character_arg_altered,
        Err(err) => {
            // キャラクター未検出時のエラーメッセージ送信
            ctx.say(err.to_string()).await?;
            println!("{}", format!("Error: {err}").red());
            return Err(AppError::CharacterNotFound(err.to_string()));
        }
    };

    Ok(character_arg_altered)
}

/// 技情報と画像データを読み込む関数
///
/// # 引数
/// * `character_arg_altered` - 正式なキャラクター名
/// * `character_move` - ユーザーが入力した技名
/// * `ctx` - コマンドコンテキスト
///
/// # 戻り値
/// 成功時は (技情報, 画像リンク情報) のタプル、失敗時はエラー
async fn find_move_and_images(
    character_arg_altered: &str,
    character_move: &str,
    ctx: &Context<'_>,
) -> Result<(MoveInfo, Vec<ImageLinks>), AppError> {
    // キャラクターJSONファイルパス構築
    let char_file_path = format!("data/{character_arg_altered}/{character_arg_altered}.json");

    // キャラクターJSONファイル読み込み
    let char_file_data = fs::read_to_string(&char_file_path).map_err(|e| {
        let error_msg =
            format!("キャラクターファイル '{char_file_path}' の読み込みに失敗しました: {e}");
        println!("{}", error_msg.red());
        AppError::FileNotFound(error_msg)
    })?;

    // キャラクターJSONデシリアライズ
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).map_err(|e| {
        let error_msg = format!("キャラクターJSONの解析に失敗しました: {e}");
        println!("{}", error_msg.red());
        AppError::Json(e)
    })?;

    // 読み込み成功ログ出力
    println!(
        "{}",
        format!("Successfully read '{char_file_path}' file.").green()
    );

    // 技インデックス検索
    let index = match find::find_move_index(
        &character_arg_altered.to_string(),
        character_move.to_string(),
        &moves_info,
    )
    .await
    {
        Ok(index) => index,
        Err(err) => {
            // 技未検出時のエラーメッセージ送信
            let error_msg = format!("{err}\nView the moves of a character by executing `/moves`.");
            ctx.say(&error_msg).await?;
            println!("{}", format!("Error: {err}").red());
            return Err(AppError::MoveNotFound(err.to_string()));
        }
    };

    // 画像JSONファイルパス構築
    let images_file_path = format!("data/{character_arg_altered}/images.json");

    // 画像JSONファイル読み込み
    let image_data = fs::read_to_string(&images_file_path).map_err(|e| {
        let error_msg = format!("画像ファイル '{images_file_path}' の読み込みに失敗しました: {e}");
        println!("{}", error_msg.red());
        AppError::FileNotFound(error_msg)
    })?;

    // 画像JSONデシリアライズ
    let image_links = serde_json::from_str::<Vec<ImageLinks>>(&image_data).map_err(|e| {
        let error_msg = format!("画像JSONの解析に失敗しました: {e}");
        println!("{}", error_msg.red());
        AppError::Json(e)
    })?;

    // 技情報取得
    let move_data = moves_info[index].clone();

    // 技情報読み込み成功ログ出力
    println!(
        "{}",
        format!(
            "Successfully read move '{}' in '{char_file_path}' file.",
            move_data.input
        )
        .green()
    );

    Ok((move_data, image_links))
}

/// ヒットボックス画像の埋め込みメッセージを作成する関数
///
/// # 引数
/// * `move_info` - 技情報
/// * `image_links` - 画像リンク情報
/// * `character_arg_altered` - 正式なキャラクター名
///
/// # 戻り値
/// 埋め込みメッセージのベクター
fn create_hitbox_embeds(
    move_info: &MoveInfo,
    image_links: &[ImageLinks],
    character_arg_altered: &str,
) -> Vec<CreateEmbed> {
    let mut vec_embeds = Vec::new();

    // 埋め込みタイトルとURL設定
    let embed_title = format!("__**{}**__", move_info.input);
    let embed_url = format!("https://dustloop.com/w/GGST/{character_arg_altered}#Overview");

    // 画像リンク走査
    for img_links in image_links {
        // 対象技の画像リンク検索
        if move_info.input == img_links.input {
            // 埋め込みの基本設定を作成する関数
            let create_base_embed = || {
                CreateEmbed::new()
                    .color(EMBED_COLOR)
                    .title(&embed_title)
                    .url(&embed_url)
            };

            println!(
                "{}",
                format!(
                    "hitbox_img: {} (raw), {} (valid)",
                    img_links.hitbox_img.len(),
                    img_links
                        .hitbox_img
                        .iter()
                        .filter(|url| !url.is_empty())
                        .count()
                )
                .blue()
            );

            // 空文字列を除外した有効なヒットボックス画像URLを収集
            let valid_hitbox_images: Vec<String> = img_links
                .hitbox_img
                .iter()
                .filter(|url| !url.is_empty())
                .cloned()
                .collect();

            match valid_hitbox_images.len() {
                // ヒットボックス画像なしの場合
                0 => {
                    // デフォルト画像で埋め込み作成
                    let empty_embed = create_base_embed().image(HITBOX_DEFAULT);
                    vec_embeds.push(empty_embed);
                }
                // ヒットボックス画像が1枚の場合
                1 => {
                    // 単一画像で埋め込み作成
                    let embed = create_base_embed().image(&valid_hitbox_images[0]);
                    vec_embeds.push(embed);
                }
                // ヒットボックス画像が複数の場合
                n => {
                    // フッター情報（画像枚数）設定
                    let embed_footer =
                        CreateEmbedFooter::new(format!("Move has {n} hitbox images."));

                    // 各画像ごとに埋め込み作成
                    for htbx_img in &valid_hitbox_images {
                        let embed = create_base_embed()
                            .image(htbx_img)
                            .footer(embed_footer.clone());
                        vec_embeds.push(embed);
                    }
                }
            }

            // 対象の技を見つけたらループを終了（重複防止）
            break;
        }
    }

    // 画像が見つからなかった場合、デフォルト埋め込みを追加
    if vec_embeds.is_empty() {
        let default_embed = CreateEmbed::new()
            .color(EMBED_COLOR)
            .title(&embed_title)
            .url(&embed_url)
            .image(HITBOX_DEFAULT)
            .description("No hitbox images found for this move.");
        vec_embeds.push(default_embed);
    }

    vec_embeds
}

/// ヒットボックス表示コマンド
///
/// 指定されたキャラクターの技のヒットボックス画像を表示する
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
        format!("Command Args: '{character}', '{character_move}'").purple()
    );

    // 各種チェック実行（データフォルダ、JSONファイル等の存在確認）
    let check_options = check::CheckOptions::DATA_FOLDER
        | check::CheckOptions::NICKNAMES_JSON
        | check::CheckOptions::CHARACTER_FOLDERS
        | check::CheckOptions::CHARACTER_JSONS
        | check::CheckOptions::CHARACTER_IMAGES;

    if check::adaptive_check(ctx, check_options).await.is_err() {
        return Ok(());
    }

    // キャラクターデータ読み込み
    let Ok(character_arg_altered) = load_character_data(&character, &ctx).await else {
        return Ok(());
    };

    // 技情報と画像データ読み込み
    let Ok((move_data, image_links)) =
        find_move_and_images(&character_arg_altered, &character_move, &ctx).await
    else {
        return Ok(());
    };

    // 埋め込みメッセージ作成
    let vec_embeds = create_hitbox_embeds(&move_data, &image_links, &character_arg_altered);

    // 返信作成と送信
    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds);
    ctx.send(reply).await?;

    Ok(())
}
