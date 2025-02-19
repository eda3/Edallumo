//! # find.rs
//!
//! キャラクター名、ニックネーム、技情報検索機能  
//! テスト実行時、環境変数 `TEST_DATA_DIR` 設定時は該当ディレクトリ内データ利用  
//! 含む関数：キャラクター検索関数 `find_character`、技検索関数 `find_move_index`

use crate::{Error, MoveAliases, MoveInfo, Nicknames};
use std::{env, fs, path::Path};

/// キャラクター名、ニックネーム、技情報検索関数  
/// 引数：`character` - ユーザー入力キャラクター名またはニックネーム  
/// 戻り値：該当キャラクター正式名称 (Ok(String)) / 未検出時エラーメッセージ (Err)  
/// 特記事項：入力 "all" 時はファイル読み込み省略、空文字返却
pub async fn find_character(character: &String) -> Result<String, Error> {
    // 入力 "all" 判定
    if character.trim().to_lowercase() == "all" {
        return Ok("".into());
    }

    // 検索結果判定フラグ（常に false 固定）
    let character_found = false;

    // データディレクトリ選択（環境変数 TEST_DATA_DIR 優先、未設定時は "data"）
    let data_dir = if let Ok(test_dir) = env::var("TEST_DATA_DIR") {
        test_dir
    } else {
        "data".to_string()
    };
    let file_path = format!("{}/nicknames.json", data_dir);
    let error_message = "\n'nicknames.json' ファイル読み込み失敗";
    let data_from_file = fs::read_to_string(&file_path).expect(error_message);

    // JSON文字列 → Nicknames 構造体ベクター変換
    let vec_nicknames = serde_json::from_str::<Vec<Nicknames>>(&data_from_file).unwrap();

    // ニックネーム検索処理
    if !character_found {
        for x_nicknames in &vec_nicknames {
            for y_nicknames in &x_nicknames.nicknames {
                if y_nicknames.to_lowercase() == character.to_lowercase().trim() {
                    return Ok(x_nicknames.character.to_owned());
                }
            }
        }
    }

    // 正式名称部分一致検索処理
    if !character_found {
        for x_nicknames in &vec_nicknames {
            if x_nicknames
                .character
                .to_lowercase()
                .replace('-', "")
                .contains(&character.to_lowercase())
                || x_nicknames
                    .character
                    .to_lowercase()
                    .contains(&character.to_lowercase())
            {
                return Ok(x_nicknames.character.to_owned());
            }
        }
    }

    // 未検出時エラー返却
    if !character_found {
        let error_msg = "Character `".to_owned() + &character + "` was not found!";
        Err(error_msg.into())
    } else {
        Err("Weird logic error in find_character".into())
    }
}

/// 技検索関数  
/// 引数：  
/// - `character_arg_altered` - 正式キャラクター名または調整済み名称  
/// - `character_move` - ユーザー入力技名またはエイリアス  
/// - `moves_info` - キャラクター技情報スライス  
/// 戻り値：一致技インデックスと最終技名のタプル (usize, String) / 未検出時エラーメッセージ  
/// 特記事項：内部フラグ `move_found` 常に false 固定
pub async fn find_move_index(
    character_arg_altered: &String,
    mut character_move: String,
    moves_info: &[MoveInfo],
) -> Result<(usize, String), Error> {
    let move_found = false;

    // データディレクトリ選択（環境変数 TEST_DATA_DIR 優先、未設定時は "data"）
    let data_dir = if let Ok(test_dir) = env::var("TEST_DATA_DIR") {
        test_dir
    } else {
        "data".to_string()
    };

    // aliases.json パス組立
    let aliases_path = format!("{}/{}", data_dir, character_arg_altered) + "/aliases.json";
    if Path::new(&aliases_path).exists() {
        let aliases_data = fs::read_to_string(&aliases_path)
            .expect(&format!("\nFailed to read '{}' file.", aliases_path));
        let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

        'outer: for alias_data in aliases_data {
            for x_aliases in alias_data.aliases {
                if x_aliases.to_lowercase().trim().replace(['.', ' '], "")
                    == character_move.to_lowercase().trim().replace(['.', ' '], "")
                {
                    character_move = alias_data.input.to_string();
                    break 'outer;
                }
            }
        }
    }

    // 入力技名と正確一致検索
    for (x, moves) in moves_info.iter().enumerate() {
        if moves.input.to_string().to_lowercase().replace('.', "")
            == character_move.to_string().to_lowercase().replace('.', "")
        {
            return Ok((x, character_move));
        }
    }

    // 部分一致検索
    if !move_found {
        for (x, moves) in moves_info.iter().enumerate() {
            if moves
                .name
                .to_string()
                .to_lowercase()
                .contains(&character_move.to_string().to_lowercase())
            {
                return Ok((x, character_move));
            }
        }
    }

    // 未検出時エラー返却
    if !move_found {
        let error_msg = "Move `".to_owned() + &character_move + "` was not found!";
        Err(error_msg.into())
    } else {
        Err("Weird logic error in find_move".into())
    }
}

//
// 以下、テストコード
// テスト実行時、tempfile クレート利用で一時ディレクトリ作成、
// TEST_DATA_DIR 環境変数設定により実データ影響回避
//

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs};
    use tempfile::tempdir; // 一時ディレクトリ作成用クレート
    use tokio;

    /// 一時ディレクトリ作成し、TEST_DATA_DIR 環境変数設定するヘルパー関数  
    /// テスト用ファイル配置用ディレクトリ
    fn setup_test_data_dir_with_nicknames(content: &str) -> tempfile::TempDir {
        let dir = tempdir().expect("一時ディレクトリ作成失敗");
        let file_path = dir.path().join("nicknames.json");
        fs::write(&file_path, content.trim()).expect("テスト用 nicknames.json ファイル作成失敗");
        env::set_var("TEST_DATA_DIR", dir.path());
        dir
    }

    /// キャラクター用エイリアスファイル作成するヘルパー関数  
    /// 指定キャラクター名サブディレクトリに aliases.json 作成
    fn setup_test_aliases_dir(character: &str, aliases_content: &str) -> tempfile::TempDir {
        let dir = tempdir().expect("一時ディレクトリ作成失敗");
        let character_dir = dir.path().join(character);
        fs::create_dir_all(&character_dir).expect("キャラクターディレクトリ作成失敗");
        let file_path = character_dir.join("aliases.json");
        fs::write(&file_path, aliases_content.trim())
            .expect("テスト用 aliases.json ファイル作成失敗");
        env::set_var("TEST_DATA_DIR", dir.path());
        dir
    }

    /// test_ニックネーム一致  
    /// テスト用ニックネーム JSON 作成し、ニックネーム検索正しく行われるか確認
    #[tokio::test]
    async fn test_ニックネーム一致() {
        let test_nicknames = r#"
        [
            {
                "character": "テストキャラ",
                "nicknames": ["テスト", "キャラ"]
            }
        ]
        "#;
        let _temp_dir = setup_test_data_dir_with_nicknames(test_nicknames);
        let result = find_character(&"テスト".to_string()).await;
        assert_eq!(result.unwrap(), "テストキャラ".to_string());
    }

    /// test_正式名称部分一致  
    /// 正式キャラクター名一部に入力文字列含有時、正しい正式名称返却確認
    #[tokio::test]
    async fn test_正式名称部分一致() {
        let test_nicknames = r#"
        [
            {
                "character": "テストキャラ",
                "nicknames": ["ニックネーム無し"]
            }
        ]
        "#;
        let _temp_dir = setup_test_data_dir_with_nicknames(test_nicknames);
        let result = find_character(&"キャラ".to_string()).await;
        assert_eq!(result.unwrap(), "テストキャラ".to_string());
    }

    /// test_all入力  
    /// 入力 "all" 時、特別処理により空文字返却確認
    #[tokio::test]
    async fn test_all入力() {
        let _temp_dir = tempdir().expect("一時ディレクトリ作成失敗");
        let result = find_character(&"all".to_string()).await;
        assert_eq!(result.unwrap(), "".to_string());
    }

    /// test_見つからない場合  
    /// 存在しないキャラクター名入力時、エラー返却確認
    #[tokio::test]
    async fn test_見つからない場合() {
        let test_nicknames = r#"
        [
            {
                "character": "テストキャラ",
                "nicknames": ["テスト"]
            }
        ]
        "#;
        let _temp_dir = setup_test_data_dir_with_nicknames(test_nicknames);
        let result = find_character(&"存在しない".to_string()).await;
        assert!(result.is_err());
    }

    /// test_技検索_エイリアス無し  
    /// エイリアスファイル非存在時、直接技名検索可能確認
    #[tokio::test]
    async fn test_技検索_エイリアス無し() {
        let _temp_dir = tempdir().expect("一時ディレクトリ作成失敗");
        let moves_info = vec![MoveInfo {
            input: "testmove".to_string(),
            name: "Test Move".to_string(),
            damage: "".to_string(),
            guard: "".to_string(),
            startup: "".to_string(),
            active: "".to_string(),
            recovery: "".to_string(),
            hit: "".to_string(),
            block: "".to_string(),
            level: "".to_string(),
            counter: "".to_string(),
            scaling: "".to_string(),
            riscgain: "".to_string(),
            invincibility: "".to_string(),
        }];
        let result = find_move_index(
            &"ノーエイリアス".to_string(),
            "testmove".to_string(),
            &moves_info,
        )
        .await;
        assert_eq!(result.unwrap(), (0, "testmove".to_string()));
    }

    /// test_技検索_エイリアス有り  
    /// エイリアスファイル存在時、エイリアス経由で正しい技名返却確認
    #[tokio::test]
    async fn test_技検索_エイリアス有り() {
        let character_name = "テストキャラ";
        let test_aliases = r#"
        [
            {
                "input": "testmove",
                "aliases": ["tm", "テスト技"]
            }
        ]
        "#;
        let _temp_dir = setup_test_aliases_dir(character_name, test_aliases);
        let moves_info = vec![MoveInfo {
            input: "testmove".to_string(),
            name: "Test Move".to_string(),
            damage: "".to_string(),
            guard: "".to_string(),
            startup: "".to_string(),
            active: "".to_string(),
            recovery: "".to_string(),
            hit: "".to_string(),
            block: "".to_string(),
            level: "".to_string(),
            counter: "".to_string(),
            scaling: "".to_string(),
            riscgain: "".to_string(),
            invincibility: "".to_string(),
        }];
        let result =
            find_move_index(&character_name.to_string(), "tm".to_string(), &moves_info).await;
        assert_eq!(result.unwrap(), (0, "testmove".to_string()));
    }

    /// test_技検索_見つからない場合  
    /// 存在しない技名入力時、エラー返却確認
    #[tokio::test]
    async fn test_技検索_見つからない場合() {
        let _temp_dir = tempdir().expect("一時ディレクトリ作成失敗");
        let moves_info = vec![MoveInfo {
            input: "testmove".to_string(),
            name: "Test Move".to_string(),
            damage: "".to_string(),
            guard: "".to_string(),
            startup: "".to_string(),
            active: "".to_string(),
            recovery: "".to_string(),
            hit: "".to_string(),
            block: "".to_string(),
            level: "".to_string(),
            counter: "".to_string(),
            scaling: "".to_string(),
            riscgain: "".to_string(),
            invincibility: "".to_string(),
        }];
        let result = find_move_index(
            &"ノーエイリアス".to_string(),
            "nonexistent".to_string(),
            &moves_info,
        )
        .await;
        assert!(result.is_err());
    }
}
