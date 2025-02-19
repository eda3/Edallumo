//! # check.rs
//!
//! 各種チェック機能提供モジュールである。  
//! ディレクトリ、ファイル、JSON の存在および正当性を確認する関数群を含む。

use crate::{Context, Error, Nicknames, CHARS}; // CHARS 定数：キャラクター名定数（クレートルートに定義されている前提）
use colored::Colorize; // ターミナル出力の色付けに利用するクレートである
use std::{fs, path::Path}; // ファイル操作およびパス操作用

/// データフォルダ "data" 存在チェック関数である。  
/// 引数：`init_check` - 初期チェックか否かの真偽値  
/// 戻り値：チェック成功時 None / エラー発生時エラーメッセージ (Some(String)) を返す。
pub async fn data_folder_exists(init_check: bool) -> Option<String> {
    // "data" フォルダ存在確認
    if Path::new("data").exists() {
        None
    } else {
        // "data" フォルダ未存在エラー用メッセージ
        let error_msg = "Error: The 'data' folder does not exist.\nDownload and import the 'data' folder from:\nhttps://github.com/yakiimoninja/baiken.".to_string();

        if init_check {
            // 初期チェック時、エラーメッセージをコンソール出力しパニック
            println!();
            panic!("{}", error_msg.red());
        } else {
            // Discord などで出力するためエラーメッセージを返す
            Some(error_msg)
        }
    }
}

/// nicknames.json 存在および正当性チェック関数である。  
/// 引数：`init_check` - 初期チェックか否かの真偽値  
/// 戻り値：正常時 None / エラー発生時エラーメッセージ (Some(String)) を返す。
pub async fn nicknames_json_exists(init_check: bool) -> Option<String> {
    // nicknames.json ファイル読み込み
    let data_from_file =
        fs::read_to_string("data/nicknames.json").expect("\nFailed to read 'nicknames.json' file.");

    // JSON デシリアライズ試行
    match serde_json::from_str::<Vec<Nicknames>>(&data_from_file) {
        Ok(_) => {
            println!("{}", "Successfully read 'nicknames.json' file.".green());
            None
        }
        Err(_) => {
            // nicknames.json 正当性エラー用メッセージ
            let error_msg = "Error: Failed to deserialize 'nicknames.json' file.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken.".to_string();

            if init_check {
                println!();
                panic!("{}", error_msg.red());
            } else {
                Some(error_msg)
            }
        }
    }
}

/// キャラクターフォルダ存在チェック関数である。  
/// CHARS 定数に基づき、各キャラクター用フォルダが "data" 内に存在するか確認する。  
/// 引数：`init_check` - 初期チェックか否かの真偽値  
/// 戻り値：正常時 None / エラー発生時エラーメッセージ (Some(String)) を返す。
pub async fn character_folders_exist(init_check: bool) -> Option<String> {
    // CHARS 内の各キャラクターについてフォルダ存在確認
    for char in CHARS {
        let character_path = &("data/".to_owned() + char);
        if !Path::new(&character_path).exists() {
            // キャラクターフォルダ未存在エラー用メッセージ
            let error_msg = "Error: Missing '".to_owned() + &character_path + "' folder.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken.";
            if init_check {
                println!();
                panic!("{}", error_msg.red());
            } else {
                return Some(error_msg);
            }
        }
    }
    None
}

/// キャラクター JSON 存在チェック関数である。  
/// 各キャラクター用フォルダ内に、キャラクター JSON ファイルが存在するか確認する。  
/// 引数：`init_check` - 初期チェックか否かの真偽値  
/// 戻り値：正常時 None / エラー発生時エラーメッセージ (Some(String)) を返す。
pub async fn character_jsons_exist(init_check: bool) -> Option<String> {
    // CHARS 内各キャラクターについて JSON ファイル存在確認
    for char in CHARS {
        let character_json = &("data/".to_owned() + char + "/" + char + ".json");
        if !Path::new(&character_json).exists() {
            // キャラクター JSON 未存在エラー用メッセージ
            let error_msg = "Error: Missing '".to_owned()
                + &character_json
                + "' file.\nPlease execute the '/update' command.";
            if init_check {
                println!();
                panic!("{}", error_msg.red());
            } else {
                return Some(error_msg);
            }
        }
    }
    println!(
        "{}",
        ("Successfully read ".to_owned() + &CHARS.len().to_string() + " character.json files.")
            .green()
    );
    None
}

/// キャラクター画像 JSON 存在チェック関数である。  
/// 各キャラクター用フォルダ内に、画像 JSON ファイルが存在するか確認する。  
/// 引数：`init_check` - 初期チェックか否かの真偽値  
/// 戻り値：正常時 None / エラー発生時エラーメッセージ (Some(String)) を返す。
pub async fn character_images_exist(init_check: bool) -> Option<String> {
    // CHARS 内各キャラクターについて画像 JSON ファイル存在確認
    for char in CHARS {
        let images_json = &("data/".to_owned() + char + "/images.json");
        if !Path::new(&images_json).exists() {
            // 画像 JSON 未存在エラー用メッセージ
            let error_msg = "Error: Missing '".to_owned() + &images_json + "' file.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken.";
            if init_check {
                println!();
                panic!("{}", error_msg.red());
            } else {
                return Some(error_msg);
            }
        }
    }
    None
}

/// キャラクター引数正当性チェック関数である。  
/// 引数の文字数が 2 文字未満の場合はエラー文字列を返す。  
/// 正常時は None を返す。
pub async fn correct_character_arg(character_arg: &String) -> Option<String> {
    if character_arg.len() < 2 {
        let error_msg = "Character name `".to_owned() + &character_arg + "` is invalid!";
        Some(error_msg)
    } else {
        None
    }
}

/// 技引数正当性チェック関数である。  
/// 引数の文字数が 2 文字未満の場合はエラー文字列を返す。  
/// 正常時は None を返す。
pub async fn correct_character_move_arg(character_move_arg: &String) -> Option<String> {
    if character_move_arg.len() < 2 {
        let error_msg = "Move `".to_owned() + &character_move_arg + "` is invalid!";
        Some(error_msg)
    } else {
        None
    }
}

/// Adaptive check 関数である。  
/// 複数のチェック関数を実行し、各チェック結果に基づきエラーメッセージ出力またはパニックを実施する。  
/// 引数：  
/// - `ctx` - コマンドコンテキスト  
/// - `correct_character_check` - (bool, &String) 型タプル  
/// - `correct_character_move_check` - (bool, &String) 型タプル  
/// - `data_folder_check` - データフォルダ存在チェック有無  
/// - `nicknames_json_check` - nicknames.json 存在チェック有無  
/// - `character_folders_check` - キャラクターフォルダ存在チェック有無  
/// - `character_jsons_check` - キャラクター JSON 存在チェック有無  
/// - `character_images_check` - 画像 JSON 存在チェック有無  
/// 戻り値：全チェック成功時 Ok(()) / 失敗時 Err("Failed adaptive_check")
#[allow(clippy::too_many_arguments)]
pub async fn adaptive_check(
    ctx: Context<'_>,
    correct_character_check: (bool, &String),
    correct_character_move_check: (bool, &String),
    data_folder_check: bool,
    nicknames_json_check: bool,
    character_folders_check: bool,
    character_jsons_check: bool,
    character_images_check: bool,
) -> Result<(), Error> {
    let mut checks_passed = true;

    if correct_character_check.0 {
        // キャラクター引数正当性チェック
        if let Some(error_msg) = correct_character_arg(correct_character_check.1).await {
            ctx.say(&error_msg).await?;
            println!("{}", ("Error: ".to_owned() + &error_msg).red());
            checks_passed = false;
        }
    }
    if correct_character_move_check.0 {
        // 技引数正当性チェック
        if let Some(error_msg) = correct_character_move_arg(correct_character_move_check.1).await {
            ctx.say(&error_msg).await?;
            println!("{}", ("Error: ".to_owned() + &error_msg).red());
            checks_passed = false;
        }
    }
    if data_folder_check {
        // データフォルダ存在チェック
        if let Some(error_msg) = data_folder_exists(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if nicknames_json_check {
        // nicknames.json 存在チェック
        if let Some(error_msg) = nicknames_json_exists(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if character_folders_check {
        // キャラクターフォルダ存在チェック
        if let Some(error_msg) = character_folders_exist(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if character_jsons_check {
        // キャラクター JSON 存在チェック
        if let Some(error_msg) = character_jsons_exist(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if character_images_check {
        // 画像 JSON 存在チェック
        if let Some(error_msg) = character_images_exist(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }

    if checks_passed {
        Ok(())
    } else {
        Err("Failed adaptive_check".into())
    }
}
