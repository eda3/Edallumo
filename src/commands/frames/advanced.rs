//! advanced.rs
//!
//! このファイルは、キャラクターの技データの詳細情報をDiscordの埋め込みメッセージで表示するコマンドを実装。
//! 指定されたキャラクター名（または愛称）と技名（入力またはエイリアス）をもとに、
//! JSONファイルから該当データを取得し、画像リンクや各種技パラメータを整形して表示する。

use crate::{check, error::AppError, find, Context, ImageLinks, MoveInfo, EMBED_COLOR};
use colored::Colorize;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use std::{fs, string::String};

/// デフォルト画像URL
const IMAGE_DEFAULT: &str = "https://www.dustloop.com/wiki/images/5/54/GGST_Logo_Sparkly.png";

/// キャラクターデータを読み込む関数
///
/// # 引数
/// * `character` - ユーザーが入力したキャラクター名
/// * `ctx` - コマンドコンテキスト
///
/// # 戻り値
/// 成功時は正式なキャラクター名、失敗時はエラー
async fn load_character_data(character: &str, ctx: &Context<'_>) -> Result<String, AppError> {
    // キャラクター名の正規化　入力に基づく正式名称の取得
    let character_arg_altered = match find::find_character(&character.to_string()).await {
        Ok(character_arg_altered) => character_arg_altered, // 正式名称取得
        Err(err) => {
            ctx.say(err.to_string()).await?; // エラーメッセージ送信
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Err(AppError::CharacterNotFound(err.to_string())); // 処理中断
        }
    };

    Ok(character_arg_altered)
}

/// キャラクターの技情報を読み込む関数
///
/// # 引数
/// * `character_arg_altered` - 正式なキャラクター名
///
/// # 戻り値
/// 成功時は技情報のベクター
fn load_moves_info(character_arg_altered: &str) -> Vec<MoveInfo> {
    // JSONファイルパスの組み立て　キャラクター情報ファイルのパス生成
    let char_file_path =
        "data/".to_owned() + character_arg_altered + "/" + character_arg_altered + ".json";
    // JSONファイルの読み込み　キャラクター情報の取得
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + character_arg_altered + ".json" + "' file."));

    // JSON文字列のデシリアライズ　技情報の構造体へ変換
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    // JSON読み込み成功の表示　確認メッセージ出力
    println!(
        "{}",
        ("Successfully read '".to_owned() + character_arg_altered + ".json' file.").green()
    );

    moves_info
}

/// 画像リンク情報を読み込む関数
///
/// # 引数
/// * `character_arg_altered` - 正式なキャラクター名
///
/// # 戻り値
/// 画像リンク情報のベクター
fn load_image_links(character_arg_altered: &str) -> Vec<ImageLinks> {
    // 画像リンク用JSONファイルのパス組み立て　画像情報ファイルの指定
    let images_json_path = "data/".to_owned() + character_arg_altered + "/images.json";
    println!("Loading images JSON from: {images_json_path}");
    let image_links = fs::read_to_string(&images_json_path).expect(
        &("\nFailed to read 'data/".to_owned() + character_arg_altered + "/images.json' file."),
    );

    // JSONファイルを行単位で分割
    println!("Debug: Parsing image_links JSON for {character_arg_altered}");

    // デバッグログ処理（本番では省略可能）
    debug_image_links_json(&image_links);

    // 画像リンクJSONの安全なデシリアライズ
    parse_image_links(&image_links)
}

/// 画像リンクJSONのデバッグ情報を出力する関数
///
/// # 引数
/// * `image_links` - 画像リンクJSONの文字列
fn debug_image_links_json(image_links: &str) {
    println!("==================== MANUAL DEBUGGING ====================");
    // まず、JSON全体を解析できるかチェック
    let parse_result = serde_json::from_str::<serde_json::Value>(image_links);
    match parse_result {
        Ok(json_value) => {
            println!("Entire JSON successfully parsed as generic Value");

            // JSONが配列であることを確認
            if let serde_json::Value::Array(items) = json_value {
                println!("JSON is an array with {} items", items.len());

                // 各項目を個別にテスト
                for (i, item) in items.iter().enumerate() {
                    println!("Testing item #{i}");

                    // テスト用のJSONオブジェクトを作成
                    let test_json = serde_json::to_string(item).unwrap();

                    // デシリアライズを試みる
                    match serde_json::from_str::<ImageLinks>(&test_json) {
                        Ok(_) => println!("Item #{i} deserialized successfully"),
                        Err(e) => {
                            println!("ERROR with item #{i}: {e}");
                            println!("Problem item: {test_json}");

                            // 問題のあるアイテムの詳細を出力
                            if let Some(input) = item.get("input") {
                                println!(
                                    "input field: {input:?} (type: {})",
                                    if input.is_string() {
                                        "string"
                                    } else if input.is_number() {
                                        "number"
                                    } else {
                                        "other"
                                    }
                                );
                            }

                            if let Some(move_img) = item.get("move_img") {
                                println!(
                                    "move_img field: {move_img:?} (type: {})",
                                    if move_img.is_string() {
                                        "string"
                                    } else if move_img.is_number() {
                                        "number"
                                    } else {
                                        "other"
                                    }
                                );
                            }

                            if let Some(hitbox_img) = item.get("hitbox_img") {
                                println!(
                                    "hitbox_img field: {hitbox_img:?} (type: {})",
                                    if hitbox_img.is_array() {
                                        "array"
                                    } else if hitbox_img.is_number() {
                                        "number"
                                    } else {
                                        "other"
                                    }
                                );
                            }
                        }
                    }
                }
            } else {
                println!("JSON is not an array!");
            }
        }
        Err(e) => {
            println!("Failed to parse the entire JSON: {e}");
        }
    }
    println!("================== END MANUAL DEBUGGING ==================");
}

/// 画像リンクJSONをパースする関数
///
/// # 引数
/// * `image_links` - 画像リンクJSONの文字列
///
/// # 戻り値
/// 画像リンク情報のベクター
fn parse_image_links(image_links: &str) -> Vec<ImageLinks> {
    println!("Safely deserializing image links...");
    let mut image_links_vec = Vec::new();

    // JSONが配列であることを確認
    if let Ok(serde_json::Value::Array(items)) =
        serde_json::from_str::<serde_json::Value>(image_links)
    {
        // 各要素を個別に処理
        for item in items {
            let item_json = serde_json::to_string(&item).unwrap_or_default();
            match serde_json::from_str::<ImageLinks>(&item_json) {
                Ok(link) => image_links_vec.push(link),
                Err(e) => {
                    // エラーを出力するが、処理は継続
                    println!("Warning: Failed to deserialize item: {e}");
                    println!("Skipping problematic item: {item_json}");
                }
            }
        }
    } else {
        println!("Error: images.json is not a valid JSON array");
    }

    println!(
        "Successfully deserialized {count} image links",
        count = image_links_vec.len()
    );

    image_links_vec
}

/// 技に対応する画像URLを取得する
///
/// # 引数
/// * `move_data` - 技情報
/// * `image_links` - 画像リンク情報の配列
///
/// # 戻り値
/// 画像のURL（見つからない場合はデフォルト画像）
fn get_move_image_url(move_data: &MoveInfo, image_links: &[ImageLinks]) -> String {
    let mut embed_image = IMAGE_DEFAULT.to_string();

    // 括弧を除去した技名を作成（例：「2d(2d)」→「2d」）
    let cleaned_input = if move_data.input.contains('(') {
        move_data
            .input
            .split('(')
            .next()
            .unwrap_or("")
            .trim()
            .to_string()
    } else {
        move_data.input.to_string()
    };

    // 括弧内のコマンドを取得（例：「2d(2d)」→「2d」）
    let bracket_content = if move_data.input.contains('(') && move_data.input.contains(')') {
        let start = move_data.input.find('(').unwrap_or(0) + 1;
        let end = move_data.input.find(')').unwrap_or(move_data.input.len());
        if start < end {
            move_data.input[start..end].to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // 画像リンクの探索　対象技の画像リンクを検索
    for img_links in image_links {
        // 完全一致、括弧を除去した技名との一致、または括弧内のコマンドとの一致
        if (move_data.input.to_lowercase() == img_links.input.to_lowercase()
            || (!cleaned_input.is_empty()
                && cleaned_input.to_lowercase() == img_links.input.to_lowercase())
            || (!bracket_content.is_empty()
                && bracket_content.to_lowercase() == img_links.input.to_lowercase()))
            && !img_links.move_img.is_empty()
        {
            embed_image = img_links.move_img.to_string(); // 画像リンク更新
            break; // 探索終了
        }
        // 部分一致（バックアップ）
        else if img_links
            .input
            .to_lowercase()
            .contains(&move_data.input.to_lowercase())
            && !img_links.move_img.is_empty()
        {
            embed_image = img_links.move_img.to_string(); // 画像リンク更新
        }
    }

    embed_image
}

/// 技情報と画像データを読み込む関数
///
/// # 引数
/// * `character_arg_altered` - 正式なキャラクター名
/// * `character_move` - ユーザーが入力した技名
/// * `ctx` - コマンドコンテキスト
///
/// # 戻り値
/// 成功時は (技情報, 画像URL) のタプル、失敗時はエラー
async fn find_move_and_images(
    character_arg_altered: &str,
    character_move: &str,
    ctx: &Context<'_>,
) -> Result<(MoveInfo, String), AppError> {
    // 技情報の読み込み
    let moves_info = load_moves_info(character_arg_altered);

    // 画像リンク情報の読み込み
    let image_links = load_image_links(character_arg_altered);

    // 技インデックス検索
    let move_index = match find::find_move_index(
        &character_arg_altered.to_string(),
        character_move.to_string(),
        &moves_info,
    )
    .await
    {
        Ok(index) => index,
        Err(err) => {
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.")
                .await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Err(AppError::MoveNotFound(err.to_string()));
        }
    };

    // 対象技情報の取得　入力に対応する技データの抽出
    let move_data = moves_info[move_index].clone();

    // 対象技の読み込み成功の表示　確認メッセージ出力
    println!(
        "{}",
        ("Successfully read move '".to_owned()
            + &move_data.input
            + "' in '"
            + character_arg_altered
            + ".json' file.")
            .green()
    );

    // 技画像URLの取得
    let embed_image = get_move_image_url(&move_data, &image_links);

    Ok((move_data, embed_image))
}

/// 技情報の詳細な埋め込みメッセージを作成する関数
///
/// # 引数
/// * `move_info` - 技情報
/// * `embed_image` - 埋め込む画像のURL
/// * `character_arg_altered` - 正式なキャラクター名
///
/// # 戻り値
/// 埋め込みメッセージのベクター
fn create_advanced_embeds(
    move_info: &MoveInfo,
    embed_image: &str,
    character_arg_altered: &str,
) -> Vec<CreateEmbed> {
    // 埋め込みメッセージ群生成用ベクターの初期化
    let mut vec_embeds = Vec::new();
    // 埋め込みURLの作成　Dustloop Wiki のキャラクター概要ページURL生成
    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + character_arg_altered + "#Overview";
    // 埋め込みフッターの作成　技に関するキャプションを利用
    let embed_footer = CreateEmbedFooter::new(&move_info.caption);

    // 埋め込みメッセージの生成　技データの各パラメータをフィールドとして追加
    let embed = CreateEmbed::new()
        .color(EMBED_COLOR) // 埋め込みカラー設定
        .title(format!(
            "{}の{input}",
            character_arg_altered,
            input = move_info.input
        ))
        .url(embed_url) // URL設定
        .image(embed_image) // 画像リンク設定
        .fields(vec![
            (
                "ダメージ",
                &move_info.damage.map_or("-".to_string(), |v| v.to_string()),
                true,
            ),
            ("ガード", &move_info.guard, true),
            ("無敵", &move_info.invincibility, true),
            (
                "始動",
                &move_info.startup.map_or("-".to_string(), |v| v.to_string()),
                true,
            ),
            ("持続", &move_info.active, true),
            (
                "硬直",
                &move_info
                    .recovery
                    .map_or("-".to_string(), |v| v.to_string()),
                true,
            ),
            ("ヒット時", &move_info.on_hit, true),
            ("ガード時", &move_info.on_block, true),
            ("カウンター", &move_info.counter, true),
            ("技レベル", &move_info.level, true),
            (
                "リスク増加",
                &move_info
                    .risc_gain
                    .map_or("-".to_string(), |v| v.to_string()),
                true,
            ),
            (
                "リスク減少",
                &move_info
                    .risc_loss
                    .map_or("-".to_string(), |v| v.to_string()),
                true,
            ),
            (
                "壁ダメージ",
                &move_info
                    .wall_damage
                    .map_or("-".to_string(), |v| v.to_string()),
                true,
            ),
            (
                "入力緊張度",
                &move_info
                    .input_tension
                    .map_or("-".to_string(), |v| v.to_string()),
                true,
            ),
            (
                "チップ比率",
                &move_info
                    .chip_ratio
                    .map_or("-".to_string(), |v| v.to_string()),
                true,
            ),
            (
                "スケーリング",
                &move_info.scaling.map_or("-".to_string(), |v| v.to_string()),
                true,
            ),
        ])
        .footer(embed_footer); // フッター設定

    // 生成した埋め込みメッセージをベクターに追加
    vec_embeds.push(embed);

    // 備考（notes）が存在する場合、別の埋め込みメッセージを生成
    if !move_info.notes.is_empty() {
        let embed2 = CreateEmbed::new()
            .color(EMBED_COLOR) // 埋め込みカラー設定
            .description(&move_info.notes); // 備考記述設定
        vec_embeds.push(embed2); // ベクターに追加
    }

    vec_embeds
}

/// 技の詳細情報を埋め込みメッセージで表示するコマンド
///
/// # 引数
/// * `ctx` - コマンドコンテキスト
/// * `character` - キャラクター名または愛称
/// * `character_move` - 技名、入力、またはエイリアス
///
/// # 戻り値
/// 処理結果 `Result<(), AppError>`
#[poise::command(prefix_command, slash_command)]
pub async fn advanced(
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = "キャラクター名または愛称"]
    character: String,
    #[min_length = 2]
    #[rename = "move"]
    #[description = "技名、入力、またはエイリアス"]
    character_move: String,
) -> Result<(), AppError> {
    // コマンド引数の表示　引数確認用
    println!(
        "{}",
        ("Command Args: '".to_owned() + &character + ", " + &character_move + "'").purple()
    );

    // 入力チェックの実施　各種前提条件チェック
    if (check::adaptive_check(
        ctx,
        check::CheckOptions::DATA_FOLDER
            | check::CheckOptions::NICKNAMES_JSON
            | check::CheckOptions::CHARACTER_FOLDERS
            | check::CheckOptions::CHARACTER_JSONS
            | check::CheckOptions::CHARACTER_IMAGES,
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
    let Ok((move_info, embed_image)) =
        find_move_and_images(&character_arg_altered, &character_move, &ctx).await
    else {
        return Ok(());
    };

    // 埋め込みメッセージ作成
    let vec_embeds = create_advanced_embeds(&move_info, &embed_image, &character_arg_altered);

    // 返信メッセージ用オブジェクト生成　送信用オブジェクトの初期化
    let mut reply = poise::CreateReply::default();
    // 生成した埋め込みメッセージ群を返信オブジェクトに追加
    reply.embeds.extend(vec_embeds);
    // 返信メッセージの送信　Discordへ送信
    ctx.send(reply).await?;

    // 正常終了の返却　処理完了
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MoveInfo;
    use crate::test_utils::{create_test_json_file, create_test_move_info};
    use crate::ImageLinks;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // テスト環境のセットアップ用ヘルパー関数
    fn setup_test_environment() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().expect("一時ディレクトリの作成に失敗");
        let temp_path = temp_dir.path().to_path_buf();

        // テスト用ディレクトリ構造を作成
        fs::create_dir_all(temp_path.join("data/Sol_Badguy"))
            .expect("テスト用ディレクトリの作成に失敗");

        // キャラクターJSONファイルの作成
        let moves_info = create_test_move_info();
        let json_content = serde_json::to_string(&moves_info).expect("JSONシリアライズに失敗");
        create_test_json_file(
            &temp_path.join("data/Sol_Badguy/Sol_Badguy.json"),
            &json_content,
        )
        .expect("テストJSONファイルの作成に失敗");

        // 画像JSONファイルの作成
        let image_links = vec![
            ImageLinks {
                input: "5P".to_string(),
                move_img: "http://example.com/5p.png".to_string(),
                hitbox_img: vec!["http://example.com/5p_hitbox.png".to_string()],
            },
            ImageLinks {
                input: "236K".to_string(),
                move_img: "http://example.com/236k.png".to_string(),
                hitbox_img: vec!["http://example.com/236k_hitbox.png".to_string()],
            },
        ];
        let json_content = serde_json::to_string(&image_links).expect("JSONシリアライズに失敗");
        create_test_json_file(
            &temp_path.join("data/Sol_Badguy/images.json"),
            &json_content,
        )
        .expect("画像JSONファイルの作成に失敗");

        (temp_dir, temp_path)
    }

    // 一時的に作業ディレクトリを変更するためのヘルパー構造体
    struct TempWorkingDir {
        original_dir: PathBuf,
    }

    impl TempWorkingDir {
        fn _new(path: &PathBuf) -> Self {
            let original_dir = env::current_dir().expect("現在のディレクトリの取得に失敗");
            env::set_current_dir(path).expect("ディレクトリの変更に失敗");
            Self { original_dir }
        }
    }

    impl Drop for TempWorkingDir {
        fn drop(&mut self) {
            env::set_current_dir(&self.original_dir).expect("元のディレクトリに戻れませんでした");
        }
    }

    #[test]
    fn test_create_advanced_embeds() {
        // テスト用のデータを準備
        let move_info = MoveInfo {
            input: "5P".to_string(),
            name: "Punch".to_string(),
            damage: Some(26),
            guard: "Mid".to_string(),
            startup: Some(4),
            active: "3".to_string(),
            recovery: Some(9),
            on_hit: "+2".to_string(),
            on_block: "-1".to_string(),
            level: "0".to_string(),
            counter: "3".to_string(),
            move_type: "Normal".to_string(),
            risc_gain: Some(23.0),
            risc_loss: Some(18.0),
            wall_damage: Some(9),
            input_tension: Some(0.0),
            chip_ratio: Some(0.0),
            otg_ratio: Some(0.8),
            scaling: Some(0.8),
            invincibility: "None".to_string(),
            cancel: "Special, Super".to_string(),
            caption: String::new(),
            notes: String::new(),
        };

        let embed_image = "http://example.com/image.png";
        let character_name = "Sol_Badguy";

        // 関数を実行
        let embeds = create_advanced_embeds(&move_info, embed_image, character_name);

        // 結果の検証
        assert!(!embeds.is_empty());
        // 最低限、埋め込みが1つ以上生成されていることを確認
        assert!(embeds.len() >= 1);
    }
}
