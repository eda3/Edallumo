//! error.rs
//!
//! このファイルでは、アプリケーション全体で使用するエラー型を定義します。
//! `thiserror`クレートを使用して構造化エラー処理を実装しています。

use poise::serenity_prelude as serenity;
use std::io;
use thiserror::Error;

/// アプリケーション全体で使用するエラー型
#[derive(Error, Debug)]
pub enum AppError {
    /// IOエラー
    #[error("IO エラー: {0}")]
    Io(#[from] io::Error),

    /// JSON解析エラー
    #[error("JSON 解析エラー: {0}")]
    Json(#[from] serde_json::Error),

    /// ファイル未検出エラー
    #[error("ファイルが見つかりません: {0}")]
    FileNotFound(String),

    /// キャラクター未検出エラー
    #[error("キャラクターが見つかりません: {0}")]
    CharacterNotFound(String),

    /// 技未検出エラー
    #[error("技が見つかりません: {0}")]
    MoveNotFound(String),

    /// Discord APIエラー
    #[error("Discord API エラー: {0}")]
    Discord(String),

    /// Serenity APIエラー
    #[error("Serenity API エラー: {0}")]
    Serenity(#[from] serenity::Error),

    /// 設定エラー
    #[error("設定エラー: {0}")]
    Config(String),

    /// データ処理エラー
    #[error("データ処理エラー: {0}")]
    DataProcessing(String),

    /// その他のエラー
    #[error("エラー: {0}")]
    Other(String),
}

/// Box<dyn std::error::Error>型からAppError型への変換
impl From<Box<dyn std::error::Error>> for AppError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        AppError::Other(error.to_string())
    }
}

/// 結果型の別名定義（アプリケーション全体で使用）
pub type Result<T> = std::result::Result<T, AppError>;
