//! find.rs
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
    // 出力判定用フラグ（常に false、後の分岐で利用）
    let character_found = false;

    // nicknames.json ファイル読み込み　結果：JSON文字列取得
    let data_from_file = fs::read_to_string("data/nicknames.json").map_err(|e| {
        AppError::FileNotFound(format!("nicknames.jsonの読み込みに失敗しました: {}", e))
    })?;

    // JSON文字列を Nicknames 構造体のベクターへデシリアライズ　結果：vec_nicknames
    let vec_nicknames =
        serde_json::from_str::<Vec<Nicknames>>(&data_from_file).map_err(|e| AppError::Json(e))?;

    // 各キャラクターエントリ走査　結果：該当エントリ検出時に正式名称返却
    if !character_found {
        for x_nicknames in &vec_nicknames {
            // 各ニックネーム走査　結果：入力文字列と完全一致すれば正式名称返却
            for y_nicknames in &x_nicknames.nicknames {
                if y_nicknames.to_lowercase() == character.to_lowercase().trim() {
                    return Ok(x_nicknames.character.to_owned());
                }
            }
        }
    }

    if !character_found {
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
                return Ok(x_nicknames.character.to_owned());
            }
        }
    }
    // "all" の場合の例外処理　結果：空文字列返却
    if !character_found && character.trim().to_lowercase() == "all".to_lowercase() {
        return Ok("".into());
    }

    if !character_found {
        // キャラクター未検出時エラーメッセージ作成　結果：エラー返却
        let error_msg = "Character `".to_owned() + character + "` was not found!";
        Err(AppError::CharacterNotFound(error_msg))
    } else {
        Err(AppError::Other(
            "Weird logic error in find_character".to_string(),
        ))
    }
}

/// 技のインデックスを検索し、該当する技のインデックスを返却する非同期関数
///
/// # 概要
/// 指定されたキャラクターの技情報（moves_info）から、  
/// ユーザー入力に対応する技のインデックスを検索する。  
/// 入力がエイリアスであれば、実際の技入力に変換する。
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
                "{}ファイルの読み込みに失敗しました: {}",
                aliases_path, e
            ))
        })?;

        // JSON文字列を MoveAliases 構造体のベクターにデシリアライズ　結果：aliases_data
        let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data)
            .map_err(|e| AppError::Json(e))?;

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

    if !move_found {
        // 技未検出時エラーメッセージ作成　結果：エラー返却
        let error_msg = "Move `".to_owned() + &character_move + "` was not found!";
        Err(AppError::MoveNotFound(error_msg))
    } else {
        Err(AppError::Other(
            "Weird logic error in find_move".to_string(),
        ))
    }
}
