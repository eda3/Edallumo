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
/// * `character_move` - ユーザー入力の技名または技入力
/// * `ctx` - コマンドコンテキスト
///
/// # 戻り値
/// 成功時は (MoveInfo, Vec<ImageLinks>) のタプル、失敗時はエラー
async fn find_move_and_images(
    character_arg_altered: &str,
    character_move: &str,
    ctx: &Context<'_>,
) -> Result<(MoveInfo, Vec<ImageLinks>), AppError> {
    // キャラクターデータファイルのパス設定
    let file_path = format!("data/{character_arg_altered}/{character_arg_altered}.json");
    let images_path = format!("data/{character_arg_altered}/images.json");

    // キャラクターJSONファイル読み込み
    let data = fs::read_to_string(&file_path).map_err(|_| {
        let error_msg = format!("Failed to load character data from {file_path}");
        println!("{}", error_msg.red());
        AppError::FileNotFound(error_msg)
    })?;

    // JSONデシリアライズ
    let moves_info: Vec<MoveInfo> = serde_json::from_str(&data).map_err(|e| {
        let error_msg = format!("Failed to parse character data: {e}");
        println!("{}", error_msg.red());
        AppError::Json(e)
    })?;

    println!(
        "{}",
        format!(
            "Loaded {} moves for character '{}'",
            moves_info.len(),
            character_arg_altered
        )
        .blue()
    );

    // 技名インデックス検索
    let move_index = match find::find_move_index(
        &character_arg_altered.to_string(),
        character_move.to_string(),
        &moves_info,
    )
    .await
    {
        Ok(move_index) => move_index,
        Err(err) => {
            // 技未発見時のエラーメッセージ送信
            ctx.say(err.to_string()).await?;
            println!("{}", format!("Error: {err}").red());
            return Err(AppError::MoveNotFound(err.to_string()));
        }
    };

    // 見つかった技情報の取得
    let move_data = &moves_info[move_index];

    println!(
        "{}",
        format!(
            "Found move at index {}: {} (input: {})",
            move_index, move_data.name, move_data.input
        )
        .blue()
    );

    // 画像JSONファイル読み込み
    let image_data = fs::read_to_string(&images_path).map_err(|_| {
        let error_msg = format!("Failed to load image data from {images_path}");
        println!("{}", error_msg.red());
        AppError::FileNotFound(error_msg)
    })?;

    // 画像JSONデシリアライズ
    let image_links: Vec<ImageLinks> = serde_json::from_str(&image_data).map_err(|e| {
        let error_msg = format!("Failed to parse image data: {e}");
        println!("{}", error_msg.red());
        AppError::Json(e)
    })?;

    println!(
        "{}",
        format!("Loaded {} image entries", image_links.len()).blue()
    );

    // 技の入力コマンドをログ出力
    println!(
        "{}",
        format!("Looking for move with input: '{}'", move_data.input).blue()
    );

    // image_links内の各要素のinput値をログに出力
    println!("{}", "Available image inputs:".blue());
    for (i, img) in image_links.iter().enumerate() {
        println!("  {}: '{}'", i, img.input);
    }

    Ok((move_data.clone(), image_links))
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

    // 技入力の正規化（検索用）
    // 大文字小文字を区別せず、スペースを削除
    let normalized_move_input = move_info.input.to_lowercase().replace(' ', "");
    println!(
        "{}",
        format!(
            "Normalized move input for search: '{}'",
            normalized_move_input
        )
        .cyan()
    );

    // 画像リンク走査
    let mut found_matching_move = false;
    for img_links in image_links {
        // 対象技の画像リンク検索（正規化して比較）
        let normalized_img_input = img_links.input.to_lowercase().replace(' ', "");
        println!(
            "{}",
            format!(
                "Comparing with: '{}' (normalized: '{}')",
                img_links.input, normalized_img_input
            )
            .cyan()
        );

        // 入力が一致するか確認
        if normalized_move_input == normalized_img_input {
            found_matching_move = true;
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
                    "Found matching move! hitbox_img: {} (raw), {} (valid)",
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

    // 技名の一部が含まれている場合の処理（完全一致しない場合のフォールバック）
    if !found_matching_move {
        println!(
            "{}",
            format!("No exact match found. Trying partial matching...").yellow()
        );

        // 技入力を5HやjKなど基本形に変換して再検索
        let simplified_input = simplified_move_input(&normalized_move_input);
        println!(
            "{}",
            format!("Simplified input: '{}'", simplified_input).yellow()
        );

        for img_links in image_links {
            let normalized_img_input = img_links.input.to_lowercase().replace(' ', "");

            // 部分一致または単純化した入力で一致するか確認
            if normalized_img_input.contains(&normalized_move_input)
                || normalized_img_input.contains(&simplified_input)
            {
                found_matching_move = true;
                println!(
                    "{}",
                    format!("Found partial match: '{}'", img_links.input).green()
                );

                // 埋め込みの基本設定を作成
                let base_embed = CreateEmbed::new()
                    .color(EMBED_COLOR)
                    .title(&embed_title)
                    .url(&embed_url);

                // 有効なヒットボックス画像URLを収集
                let valid_hitbox_images: Vec<String> = img_links
                    .hitbox_img
                    .iter()
                    .filter(|url| !url.is_empty())
                    .cloned()
                    .collect();

                // 画像がある場合は表示
                if !valid_hitbox_images.is_empty() {
                    let embed_footer = CreateEmbedFooter::new(format!(
                        "Partial match: '{}' - {} images",
                        img_links.input,
                        valid_hitbox_images.len()
                    ));

                    for htbx_img in &valid_hitbox_images {
                        let embed = base_embed
                            .clone()
                            .image(htbx_img)
                            .footer(embed_footer.clone());
                        vec_embeds.push(embed);
                    }
                    break;
                }
            }
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

/// 技入力を単純化する関数
/// 例: "5hs" -> "5h", "236k" -> "236k"
fn simplified_move_input(input: &str) -> String {
    let input = input.to_lowercase();

    // 基本的な技入力パターンを処理
    if input.starts_with('5') && input.contains("hs") {
        return "5h".to_string();
    } else if input.starts_with('2') && input.contains("hs") {
        return "2h".to_string();
    } else if input.starts_with('6') && input.contains("hs") {
        return "6h".to_string();
    } else if input.starts_with('j') && input.contains("hs") {
        return "jh".to_string();
    }

    // その他のケースはそのまま
    input
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

    // デバッグ出力: 正規化されたキャラクター名を表示
    println!(
        "{}",
        format!("Normalized character name: '{character_arg_altered}'").yellow()
    );

    // 技情報と画像データ読み込み
    let result = find_move_and_images(&character_arg_altered, &character_move, &ctx).await;

    if let Err(err) = &result {
        println!("{}", format!("Error finding move: {err}").red());
    }

    let Ok((move_data, image_links)) = result else {
        return Ok(());
    };

    // デバッグ出力: 見つかった技情報を表示
    println!(
        "{}",
        format!(
            "Found move: '{}' (input: '{}')",
            move_data.name, move_data.input
        )
        .yellow()
    );

    // デバッグ出力: 見つかった画像リンクの数
    println!(
        "{}",
        format!("Found {} image links", image_links.len()).yellow()
    );

    // 埋め込みメッセージ作成
    let vec_embeds = create_hitbox_embeds(&move_data, &image_links, &character_arg_altered);

    // デバッグ出力: 作成された埋め込みの数
    println!(
        "{}",
        format!("Created {} embeds", vec_embeds.len()).yellow()
    );

    // 返信作成と送信
    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds);
    ctx.send(reply).await?;

    Ok(())
}
