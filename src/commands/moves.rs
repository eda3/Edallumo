//! # moves.rs
//!
//! キャラクター技一覧・エイリアス表示モジュール  
//! Discordコマンド /moves 実装モジュール  
//! JSONファイルより技情報・エイリアス情報取得、複数メッセージ送信処理

// 必要なモジュールや型をインポートする
use crate::{check, find};
// - check: 入力の検証や各種ファイル・ディレクトリの存在確認用関数群
// - find: キャラクター名や技情報検索用関数群

use crate::{Context, Error, MoveAliases, MoveInfo};
// - Context: コマンド実行時のコンテキスト（ユーザー、チャンネル等の情報）
// - Error: 汎用エラー型
// - MoveAliases: 技のエイリアス情報保持構造体
// - MoveInfo: キャラクター技詳細情報保持構造体（ダメージ、入力、フレーム数等）

use colored::Colorize;
// コンソール出力時、文字列に色付けする拡張メソッド提供

use std::{fs, string::String};
// ファイル操作用fsモジュール、文字列操作用String型インポート

/// キャラクターの技一覧を表示
#[poise::command(prefix_command, slash_command, aliases("m"))]
pub async fn moves(
    ctx: Context<'_>, // Discordコマンド実行時コンテキスト
    #[description = "Character name or nickname."] character: String, // ユーザー入力キャラクター名またはニックネーム
) -> Result<(), Error> {
    // 入力キャラクター名紫色表示（デバッグ用）
    println!(
        "{}",
        ("Command Args: '".to_owned() + &character + "'").purple()
    );

    // 入力検証・必要ファイル存在確認処理
    // adaptive_check: キャラクター名存在、ファイル整合性確認（失敗時中断）
    if (check::adaptive_check(
        ctx,
        (true, &character),      // キャラクター名正当性チェック有効
        (false, &String::new()), // 技名チェック不要（未使用）
        true,                    // データフォルダ存在チェック
        true,                    // nicknames.json存在チェック
        true,                    // キャラクターフォルダ存在チェック
        true,                    // キャラクターJSON存在チェック
        false,                   // 画像JSON存在チェック不要
    )
    .await)
        .is_err()
    {
        return Ok(());
    }

    // キャラクター検索処理
    // find::find_characterにより、入力キャラクター（またはニックネーム）から正式名取得
    let character_arg_altered = match find::find_character(&character).await {
        Ok(name) => name, // 検索成功：正式キャラクター名取得
        Err(err) => {
            // 検索失敗：Discordへエラーメッセージ送信、コンソール赤色出力
            ctx.say(err.to_string()).await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(());
        }
    };

    // キャラクターJSONファイルパス組立
    // 例: "data/Baiken/Baiken.json"
    let char_file_path =
        "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    // JSONファイル読み込み処理
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character + ".json" + "' file."));

    // JSON文字列をMoveInfo型Vecにデシリアライズ
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    // 読み込み成功緑色表示（コンソール出力）
    println!(
        "{}",
        ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green()
    );

    // エイリアス情報JSONファイルパス組立
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
    // エイリアスJSONファイル読み込み
    let aliases_data = fs::read_to_string(&aliases_path)
        .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));
    // JSON文字列をMoveAliases型Vecにデシリアライズ
    let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

    // Discord送信用メッセージ文字列初期化
    // キャラクター名のアンダースコアをスペースに置換し、見やすい表示にする
    let mut moves_as_msg = "__**".to_string()
        + &character_arg_altered.replace('_', " ")
        + " Moves / Aliases**__\n
diff";

    // メッセージ文字数制限対応のため、最初の1/4分を作成
    for moves in moves_info.iter().take(moves_info.len() / 4) {
        // 各技名と入力情報を整形して追加
        moves_as_msg =
            moves_as_msg.to_owned() + "\n* Move: " + &moves.name + " -> Input: " + &moves.input;
        // 同じ技のエイリアス情報追記処理
        for moves_aliases in aliases_data.iter() {
            if moves.input == moves_aliases.input {
                moves_as_msg += "\n+ Aliases: ";
                // エイリアス複数時カンマ区切り整形
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a] + ", ";
                    } else {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a];
                    }
                }
            } else {
                continue;
            }
        }
        // 各技情報終了のピリオド追加
        moves_as_msg = moves_as_msg.to_owned() + ".";
    }
    // コードブロック終了記号追加（メッセージ終端マーク）
    moves_as_msg += "\n
";
    // 最初のメッセージをDiscordに送信
    ctx.say(&moves_as_msg).await?;

    // 2番目のメッセージ作成（全体の1/4～1/2部分）
    moves_as_msg = "
diff"
        .to_string();
    for moves in moves_info
        .iter()
        .take((moves_info.len() / 4) * 2)
        .skip(moves_info.len() / 4)
    {
        moves_as_msg =
            moves_as_msg.to_owned() + "\n* Move: " + &moves.name + " -> Input: " + &moves.input;
        for moves_aliases in aliases_data.iter() {
            if moves.input == moves_aliases.input {
                moves_as_msg += "\n+ Aliases: ";
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a] + ", ";
                    } else {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a];
                    }
                }
            } else {
                continue;
            }
        }
        moves_as_msg = moves_as_msg.to_owned() + ".";
    }
    moves_as_msg += "\n
";
    // 2番目のメッセージをDiscordチャネルに送信
    ctx.channel_id().say(ctx, &moves_as_msg).await?;

    // 3番目のメッセージ作成（全体の1/2～3/4部分）
    moves_as_msg = "
diff"
        .to_string();
    for moves in moves_info
        .iter()
        .take((moves_info.len() / 4) * 3)
        .skip((moves_info.len() / 4) * 2)
    {
        moves_as_msg =
            moves_as_msg.to_owned() + "\n* Move: " + &moves.name + " -> Input: " + &moves.input;
        for moves_aliases in aliases_data.iter() {
            if moves.input == moves_aliases.input {
                moves_as_msg += "\n+ Aliases: ";
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a] + ", ";
                    } else {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a];
                    }
                }
            } else {
                continue;
            }
        }
        moves_as_msg = moves_as_msg.to_owned() + ".";
    }
    moves_as_msg += "\n
";
    // 3番目のメッセージをDiscordチャネルに送信
    ctx.channel_id().say(ctx, &moves_as_msg).await?;

    // 4番目のメッセージ作成（残り、最後の1/4部分）
    moves_as_msg = "
diff"
        .to_string();
    for moves in moves_info.iter().skip((moves_info.len() / 4) * 3) {
        moves_as_msg =
            moves_as_msg.to_owned() + "\n* Move: " + &moves.name + " -> Input: " + &moves.input;
        for moves_aliases in aliases_data.iter() {
            if moves.input == moves_aliases.input {
                moves_as_msg += "\n+ Aliases: ";
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a] + ", ";
                    } else {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a];
                    }
                }
            } else {
                continue;
            }
        }
        moves_as_msg = moves_as_msg.to_owned() + ".";
    }
    moves_as_msg += "\n
";

    // 4番目のメッセージをDiscordチャネルに送信
    ctx.channel_id().say(ctx, &moves_as_msg).await?;

    // 正常終了の返却
    Ok(())
}
