//! models.rs
//!
//! このファイルでは、アプリケーションの主要なデータモデルを定義します。
//! 従来の全てString型だった構造体からより適切なデータ型へ変換しています。

use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

/// キャラクター情報構造体
///
/// 各キャラクターの各種ステータスを保持
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharInfo {
    /// 防御値
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub defense: Option<f64>,

    /// ガッツ
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub guts: Option<f64>,

    /// ガードバランス
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub guard_balance: Option<f64>,

    /// ジャンプ前の状態フレーム数
    #[serde(deserialize_with = "deserialize_option_i32")]
    pub prejump: Option<i32>,

    /// 未使用（予備）
    pub umo: String,

    /// 前方ダッシュ速度
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub forward_dash: Option<f64>,

    /// バックダッシュ速度
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub backdash: Option<f64>,

    /// バックダッシュ持続時間（フレーム）
    #[serde(deserialize_with = "deserialize_option_i32")]
    pub backdash_duration: Option<i32>,

    /// バックダッシュ無敵時間（フレーム）
    #[serde(deserialize_with = "deserialize_option_i32")]
    pub backdash_invincibility: Option<i32>,

    /// 空中バックダッシュ有無
    #[serde(deserialize_with = "deserialize_option_bool")]
    pub backdash_airborne: Option<bool>,

    /// バックダッシュ移動距離
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub backdash_distance: Option<f64>,

    /// ジャンプ持続時間（フレーム）
    #[serde(deserialize_with = "deserialize_option_i32")]
    pub jump_duration: Option<i32>,

    /// ジャンプ高さ
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub jump_height: Option<f64>,

    /// ハイジャンプ持続時間（フレーム）
    #[serde(deserialize_with = "deserialize_option_i32")]
    pub high_jump_duration: Option<i32>,

    /// ハイジャンプ高さ
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub high_jump_height: Option<f64>,

    /// 最速IAD（未使用）
    pub earliest_iad: String,

    /// AD持続時間（未使用）
    pub ad_duration: String,

    /// AD移動距離（未使用）
    pub ad_distance: String,

    /// ABD持続時間（未使用）
    pub abd_duration: String,

    /// ABD移動距離（未使用）
    pub abd_distance: String,

    /// 移動緊張度
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub movement_tension: Option<f64>,

    /// ジャンプ緊張度
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub jump_tension: Option<f64>,

    /// エアダッシュ緊張度
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub airdash_tension: Option<f64>,

    /// 歩行速度
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub walk_speed: Option<f64>,

    /// 後ろ歩行速度
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub back_walk_speed: Option<f64>,

    /// ダッシュ初速
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub dash_initial_speed: Option<f64>,

    /// ダッシュ加速
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub dash_acceleration: Option<f64>,

    /// ダッシュ摩擦
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub dash_friction: Option<f64>,

    /// ジャンプ重力
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub jump_gravity: Option<f64>,

    /// ハイジャンプ重力
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub high_jump_gravity: Option<f64>,
}

/// 技情報構造体
///
/// 各技の入力、名称、フレームデータなどを保持
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoveInfo {
    /// 入力コマンド
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub input: String,

    /// 技名称
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub name: String,

    /// ダメージ値
    #[serde(deserialize_with = "deserialize_option_i32")]
    pub damage: Option<i32>,

    /// ガード値
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub guard: String,

    /// 始動フレーム
    #[serde(deserialize_with = "deserialize_option_i32")]
    pub startup: Option<i32>,

    /// アクティブフレーム
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub active: String,

    /// リカバリーフレーム
    #[serde(deserialize_with = "deserialize_option_i32")]
    pub recovery: Option<i32>,

    /// ヒット時効果
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub on_hit: String,

    /// ブロック時効果
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub on_block: String,

    /// 技レベル
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub level: String,

    /// カウンター情報
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub counter: String,

    /// 技種別
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub move_type: String,

    /// リスクゲイン
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub risc_gain: Option<f64>,

    /// リスクロス
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub risc_loss: Option<f64>,

    /// 壁ダメージ
    #[serde(deserialize_with = "deserialize_option_i32")]
    pub wall_damage: Option<i32>,

    /// 入力緊張度
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub input_tension: Option<f64>,

    /// チップダメージ比率
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub chip_ratio: Option<f64>,

    /// ダメージスケーリング
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub scaling: Option<f64>,

    /// 無敵フレーム
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub invincibility: String,

    /// キャンセル情報
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub cancel: String,

    /// キャプション
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub caption: String,

    /// 備考
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub notes: String,
}

/// 技のエイリアス情報
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoveAliases {
    /// 入力コマンド
    pub input: String,

    /// エイリアス（別名）のリスト
    pub aliases: Vec<String>,
}

/// ガードタイプ（上段/中段/下段など）
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GuardType {
    /// 上段
    High,
    /// 中段
    Mid,
    /// 下段
    Low,
    /// ガード不能
    Unblockable,
    /// 投げ
    Throw,
}

impl std::str::FromStr for GuardType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "High" => Ok(GuardType::High),
            "Mid" => Ok(GuardType::Mid),
            "Low" => Ok(GuardType::Low),
            "Unblockable" => Ok(GuardType::Unblockable),
            "Throw" => Ok(GuardType::Throw),
            _ => Err(format!("不正なガードタイプ: {s}")),
        }
    }
}

/// String から Option<f64> へのデシリアライザ
fn deserialize_option_f64<'de, D>(deserializer: D) -> std::result::Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    // 値が数値の場合
    if let Some(n) = value.as_f64() {
        return Ok(Some(n));
    }

    // 値が文字列の場合
    if let Some(s) = value.as_str() {
        if s.trim().is_empty() {
            return Ok(None);
        }

        return match f64::from_str(s) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Ok(None),
        };
    }

    // nullまたは他の型の場合はNoneを返す
    Ok(None)
}

/// String から Option<i32> へのデシリアライザ
fn deserialize_option_i32<'de, D>(deserializer: D) -> std::result::Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    // 値が整数の場合
    if let Some(i) = value.as_i64() {
        return match i32::try_from(i) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None), // 変換できない場合は None を返す
        };
    }

    // 値が文字列の場合
    if let Some(s) = value.as_str() {
        if s.trim().is_empty() {
            return Ok(None);
        }

        return match i32::from_str(s) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Ok(None),
        };
    }

    // nullまたは他の型の場合はNoneを返す
    Ok(None)
}

/// String から Option<bool> へのデシリアライザ
fn deserialize_option_bool<'de, D>(deserializer: D) -> std::result::Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.trim().is_empty() {
        return Ok(None);
    }

    match s.to_lowercase().as_str() {
        "true" | "yes" | "1" => Ok(Some(true)),
        "false" | "no" | "0" => Ok(Some(false)),
        _ => Ok(None),
    }
}

/// 文字列型か整数型のどちらでも受け入れ可能なデシリアライズ
///
/// 整数値が来た場合は文字列に変換します
pub fn deserialize_string_or_int<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Valueとして受け取り、型を判別
    let value = serde_json::Value::deserialize(deserializer)?;

    // 整数か文字列かで処理を分岐
    if let Some(s) = value.as_str() {
        // 文字列ならそのまま返す
        Ok(s.to_string())
    } else if let Some(i) = value.as_i64() {
        // 整数なら文字列に変換
        Ok(i.to_string())
    } else {
        // それ以外は型エラー
        Err(serde::de::Error::custom(format!(
            "expected string or int, got: {value:?}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn test_char_info_serialization() {
        // テスト用のCharInfo構造体インスタンス作成
        let char_info = CharInfo {
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
        };

        // JSONとして文字列にシリアライズ
        let json_str = r#"{"defense":"0.9","guts":"2.0","guard_balance":"1.5","prejump":"4","umo":"","forward_dash":"7.5","backdash":"6.0","backdash_duration":"20","backdash_invincibility":"7","backdash_airborne":"true","backdash_distance":"4.2","jump_duration":"45","jump_height":"3.5","high_jump_duration":"55","high_jump_height":"4.7","earliest_iad":"","ad_duration":"","ad_distance":"","abd_duration":"","abd_distance":"","movement_tension":"0.1","jump_tension":"0.2","airdash_tension":"0.15","walk_speed":"2.2","back_walk_speed":"1.8","dash_initial_speed":"5.5","dash_acceleration":"0.3","dash_friction":"0.05","jump_gravity":"0.25","high_jump_gravity":"0.2"}"#;

        // デシリアライズ
        let deserialized: CharInfo = serde_json::from_str(json_str).expect("デシリアライズに失敗");

        // 値を検証
        assert_eq!(deserialized.defense, char_info.defense);
        assert_eq!(deserialized.guts, char_info.guts);
        assert_eq!(deserialized.guard_balance, char_info.guard_balance);
        assert_eq!(deserialized.prejump, char_info.prejump);
        assert_eq!(deserialized.backdash_airborne, char_info.backdash_airborne);
        assert_eq!(deserialized.walk_speed, char_info.walk_speed);
        assert_eq!(deserialized.back_walk_speed, char_info.back_walk_speed);
    }

    #[test]
    fn test_move_info_serialization() {
        // テスト用のMoveInfo構造体インスタンス作成
        let move_info = MoveInfo {
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
            scaling: Some(0.8),
            invincibility: "None".to_string(),
            cancel: "Special, Super".to_string(),
            caption: String::new(),
            notes: String::new(),
        };

        // JSONとして文字列にシリアライズ
        let json_str = r#"{"input":"5P","name":"Punch","damage":"26","guard":"Mid","startup":"4","active":"3","recovery":"9","on_hit":"+2","on_block":"-1","level":"0","counter":"3","move_type":"Normal","risc_gain":"23.0","risc_loss":"18.0","wall_damage":"9","input_tension":"0.0","chip_ratio":"0.0","otg_ratio":"0.8","scaling":"0.8","invincibility":"None","cancel":"Special, Super","caption":"","notes":""}"#;

        // デシリアライズ
        let deserialized: MoveInfo = serde_json::from_str(json_str).expect("デシリアライズに失敗");

        // 値を検証
        assert_eq!(deserialized.input, move_info.input);
        assert_eq!(deserialized.name, move_info.name);
        assert_eq!(deserialized.damage, move_info.damage);
        assert_eq!(deserialized.guard, move_info.guard);
        assert_eq!(deserialized.startup, move_info.startup);
        assert_eq!(deserialized.recovery, move_info.recovery);
        assert_eq!(deserialized.move_type, move_info.move_type);
        assert_eq!(deserialized.wall_damage, move_info.wall_damage);
    }

    #[test]
    fn test_move_aliases_serialization() {
        // テスト用のMoveAliases構造体インスタンス作成
        let move_aliases = MoveAliases {
            input: "236K".to_string(),
            aliases: vec!["Stun Edge".to_string(), "Fireball".to_string()],
        };

        // シリアライズとデシリアライズ
        let serialized = serde_json::to_string(&move_aliases).expect("シリアライズに失敗");
        let deserialized: MoveAliases =
            serde_json::from_str(&serialized).expect("デシリアライズに失敗");

        // 値を検証
        assert_eq!(deserialized.input, move_aliases.input);
        assert_eq!(deserialized.aliases.len(), move_aliases.aliases.len());
        assert_eq!(deserialized.aliases[0], move_aliases.aliases[0]);
        assert_eq!(deserialized.aliases[1], move_aliases.aliases[1]);
    }

    #[test]
    fn test_guard_type_from_str() {
        // 文字列からGuardTypeへの変換テスト
        assert_eq!(GuardType::from_str("High").unwrap(), GuardType::High);
        assert_eq!(GuardType::from_str("Mid").unwrap(), GuardType::Mid);
        assert_eq!(GuardType::from_str("Low").unwrap(), GuardType::Low);
        assert_eq!(
            GuardType::from_str("Unblockable").unwrap(),
            GuardType::Unblockable
        );
        assert_eq!(GuardType::from_str("Throw").unwrap(), GuardType::Throw);

        // 無効な入力のテスト
        assert!(GuardType::from_str("Invalid").is_err());
    }

    #[test]
    fn test_option_deserializers() {
        // Option<f64>のデシリアライズテスト
        let json = r#"{"value": "1.5"}"#;
        let deserialized: serde_json::Value = serde_json::from_str(json).unwrap();
        let result = deserialize_option_f64(&deserialized["value"]).unwrap();
        assert_eq!(result, Some(1.5));

        // Option<i32>のデシリアライズテスト
        let json = r#"{"value": "42"}"#;
        let deserialized: serde_json::Value = serde_json::from_str(json).unwrap();
        let result = deserialize_option_i32(&deserialized["value"]).unwrap();
        assert_eq!(result, Some(42));

        // Option<bool>のデシリアライズテスト
        let json = r#"{"value": "true"}"#;
        let deserialized: serde_json::Value = serde_json::from_str(json).unwrap();
        let result = deserialize_option_bool(&deserialized["value"]).unwrap();
        assert_eq!(result, Some(true));
    }
}
