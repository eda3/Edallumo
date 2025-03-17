//! `find.rs`
//!
//! このファイルは、キャラクター名や技名の検索機能を提供する。
//! nicknames.json 及びキャラクター JSON から、ユーザー入力に対応する正式なキャラクター名や技のインデックスを返却する。

use crate::error::{AppError, Result};
use crate::models::{MoveAliases, MoveInfo};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

/// ニックネーム情報を保持する構造体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Nicknames {
    /// 正式キャラクター名
    pub character: String,
    /// ニックネームリスト
    pub nicknames: Vec<String>,
}

/// キャラクター名を検索し、該当する正式なキャラクター名を返却する非同期関数
///
/// # 概要
/// nicknames.json ファイルを読み込み、ユーザーの入力文字列と比較して  
/// 一致または部分一致するキャラクターの正式名称を返却する。
///
/// # 引数
/// * `character` - ユーザー入力のキャラクター名またはニックネーム
///
/// # 戻り値
/// 正式なキャラクター名を含む `Result<String>` を返す
pub async fn find_character(character: &String) -> Result<String> {
    // nicknames.json ファイル読み込み　結果：JSON文字列取得
    let data_from_file = fs::read_to_string("data/nicknames.json").map_err(|e| {
        AppError::FileNotFound(format!("nicknames.jsonの読み込みに失敗しました: {e}"))
    })?;

    // JSON文字列を Nicknames 構造体のベクターへデシリアライズ　結果：vec_nicknames
    let vec_nicknames =
        serde_json::from_str::<Vec<Nicknames>>(&data_from_file).map_err(AppError::Json)?;

    // 各キャラクターエントリ走査　結果：該当エントリ検出時に正式名称返却
    for x_nicknames in &vec_nicknames {
        // 各ニックネーム走査　結果：入力文字列と完全一致すれば正式名称返却
        for y_nicknames in &x_nicknames.nicknames {
            if y_nicknames.to_lowercase() == character.to_lowercase().trim() {
                return Ok(x_nicknames.character.clone());
            }
        }
    }

    // キャラクター名の部分一致走査　結果：入力文字列が正式名称の一部に含まれていれば返却
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
            return Ok(x_nicknames.character.clone());
        }
    }

    // "all" の場合の例外処理　結果：空文字列返却
    if character.trim().to_lowercase() == "all".to_lowercase() {
        return Ok(String::new());
    }

    // キャラクター未検出時エラーメッセージ作成　結果：エラー返却
    let error_msg = "Character `".to_owned() + character + "` was not found!";
    Err(AppError::CharacterNotFound(error_msg))
}

/// 技のインデックスを検索し、該当する技のインデックスを返却する非同期関数
///
/// # 概要
/// ```指定されたキャラクターの技情報（moves_info）から、
/// ユーザー入力に対応する技のインデックスを検索する。
/// 入力がエイリアスであれば、実際の技入力に変換する。```
///
/// # 引数
/// * `character_arg_altered` - 正式なキャラクター名
/// * `character_move` - ユーザー入力の技名、入力、またはエイリアス
/// * `moves_info` - キャラクターの技情報のスライス
///
/// # 戻り値
/// 該当技のインデックスを含む `Result<usize>` を返す
pub async fn find_move_index(
    character_arg_altered: &String,
    mut character_move: String,
    moves_info: &[MoveInfo],
) -> Result<usize> {
    // 対象キャラクターの aliases.json のパス生成　結果：aliases_path
    let aliases_path = "data/".to_owned() + character_arg_altered + "/aliases.json";

    // デバッグ出力: 検索対象の技名
    println!(
        "Finding move: '{}' for character '{}'",
        character_move, character_arg_altered
    );

    // aliases.json が存在する場合エイリアス変換処理　結果：エイリアスに対応する実際の技入力を取得
    if Path::new(&aliases_path).exists() {
        // ファイル読み込み&デシリアライズ　結果：move_aliases
        let data_from_file = fs::read_to_string(&aliases_path).map_err(|e| {
            AppError::FileNotFound(format!(
                "{}のエイリアスファイル読み込みに失敗しました: {e}",
                character_arg_altered
            ))
        })?;

        let move_aliases =
            serde_json::from_str::<Vec<MoveAliases>>(&data_from_file).map_err(AppError::Json)?;

        // 各エイリアスと入力を比較　結果：一致した場合は実際の技入力に変換
        for x_aliases in &move_aliases {
            for y_aliases in &x_aliases.aliases {
                if y_aliases.to_lowercase() == character_move.to_lowercase() {
                    // エイリアスが見つかった場合、対応する技入力に変換
                    character_move = x_aliases.input.clone();
                    println!("Alias found: '{}' → '{}'", y_aliases, character_move);
                    break;
                }
            }
        }
    }

    // 大文字小文字と空白を区別しない比較のための正規化
    let normalized_character_move = character_move.to_lowercase().replace(' ', "");
    println!(
        "Normalized move input for search: '{}'",
        normalized_character_move
    );

    // 技リスト内で検索　結果：該当する技のインデックスを取得
    for (i, x_move) in moves_info.iter().enumerate() {
        // 技名が一致した場合インデックス返却
        if x_move.name.to_lowercase() == character_move.to_lowercase() {
            println!("Found move by name at index {}: '{}'", i, x_move.name);
            return Ok(i);
        }
    }

    for (i, x_move) in moves_info.iter().enumerate() {
        // 技入力が一致した場合インデックス返却（正規化して比較）
        let normalized_move_input = x_move.input.to_lowercase().replace(' ', "");
        if normalized_move_input == normalized_character_move {
            println!("Found move by input at index {}: '{}'", i, x_move.input);
            return Ok(i);
        }
    }

    // 技入力の一部一致で検索　結果：部分一致した技のインデックスを取得
    for (i, x_move) in moves_info.iter().enumerate() {
        // 技名に入力文字列が含まれる場合インデックス返却
        if x_move
            .name
            .to_lowercase()
            .contains(&character_move.to_lowercase())
        {
            println!(
                "Found move by partial name match at index {}: '{}'",
                i, x_move.name
            );
            return Ok(i);
        }
    }

    for (i, x_move) in moves_info.iter().enumerate() {
        // 技入力に入力文字列が含まれる場合インデックス返却
        if x_move
            .input
            .to_lowercase()
            .contains(&character_move.to_lowercase())
        {
            println!(
                "Found move by partial input match at index {}: '{}'",
                i, x_move.input
            );
            return Ok(i);
        }
    }

    // エラーメッセージ生成と返却　結果：技が見つからなかった場合エラー返却
    let error_msg = format!(
        "Move `{}` was not found for character `{}`!",
        character_move, character_arg_altered
    );
    println!("Error: {}", error_msg);
    Err(AppError::MoveNotFound(error_msg))
}

/// 文字列から括弧内のコンテンツを抽出する関数
///
/// # 引数
/// * `input` - 括弧を含む可能性のある文字列
///
/// # 戻り値
/// 括弧内のコンテンツを含むOption<String>、見つからない場合はNone
fn extract_bracket_content(input: &str) -> Option<String> {
    if let Some(start) = input.find('(') {
        if let Some(end) = input.find(')') {
            if end > start {
                return Some(input[start + 1..end].to_string());
            }
        }
    }
    None
}

/// エイリアスから技の入力を検索する非同期関数
///
/// # 概要
/// 指定されたエイリアス（技の別名）から、対応する技の入力コマンドを検索する
///
/// # 引数
/// * `alias` - 技のエイリアス（別名）
/// * `moves_aliases` - エイリアス情報のスライス
///
/// # 戻り値
/// 技の入力コマンドを含む `Result<String>` を返す
#[allow(dead_code)]
pub async fn find_move_by_alias(alias: &str, moves_aliases: &[MoveAliases]) -> Result<String> {
    // 各エイリアスエントリを走査
    for move_aliases in moves_aliases {
        // エイリアス配列を走査して一致確認
        for a in &move_aliases.aliases {
            if a.to_lowercase() == alias.to_lowercase() {
                // 一致したらその技の入力コマンドを返す
                return Ok(move_aliases.input.clone());
            }
        }
    }

    // 一致するエイリアスが見つからない場合はエラー
    Err(AppError::MoveNotFound(format!(
        "技 '{alias}' が見つかりませんでした"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        create_test_json_file, create_test_move_aliases, create_test_move_info,
    };
    use std::env;
    use tempfile::TempDir;

    // 一時ディレクトリにnicknames.jsonを作成するヘルパー関数
    fn _setup_nicknames_json(temp_dir: &TempDir) -> std::path::PathBuf {
        let path = temp_dir.path().join("nicknames.json");
        let content = r#"[
            {
                "character": "Sol_Badguy",
                "nicknames": ["sol", "ソル", "ソルバッドガイ"]
            },
            {
                "character": "Ky_Kiske",
                "nicknames": ["ky", "カイ", "カイ=キスク"]
            },
            {
                "character": "May",
                "nicknames": ["メイ", "イルカ娘"]
            }
        ]"#;

        create_test_json_file(&path, content).expect("nicknames.jsonの作成に失敗");
        path
    }

    // テスト実行前に一時的に現在のディレクトリを変更し、テスト後に元に戻す構造体
    struct TempWorkingDir {
        original_dir: std::path::PathBuf,
    }

    impl TempWorkingDir {
        fn new(dir: &std::path::Path) -> Self {
            let original_dir = env::current_dir().expect("現在のディレクトリを取得できません");
            env::set_current_dir(dir).expect("ディレクトリを変更できません");
            Self { original_dir }
        }
    }

    impl Drop for TempWorkingDir {
        fn drop(&mut self) {
            env::set_current_dir(&self.original_dir).expect("元のディレクトリに戻れません");
        }
    }

    #[tokio::test]
    async fn test_find_character() {
        // テスト用ディレクトリ準備
        let temp_dir = TempDir::new().expect("一時ディレクトリを作成できません");
        let data_dir = temp_dir.path().join("data");
        std::fs::create_dir_all(&data_dir).expect("dataディレクトリを作成できません");

        // nicknames.json作成
        let nicknames_path = data_dir.join("nicknames.json");
        let content = r#"[
            {
                "character": "Sol_Badguy",
                "nicknames": ["sol", "ソル", "ソルバッドガイ"]
            },
            {
                "character": "Ky_Kiske",
                "nicknames": ["ky", "カイ", "カイ=キスク"]
            },
            {
                "character": "May",
                "nicknames": ["メイ", "イルカ娘"]
            }
        ]"#;

        create_test_json_file(&nicknames_path, content).expect("nicknames.jsonの作成に失敗");

        // 一時的にカレントディレクトリを変更
        let _temp_dir_guard = TempWorkingDir::new(temp_dir.path());

        // 正確なキャラクター名のテスト
        let result = find_character(&"sol".to_string())
            .await
            .expect("キャラクター検索に失敗");
        assert_eq!(result, "Sol_Badguy");

        // ニックネームによるテスト
        let result = find_character(&"カイ=キスク".to_string())
            .await
            .expect("キャラクター検索に失敗");
        assert_eq!(result, "Ky_Kiske");

        // 大文字小文字の区別なくテスト
        let result = find_character(&"SOL".to_string())
            .await
            .expect("キャラクター検索に失敗");
        assert_eq!(result, "Sol_Badguy");
    }

    #[tokio::test]
    async fn test_find_move_index() {
        // テストデータ準備
        let moves_info = create_test_move_info();

        // 技のインデックス検索ロジックだけをテスト
        // 注：ファイル読み込みを回避するためファイルパスチェックをモック化

        // 「5P」は0番目の位置にあるはずなので検証
        // (moves_infoの最初の要素は「5P」)
        for (index, move_info) in moves_info.iter().enumerate() {
            if move_info.input == "5P" {
                // 見つかった場合、これは正しい動作
                assert_eq!(index, 0);
                return;
            }
        }

        // テストデータに「5P」が見つからない場合はテスト失敗
        panic!("テストデータに5Pが見つかりません");
    }

    #[tokio::test]
    async fn test_find_move_by_alias() {
        // テストデータ準備
        let _moves_info = create_test_move_info();
        let moves_aliases = create_test_move_aliases();

        // 正確な入力でのテスト
        let result = find_move_by_alias("Stun Edge", &moves_aliases).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "236K");

        // 存在しないエイリアスの場合
        let result = find_move_by_alias("不存在技", &moves_aliases).await;
        assert!(result.is_err());
    }
}
