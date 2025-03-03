//! # frames.rs
//!
//! フレームデータ表示コマンドモジュール。
//! 技のフレームデータを表示するためのサブコマンド（simple、advanced、meter）を提供する。
//! 各サブコマンドは別モジュールとして実装され、このファイルから呼び出される。

// サブモジュール定義
mod advanced; // 詳細フレームデータ表示モジュール
mod meter; // メーター関連フレームデータ表示モジュール
mod simple; // 簡易フレームデータ表示モジュール

// 必要なインポート
use crate::{error::AppError, Context}; // コンテキストとエラー型
use advanced::advanced; // advanced サブコマンド関数
use meter::meter; // meter サブコマンド関数
use simple::simple; // simple サブコマンド関数
use std::string::String; // 標準文字列型

/// フレームデータ表示コマンド
///
/// # 概要
/// 技のフレームデータを表示するためのコマンド。
/// 以下のサブコマンドを提供する：
/// - simple: 基本的なフレームデータを表示
/// - advanced: 詳細なフレームデータを表示
/// - meter: メーター関連のフレームデータを表示
///
/// # 戻り値
/// 成功時は `Ok(())`, エラー時は `Err(AppError)` を返す
#[poise::command(
    prefix_command,
    slash_command,
    subcommands("simple", "advanced", "meter"),
    subcommand_required
)]
pub async fn frames(_: Context<'_>) -> Result<(), AppError> {
    Ok(())
}
