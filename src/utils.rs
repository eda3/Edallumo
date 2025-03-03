//! `utils.rs`
//!
//! このファイルでは、アプリケーション全体で使用される共通ユーティリティ関数を提供します。
//! ファイル操作、文字列処理、データ変換などの汎用的な機能を含みます。

use md5::{Digest, Md5};
use serde::{de, ser};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

use crate::error::{AppError, Result};
use crate::models::{CharInfo, MoveAliases, MoveInfo};
use colored::Colorize;

/// JSONファイルを読み込む
///
/// 指定されたパスのJSONファイルを読み込み、指定された型にデシリアライズします。
///
/// # 引数
/// * `path` - 読み込むファイルのパス
///
/// # 戻り値
/// `Result<T>` - デシリアライズされたデータ
#[allow(dead_code)]
pub fn read_json_file<T>(path: impl AsRef<Path>) -> Result<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let path = path.as_ref();
    let mut file = File::open(path).map_err(|e| {
        AppError::FileNotFound(format!(
            "ファイルを開けませんでした: {} - {}",
            path.display(),
            e
        ))
    })?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| {
        AppError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "ファイルの読み込みに失敗しました: {} - {}",
                path.display(),
                e
            ),
        ))
    })?;

    serde_json::from_str(&contents).map_err(|e| {
        AppError::Json(de::Error::custom(format!(
            "JSONの解析に失敗しました: {} - {}",
            path.display(),
            e
        )))
    })
}

/// JSONデータをファイルに書き込む
///
/// 指定されたデータをJSONに変換し、指定されたパスのファイルに書き込みます。
///
/// # 引数
/// * `path` - 書き込み先ファイルのパス
/// * `data` - 書き込むデータ
///
/// # 戻り値
/// `Result<()>` - 書き込み結果
#[allow(dead_code)]
pub fn write_json_file<T>(path: impl AsRef<Path>, data: &T) -> Result<()>
where
    T: serde::Serialize,
{
    let path = path.as_ref();
    let json = serde_json::to_string_pretty(data).map_err(|e| {
        AppError::Json(ser::Error::custom(format!(
            "JSONへの変換に失敗しました: {e}"
        )))
    })?;

    let mut file = File::create(path).map_err(|e| {
        AppError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "ファイルの作成に失敗しました: {path_display} - {e}",
                path_display = path.display()
            ),
        ))
    })?;

    file.write_all(json.as_bytes()).map_err(|e| {
        AppError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "ファイルの書き込みに失敗しました: {path_display} - {e}",
                path_display = path.display()
            ),
        ))
    })?;

    Ok(())
}

/// 複数のJSONファイルを読み込む
///
/// 指定されたディレクトリ内のJSONファイルをすべて読み込み、指定された型にデシリアライズします。
///
/// # 引数
/// * `dir` - 読み込むディレクトリのパス
/// * `extension` - 読み込むファイルの拡張子（デフォルトは "json"）
///
/// # 戻り値
/// `Result<Vec<T>>` - デシリアライズされたデータのベクター
#[allow(dead_code)]
pub fn read_all_json_files<T>(dir: impl AsRef<Path>, extension: Option<&str>) -> Result<Vec<T>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let dir_path = dir.as_ref();
    let mut result = Vec::new();

    for entry in fs::read_dir(dir_path).map_err(|e| {
        AppError::FileNotFound(format!(
            "ディレクトリを読み込めませんでした: {} - {}",
            dir_path.display(),
            e
        ))
    })? {
        let entry = entry.map_err(|e| {
            AppError::Io(io::Error::new(
                io::ErrorKind::Other,
                format!("ディレクトリエントリの読み込みに失敗しました: {e}"),
            ))
        })?;

        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .map_or(false, |ext| ext == extension.unwrap_or("json"))
        {
            let data = read_json_file(&path)?;
            result.push(data);
        }
    }

    Ok(result)
}

/// キャラクター情報を読み込む
///
/// 指定されたキャラクターの情報をJSONファイルから読み込みます。
///
/// # 引数
/// * `data_dir` - データディレクトリのパス
/// * `char_name` - キャラクター名
///
/// # 戻り値
/// `Result<CharInfo>` - キャラクター情報
#[allow(dead_code)]
pub fn load_character_info(data_dir: &str, char_name: &str) -> Result<CharInfo> {
    let char_path = Path::new(data_dir).join(char_name).join("character.json");

    if !char_path.exists() {
        return Err(AppError::CharacterNotFound(format!(
            "キャラクター情報が見つかりません: {char_name}"
        )));
    }

    read_json_file(char_path)
}

/// 技情報を読み込む
///
/// 指定されたキャラクターの技情報をJSONファイルから読み込みます。
///
/// # 引数
/// * `data_dir` - データディレクトリのパス
/// * `char_name` - キャラクター名
///
/// # 戻り値
/// `Result<Vec<MoveInfo>>` - 技情報のベクター
#[allow(dead_code)]
pub fn load_move_info(data_dir: &str, char_name: &str) -> Result<Vec<MoveInfo>> {
    let moves_path = Path::new(data_dir).join(char_name).join("moves.json");

    if !moves_path.exists() {
        return Err(AppError::CharacterNotFound(format!(
            "技情報が見つかりません: {char_name}"
        )));
    }

    read_json_file(moves_path)
}

/// 技エイリアス情報を読み込む
///
/// 指定されたキャラクターの技エイリアス情報をJSONファイルから読み込みます。
///
/// # 引数
/// * `data_dir` - データディレクトリのパス
/// * `char_name` - キャラクター名
///
/// # 戻り値
/// `Result<Vec<MoveAliases>>` - 技エイリアス情報のベクター
#[allow(dead_code)]
pub fn load_move_aliases(data_dir: &str, char_name: &str) -> Result<Vec<MoveAliases>> {
    let aliases_path = Path::new(data_dir).join(char_name).join("aliases.json");

    if !aliases_path.exists() {
        return Ok(Vec::new()); // エイリアスファイルがない場合は空のリストを返す
    }

    read_json_file(aliases_path)
}

/// 入力文字列を正規化する
///
/// 入力文字列から空白や記号を削除し、小文字に変換します。
///
/// # 引数
/// * `input` - 正規化する入力文字列
///
/// # 戻り値
/// `String` - 正規化された文字列
#[allow(dead_code)]
pub fn normalize_input(input: &str) -> String {
    let mut result = input.to_lowercase();
    for c in [' ', '　', '.'] {
        result = result.replace(c, "");
    }
    result
}

/// 技名称を正規化する
///
/// 技名称から空白や記号を削除し、正規化します。
///
/// # 引数
/// * `name` - 正規化する技名称
///
/// # 戻り値
/// `String` - 正規化された技名称
#[allow(dead_code)]
pub fn normalize_move_name(name: &str) -> String {
    let mut result = name.to_lowercase();
    for c in [' ', '　', '-', '_'] {
        result = result.replace(c, "");
    }
    result
}

/// 入力に一致する技を検索する
///
/// # 引数
/// * `moves` - 検索対象の技情報のベクター
/// * `aliases` - 技エイリアス情報のベクター
/// * `query` - 検索クエリ
///
/// # 戻り値
/// `Option<MoveInfo>` - 一致した技情報（見つからなかった場合は None）
#[allow(dead_code)]
pub fn find_move(moves: &[MoveInfo], aliases: &[MoveAliases], query: &str) -> Option<MoveInfo> {
    let normalized_query = normalize_input(query);

    // 入力コマンドで検索
    for move_info in moves {
        if normalize_input(&move_info.input) == normalized_query {
            return Some(move_info.clone());
        }
    }

    // 技名で検索
    for move_info in moves {
        if normalize_move_name(&move_info.name) == normalize_move_name(&normalized_query) {
            return Some(move_info.clone());
        }
    }

    // エイリアスで検索
    for alias in aliases {
        if alias
            .aliases
            .iter()
            .any(|a| normalize_input(a) == normalized_query)
        {
            // エイリアスが見つかったら、対応する技を返す
            for move_info in moves {
                if normalize_input(&move_info.input) == normalize_input(&alias.input) {
                    return Some(move_info.clone());
                }
            }
        }
    }

    None
}

/// ファイルのMD5ハッシュを計算する
///
/// 指定されたファイルのMD5ハッシュを計算します。
///
/// # 引数
/// * `path` - ファイルのパス
///
/// # 戻り値
/// `Result<String>` - MD5ハッシュ文字列
#[allow(dead_code)]
pub fn calculate_file_hash(path: impl AsRef<Path>) -> Result<String> {
    let path = path.as_ref();
    let mut file = File::open(path).map_err(|e| {
        AppError::FileNotFound(format!(
            "ファイルを開けませんでした: {} - {}",
            path.display(),
            e
        ))
    })?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| {
        AppError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "ファイルの読み込みに失敗しました: {} - {}",
                path.display(),
                e
            ),
        ))
    })?;

    let mut hasher = Md5::new();
    hasher.update(&buffer);
    let result = hasher.finalize();

    Ok(format!("{result:x}"))
}

/// ディレクトリが存在することを確認し、存在しない場合は作成する
///
/// 指定されたディレクトリが存在することを確認し、存在しない場合は作成します。
///
/// # 引数
/// * `dir_path` - ディレクトリのパス
///
/// # 戻り値
/// `Result<()>` - 結果
#[allow(dead_code)]
pub fn ensure_directory_exists(dir_path: impl AsRef<Path>) -> Result<()> {
    let dir_path = dir_path.as_ref();

    if !dir_path.exists() {
        fs::create_dir_all(dir_path).map_err(|e| {
            AppError::Io(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "ディレクトリの作成に失敗しました: {} - {}",
                    dir_path.display(),
                    e
                ),
            ))
        })?;
        println!(
            "{}",
            format!(
                "ディレクトリを作成しました: {dir_path_display}",
                dir_path_display = dir_path.display()
            )
            .green()
        );
    }

    Ok(())
}

/// 文字列を指定した長さに切り詰める
///
/// 文字列が指定した長さを超える場合、切り詰めて末尾に「...」を追加します。
///
/// # 引数
/// * `s` - 切り詰める文字列
/// * `max_len` - 最大長
///
/// # 戻り値
/// `String` - 切り詰められた文字列
#[allow(dead_code)]
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let mut result = s.chars().take(max_len - 3).collect::<String>();
        result.push_str("...");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_test_dir_structure;
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_normalize_input() {
        assert_eq!(normalize_input("5P"), "5p");
        assert_eq!(normalize_input("j.H"), "jh");
        assert_eq!(normalize_input("236 S"), "236s");
    }

    #[test]
    fn test_normalize_move_name() {
        assert_eq!(normalize_move_name("Volcanic Viper"), "volcanicviper");
        assert_eq!(normalize_move_name("Gun Flame"), "gunflame");
        assert_eq!(
            normalize_move_name("Heavy Mob Cemetery"),
            "heavymobcemetery"
        );
    }

    #[test]
    fn test_find_move() {
        let moves = vec![
            MoveInfo {
                input: "5P".to_string(),
                name: "Punch".to_string(),
                damage: Some(26),
                guard: "Mid".to_string(),
                startup: Some(4),
                active: "3".to_string(),
                recovery: Some(9),
                on_hit: "+2".to_string(),
                on_block: "-1".to_string(),
                level: "0".to_string(),
                counter: "3".to_string(),
                move_type: "Normal".to_string(),
                risc_gain: Some(23.0),
                risc_loss: Some(18.0),
                wall_damage: Some(9),
                input_tension: Some(0.0),
                chip_ratio: Some(0.0),
                otg_ratio: Some(0.8),
                scaling: Some(0.8),
                invincibility: "None".to_string(),
                cancel: "Special, Super".to_string(),
                caption: String::new(),
                notes: String::new(),
            },
            MoveInfo {
                input: "236P".to_string(),
                name: "Gun Flame".to_string(),
                damage: Some(45),
                guard: "Mid".to_string(),
                startup: Some(20),
                active: "Until hit".to_string(),
                recovery: Some(24),
                on_hit: "+15".to_string(),
                on_block: "+8".to_string(),
                level: "2".to_string(),
                counter: "4".to_string(),
                move_type: "Special".to_string(),
                risc_gain: Some(10.0),
                risc_loss: Some(15.0),
                wall_damage: Some(12),
                input_tension: Some(0.0),
                chip_ratio: Some(0.1),
                otg_ratio: Some(0.7),
                scaling: Some(0.9),
                invincibility: "None".to_string(),
                cancel: "Super".to_string(),
                caption: String::new(),
                notes: String::new(),
            },
        ];

        let aliases = vec![MoveAliases {
            input: "236P".to_string(),
            aliases: vec!["fireball".to_string(), "gf".to_string()],
        }];

        // 入力コマンドで検索
        let result = find_move(&moves, &aliases, "5p");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Punch");

        // 技名で検索
        let result = find_move(&moves, &aliases, "gun flame");
        assert!(result.is_some());
        assert_eq!(result.unwrap().input, "236P");

        // エイリアスで検索
        let result = find_move(&moves, &aliases, "fireball");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Gun Flame");

        // 存在しない技
        let result = find_move(&moves, &aliases, "dragon install");
        assert!(result.is_none());
    }

    #[test]
    fn test_truncate_string() {
        // 空文字列
        assert_eq!(truncate_string("", 10), "");

        // 最大長より短い場合
        assert_eq!(truncate_string("Hello", 10), "Hello");

        // 最大長と同じ場合
        assert_eq!(truncate_string("Hello World", 11), "Hello World");

        // 最大長より長い場合
        assert_eq!(truncate_string("Hello, World", 8), "Hello...");
    }

    #[test]
    fn test_read_write_json_file() {
        let (_temp_dir, temp_path) = create_test_dir_structure();

        // テスト用の単純なデータ構造
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct TestData {
            value: String,
        }

        let test_data = TestData {
            value: "テスト値".to_string(),
        };

        let file_path = temp_path.join("test.json");

        // ファイルへの書き込みテスト
        let write_result = write_json_file(&file_path, &test_data);
        assert!(write_result.is_ok());

        // ファイルからの読み込みテスト
        let read_result: Result<TestData> = read_json_file(&file_path);
        assert!(read_result.is_ok());

        // 読み込んだ値が元の値と一致することを確認
        let read_data = read_result.unwrap();
        assert_eq!(read_data, test_data);
    }
}
