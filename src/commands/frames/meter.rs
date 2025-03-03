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
                    if !startup_bra {
                        meter_msg += GREEN_CIRCLE; // 括弧前：緑丸追加
                    } else {
                        // 括弧内：前エントリとの差分回数緑丸追加
                        for _ in 0..((startup_vec[x].parse::<u16>().unwrap())
                            - (startup_vec[x - 2].parse::<u16>()).unwrap())
                        {
                            meter_msg += GREEN_CIRCLE; // 括弧内緑丸追加
                        }
                        break; // ループ中断
                    }
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
                            meter_msg = meter_msg + GREEN_CIRCLE + &startup_vec[x];
                            // 複数数値：緑丸＋"+"追加
                        }
                    } else {
                        meter_msg = meter_msg + &startup_vec[x]; // 数字変換失敗：記号そのまま追加
                    }
                }
                // その他の記号処理
                else {
                    meter_msg = meter_msg + &startup_vec[x]; // 記号追加
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
                    if !hit_recovery {
                        meter_msg += RED_SQUARE; // 括弧前：赤四角追加
                    } else {
                        meter_msg += BLUE_DIAMOND; // 括弧内：青菱形追加
                    }
                }
            }
            // 数値以外のエントリの場合
            else {
                meter_msg = meter_msg + active_vec_string; // 記号追加
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
                    if !recovery_tilde {
                        meter_msg += BLUE_DIAMOND; // チルダ前：青菱形追加
                    } else {
                        // チルダ内：前エントリとの差分回数青菱形追加
                        for _ in 0..((recovery_vec[x].parse::<u16>().unwrap())
                            - (recovery_vec[x - 2].parse::<u16>()).unwrap())
                        {
                            meter_msg += BLUE_DIAMOND; // チルダ内青菱形追加
                        }
                        break; // ループ中断
                    }
                }
            }
            // 数値以外のエントリの場合
            else {
                meter_msg = meter_msg + &recovery_vec[x]; // 記号追加
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

    if (check::adaptive_check(ctx, true, true, true, true, true).await).is_err() {
        return Ok(()); // チェック失敗時早期返却
    }

    // キャラクター探索処理（エイリアス対応）
    let character_arg_altered = match find::find_character(&character).await {
        Ok(character_arg_altered) => character_arg_altered, // キャラクター名称確定
        Err(err) => {
            ctx.say(err.to_string()).await?; // エラーメッセージ送信
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red()); // エラー出力
            return Ok(()); // エラー時早期返却
        }
    };

    // キャラクターファイルパス生成
    let char_file_path =
        "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json"; // JSONファイルパス生成
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character + ".json" + "' file.")); // ファイル読み込み

    // キャラクター情報デシリアライズ
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap(); // ムーブ情報抽出

    println!(
        "{}",
        ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green()
    ); // 成功出力

    // ムーブ探索処理（インデックス取得）
    let index =
        match find::find_move_index(&character_arg_altered, character_move, &moves_info).await {
            Ok(index) => index, // ムーブインデックス確定
            Err(err) => {
                ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.")
                    .await?; // エラーメッセージ送信
                println!("{}", ("Error: ".to_owned() + &err.to_string()).red()); // エラー出力
                return Ok(()); // エラー時早期返却
            }
        };

    // 画像情報ファイル読み込み
    let image_links = fs::read_to_string(
        "data/".to_owned() + &character_arg_altered + "/images.json",
    )
    .expect(
        &("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."),
    ); // 画像ファイル読み込み

    // 画像情報デシリアライズ
    let image_links = serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap(); // 画像リンク抽出
    let selected_move_info = &moves_info[index]; // 対象ムーブ情報取得
    let mut embed_image = String::new(); // 埋め込み画像初期化

    // ムーブ画像送信処理
    for img_links in image_links {
        // 画像リンク走査ループ
        if selected_move_info.input == img_links.input {
            // ヒット判定
            println!(
                "{}",
                ("Successfully read move '".to_owned()
                    + &selected_move_info.input.to_string()
                    + "' in '"
                    + &character_arg_altered
                    + ".json' file.")
                    .green()
            ); // 成功出力

            if !img_links.move_img.is_empty() {
                embed_image = img_links.move_img; // ムーブ画像設定
            } else {
                embed_image = String::from(IMAGE_DEFAULT); // デフォルト画像設定
            }
        }
    }

    // フレームメーター文字列生成処理
    let mut meter_msg = String::from("`"); // バッククォート開始
    meter_msg += &startup_frames(selected_move_info).await; // 開始フレーム処理
    meter_msg += &active_frames(selected_move_info).await; // アクティブフレーム処理
    meter_msg += &recovery_frames(selected_move_info).await; // リカバリーフレーム処理
    meter_msg += "`"; // バッククォート終了

    let embed_title = "__**".to_owned() + &selected_move_info.input + "**__"; // 埋め込みタイトル生成

    let embed_url =
        "https://dustloop.com/w/GGST/".to_owned() + &character_arg_altered + "#Overview"; // 埋め込みURL生成

    let embed = CreateEmbed::new()
        .color(EMBED_COLOR) // 埋め込み色設定
        .title(embed_title) // タイトル設定
        .url(embed_url) // URL設定
        .fields(vec![
            ("Startup", &startup_frames(selected_move_info).await, true), // 開始フレームフィールド
            ("Active", &active_frames(selected_move_info).await, true), // アクティブフレームフィールド
            ("Recovery", &recovery_frames(selected_move_info).await, true), // リカバリーフレームフィールド
        ])
        .image(embed_image); // 画像設定

    let embed2 = CreateEmbed::new()
        .color(EMBED_COLOR) // 埋め込み色設定
        .description(&meter_msg); // 説明文設定

    let vec_embeds = vec![embed, embed2]; // 埋め込みベクター作成
    let mut reply = poise::CreateReply::default(); // 返信オブジェクト初期化
    reply.embeds.extend(vec_embeds); // 埋め込み追加
    ctx.send(reply).await?; // 返信送信
    Ok(()) // 正常終了
}
