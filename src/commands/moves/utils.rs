//! utils.rs
//!
//! このファイルは、ムーブのフレームメーター表示機能に関連する関数群を定義する。
//! 各ムーブの開始、アクティブ、リカバリーフレームをシンボルとして表現し、
//! コマンド実行時に視覚的なフレームメーターを生成する。

use crate::{MoveAliases, MoveInfo};

/// 与えられた MoveInfo と MoveAliases のデータから、
/// 通常技の情報を文字列として整形して返す非同期関数。
///
/// # 引数
/// * `moves_info` - ムーブ情報のスライス
/// * `aliases_data` - ムーブのエイリアス情報のスライス
///
/// # 戻り値
/// 通常技の一覧を改行区切りの文字列で返す。
pub async fn get_normal_moves(moves_info: &[MoveInfo], aliases_data: &[MoveAliases]) -> String {
    // 通常技情報格納用文字列初期化
    let mut normal_moves = String::new();
    // moves_info から通常技のみを抽出（move_type が "normal" であるもの）
    for moves in moves_info
        .iter()
        .take_while(|x| x.move_type.to_lowercase() == "normal")
    {
        // 技情報を整形して追加：入力と名称を表示
        normal_moves =
            normal_moves.to_owned() + "\n- **" + &moves.input + " / " + &moves.name + "**";
        // エイリアス情報を確認
        for moves_aliases in aliases_data.iter() {
            // 一致する入力があればエイリアス情報を追加
            if moves.input == moves_aliases.input {
                normal_moves += "\n\tAliases → `";
                // エイリアス一覧をカンマ区切りで追加
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        normal_moves = normal_moves.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    } else {
                        normal_moves = normal_moves.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                normal_moves += "`\n";
            }
        }
    }
    // 整形済み通常技情報文字列を返却
    normal_moves
}

/// 与えられた MoveInfo と MoveAliases のデータから、
/// スペシャル技の情報を文字列として整形して返す非同期関数。
///
/// # 引数
/// * `moves_info` - ムーブ情報のスライス
/// * `aliases_data` - ムーブのエイリアス情報のスライス
///
/// # 戻り値
/// スペシャル技の一覧を改行区切りの文字列で返す。
pub async fn get_special_moves(moves_info: &[MoveInfo], aliases_data: &[MoveAliases]) -> String {
    let mut special_moves = String::new();
    // 通常技をスキップし、move_type が "special" または "other" の技を対象
    for moves in moves_info
        .iter()
        .skip_while(|x| x.move_type.to_lowercase() == "normal")
        .take_while(|x| {
            x.move_type.to_lowercase() == "special" || x.move_type.to_lowercase() == "other"
        })
    {
        special_moves =
            special_moves.to_owned() + "\n- **" + &moves.input + " / " + &moves.name + "**";
        for moves_aliases in aliases_data.iter() {
            // 一致する入力があればエイリアス情報を追加
            if moves.input == moves_aliases.input {
                special_moves += "\n\tAliases → `";
                // エイリアス一覧をカンマ区切りで追加
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        special_moves =
                            special_moves.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    } else {
                        special_moves = special_moves.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                special_moves = special_moves.to_owned() + "`\n";
            } else {
                continue;
            }
        }
    }
    special_moves
}

/// 与えられた MoveInfo と MoveAliases のデータから、
/// スーパー技の情報を文字列として整形して返す非同期関数。
///
/// # 引数
/// * `moves_info` - ムーブ情報のスライス
/// * `aliases_data` - ムーブのエイリアス情報のスライス
///
/// # 戻り値
/// スーパー技の一覧を改行区切りの文字列で返す。
pub async fn get_super_moves(moves_info: &[MoveInfo], aliases_data: &[MoveAliases]) -> String {
    let mut super_moves = String::new();
    // move_type が "super" の技を対象に抽出
    for moves in moves_info
        .iter()
        .skip_while(|x| x.move_type.to_lowercase() != "super")
    {
        super_moves = super_moves.to_owned() + "\n- **" + &moves.input + " / " + &moves.name + "**";
        for moves_aliases in aliases_data.iter() {
            // 一致する入力があればエイリアス情報を追加
            if moves.input == moves_aliases.input {
                super_moves += "\n\tAliases → `";
                // エイリアス一覧をカンマ区切りで追加
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        super_moves = super_moves.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    } else {
                        super_moves = super_moves.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                super_moves = super_moves.to_owned() + "`\n";
            } else {
                continue;
            }
        }
    }
    super_moves
}
