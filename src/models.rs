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
    pub input: String,

    /// 技名称
    pub name: String,

    /// ダメージ値
    #[serde(deserialize_with = "deserialize_option_i32")]
    pub damage: Option<i32>,

    /// ガード値
    pub guard: String,

    /// 始動フレーム
    #[serde(deserialize_with = "deserialize_option_i32")]
    pub startup: Option<i32>,

    /// アクティブフレーム
    pub active: String,

    /// リカバリーフレーム
    #[serde(deserialize_with = "deserialize_option_i32")]
    pub recovery: Option<i32>,

    /// ヒット時効果
    pub on_hit: String,

    /// ブロック時効果
    pub on_block: String,

    /// 技レベル
    pub level: String,

    /// カウンター情報
    pub counter: String,

    /// 技種別
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

    /// OTG比率
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub otg_ratio: Option<f64>,

    /// ダメージスケーリング
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub scaling: Option<f64>,

    /// 無敵フレーム
    pub invincibility: String,

    /// キャンセル情報
    pub cancel: String,

    /// キャプション
    pub caption: String,

    /// 備考
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

/// ガード種別を表す列挙型
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

/// String から Option<f64> へのデシリアライザ
fn deserialize_option_f64<'de, D>(deserializer: D) -> std::result::Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.trim().is_empty() {
        return Ok(None);
    }

    match f64::from_str(&s) {
        Ok(val) => Ok(Some(val)),
        Err(_) => Ok(None),
    }
}

/// String から Option<i32> へのデシリアライザ
fn deserialize_option_i32<'de, D>(deserializer: D) -> std::result::Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.trim().is_empty() {
        return Ok(None);
    }

    match i32::from_str(&s) {
        Ok(val) => Ok(Some(val)),
        Err(_) => Ok(None),
    }
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
