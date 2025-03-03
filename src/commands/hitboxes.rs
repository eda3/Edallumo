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
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
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
    // キャラクターJSONファイル読み込み
    let char_file_path =
        "data/".to_owned() + character_arg_altered + "/" + character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + character_arg_altered + ".json" + "' file."));

    // キャラクターJSONデシリアライズ
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    // 読み込み成功ログ出力
    println!(
        "{}",
        ("Successfully read '".to_owned() + character_arg_altered + ".json' file.").green()
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
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.")
                .await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Err(AppError::MoveNotFound(err.to_string()));
        }
    };

    // 画像JSONファイル読み込み
    let image_links = fs::read_to_string(
        "data/".to_owned() + character_arg_altered + "/images.json",
    )
    .expect(
        &("\nFailed to read 'data/".to_owned() + character_arg_altered + "'/images.json' file."),
    );

    // 画像JSONデシリアライズ
    let image_links = serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();

    // 技情報取得
    let move_info = moves_info[index].clone();

    // 技情報読み込み成功ログ出力
    println!(
        "{}",
        ("Successfully read move '".to_owned()
            + &move_info.input
            + "' in '"
            + character_arg_altered
            + ".json' file.")
            .green()
    );

    Ok((move_info, image_links))
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
    let embed_title = "__**".to_owned() + &move_info.input + "**__";
    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + character_arg_altered + "#Overview";

    // 画像リンク走査
    for img_links in image_links {
        // 対象技の画像リンク検索
        if move_info.input == img_links.input {
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

    // キャラクターデータ読み込み
    let Ok(character_arg_altered) = load_character_data(&character, &ctx).await else {
        return Ok(());
    };

    // 技情報と画像データ読み込み
    let Ok((move_info, image_links)) =
        find_move_and_images(&character_arg_altered, &character_move, &ctx).await
    else {
        return Ok(());
    };

    // 埋め込みメッセージ作成
    let vec_embeds = create_hitbox_embeds(&move_info, &image_links, &character_arg_altered);

    // 返信作成と送信
    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds);
    ctx.send(reply).await?;

    Ok(())
}
