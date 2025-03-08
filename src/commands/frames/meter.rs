//! meter.rs
//! ファイル全体説明コメント
//! フレームメーター表示機能全体
//! 開始・アクティブ・リカバリーフレーム情報処理
//! コマンド実行機能

use crate::{check, error::AppError, find, Context, ImageLinks, MoveInfo, EMBED_COLOR}; // 依存モジュール群
use colored::Colorize; // 文字色変換ライブラリ
use poise::serenity_prelude::CreateEmbed; // 埋め込み作成ライブラリ
use std::{fs, string::String}; // ファイル操作・文字列操作

const GREEN_CIRCLE: &str = "🟢\u{200b}"; // 緑丸定数
const RED_SQUARE: &str = "🟥\u{200b}"; // 赤四角定数
const BLUE_DIAMOND: &str = "🔷\u{200b}"; // 青菱形定数

/// デフォルト画像URL
const IMAGE_DEFAULT: &str = "https://www.dustloop.com/wiki/images/5/54/GGST_Logo_Sparkly.png";

/// 指定ムーブの開始フレーム情報からシンボル文字列生成
///
/// # 引数
/// * `move_info` - ムーブ情報構造体参照
///
/// # 戻り値
/// 開始フレームシンボル文字列
async fn startup_frames(move_info: &MoveInfo) -> String {
    // Option<i32>の場合は文字列に変換
    let startup_str = move_info.startup.map_or("-".to_string(), |v| v.to_string());
    let startup_vec = sep_frame_vec(&startup_str).await; // 開始フレーム分割結果取得
    let mut meter_msg = String::new(); // メーター文字列初期化
                                       // println!("startup_vec: {:?}", startup_vec); // デバッグ出力用

    // 単一エントリかつ空または "-"、または1フレームのみの場合
    if (startup_vec.len() == 1 && startup_vec[0] == "-")
        || (startup_vec.len() == 1 && startup_vec[0].parse::<u16>().unwrap() == 1)
    {
        meter_msg += "-"; // 単一フレーム表現
    }
    // 複数エントリの場合の処理
    else {
        let mut startup_bra = false; // 括弧有無判定フラグ初期化

        // 各エントリ処理ループ
        for x in 0..startup_vec.len() {
            // 数字エントリの場合
            if let Ok(num) = startup_vec[x].parse::<u16>() {
                // 数値-1回分ループ処理
                for _ in 0..num - 1 {
                    if startup_bra {
                        // 括弧内：前エントリとの差分回数緑丸追加
                        for _ in 0..((startup_vec[x].parse::<u16>().unwrap())
                            - (startup_vec[x - 2].parse::<u16>()).unwrap())
                        {
                            meter_msg += GREEN_CIRCLE; // 括弧内緑丸追加
                        }
                        break; // ループ中断
                    }
                    meter_msg += GREEN_CIRCLE; // 括弧前：緑丸追加
                }
            }
            // 数字以外のエントリの場合
            else {
                // "+"記号処理（末尾直前の場合）
                if x == startup_vec.len() - 2 && startup_vec[x] == "+" {
                    if let Ok(num) = startup_vec[x + 1].parse::<u16>() {
                        // 数字変換試行
                        if num == 1 {
                            meter_msg += GREEN_CIRCLE; // 単一数値：緑丸置換
                        } else {
                            meter_msg.push_str(GREEN_CIRCLE);
                            meter_msg.push_str(&startup_vec[x]);
                            // 複数数値：緑丸＋"+"追加
                        }
                    } else {
                        meter_msg.push_str(&startup_vec[x]); // 数字変換失敗：記号そのまま追加
                    }
                }
                // その他の記号処理
                else {
                    meter_msg.push_str(&startup_vec[x]); // 記号追加
                }

                // 括弧・チルダ判定更新
                if startup_vec[x] == "[" || startup_vec[x] == "~" {
                    startup_bra = true; // 括弧開始
                } else if startup_vec[x] == "]" {
                    startup_bra = false; // 括弧終了
                }
            }
        }
    }
    meter_msg // シンボル文字列返却
}

/// 指定ムーブのアクティブフレーム情報からシンボル文字列生成
///
/// # 引数
/// * `move_info` - ムーブ情報構造体参照
///
/// # 戻り値
/// アクティブフレームシンボル文字列
async fn active_frames(move_info: &MoveInfo) -> String {
    let active_vec = sep_frame_vec(&move_info.active).await; // アクティブフレーム分割結果取得
    let mut meter_msg = String::new(); // メーター文字列初期化
                                       // println!("Active vec: {:?}", active_vec); // デバッグ出力用

    if active_vec.len() == 1 && active_vec[0] == "-" {
        meter_msg += "-"; // 単一ハイフン表現
    } else {
        let mut hit_recovery = false; // 括弧有無判定フラグ初期化

        // 各エントリ処理ループ（参照）
        for active_vec_string in &active_vec {
            if let Ok(num) = active_vec_string.parse::<u16>() {
                // 数値エントリの場合、数値分ループ
                for _ in 0..num {
                    if hit_recovery {
                        meter_msg += BLUE_DIAMOND; // 括弧内：青菱形追加
                    } else {
                        meter_msg += RED_SQUARE; // 括弧前：赤四角追加
                    }
                }
            }
            // 数値以外のエントリの場合
            else {
                meter_msg.push_str(active_vec_string); // 記号追加
                if active_vec_string == "(" {
                    hit_recovery = true; // 括弧開始
                } else if active_vec_string == ")" {
                    hit_recovery = false; // 括弧終了
                }
            }
        }
    }
    meter_msg // シンボル文字列返却
}

/// 指定ムーブのリカバリーフレーム情報からシンボル文字列生成
///
/// # 引数
/// * `move_info` - ムーブ情報構造体参照
///
/// # 戻り値
/// リカバリーフレームシンボル文字列
async fn recovery_frames(move_info: &MoveInfo) -> String {
    // Option<i32>の場合は文字列に変換
    let recovery_str = move_info
        .recovery
        .map_or("-".to_string(), |v| v.to_string());
    let recovery_vec = sep_frame_vec(&recovery_str).await; // リカバリーフレーム分割結果取得
    let mut meter_msg = String::new(); // メーター文字列初期化

    if recovery_vec.len() == 1 && recovery_vec[0] == "-" {
        meter_msg += "-"; // 単一ハイフン表現
    } else {
        let mut recovery_tilde = false; // チルダ有無判定フラグ初期化

        // 各エントリ処理ループ（添字利用）
        for x in 0..recovery_vec.len() {
            if let Ok(num) = recovery_vec[x].parse::<u16>() {
                // 数値エントリの場合、数値分ループ
                for _ in 0..num {
                    if recovery_tilde {
                        // チルダ内：前エントリとの差分回数青菱形追加
                        for _ in 0..((recovery_vec[x].parse::<u16>().unwrap())
                            - (recovery_vec[x - 2].parse::<u16>()).unwrap())
                        {
                            meter_msg += BLUE_DIAMOND; // チルダ内青菱形追加
                        }
                        break; // ループ中断
                    }
                    meter_msg += BLUE_DIAMOND; // チルダ前：青菱形追加
                }
            }
            // 数値以外のエントリの場合
            else {
                meter_msg.push_str(&recovery_vec[x]); // 記号追加
                                                      // チルダ・括弧判定更新
                recovery_tilde = recovery_vec[x] == "~" || recovery_vec[x] == "(";
            }
        }
    }
    meter_msg // シンボル文字列返却
}

/// 指定文字列を分割しセパレータを保持したベクター返却
///
/// # 引数
/// * `text` - 分割対象文字列
///
/// # 戻り値
/// 分割結果ベクター（空文字・"total"除去済）
async fn sep_frame_vec(text: &str) -> Vec<String> {
    let mut result = Vec::new(); // 分割結果格納ベクター初期化
    let mut last = 0; // 前回インデックス保持用

    // セパレータ（英数字以外）で分割処理
    for (index, matched) in text.match_indices(|c: char| !(c.is_alphanumeric())) {
        if last != index {
            result.push(text[last..index].to_string()); // 文字列部分抽出
        }
        result.push(matched.to_string()); // セパレータ抽出
        last = index + matched.len(); // インデックス更新
    }
    if last < text.len() {
        result.push(text[last..].to_string()); // 残余部分抽出
    }

    // 空文字および "total" 削除処理（複数エントリの場合）
    if result.len() > 1 {
        'outer: loop {
            let cur_it_len = result.len(); // 現在長さ保持

            // 各エントリ検査ループ
            for r in 0..result.len() - 1 {
                if result[r].to_lowercase() == "total" || result[r].is_empty() || result[r] == " " {
                    result.remove(r); // 不要項目削除
                    break; // 削除後ループ再開
                }
            }

            if cur_it_len == result.len() {
                break 'outer; // 安定状態判定
            }
        }
    }
    result // 分割結果返却
}

/// キャラクターデータを読み込む関数
///
/// # 引数
/// * `character` - ユーザーが入力したキャラクター名
/// * `ctx` - コマンドコンテキスト
///
/// # 戻り値
/// 成功時は正式なキャラクター名、失敗時はエラー
async fn load_character_data(character: &str, ctx: &Context<'_>) -> Result<String, AppError> {
    // キャラクター探索処理（エイリアス対応）
    let character_arg_altered = match find::find_character(&character.to_string()).await {
        Ok(character_arg_altered) => character_arg_altered, // キャラクター名称確定
        Err(err) => {
            ctx.say(err.to_string()).await?; // エラーメッセージ送信
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red()); // エラー出力
            return Err(AppError::CharacterNotFound(err.to_string())); // エラー時早期返却
        }
    };

    Ok(character_arg_altered)
}

/// 技データのJSONファイルを読み込み、技情報を取得する
///
/// # 引数
/// * `character_arg_altered` - 正式なキャラクター名
/// * `character_move` - ユーザーが入力した技名
/// * `ctx` - コマンドコンテキスト
///
/// # 戻り値
/// (技情報配列, 技のインデックス)
async fn load_move_data(
    character_arg_altered: &str,
    character_move: &str,
    ctx: &Context<'_>,
) -> Result<(Vec<MoveInfo>, usize), AppError> {
    // キャラクターJSONパス取得
    let char_file_path =
        "data/".to_owned() + character_arg_altered + "/" + character_arg_altered + ".json";

    // ファイル読み込み
    let char_file_data = fs::read_to_string(char_file_path.clone()).map_err(|_| {
        AppError::FileNotFound(format!(
            "Failed to read '{character_arg_altered}.json' file."
        ))
    })?;

    // JSONデシリアライズ　技データ抽出
    let moves_info: Vec<MoveInfo> =
        serde_json::from_str(&char_file_data).map_err(AppError::Json)?;

    // 読み込み成功表示
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
        Ok(idx) => idx, // 技インデックス取得
        Err(err) => {
            // エラー表示
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.")
                .await?;
            return Err(AppError::MoveNotFound(err.to_string()));
        }
    };

    Ok((moves_info, index))
}

/// 技情報から適切な画像URLを検索する
///
/// # 引数
/// * `character_arg_altered` - 正式なキャラクター名
/// * `selected_move_info` - 選択された技情報
///
/// # 戻り値
/// 画像のURL
async fn find_move_image(
    character_arg_altered: &str,
    selected_move_info: &MoveInfo,
) -> Result<String, AppError> {
    // 画像JSONパス
    let image_json_path = "data/".to_owned() + character_arg_altered + "/images.json";

    // 画像JSONファイル読み込み
    let image_links = fs::read_to_string(image_json_path).map_err(|_| {
        AppError::FileNotFound(format!(
            "Failed to read '{character_arg_altered}' images.json file."
        ))
    })?;

    // 画像情報デシリアライズ
    let image_links =
        serde_json::from_str::<Vec<ImageLinks>>(&image_links).map_err(AppError::Json)?;

    let mut embed_image = String::new(); // 埋め込み画像初期化

    // 括弧を除去した技名を作成（例：「2d(2d)」→「2d」）
    let cleaned_input = if selected_move_info.input.contains('(') {
        selected_move_info
            .input
            .split('(')
            .next()
            .unwrap_or("")
            .trim()
            .to_string()
    } else {
        selected_move_info.input.to_string()
    };

    // 括弧内のコマンドを取得（例：「2d(2d)」→「2d」）
    let bracket_content =
        if selected_move_info.input.contains('(') && selected_move_info.input.contains(')') {
            let start = selected_move_info.input.find('(').unwrap_or(0) + 1;
            let end = selected_move_info
                .input
                .find(')')
                .unwrap_or(selected_move_info.input.len());
            if start < end {
                selected_move_info.input[start..end].to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

    // ムーブ画像送信処理
    for img_links in &image_links {
        // 完全一致、括弧を除去した技名との一致、または括弧内のコマンドとの一致
        if selected_move_info.input.to_lowercase() == img_links.input.to_lowercase()
            || (!cleaned_input.is_empty()
                && cleaned_input.to_lowercase() == img_links.input.to_lowercase())
            || (!bracket_content.is_empty()
                && bracket_content.to_lowercase() == img_links.input.to_lowercase())
        {
            // ヒット判定
            println!(
                "{}",
                ("Successfully read move '".to_owned()
                    + &selected_move_info.input
                    + "' in '"
                    + character_arg_altered
                    + ".json' file.")
                    .green()
            ); // 成功出力

            embed_image = if img_links.move_img.is_empty() {
                String::from(IMAGE_DEFAULT) // デフォルト画像設定
            } else {
                img_links.move_img.clone() // ムーブ画像設定
            };
            break; // 検索終了
        }
    }

    // 画像が見つからなかった場合、部分一致で再検索
    if embed_image.is_empty() {
        for img_links in &image_links {
            if img_links
                .input
                .to_lowercase()
                .contains(&selected_move_info.input.to_lowercase())
                && !img_links.move_img.is_empty()
            {
                embed_image = img_links.move_img.clone();
                break;
            }
        }
    }

    // デフォルト画像がセットされていなかった場合
    if embed_image.is_empty() {
        embed_image = String::from(IMAGE_DEFAULT);
    }

    Ok(embed_image)
}

/// 技情報と画像を検索する関数
///
/// # 引数
/// * `character_arg_altered` - 正式なキャラクター名
/// * `character_move` - ユーザーが入力した技名
/// * `ctx` - コマンドコンテキスト
///
/// # 戻り値
/// (技情報, 画像URL)のタプル
async fn find_move_and_images(
    character_arg_altered: &str,
    character_move: &str,
    ctx: &Context<'_>,
    _: &str, // character引数は使用しないため無名化
) -> Result<(MoveInfo, String), AppError> {
    // 技データの読み込みと検索
    let (moves_info, index) = load_move_data(character_arg_altered, character_move, ctx).await?;

    // 選択された技情報の取得
    let selected_move_info = moves_info[index].clone();

    // 画像の検索
    let embed_image = find_move_image(character_arg_altered, &selected_move_info).await?;

    Ok((selected_move_info, embed_image))
}

/// フレームメーター表示用の埋め込みメッセージを作成する関数
///
/// # 引数
/// * `move_info` - 技情報
/// * `embed_image` - 埋め込む画像のURL
/// * `character_arg_altered` - 正式なキャラクター名
///
/// # 戻り値
/// 埋め込みメッセージのベクター
async fn create_meter_embeds(
    move_info: &MoveInfo,
    embed_image: &str,
    character_arg_altered: &str,
) -> Vec<CreateEmbed> {
    // フレームメーター文字列生成処理
    let mut meter_msg = String::from("`"); // バッククォート開始
    meter_msg += &startup_frames(move_info).await; // 開始フレーム処理
    meter_msg += &active_frames(move_info).await; // アクティブフレーム処理
    meter_msg += &recovery_frames(move_info).await; // リカバリーフレーム処理
    meter_msg += "`"; // バッククォート終了

    let embed_title = "__**".to_owned() + &move_info.input + "**__"; // 埋め込みタイトル生成

    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + character_arg_altered + "#Overview"; // 埋め込みURL生成

    let embed = CreateEmbed::new()
        .color(EMBED_COLOR) // 埋め込み色設定
        .title(embed_title) // タイトル設定
        .url(embed_url) // URL設定
        .fields(vec![
            ("Startup", &startup_frames(move_info).await, true), // 開始フレームフィールド
            ("Active", &active_frames(move_info).await, true),   // アクティブフレームフィールド
            ("Recovery", &recovery_frames(move_info).await, true), // リカバリーフレームフィールド
        ])
        .image(embed_image); // 画像設定

    let embed2 = CreateEmbed::new()
        .color(EMBED_COLOR) // 埋め込み色設定
        .description(&meter_msg); // 説明文設定

    vec![embed, embed2] // 埋め込みベクター作成
}

/// ムーブのフレームメーターを視覚表示するコマンド処理
///
/// # 引数
/// * `ctx` - コマンド実行コンテキスト
/// * `character` - キャラクター名またはニックネーム
/// * `character_move` - ムーブ名・入力またはエイリアス
///
/// # 戻り値
/// 処理結果 `Result<(), AppError>`
#[poise::command(prefix_command, slash_command)]
pub async fn meter(
    ctx: Context<'_>, // コマンドコンテキスト
    #[min_length = 2]
    #[description = "Character name or nickname."]
    character: String, // キャラクター指定文字列
    #[min_length = 2]
    #[rename = "move"]
    #[description = "Move name, input or alias."]
    character_move: String, // ムーブ指定文字列
) -> Result<(), AppError> {
    println!(
        "{}",
        ("Command Args: '".to_owned() + &character + ", " + &character_move + "'").purple()
    ); // コマンド引数出力

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
        return Ok(()); // チェック失敗時早期返却
    }

    // キャラクターデータ読み込み
    let Ok(character_arg_altered) = load_character_data(&character, &ctx).await else {
        return Ok(());
    };

    // 技情報と画像データ読み込み
    let Ok((selected_move_info, embed_image)) =
        find_move_and_images(&character_arg_altered, &character_move, &ctx, &character).await
    else {
        return Ok(());
    };

    // 埋め込みメッセージ作成
    let vec_embeds =
        create_meter_embeds(&selected_move_info, &embed_image, &character_arg_altered).await;

    // 返信作成と送信
    let mut reply = poise::CreateReply::default(); // 返信オブジェクト初期化
    reply.embeds.extend(vec_embeds); // 埋め込み追加
    ctx.send(reply).await?; // 返信送信

    Ok(()) // 正常終了
}
