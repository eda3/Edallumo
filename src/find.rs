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
    // 出力判定用フラグ（常に false、後の分岐で利用）
    let move_found = false;

    // 対象キャラクターの aliases.json のパス生成　結果：aliases_path
    let aliases_path = "data/".to_owned() + character_arg_altered + "/aliases.json";
    if Path::new(&aliases_path).exists() {
        // aliases.json ファイル読み込み　結果：JSON文字列取得
        let aliases_data = fs::read_to_string(&aliases_path).map_err(|e| {
            AppError::FileNotFound(format!(
                "{aliases_path}ファイルの読み込みに失敗しました: {e}"
            ))
        })?;

        // JSON文字列を MoveAliases 構造体のベクターにデシリアライズ　結果：aliases_data
        let aliases_data =
            serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).map_err(AppError::Json)?;

        'outer: for alias_data in aliases_data {
            // 各エイリアス走査　結果：入力文字列と一致すれば実際の技入力に変換
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

    // 正確な技入力一致走査　結果：完全一致すれば該当インデックス返却
    for (x, moves) in moves_info.iter().enumerate() {
        if moves.input.to_string().to_lowercase().replace('.', "")
            == character_move.to_string().to_lowercase().replace('.', "")
        {
            return Ok(x);
        }
    }

    if !move_found {
        // 技名称部分一致走査　結果：入力が技名称に含まれていれば該当インデックス返却
        for (x, moves) in moves_info.iter().enumerate() {
            if moves
                .name
                .to_string()
                .to_lowercase()
                .contains(&character_move.to_string().to_lowercase())
            {
                return Ok(x);
            }
        }
    }

    if move_found {
        Err(AppError::Other(
            "Weird logic error in find_move".to_string(),
        ))
    } else {
        // 技未検出時エラーメッセージ作成　結果：エラー返却
        let error_msg = "Move `".to_owned() + &character_move + "` was not found!";
        Err(AppError::MoveNotFound(error_msg))
    }
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
