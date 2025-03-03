//! `error.rs`
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

// `Box<dyn std::error::Error>型からAppError型への変換
impl From<Box<dyn std::error::Error>> for AppError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        AppError::Other(error.to_string())
    }
}

/// `結果型の別名定義（アプリケーション全体で使用）`
pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_app_error_io() {
        let io_error = IoError::new(ErrorKind::NotFound, "テスト用IOエラー");
        let app_error = AppError::from(io_error);

        if let AppError::Io(e) = app_error {
            assert_eq!(e.kind(), ErrorKind::NotFound);
        } else {
            panic!("変換されたエラーの型が正しくありません");
        }
    }

    #[test]
    fn test_app_error_file_not_found() {
        let error_message = "test.json";
        let app_error = AppError::FileNotFound(error_message.to_string());

        let error_string = app_error.to_string();
        assert!(error_string.contains(error_message));
    }

    #[test]
    fn test_app_error_character_not_found() {
        let character = "存在しない_キャラ";
        let app_error = AppError::CharacterNotFound(character.to_string());

        let error_string = app_error.to_string();
        assert!(error_string.contains(character));
    }

    #[test]
    fn test_app_error_move_not_found() {
        let move_name = "存在しない技";
        let app_error = AppError::MoveNotFound(move_name.to_string());

        let error_string = app_error.to_string();
        assert!(error_string.contains(move_name));
    }

    #[test]
    fn test_app_error_from_box_dyn_error() {
        let original_error: Box<dyn std::error::Error> =
            Box::new(IoError::new(ErrorKind::Other, "ボックス化されたエラー"));

        let app_error = AppError::from(original_error);

        if let AppError::Other(message) = app_error {
            assert!(message.contains("ボックス化されたエラー"));
        } else {
            panic!("変換されたエラーの型が正しくありません");
        }
    }
}
