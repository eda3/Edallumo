//! utils.rs
//!
//! このファイルでは、アプリケーション全体で使用される共通ユーティリティ関数を提供します。
//! ファイル操作、文字列処理、データ変換などの汎用的な機能を含みます。

use crate::error::{AppError, Result};
use crate::models::{CharInfo, MoveAliases, MoveInfo};
use colored::Colorize;
use serde::{de, ser};
use serde_json;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

/// ファイルからJSONデータを読み込む
///
/// 指定されたパスのJSONファイルを読み込み、指定された型にデシリアライズします。
///
/// # 引数
/// * `path` - 読み込むJSONファイルのパス
///
/// # 戻り値
/// `Result<T>` - デシリアライズされたデータ
pub fn read_json_file<T>(path: impl AsRef<Path>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
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
pub fn write_json_file<T>(path: impl AsRef<Path>, data: &T) -> Result<()>
where
    T: serde::Serialize,
{
    let path = path.as_ref();
    let json = serde_json::to_string_pretty(data).map_err(|e| {
        AppError::Json(ser::Error::custom(format!(
            "JSONへの変換に失敗しました: {}",
            e
        )))
    })?;

    let mut file = File::create(path).map_err(|e| {
        AppError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!("ファイルの作成に失敗しました: {} - {}", path.display(), e),
        ))
    })?;

    file.write_all(json.as_bytes()).map_err(|e| {
        AppError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "ファイルの書き込みに失敗しました: {} - {}",
                path.display(),
                e
            ),
        ))
    })?;

    Ok(())
}

/// ディレクトリ内のすべてのJSONファイルを読み込む
///
/// 指定されたディレクトリ内のすべてのJSONファイルを読み込み、
/// ファイル名とデシリアライズされたデータのマップを返します。
///
/// # 引数
/// * `dir_path` - 読み込むディレクトリのパス
///
/// # 戻り値
/// `Result<HashMap<String, T>>` - ファイル名とデータのマップ
pub fn read_all_json_files<T>(
    dir_path: impl AsRef<Path>,
) -> Result<std::collections::HashMap<String, T>>
where
    T: serde::de::DeserializeOwned,
{
    let dir_path = dir_path.as_ref();
    let mut result = std::collections::HashMap::new();

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
                format!("ディレクトリエントリの読み込みに失敗しました: {}", e),
            ))
        })?;

        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            let file_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| AppError::Other(format!("無効なファイル名: {}", path.display())))?
                .to_string();

            let data = read_json_file(&path)?;
            result.insert(file_name, data);
        }
    }

    Ok(result)
}

/// キャラクター情報を読み込む
///
/// 指定されたキャラクター名の情報をデータディレクトリから読み込みます。
///
/// # 引数
/// * `data_dir` - データディレクトリのパス
/// * `char_name` - キャラクター名
///
/// # 戻り値
/// `Result<CharInfo>` - キャラクター情報
pub fn load_character_info(data_dir: &str, char_name: &str) -> Result<CharInfo> {
    let char_path = Path::new(data_dir).join(char_name).join("character.json");

    if !char_path.exists() {
        return Err(AppError::CharacterNotFound(format!(
            "キャラクター情報が見つかりません: {}",
            char_name
        )));
    }

    read_json_file(char_path)
}

/// キャラクターの技情報を読み込む
///
/// 指定されたキャラクター名の技情報をデータディレクトリから読み込みます。
///
/// # 引数
/// * `data_dir` - データディレクトリのパス
/// * `char_name` - キャラクター名
///
/// # 戻り値
/// `Result<Vec<MoveInfo>>` - 技情報のリスト
pub fn load_move_info(data_dir: &str, char_name: &str) -> Result<Vec<MoveInfo>> {
    let moves_path = Path::new(data_dir).join(char_name).join("moves.json");

    if !moves_path.exists() {
        return Err(AppError::CharacterNotFound(format!(
            "技情報が見つかりません: {}",
            char_name
        )));
    }

    read_json_file(moves_path)
}

/// キャラクターの技エイリアス情報を読み込む
///
/// 指定されたキャラクター名の技エイリアス情報をデータディレクトリから読み込みます。
/// エイリアスファイルが存在しない場合は空のリストを返します。
///
/// # 引数
/// * `data_dir` - データディレクトリのパス
/// * `char_name` - キャラクター名
///
/// # 戻り値
/// `Result<Vec<MoveAliases>>` - 技エイリアス情報のリスト
pub fn load_move_aliases(data_dir: &str, char_name: &str) -> Result<Vec<MoveAliases>> {
    let aliases_path = Path::new(data_dir).join(char_name).join("aliases.json");

    if !aliases_path.exists() {
        return Ok(Vec::new()); // エイリアスファイルがない場合は空のリストを返す
    }

    read_json_file(aliases_path)
}

/// 入力コマンドを正規化する
///
/// 入力コマンドを正規化し、検索しやすい形式に変換します。
///
/// # 引数
/// * `input` - 正規化する入力コマンド
///
/// # 戻り値
/// `String` - 正規化された入力コマンド
pub fn normalize_input(input: &str) -> String {
    input
        .to_lowercase()
        .replace(" ", "")
        .replace("　", "")
        .replace(".", "")
}

/// 技名を正規化する
///
/// 技名を正規化し、検索しやすい形式に変換します。
///
/// # 引数
/// * `name` - 正規化する技名
///
/// # 戻り値
/// `String` - 正規化された技名
pub fn normalize_move_name(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "")
        .replace("　", "")
        .replace("-", "")
        .replace("_", "")
}

/// 指定された技を検索する
///
/// 指定されたキャラクターの技リストから、指定された入力または名前に一致する技を検索します。
///
/// # 引数
/// * `moves` - 検索対象の技リスト
/// * `aliases` - 技のエイリアスリスト
/// * `query` - 検索クエリ（入力コマンドまたは技名）
///
/// # 戻り値
/// `Option<MoveInfo>` - 見つかった技情報
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
/// * `path` - ハッシュを計算するファイルのパス
///
/// # 戻り値
/// `Result<String>` - MD5ハッシュ文字列
pub fn calculate_file_hash(path: impl AsRef<Path>) -> Result<String> {
    use md5::{Digest, Md5};

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

    Ok(format!("{:x}", result))
}

/// ディレクトリが存在することを確認し、存在しない場合は作成する
///
/// 指定されたディレクトリが存在することを確認し、存在しない場合は作成します。
///
/// # 引数
/// * `dir_path` - 確認または作成するディレクトリのパス
///
/// # 戻り値
/// `Result<()>` - 処理結果
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
            format!("ディレクトリを作成しました: {}", dir_path.display()).green()
        );
    }

    Ok(())
}

/// 文字列を指定された長さで切り詰める
///
/// 文字列が指定された長さを超える場合、切り詰めて末尾に「...」を追加します。
///
/// # 引数
/// * `s` - 切り詰める文字列
/// * `max_len` - 最大長
///
/// # 戻り値
/// `String` - 切り詰められた文字列
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
    use crate::test_utils::{create_test_dir_structure, create_test_json_file};
    use tempfile::TempDir;

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
                caption: "".to_string(),
                notes: "".to_string(),
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
                caption: "".to_string(),
                notes: "".to_string(),
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
        assert_eq!(truncate_string("Hello", 10), "Hello");
        assert_eq!(truncate_string("Hello, World!", 10), "Hello,...");
        assert_eq!(
            truncate_string("This is a very long string that should be truncated", 20),
            "This is a very lo..."
        );
    }

    #[test]
    fn test_read_write_json_file() {
        let (temp_dir, temp_path) = create_test_dir_structure();

        let test_data = vec![MoveInfo {
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
            caption: "".to_string(),
            notes: "".to_string(),
        }];

        let test_file = temp_path.join("test_moves.json");

        // ファイルに書き込み
        let write_result = write_json_file(&test_file, &test_data);
        assert!(write_result.is_ok());

        // ファイルから読み込み
        let read_result: Result<Vec<MoveInfo>> = read_json_file(&test_file);
        assert!(read_result.is_ok());

        let read_data = read_result.unwrap();
        assert_eq!(read_data.len(), 1);
        assert_eq!(read_data[0].input, "5P");
        assert_eq!(read_data[0].name, "Punch");
    }
}
