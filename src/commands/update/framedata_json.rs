//! framedata_json.rs
//!
//! このファイルは、Dustloop Wiki から取得したフレームデータの JSON を前処理し、  
//! MoveInfo 構造体のシリアライズ可能な形式に変換するための関数群を定義する。

// 外部クレートおよびモジュールのインポート
use crate::{MoveInfo, CHARS}; // MoveInfo 構造体およびキャラクター定数群
use serde::Deserialize; // JSON のデシリアライズ用
use std::fs::File; // ファイル操作用
use std::io::Write; // ファイル書き込み用

extern crate ureq; // HTTP クライアント（使用例として記載）

// ======================================================================
// 以下、JSON デシリアライズ用の構造体定義
// ======================================================================

#[derive(Deserialize, Debug)]
struct Response {
    cargoquery: Vec<Data>, // 複数のデータエントリ
}

#[derive(Deserialize, Debug)]
struct Data {
    title: Title, // 各エントリのタイトル情報
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Title {
    input: Option<String>,    // 技入力文字列　未定義の場合は None
    name: Option<String>,     // 技名称　未定義の場合は None
    damage: Option<String>,   // ダメージ数値　未定義の場合は None
    guard: Option<String>,    // ガード値　未定義の場合は None
    startup: Option<String>,  // 始動フレーム　未定義の場合は None
    active: Option<String>,   // アクティブフレーム　未定義の場合は None
    recovery: Option<String>, // リカバリーフレーム　未定義の場合は None
    on_hit: Option<String>,   // ヒット時効果　未定義の場合は None
    #[serde(rename = "onBlock")]
    on_block: Option<String>, // ブロック時効果　未定義の場合は None
    level: Option<String>,    // 技レベル　未定義の場合は None
    counter: Option<String>,  // カウンター情報　未定義の場合は None
    #[serde(rename = "type")]
    move_type: Option<String>, // 技種別　未定義の場合は None
    #[serde(rename = "riscGain")]
    risc_gain: Option<String>, // リスクゲイン　未定義の場合は None
    #[serde(rename = "riscLoss")]
    risc_loss: Option<String>, // リスクロス　未定義の場合は None
    #[serde(rename = "wallDamage")]
    wall_damage: Option<String>, // 壁ダメージ　未定義の場合は None
    input_tension: Option<String>, // 入力緊張度　未定義の場合は None
    chip_ratio: Option<String>, // チップダメージ比率　未定義の場合は None
    #[serde(rename = "OTGRatio")]
    otg_ratio: Option<String>, // OTG 比率　未定義の場合は None
    #[serde(rename = "prorate")]
    scaling: Option<String>, // ダメージスケーリング　未定義の場合は None
    #[serde(rename = "invuln")]
    invincibility: Option<String>, // 無敵フレーム　未定義の場合は None
    cancel: Option<String>,   // キャンセル情報　未定義の場合は None
    caption: Option<String>,  // キャプション情報　未定義の場合は None
    notes: Option<String>,    // 備考　未定義の場合は None
                              // hitbox_caption, images, hitboxes などはコメントアウト
}

// ======================================================================
// 以下、JSON 前処理関数
// ======================================================================

/// 与えられた JSON 文字列から不要なタグやエンティティを除去する非同期関数
///
/// # 引数
/// * `char_page_response_json` - 前処理対象の JSON 文字列
///
/// # 戻り値
/// タグや不要文字列を除去した後の JSON 文字列
async fn remove_tags(mut char_page_response_json: String) -> String {
    // 不要な span タグ（色指定）を除去
    char_page_response_json = char_page_response_json
        .replace(r#"&lt;span class=&quot;colorful-text-4&quot; &gt;"#, "") // 赤色テキストタグ除去
        .replace(r#"&lt;span class=&quot;colorful-text-2&quot; &gt;"#, "") // 青色テキストタグ除去
        .replace(r#"&lt;span class=&quot;colorful-text-3&quot; &gt;"#, "") // 緑色テキストタグ除去
        .replace(r#"&lt;span class=&quot;colorful-text-1&quot; &gt;"#, "") // 紫色テキストタグ除去
        // span タグの閉じタグを除去
        .replace(r#"&lt;/span&gt;"#, "")
        // 改行タグをカンマとスペースに置換
        .replace(r#"&lt;br&gt;"#, ", ")
        .replace(r#"&lt;br/&gt;"#, ", ")
        // Ino low profile 説明文のタグを除去
        .replace(r#" &lt;span class=&quot;tooltip&quot; &gt;Low Profile&lt;span class=&quot;tooltiptext&quot; style=&quot;&quot;&gt;When a character's hurtbox is entirely beneath an opponent's attack. This can be caused by crouching, certain moves, and being short.&lt;/span&gt;&lt;/span&gt;"#, "")
        // HTML エンティティ（アポストロフィ等）の置換
        .replace(r#"&#039;"#, "'")
        .replace(r#"&amp;#32;"#, "")
        .replace(r#"'''"#, "")
        // セミコロン区切りの置換　改行コードに置換
        .replace(r#"; "#, r#"\n"#)
        .replace(r#";"#, r#"\n"#)
        // バックスラッシュの除去
        .replace(r#"\\"#, "");
    char_page_response_json
}

// ======================================================================
// 以下、JSON 変換関数
// ======================================================================

/// フレームデータ JSON 文字列を MoveInfo 構造体のベクターに変換し、  
/// 指定されたファイルに整形済み JSON として書き込む非同期関数
///
/// # 引数
/// * `char_page_response_json` - 取得したキャラクターページの JSON 文字列
/// * `file` - 書き込み対象のファイルハンドル
/// * `char_count` - キャラクター定数配列 CHARS 内の対象インデックス
///
/// # 動作
/// 1. 入力 JSON 文字列から不要なタグ・エンティティを除去する。  
/// 2. 除去後の JSON を Response 構造体にデシリアライズする。  
/// 3. 各技の情報を MoveInfo 構造体に変換する。  
/// 4. 変換した MoveInfo 構造体のベクターを整形済み JSON としてファイルに書き込む。
pub async fn frames_to_json(
    mut char_page_response_json: String,
    mut file: &File,
    char_count: usize,
) {
    // 空文字列の代わりに使用するプレースホルダー　"-" を設定
    let empty = String::from("-");

    // JSON 文字列から不要なタグを除去　結果：クリーンな JSON 文字列
    char_page_response_json = remove_tags(char_page_response_json).await;

    // 除去後の JSON を Response 構造体にデシリアライズ　結果：move_data_response
    let mut move_data_response: Response = serde_json::from_str(&char_page_response_json).unwrap();
    // キャラクターの技情報のベクターを可変参照として取得
    let char_move_data = &mut move_data_response.cargoquery;
    // 変換後の MoveInfo 構造体を格納するベクターを初期化
    let mut vec_processed_moves_info = Vec::new();

    // 各技情報の処理ループ
    for move_data in char_move_data {
        // 入力情報が None ならば "-" を設定
        if move_data.title.input.is_none() {
            move_data.title.input = Some("-".to_string());
        } else {
            // 特定の入力（"j.XX during Homing Jump"）の場合はスキップ
            if *move_data.title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
                continue;
            }
        }
        // 技名称が None ならば、入力情報を名称として設定
        if move_data.title.name.is_none() {
            move_data.title.name = Some(move_data.title.input.as_ref().unwrap().to_string());
        } else {
            // 特定の技名称（"Dash Cancel", "Hoverdash", "Finish Blow", "Flight", "Escape"）の場合はスキップ
            if *move_data.title.name.as_ref().unwrap() == "Dash Cancel"
                || *move_data.title.name.as_ref().unwrap() == "Hoverdash"
                || *move_data.title.name.as_ref().unwrap() == "Finish Blow"
                || *move_data.title.name.as_ref().unwrap() == "Flight"
                || *move_data.title.name.as_ref().unwrap() == "Escape"
            {
                continue;
            }
        }

        // キャプションが "Ground" または "Air" の場合は空文字に置換
        if move_data.title.caption.is_some()
            && (move_data.title.caption.as_ref().unwrap() == "Ground"
                || move_data.title.caption.as_ref().unwrap() == "Air")
        {
            move_data.title.caption = Some(String::from(""));
        }

        // MoveInfo 構造体へ変換　各フィールドが None ならプレースホルダーを使用
        let processed_moves_info = MoveInfo {
            input: move_data.title.input.as_ref().unwrap_or(&empty).to_string(),
            name: move_data.title.name.as_ref().unwrap_or(&empty).to_string(),
            damage: move_data
                .title
                .damage
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            guard: move_data.title.guard.as_ref().unwrap_or(&empty).to_string(),
            startup: move_data
                .title
                .startup
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            active: move_data
                .title
                .active
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            recovery: move_data
                .title
                .recovery
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            on_hit: move_data
                .title
                .on_hit
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            on_block: move_data
                .title
                .on_block
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            level: move_data.title.level.as_ref().unwrap_or(&empty).to_string(),
            counter: move_data
                .title
                .counter
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            move_type: move_data
                .title
                .move_type
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            risc_gain: move_data
                .title
                .risc_gain
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            risc_loss: move_data
                .title
                .risc_loss
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            wall_damage: move_data
                .title
                .wall_damage
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            input_tension: move_data
                .title
                .input_tension
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            chip_ratio: move_data
                .title
                .chip_ratio
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            otg_ratio: move_data
                .title
                .otg_ratio
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            scaling: move_data
                .title
                .scaling
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            invincibility: move_data
                .title
                .invincibility
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            cancel: move_data
                .title
                .cancel
                .as_ref()
                .unwrap_or(&empty)
                .to_string(),
            caption: move_data
                .title
                .caption
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
            notes: move_data
                .title
                .notes
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
        };

        // 処理済み技情報をベクターへ追加
        vec_processed_moves_info.push(processed_moves_info);
    }

    // 変換済みの MoveInfo ベクターを整形済み JSON としてファイルへ書き込み
    file.write_all(&(serde_json::to_vec_pretty(&vec_processed_moves_info).unwrap()))
        .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count] + ".json'."));
}
