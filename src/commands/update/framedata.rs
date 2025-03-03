//! # framedata.rs
//!
//! フレームデータ更新モジュール。
//! Dustloopウェブサイトからキャラクターの技フレームデータを取得し、
//! ローカルのJSONファイルに保存する機能を提供する。

// 外部クレート読み込み
extern crate ureq; // HTTPリクエスト用クレート

// 必要なインポート
use crate::{commands::update::framedata_json::frames_to_json, CHARS}; // フレームデータJSON変換関数とキャラクター定数
use colored::Colorize; // ターミナル出力の色付け
use std::{fs::OpenOptions, time::Instant}; // ファイル操作と時間計測

// 定数定義
const SITE_LINK: &str = "https://www.dustloop.com/wiki/api.php?action=cargoquery&format=json&limit=100&tables=MoveData_GGST&fields=MoveData_GGST.input%2C%20MoveData_GGST.name%2C%20MoveData_GGST.damage%2C%20MoveData_GGST.guard%2C%20MoveData_GGST.startup%2C%20MoveData_GGST.active%2C%20MoveData_GGST.recovery%2C%20MoveData_GGST.onHit%2C%20MoveData_GGST.onBlock%2C%20MoveData_GGST.level%2C%20MoveData_GGST.counter%2C%20MoveData_GGST.type%2C%20MoveData_GGST.riscGain%2C%20MoveData_GGST.riscLoss%2C%20MoveData_GGST.wallDamage%2C%20MoveData_GGST.inputTension%2C%20MoveData_GGST.chipRatio%2C%20MoveData_GGST.OTGRatio%2C%20MoveData_GGST.prorate%2C%20MoveData_GGST.invuln%2C%20MoveData_GGST.cancel%2C%20MoveData_GGST.caption%2C%20MoveData_GGST.notes%2C%20MoveData_GGST.hitboxCaption%2C%20MoveData_GGST.images%2C%20MoveData_GGST.hitboxes%2C&where=chara%3D%22"; // Dustloop API リクエスト前半部（フレームデータ用）
const SITE_HALF: &str =
    "%22&order_by=MoveData_GGST.type%20ASC%2C%20MoveData_GGST.input%20ASC&utf8=1"; // Dustloop API リクエスト後半部

/// キャラクターフレームデータ取得関数
///
/// # 概要
/// Dustloopウェブサイトから指定されたキャラクターの技フレームデータを取得し、
/// ローカルのJSONファイルに保存する。
/// 全キャラクターまたは特定のキャラクターを対象に実行可能。
///
/// # 引数
/// * `chars_ids` - キャラクターIDの配列（CHARS.len()サイズ）
/// * `specific_char` - 特定のキャラクターID（"all"の場合は全キャラクター対象）
///
/// # 例
/// ```rust,no_run
/// get_char_data(CHARS, "Sol_Badguy").await;
/// ```
pub async fn get_char_data(chars_ids: &[&str; CHARS.len()], specific_char: &str) {
    // 更新時間計測開始
    let now = Instant::now();

    if specific_char == "all" {
        // 全キャラクター処理
        for (x, char_id) in chars_ids.iter().enumerate() {
            // 処理開始ログ出力
            println!(
                "{}",
                ("Creating '".to_owned() + char_id + ".json' file.").green()
            );

            // キャラクターJSONファイルパス生成
            let char_json_path = "data/".to_owned() + char_id + "/" + char_id + ".json";

            // 複数キャラクターのJSONファイル作成
            let file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(char_json_path)
                .expect(&("\nFailed to open '".to_owned() + char_id + ".json' file."));

            // リクエストリンク生成
            let character_link = SITE_LINK.to_owned() + &char_id.replace('_', " ") + SITE_HALF;

            // Dustloopサイトへリクエスト送信
            let mut char_page_response_json = ureq::get(&character_link).call().unwrap();

            // Dustloopサイトが500エラーを返す場合の再試行処理
            while char_page_response_json.status() == 500 {
                char_page_response_json = ureq::get(&character_link).call().unwrap();
            }

            // レスポンスを文字列に変換
            let char_page_response_json = char_page_response_json.into_string().unwrap();

            // レスポンスを処理してJSONファイルにシリアライズ
            // char_countはどのJSONファイルが更新に失敗したかを特定するためのカウンター
            frames_to_json(char_page_response_json, &file, x).await;
        }
    } else {
        // 特定キャラクター処理
        println!(
            "{}",
            ("Creating '".to_owned() + specific_char + ".json' file.").green()
        );

        // キャラクターJSONファイルパス生成
        let char_json_path = "data/".to_owned() + specific_char + "/" + specific_char + ".json";

        // 単一キャラクターのJSONファイル作成
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(char_json_path)
            .expect(&("\nFailed to open '".to_owned() + specific_char + ".json' file."));

        // リクエストリンク生成
        let character_link = SITE_LINK.to_owned() + &specific_char.replace('_', " ") + SITE_HALF;

        // Dustloopサイトへリクエスト送信
        let mut char_page_response_json = ureq::get(&character_link).call().unwrap();

        // Dustloopサイトが500エラーを返す場合の再試行処理
        while char_page_response_json.status() == 500 {
            char_page_response_json = ureq::get(&character_link).call().unwrap();
        }

        // レスポンスを文字列に変換
        let char_page_response_json = char_page_response_json.into_string().unwrap();

        // レスポンスを処理してJSONファイルにシリアライズ
        // char_countはどのJSONファイルが更新に失敗したかを特定するためのカウンター
        frames_to_json(char_page_response_json, &file, 0).await;
    }

    // 経過時間計測と表示
    let elapsed_time = now.elapsed();
    println!(
        "{}",
        ("Updated in ".to_owned() + &elapsed_time.as_secs().to_string() + " seconds.").yellow()
    );
}
