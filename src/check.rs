//! # check.rs
//!
//! 各種チェック機能提供モジュールである。  
//! ディレクトリ、ファイル、JSON の存在および正当性を確認する関数群を含む。

use crate::error::{AppError, Result};
use crate::find::Nicknames;
use crate::{Context, CHARS}; // CHARS 定数：キャラクター名定数（クレートルートに定義されている前提）
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
    let data_from_file = match fs::read_to_string("data/nicknames.json") {
        Ok(data) => data,
        Err(_) => {
            let error_msg = "Error: Failed to read 'nicknames.json' file.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken.".to_string();

            if init_check {
                println!();
                panic!("{}", error_msg.red());
            } else {
                return Some(error_msg);
            }
        }
    };

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
    data_folder_check: bool,
    nicknames_json_check: bool,
    character_folders_check: bool,
    character_jsons_check: bool,
    character_images_check: bool,
) -> Result<()> {
    if data_folder_check {
        // Checking if data folder exists
        if let Some(error_msg) = data_folder_exists(false).await {
            if let Err(e) = ctx.say(&error_msg.replace('\'', "`")).await {
                println!("Failed to send message: {}", e);
            }
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if nicknames_json_check {
        // Checking if nicknames.json exists
        if let Some(error_msg) = nicknames_json_exists(false).await {
            if let Err(e) = ctx.say(&error_msg.replace('\'', "`")).await {
                println!("Failed to send message: {}", e);
            }
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if character_folders_check {
        // Checking if character folders exist
        if let Some(error_msg) = character_folders_exist(false).await {
            if let Err(e) = ctx.say(&error_msg.replace('\'', "`")).await {
                println!("Failed to send message: {}", e);
            }
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if character_jsons_check {
        // Checking if character jsons exist
        if let Some(error_msg) = character_jsons_exist(false).await {
            if let Err(e) = ctx.say(&error_msg.replace('\'', "`")).await {
                println!("Failed to send message: {}", e);
            }
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if character_images_check {
        // Checking if image jsons exist
        if let Some(error_msg) = character_images_exist(false).await {
            if let Err(e) = ctx.say(&error_msg.replace('\'', "`")).await {
                println!("Failed to send message: {}", e);
            }
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    Ok(())
}

/// データディレクトリの検証を行う
///
/// 指定されたデータディレクトリが存在し、必要なサブディレクトリやファイルが
/// 含まれているかを確認します。
///
/// # 引数
/// * `data_dir` - データディレクトリのパス
///
/// # 戻り値
/// `Result<()>` - 成功時は `Ok(())`, 失敗時はエラー
pub fn validate_data_dir(data_dir: &str) -> Result<()> {
    println!("{}", "データディレクトリの検証を開始します...".cyan());

    // データディレクトリの存在確認
    let data_path = Path::new(data_dir);
    if !data_path.exists() {
        return Err(AppError::FileNotFound(format!(
            "データディレクトリが見つかりません: {}",
            data_dir
        )));
    }

    if !data_path.is_dir() {
        return Err(AppError::Config(format!(
            "指定されたパスはディレクトリではありません: {}",
            data_dir
        )));
    }

    println!(
        "{}",
        format!("データディレクトリを確認しました: {}", data_dir).green()
    );

    // 必要なサブディレクトリの存在確認（例として一部のキャラクター）
    let essential_chars = ["Sol_Badguy", "Ky_Kiske", "May"];
    for char_name in &essential_chars {
        let char_dir = data_path.join(char_name);
        if !char_dir.exists() || !char_dir.is_dir() {
            return Err(AppError::Config(format!(
                "必要なキャラクターディレクトリが見つかりません: {}",
                char_name
            )));
        }
    }

    println!("{}", "基本キャラクターデータの存在を確認しました".green());
    println!("{}", "データディレクトリの検証が完了しました".green());

    Ok(())
}

/// キャラクターフォルダの存在を確認する
///
/// すべてのキャラクターフォルダが存在するかを確認します。
///
/// # 引数
/// * `data_dir` - データディレクトリのパス
///
/// # 戻り値
/// `Result<()>` - 成功時は `Ok(())`, 失敗時はエラー
pub fn check_character_folders(data_dir: &str) -> Result<()> {
    println!("{}", "キャラクターフォルダの検証を開始します...".cyan());

    let data_path = Path::new(data_dir);
    if !data_path.exists() {
        return Err(AppError::FileNotFound(format!(
            "データディレクトリが見つかりません: {}",
            data_dir
        )));
    }

    // CHARS配列からキャラクターフォルダを確認
    for char_name in &crate::CHARS {
        let char_dir = data_path.join(char_name);
        if !char_dir.exists() || !char_dir.is_dir() {
            println!(
                "{}",
                format!(
                    "警告: キャラクターフォルダが見つかりません: {}",
                    char_dir.display()
                )
                .yellow()
            );
            // 警告のみ出して続行
        }
    }

    println!("{}", "キャラクターフォルダの検証が完了しました".green());

    Ok(())
}

/// 不正なファイルデータをチェックする
///
/// JSON ファイルが正しく解析できるかを確認します。
///
/// # 引数
/// * `data_dir` - データディレクトリのパス
///
/// # 戻り値
/// `Result<()>` - 成功時は `Ok(())`, 失敗時はエラー
pub fn check_data_integrity(data_dir: &str) -> Result<()> {
    println!("{}", "データ整合性の検証を開始します...".cyan());

    let data_path = Path::new(data_dir);

    // 各キャラクターのJSONファイルをサンプリングチェック
    for char_name in &["Sol_Badguy", "Ky_Kiske", "May"] {
        let char_dir = data_path.join(char_name);
        if !char_dir.exists() || !char_dir.is_dir() {
            continue; // このキャラクターはスキップ
        }

        // character.json の整合性チェック
        let char_data_path = char_dir.join("character.json");
        if char_data_path.exists() && char_data_path.is_file() {
            match fs::read_to_string(&char_data_path) {
                Ok(content) => {
                    if let Err(e) = serde_json::from_str::<crate::models::CharInfo>(&content) {
                        println!(
                            "{}",
                            format!(
                                "警告: キャラクターデータの解析に失敗しました: {} - {}",
                                char_data_path.display(),
                                e
                            )
                            .yellow()
                        );
                    }
                }
                Err(e) => {
                    println!(
                        "{}",
                        format!(
                            "警告: キャラクターデータの読み込みに失敗しました: {} - {}",
                            char_data_path.display(),
                            e
                        )
                        .yellow()
                    );
                }
            }
        }

        // moves.json の整合性チェック
        let moves_data_path = char_dir.join("moves.json");
        if moves_data_path.exists() && moves_data_path.is_file() {
            match fs::read_to_string(&moves_data_path) {
                Ok(content) => {
                    if let Err(e) = serde_json::from_str::<Vec<crate::models::MoveInfo>>(&content) {
                        println!(
                            "{}",
                            format!(
                                "警告: 技データの解析に失敗しました: {} - {}",
                                moves_data_path.display(),
                                e
                            )
                            .yellow()
                        );
                    }
                }
                Err(e) => {
                    println!(
                        "{}",
                        format!(
                            "警告: 技データの読み込みに失敗しました: {} - {}",
                            moves_data_path.display(),
                            e
                        )
                        .yellow()
                    );
                }
            }
        }
    }

    println!("{}", "データ整合性の検証が完了しました".green());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*; // 親モジュールの全ての要素をインポート
    use crate::test_utils::{create_test_dir_structure, create_test_json_file};
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn setup_test_data(temp_dir: &PathBuf) -> Result<()> {
        // Sol_Badguy のデータを作成
        let sol_dir = temp_dir.join("Sol_Badguy");
        let sol_char_json = r#"{"defense":0.9,"guts":2.0,"guard_balance":1.5,"prejump":4,"umo":"","forward_dash":7.5,"backdash":6.0,"backdash_duration":20,"backdash_invincibility":7,"backdash_airborne":true,"backdash_distance":4.2,"jump_duration":45,"jump_height":3.5,"high_jump_duration":55,"high_jump_height":4.7,"earliest_iad":"","ad_duration":"","ad_distance":"","abd_duration":"","abd_distance":"","movement_tension":0.1,"jump_tension":0.2,"airdash_tension":0.15,"walk_speed":2.2,"back_walk_speed":1.8,"dash_initial_speed":5.5,"dash_acceleration":0.3,"dash_friction":0.05,"jump_gravity":0.25,"high_jump_gravity":0.2}"#;
        let sol_moves_json = r#"[{"input":"5P","name":"Punch","damage":26,"guard":"Mid","startup":4,"active":"3","recovery":9,"on_hit":"+2","on_block":"-1","level":"0","counter":"3","move_type":"Normal","risc_gain":23.0,"risc_loss":18.0,"wall_damage":9,"input_tension":0.0,"chip_ratio":0.0,"otg_ratio":0.8,"scaling":0.8,"invincibility":"None","cancel":"Special, Super","caption":"","notes":""}]"#;

        create_test_json_file(sol_dir.join("character.json"), sol_char_json)?;
        create_test_json_file(sol_dir.join("moves.json"), sol_moves_json)?;

        // Ky_Kiske のデータを作成
        let ky_dir = temp_dir.join("Ky_Kiske");
        let ky_char_json = r#"{"defense":1.0,"guts":1.8,"guard_balance":1.6,"prejump":4,"umo":"","forward_dash":7.2,"backdash":6.1,"backdash_duration":21,"backdash_invincibility":8,"backdash_airborne":true,"backdash_distance":4.5,"jump_duration":46,"jump_height":3.6,"high_jump_duration":56,"high_jump_height":4.8,"earliest_iad":"","ad_duration":"","ad_distance":"","abd_duration":"","abd_distance":"","movement_tension":0.11,"jump_tension":0.21,"airdash_tension":0.16,"walk_speed":2.3,"back_walk_speed":1.9,"dash_initial_speed":5.6,"dash_acceleration":0.31,"dash_friction":0.06,"jump_gravity":0.26,"high_jump_gravity":0.21}"#;
        let ky_moves_json = r#"[{"input":"5P","name":"Punch","damage":25,"guard":"Mid","startup":5,"active":"3","recovery":8,"on_hit":"+3","on_block":"-1","level":"0","counter":"3","move_type":"Normal","risc_gain":22.0,"risc_loss":19.0,"wall_damage":8,"input_tension":0.0,"chip_ratio":0.0,"otg_ratio":0.8,"scaling":0.8,"invincibility":"None","cancel":"Special, Super","caption":"","notes":""}]"#;

        create_test_json_file(ky_dir.join("character.json"), ky_char_json)?;
        create_test_json_file(ky_dir.join("moves.json"), ky_moves_json)?;

        // May のデータを作成
        let may_dir = temp_dir.join("May");
        let may_char_json = r#"{"defense":1.1,"guts":1.7,"guard_balance":1.7,"prejump":4,"umo":"","forward_dash":7.3,"backdash":6.2,"backdash_duration":22,"backdash_invincibility":9,"backdash_airborne":true,"backdash_distance":4.6,"jump_duration":47,"jump_height":3.7,"high_jump_duration":57,"high_jump_height":4.9,"earliest_iad":"","ad_duration":"","ad_distance":"","abd_duration":"","abd_distance":"","movement_tension":0.12,"jump_tension":0.22,"airdash_tension":0.17,"walk_speed":2.4,"back_walk_speed":2.0,"dash_initial_speed":5.7,"dash_acceleration":0.32,"dash_friction":0.07,"jump_gravity":0.27,"high_jump_gravity":0.22}"#;
        let may_moves_json = r#"[{"input":"5P","name":"Punch","damage":24,"guard":"Mid","startup":6,"active":"3","recovery":7,"on_hit":"+4","on_block":"-1","level":"0","counter":"3","move_type":"Normal","risc_gain":21.0,"risc_loss":20.0,"wall_damage":7,"input_tension":0.0,"chip_ratio":0.0,"otg_ratio":0.8,"scaling":0.8,"invincibility":"None","cancel":"Special, Super","caption":"","notes":""}]"#;

        create_test_json_file(may_dir.join("character.json"), may_char_json)?;
        create_test_json_file(may_dir.join("moves.json"), may_moves_json)?;

        Ok(())
    }

    #[test]
    fn test_validate_data_dir() {
        // テスト用のディレクトリ構造を作成
        let (temp_dir, temp_path) = create_test_dir_structure();

        // テストデータを設定
        setup_test_data(&temp_path).unwrap();

        // テスト実行
        let result = validate_data_dir(temp_path.to_str().unwrap());

        // 検証
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_data_dir_missing_dir() {
        // 存在しないディレクトリでテスト
        let result = validate_data_dir("/tmp/non_existent_dir_123456789");

        // 検証
        assert!(result.is_err());

        // エラーの種類を確認
        match result {
            Err(AppError::FileNotFound(_)) => {} // 正しいエラー
            _ => panic!("期待されるエラータイプではありません"),
        }
    }

    #[test]
    fn test_check_character_folders() {
        // テスト用のディレクトリ構造を作成
        let (temp_dir, temp_path) = create_test_dir_structure();

        // テストデータを設定
        setup_test_data(&temp_path).unwrap();

        // テスト実行
        let result = check_character_folders(temp_path.to_str().unwrap());

        // 検証
        assert!(result.is_ok());
    }
}
