//! hitboxes.rs
//!
//! ヒットボックス画像表示モジュール  
//! Discord コマンド /hitboxes 実装モジュール  
//! 指定されたキャラクターの技のヒットボックス画像を取得し、表示する処理を提供  

// 必要なモジュールや型をインポートする
use crate::{check, find, HITBOX_DEFAULT};
// - check: 入力チェックなどの補助関数群
// - find: キャラクターや技情報の検索処理
// - HITBOX_DEFAULT: 画像が存在しない場合のデフォルトヒットボックス画像の URL

use crate::{Context, Error, ImageLinks, MoveInfo};
// - Context: Discord コマンド実行時のコンテキスト（ユーザー、チャンネル情報など）
// - Error: 汎用エラー型
// - ImageLinks: 画像関連データ（技画像、ヒットボックス画像など）の構造体
// - MoveInfo: 各技の情報（ダメージ、フレーム数など）を保持する構造体

use colored::Colorize;
// コンソール出力に色付けを行うための拡張メソッド

use std::{fs, string::String};
// 標準ライブラリのファイル操作と文字列操作用モジュール

/// キャラクターの技に対するヒットボックス画像を表示する処理
///
/// 入力されたキャラクター名（またはニックネーム）と技名（またはエイリアス）を基に、
/// 対象キャラクターの JSON ファイルから技情報を取得し、
/// 対応する画像 JSON からヒットボックス画像のリンクを検索、
/// 見つかった場合はその画像リンク（またはデフォルト画像）を Discord に送信する。
#[allow(unused_assignments)]
#[poise::command(prefix_command, slash_command, aliases("h"))]
pub async fn hitboxes(
    ctx: Context<'_>, // Discord コマンド実行時のコンテキスト（ユーザー、チャンネル情報など）
    #[description = "Character name or nickname."] character: String, // キャラクター名またはニックネーム
    #[description = "Move name, input or alias."] mut character_move: String, // 技名、入力、またはエイリアス（必要に応じて変更可能）
) -> Result<(), Error> {
    // コマンド実行時の引数を紫色でコンソール出力（デバッグ用）
    println!(
        "{}",
        ("Command Args: '".to_owned() + &character + ", " + &character_move + "'").purple()
    );

    // ユーザー入力がエイリアスの場合に、正式なキャラクター名を保持するための変数を初期化
    let mut character_arg_altered = String::new();

    // 各種入力チェック・環境整合性確認を実施
    // check::adaptive_check で、キャラクター名、技名、必要なファイルの存在などを確認する
    if (check::adaptive_check(
        ctx,
        (true, &character),      // キャラクター名のチェック有効
        (true, &character_move), // 技名のチェック有効
        true,                    // データフォルダの存在チェック
        true,                    // nicknames.json の存在チェック
        true,                    // キャラクターフォルダの存在チェック
        true,                    // キャラクター JSON の存在チェック
        true,                    // 画像 JSON の存在チェック
    )
    .await)
        .is_err()
    {
        // チェックに失敗した場合は以降の処理をスキップし、正常終了として返す
        return Ok(());
    }

    // キャラクター検索処理
    // find::find_character を利用し、入力された文字列から正式なキャラクター名を取得する
    character_arg_altered = match find::find_character(&character).await {
        Ok(name) => name, // 検索成功時、正式名を返す
        Err(err) => {
            // 検索失敗時、Discord にエラーメッセージを送信し、コンソールにエラー出力
            ctx.say(err.to_string()).await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(());
        }
    };

    // キャラクター JSON ファイルのパスを組み立てる
    // 例: "data/Baiken/Baiken.json"
    let char_file_path =
        "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    // 指定したファイルを読み込み、ファイル内容（JSON文字列）を取得
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character_arg_altered + ".json" + "' file."));

    // 読み込んだ JSON 文字列を、MoveInfo 型の Vec にデシリアライズする
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    // JSON の読み込み成功をコンソールに緑色で出力
    println!(
        "{}",
        ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green()
    );

    // 入力された技名（またはエイリアス）から、対象技のインデックスを検索する
    let mframes_index =
        find::find_move_index(&character_arg_altered, character_move, &moves_info).await;
    let mframes_index = match mframes_index {
        Ok(index) => index, // 検索成功時は (index, normalized_move) のタプルを返す
        Err(err) => {
            // 検索失敗時はエラーメッセージを送信し、Discord 上で `/moves` コマンドの利用を促す
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.")
                .await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(());
        }
    };

    // TODO: 現在、character_move 変数を上書きしているため、改善の余地あり
    character_move = mframes_index.1;

    // キャラクターの画像データが格納された JSON ファイルのパスを組み立て、読み込む
    let image_links = fs::read_to_string(
        "data/".to_owned() + &character_arg_altered + "/images.json",
    )
    .expect(
        &("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."),
    );

    // 読み込んだ画像 JSON を、ImageLinks 型の Vec にデシリアライズする
    let image_links = serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();

    // 対象の技情報を取得する（find::find_move_index で得たインデックスを利用）
    let mframes = &moves_info[mframes_index.0];

    // 対象技の読み込み成功を緑色のログで出力する
    println!(
        "{}",
        ("Successfully read move '".to_owned()
            + &mframes.input.to_string()
            + "' in '"
            + &character_arg_altered
            + ".json' file.")
            .green()
    );

    // 以下、ヒットボックス画像を Discord に送信する処理
    // 対象技の画像リンクが格納された JSON 内の各エントリについてループ
    for img_links in image_links {
        // 対象技と一致するエントリがあれば
        if mframes.input == img_links.input {
            // 成功ログを緑色で出力
            println!(
                "{}",
                ("Successfully read move '".to_owned()
                    + &mframes.input.to_string()
                    + "' in '"
                    + &character_arg_altered
                    + ".json' file.")
                    .green()
            );

            // もしヒットボックス画像が存在する場合（配列の先頭要素が空でない）
            if !img_links.hitbox_img[0].is_empty() {
                // Discord に送信するメッセージを作成（対象技の名前を強調表示）
                let bot_msg = "__**Move: ".to_owned() + &img_links.input + "**__";
                ctx.say(&bot_msg).await?;

                // 配列内の各ヒットボックス画像リンクを順次 Discord チャンネルに送信する
                for htbx_img in img_links.hitbox_img {
                    ctx.channel_id().say(ctx, &htbx_img).await?;
                }
            } else {
                // ヒットボックス画像が存在しない場合は、デフォルトの画像（HITBOX_DEFAULT）を送信する
                let bot_msg = "__**Move: ".to_owned() + &img_links.input + "**__";
                ctx.say(&bot_msg).await?;
                ctx.channel_id().say(ctx, HITBOX_DEFAULT).await?;
            }
        }
    }

    Ok(()) // 正常終了を返す
}
