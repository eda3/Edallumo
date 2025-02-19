//! fmeter.rs
//!
//! フレームメーター表示モジュール  
//! Discord コマンド /fmeter 実装モジュール  
//! JSON ファイルから技のフレームデータを取得し、各フレームの情報を絵文字で視覚化して送信する処理を提供  

// 必要なモジュールや型をインポートする
use crate::{check, find, IMAGE_DEFAULT}; // 入力チェック、キャラクター検索、デフォルト画像定数
use crate::{Context, Error, ImageLinks, MoveInfo}; // コマンド実行用コンテキスト、エラー型、画像リンク構造体、技情報構造体
use colored::Colorize; // コンソール出力に色を付けるための拡張メソッド
use std::{fs, string::String}; // ファイル操作および文字列操作

// フレームメーター表示で使用する絵文字の定数定義
const GREEN_CIRCLE: &str = "🟢"; // 発生フレームの表現用
const RED_SQUARE: &str = "🟥"; // 持続フレームの表現用
const BLUE_DIAMOND: &str = "🔷"; // 後隙（硬直）フレームの表現用

/// 指定された技のフレームメーターを表示する処理
///
/// 入力されたキャラクター名と技名（またはそのエイリアス）に基づいて、
/// 対応する技情報を JSON ファイルから読み込み、各フレームの数値情報を絵文字で視覚化して表示する。
#[allow(unused_assignments)]
#[poise::command(prefix_command, slash_command, aliases("fm"))]
pub async fn fmeter(
    ctx: Context<'_>, // Discord コマンド実行時のコンテキスト（ユーザー、チャンネル情報など）
    #[description = "Character name or nickname."] character: String, // キャラクター名またはニックネーム
    #[description = "Move name, input or alias."] mut character_move: String, // 技名、入力、またはエイリアス（変更可能）
) -> Result<(), Error> {
    // コマンド実行時の引数をログ出力（紫色で強調）
    println!(
        "{}",
        ("Command Args: '".to_owned() + &character + ", " + &character_move + "'").purple()
    );

    // ユーザー入力がエイリアスの場合、正式なキャラクター名を保持する変数（初期は空文字）
    let mut character_arg_altered = String::new();

    // 入力チェック・環境整合性確認のための複数条件チェックを実施する
    // チェックに失敗した場合は、処理を中断して正常終了とする
    if (check::adaptive_check(
        ctx,
        (true, &character),      // キャラクター名チェックを有効化
        (true, &character_move), // 技名チェックを有効化
        true,                    // データフォルダの存在確認
        true,                    // nicknames.json の存在確認
        true,                    // 各キャラクターフォルダの存在確認
        true,                    // 各キャラクター JSON の存在確認
        true,                    // 画像 JSON の存在確認
    )
    .await)
        .is_err()
    {
        return Ok(());
    }

    // キャラクター検索処理：入力された文字列から正式なキャラクター名を取得する
    character_arg_altered = match find::find_character(&character).await {
        Ok(name) => name, // 検索成功時は正式名を代入
        Err(err) => {
            // 検索失敗時はエラーメッセージを Discord チャンネルに送信し、エラー内容をコンソールに表示する
            ctx.say(err.to_string()).await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(());
        }
    };

    // キャラクター JSON ファイルのパスを組み立て、内容を読み込む
    let char_file_path =
        "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character + ".json" + "' file."));

    // JSON 文字列を技情報（MoveInfo）のベクターにデシリアライズする
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    // 読み込み成功をコンソールに緑色で表示
    println!(
        "{}",
        ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green()
    );

    // 入力された技名（またはエイリアス）から、対象技のインデックスと正規化された技名を取得する
    let mframes_index =
        find::find_move_index(&character_arg_altered, character_move, &moves_info).await;
    let mframes_index = match mframes_index {
        Ok(index) => index, // 検索成功時はインデックスと正規技名を返す
        Err(err) => {
            // 検索失敗時はエラーメッセージを送信し、エラー内容をログ出力する
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.")
                .await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(());
        }
    };

    // TODO: 現在の処理では character_move を再代入しているため、改善の余地あり
    character_move = mframes_index.1;

    // 画像 JSON ファイルのパスを組み立て、内容を読み込む
    let image_links = fs::read_to_string(
        "data/".to_owned() + &character_arg_altered + "/images.json",
    )
    .expect(
        &("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."),
    );

    // 画像 JSON の内容を ImageLinks のベクターにデシリアライズする
    let image_links = serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();

    // 対象技の情報を取得する（先ほどのインデックスを利用）
    let mframes = &moves_info[mframes_index.0];

    // 対象技に対応する画像リンクを探すため、images.json の各エントリを順次確認する
    for img_links in image_links {
        // JSON 内の技入力が対象技の入力と一致した場合
        if mframes.input == img_links.input {
            // 一致した場合、コンソールに成功メッセージを表示する
            println!(
                "{}",
                ("Successfully read move '".to_owned()
                    + &mframes.input.to_string()
                    + "' in '"
                    + &character_arg_altered
                    + ".json' file.")
                    .green()
            );

            // 技画像が存在する場合
            if !img_links.move_img.is_empty() {
                // Discord に送信するメッセージを組み立て、技画像 URL を送信する
                let bot_msg = "__**Move: ".to_owned() + &img_links.input + "**__";
                ctx.say(&bot_msg).await?;
                ctx.channel_id().say(ctx, &img_links.move_img).await?;
            } else {
                // 技画像が存在しない場合、デフォルト画像（IMAGE_DEFAULT）を送信する
                let bot_msg = "__**Move: ".to_owned() + &img_links.input + "**__";
                ctx.say(&bot_msg).await?;
                ctx.channel_id().say(ctx, IMAGE_DEFAULT).await?;
            }
        }
    }

    // フレームメーター表示用メッセージの初期部分を組み立てる
    // ここでは「発生」フレーム（startup）が表示される
    let mut frame_meter_msg = r#"__発生__: "#.to_owned() + &mframes.startup + " → `";

    // 発生フレームの情報を個別に分割してベクター化する処理（sep_frame_vec 関数）
    let startup_vec = sep_frame_vec(&mframes.startup).await;
    // println!("startup_vec: {:?}", startup_vec); // デバッグ用

    // 発生フレームが "-" または 1 のみの場合、表示内容を "-" にする
    if (startup_vec.len() == 1 && startup_vec[0] == "-")
        || (startup_vec.len() == 1 && startup_vec[0].parse::<i8>().unwrap() == 1)
    {
        frame_meter_msg += "-";
    }
    // 複数フレームの場合、各フレームを絵文字で視覚化するロジック
    else {
        // ブラケット（角括弧等）が現れたかどうかを判定するフラグ
        let mut startup_bra = false;

        // 発生フレームの各部分について処理を行う
        for x in 0..startup_vec.len() {
            // 数字に変換可能な場合、フレーム数として扱う
            if let Ok(num) = startup_vec[x].parse::<i8>() {
                // 数字の値 - 1 回、GREEN_CIRCLE を表示（ただし、括弧がある場合は別処理）
                for _ in 0..num - 1 {
                    if !startup_bra {
                        frame_meter_msg += GREEN_CIRCLE;
                    } else {
                        // 括弧がある場合、前の値との差分だけ GREEN_CIRCLE を追加
                        for _ in 0..((startup_vec[x].parse::<i8>().unwrap())
                            - (startup_vec[x - 2].parse::<i8>()).unwrap())
                        {
                            frame_meter_msg += GREEN_CIRCLE;
                        }
                        break;
                    }
                }
            }
            // 数字でない場合、記号などとしてそのまま表示
            else {
                // "+" 記号の場合、次の数字が 1 なら GREEN_CIRCLE に置き換え、それ以外なら "+" を残す
                if x == startup_vec.len() - 2 && startup_vec[x] == "+" {
                    if let Ok(num) = startup_vec[x + 1].parse::<i8>() {
                        if num == 1 {
                            frame_meter_msg += GREEN_CIRCLE;
                        } else {
                            frame_meter_msg = frame_meter_msg + GREEN_CIRCLE + &startup_vec[x];
                        }
                    } else {
                        frame_meter_msg = frame_meter_msg + &startup_vec[x];
                    }
                }
                // それ以外の記号はそのまま表示
                else {
                    frame_meter_msg = frame_meter_msg + &startup_vec[x];
                }

                // "[" または "~" を検出した場合は、ブロック開始フラグを立てる
                if startup_vec[x] == "[" || startup_vec[x] == "~" {
                    startup_bra = true;
                } else if startup_vec[x] == "]" {
                    // "]" を検出したらフラグを解除
                    startup_bra = false;
                }
            }
        }
    }

    // 「持続」フレーム（active）の表示開始
    frame_meter_msg = frame_meter_msg + "`\n__持続__: " + &mframes.active + " → `";

    // 持続フレームの文字列を分割してベクターに変換
    let active_vec = sep_frame_vec(&mframes.active).await;
    // println!("Active vec: {:?}", active_vec); // デバッグ用

    // 持続フレームが "-" のみの場合は "-" を表示
    if active_vec.len() == 1 && active_vec[0] == "-" {
        frame_meter_msg += "-";
    } else {
        // 括弧の開始があったかどうかを判定するフラグ（ヒット時・リカバリの場合）
        let mut hit_recovery = false;

        // 持続フレームの各要素について処理
        for active_vec_string in &active_vec {
            if let Ok(num) = active_vec_string.parse::<i8>() {
                // 数字の場合、数字の分だけ RED_SQUARE または BLUE_DIAMOND を追加
                for _ in 0..num {
                    if !hit_recovery {
                        frame_meter_msg += RED_SQUARE;
                    } else {
                        frame_meter_msg += BLUE_DIAMOND;
                    }
                }
            } else {
                // 数字以外（記号など）の場合はそのまま追加
                frame_meter_msg = frame_meter_msg + &active_vec_string;
                // "(" があれば hit_recovery フラグを立て、")" で解除
                if active_vec_string == "(" {
                    hit_recovery = true;
                } else if active_vec_string == ")" {
                    hit_recovery = false;
                }
            }
        }
    }

    // 「硬直」フレーム（recovery）の表示開始
    frame_meter_msg = frame_meter_msg + "`\n__硬直__: " + &mframes.recovery + " → `";

    // リカバリーフレームの文字列を分割してベクターに変換
    let recovery_vec = sep_frame_vec(&mframes.recovery).await;

    if recovery_vec.len() == 1 && recovery_vec[0] == "-" {
        frame_meter_msg += "-";
    } else {
        // "~" 記号などの処理のためのフラグ
        let mut recovery_tilde = false;

        // リカバリーフレーム各要素の処理
        for x in 0..recovery_vec.len() {
            if let Ok(num) = recovery_vec[x].parse::<i8>() {
                for _ in 0..num {
                    if !recovery_tilde {
                        frame_meter_msg += BLUE_DIAMOND;
                    } else {
                        for _ in 0..((recovery_vec[x].parse::<i8>().unwrap())
                            - (recovery_vec[x - 2].parse::<i8>()).unwrap())
                        {
                            frame_meter_msg += BLUE_DIAMOND;
                        }
                        break;
                    }
                }
            } else {
                frame_meter_msg = frame_meter_msg + &recovery_vec[x];
                // "(" または "~" を検出した場合、フラグを立てる
                recovery_tilde = recovery_vec[x] == "~" || recovery_vec[x] == "(";
            }
        }
    }

    // 最終的に組み立てたフレームメーター文字列の末尾にバッククォートを追加
    frame_meter_msg += "`";

    // Discord のチャネルにフレームメーターのメッセージを送信する
    ctx.channel_id().say(ctx, frame_meter_msg).await?;

    // （デバッグ用出力文はコメントアウト済み）
    // println!("始動: {:?}", startup_vec);
    // println!("Active: {:?}", active_vec);
    // println!("Recovery: {:?}", recovery_vec);

    Ok(()) // 正常終了
}

/// 指定された文字列を分割し、区切り文字も保持して Vec<String> に変換する非同期処理
///
/// 例： "236K" → 数字部分と記号部分に分割して ["236", "K"] のようなベクターに変換
async fn sep_frame_vec(text: &String) -> Vec<String> {
    // 結果を格納するための空のベクターを用意
    let mut result = Vec::new();
    // 分割開始位置を記録する変数
    let mut last = 0;

    // 文字列内の各文字について、英数字以外の文字（区切り文字）を見つける
    // match_indices で、区切り文字の位置とその文字列を取得する
    for (index, matched) in text.match_indices(|c: char| !(c.is_alphanumeric())) {
        // 直前の位置から区切り文字の前までの部分文字列が空でなければ、結果に追加
        if last != index {
            result.push(text[last..index].to_string());
        }
        // 区切り文字そのものも結果に追加
        result.push(matched.to_string());
        // 次の部分文字列の開始位置を更新
        last = index + matched.len();
    }
    // 最後の部分が残っていれば追加
    if last < text.len() {
        result.push(text[last..].to_string());
    }

    // 結果ベクター内の不要な空文字列や "total"（大文字・小文字を問わず）を除去するループ
    if result.len() > 1 {
        'outer: loop {
            let cur_it_len = result.len();
            for r in 0..result.len() - 1 {
                if result[r].to_lowercase() == "total" || result[r].is_empty() || result[r] == " " {
                    // 条件に一致した要素を削除し、ループ内で再処理する
                    result.remove(r);
                    break;
                }
            }
            // 変更がなければループを抜ける
            if cur_it_len == result.len() {
                break 'outer;
            }
        }
    }

    result // 分割結果のベクターを返す
}
