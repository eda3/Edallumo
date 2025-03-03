//! `test_utils.rs`
//!
//! このファイルでは、テスト用のユーティリティ関数とモックデータを提供します。
//! ユニットテストや結合テストで使用するためのヘルパー関数、テストデータ生成機能などを含みます。

#[cfg(test)]
use crate::models::{CharInfo, MoveAliases, MoveInfo};
#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::path::{Path, PathBuf};
#[cfg(test)]
use tempfile::TempDir;

/// テスト用の一時ディレクトリ構造を作成する
///
/// テスト用の一時ディレクトリとその中にキャラクターデータ用のサブディレクトリを作成します。
///
/// # 戻り値
/// `(TempDir, PathBuf)` - 一時ディレクトリハンドルとそのパス
#[cfg(test)]
pub fn create_test_dir_structure() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("一時ディレクトリの作成に失敗しました");
    let temp_path = temp_dir.path().to_path_buf();

    fs::create_dir_all(temp_path.join("Sol_Badguy"))
        .expect("Sol_Badguyディレクトリの作成に失敗しました");
    fs::create_dir_all(temp_path.join("Ky_Kiske"))
        .expect("Ky_Kiskeディレクトリの作成に失敗しました");
    fs::create_dir_all(temp_path.join("May")).expect("Mayディレクトリの作成に失敗しました");

    (temp_dir, temp_path)
}

/// テスト用のキャラクター情報を生成する
///
/// テスト用のダミーキャラクター情報を生成します。
///
/// # 戻り値
/// `CharInfo` - ダミーキャラクター情報
#[cfg(test)]
pub fn _create_test_char_info() -> CharInfo {
    CharInfo {
        defense: Some(0.9),
        guts: Some(2.0),
        guard_balance: Some(1.5),
        prejump: Some(4),
        umo: String::new(),
        forward_dash: Some(7.5),
        backdash: Some(6.0),
        backdash_duration: Some(20),
        backdash_invincibility: Some(7),
        backdash_airborne: Some(true),
        backdash_distance: Some(4.2),
        jump_duration: Some(45),
        jump_height: Some(3.5),
        high_jump_duration: Some(55),
        high_jump_height: Some(4.7),
        earliest_iad: String::new(),
        ad_duration: String::new(),
        ad_distance: String::new(),
        abd_duration: String::new(),
        abd_distance: String::new(),
        movement_tension: Some(0.1),
        jump_tension: Some(0.2),
        airdash_tension: Some(0.15),
        walk_speed: Some(2.2),
        back_walk_speed: Some(1.8),
        dash_initial_speed: Some(5.5),
        dash_acceleration: Some(0.3),
        dash_friction: Some(0.05),
        jump_gravity: Some(0.25),
        high_jump_gravity: Some(0.2),
    }
}

/// テスト用の技情報を生成する
///
/// テスト用のダミー技情報を生成します。
///
/// # 戻り値
/// `Vec<MoveInfo>` - ダミー技情報のベクター
#[cfg(test)]
pub fn create_test_move_info() -> Vec<MoveInfo> {
    vec![
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
            input: "236K".to_string(),
            name: "Stun Edge".to_string(),
            damage: Some(35),
            guard: "Mid".to_string(),
            startup: Some(13),
            active: "Until Hit".to_string(),
            recovery: Some(24),
            on_hit: "-7".to_string(),
            on_block: "-10".to_string(),
            level: "2".to_string(),
            counter: "3".to_string(),
            move_type: "Special".to_string(),
            risc_gain: Some(45.0),
            risc_loss: Some(35.0),
            wall_damage: Some(12),
            input_tension: Some(0.1),
            chip_ratio: Some(0.1),
            otg_ratio: Some(0.8),
            scaling: Some(0.9),
            invincibility: "None".to_string(),
            cancel: "None".to_string(),
            caption: String::new(),
            notes: String::new(),
        },
    ]
}

/// テスト用の技エイリアス情報を生成する
///
/// テスト用のダミー技エイリアス情報を生成します。
///
/// # 戻り値
/// `Vec<MoveAliases>` - ダミー技エイリアス情報のベクター
#[cfg(test)]
pub fn create_test_move_aliases() -> Vec<MoveAliases> {
    vec![
        MoveAliases {
            input: "5P".to_string(),
            aliases: vec!["Punch".to_string(), "P".to_string()],
        },
        MoveAliases {
            input: "236K".to_string(),
            aliases: vec!["Stun Edge".to_string(), "Fireball".to_string()],
        },
    ]
}

/// テスト用のJSONファイルを作成する
///
/// テスト用のJSONファイルを指定されたパスに作成します。
///
/// # 引数
/// * `path` - ファイルパス
/// * `content` - ファイルの内容（JSON文字列）
///
/// # 戻り値
/// `Result<(), Box<dyn std::error::Error>>` - 成功時は `Ok(())`, 失敗時はエラー
#[cfg(test)]
pub fn create_test_json_file<P: AsRef<Path>>(
    path: P,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(path, content)?;
    Ok(())
}

/// モック用のDiscordコンテキストを作成する
///
/// テスト用のモックDiscordコンテキストを作成します。
/// 実際のAPIは呼び出さず、内部処理のテストに使用します。
///
/// このモックは実際のコンテキストと同じインターフェースを持ちますが、
/// 常に成功するレスポンスを返します。
#[cfg(test)]
pub struct _MockDiscordContext {
    pub messages: Vec<String>,
    pub embeds: Vec<String>,
}

#[cfg(test)]
impl _MockDiscordContext {
    /// 新しいモックコンテキストを作成する
    ///
    /// # 戻り値
    /// `MockDiscordContext` - 新しいモックコンテキスト
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            embeds: Vec::new(),
        }
    }

    /// メッセージを送信する（モック）
    ///
    /// # 引数
    /// * `content` - メッセージ内容
    ///
    /// # 戻り値
    /// `Result<(), ()>` - 常に `Ok(())`
    pub fn send_message(&mut self, content: &str) -> Result<(), ()> {
        self.messages.push(content.to_string());
        Ok(())
    }

    /// 埋め込みメッセージを送信する（モック）
    ///
    /// # 引数
    /// * `content` - 埋め込みメッセージ内容
    ///
    /// # 戻り値
    /// `Result<(), ()>` - 常に `Ok(())`
    pub fn send_embed(&mut self, content: &str) -> Result<(), ()> {
        self.embeds.push(content.to_string());
        Ok(())
    }
}
