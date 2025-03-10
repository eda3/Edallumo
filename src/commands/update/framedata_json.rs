#![allow(clippy::needless_raw_string_hashes)]

//! `framedata_json.rs`
//!
//! Dustloop Wiki から取得したフレームデータ JSON を前処理し、
//! `MoveInfo` 構造体として整形するための機能群。
//! 不要なタグやエンティティの除去、各フィールドの補完処理を行う。

// 外部クレートおよびモジュールのインポート
use crate::common::preprocess;
use crate::{MoveInfo, CHARS}; // MoveInfo構造体、キャラクター定数群
use serde::Deserialize; // JSONデシリアライズ用
use std::fs::File; // ファイル操作用
use std::io::Write; // ファイル書き込み用

extern crate ureq; // HTTPクライアント（参考用）

// ======================================================================
// JSON デシリアライズ用構造体定義
// ======================================================================

#[derive(Deserialize, Debug)]
struct Response {
    cargoquery: Vec<Data>, // 複数データエントリ群
}

#[derive(Deserialize, Debug)]
struct Data {
    title: Title, // 各エントリのタイトル情報
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Title {
    input: Option<String>,    // 入力情報（技入力）　未定義時は None
    name: Option<String>,     // 技名称　未定義時は None
    damage: Option<String>,   // ダメージ値　未定義時は None
    guard: Option<String>,    // ガード値　未定義時は None
    startup: Option<String>,  // 始動フレーム　未定義時は None
    active: Option<String>,   // アクティブフレーム　未定義時は None
    recovery: Option<String>, // リカバリーフレーム　未定義時は None
    on_hit: Option<String>,   // ヒット時効果　未定義時は None
    #[serde(rename = "onBlock")]
    on_block: Option<String>, // ブロック時効果　未定義時は None
    level: Option<String>,    // 技レベル　未定義時は None
    counter: Option<String>,  // カウンター情報　未定義時は None
    #[serde(rename = "type")]
    move_type: Option<String>, // 技種別　未定義時は None
    #[serde(rename = "riscGain")]
    risc_gain: Option<String>, // リスクゲイン　未定義時は None
    #[serde(rename = "riscLoss")]
    risc_loss: Option<String>, // リスクロス　未定義時は None
    #[serde(rename = "wallDamage")]
    wall_damage: Option<String>, // 壁ダメージ　未定義時は None
    input_tension: Option<String>, // 入力緊張度　未定義時は None
    chip_ratio: Option<String>, // チップダメージ比率　未定義時は None
    #[serde(rename = "prorate")]
    scaling: Option<String>, // ダメージスケーリング　未定義時は None
    #[serde(rename = "invuln")]
    invincibility: Option<String>, // 無敵フレーム　未定義時は None
    cancel: Option<String>,   // キャンセル情報　未定義時は None
    caption: Option<String>,  // キャプション　未定義時は None
    notes: Option<String>,    // 備考　未定義時は None
                              // hitbox_caption, images, hitboxes は未使用
}

// ======================================================================
// JSON 前処理関数
// ======================================================================

/// 与えられた JSON 文字列から不要なタグやエンティティを除去する非同期関数
///
/// # 引数
/// * `char_page_response_json` - 前処理対象の JSON 文字列
///
/// # 戻り値
/// 除去後のクリーンな JSON 文字列
async fn remove_tags(mut char_page_response_json: String) -> String {
    char_page_response_json = preprocess::preprocess_json(char_page_response_json);

    // 不要な span タグ（色指定）除去
    char_page_response_json = char_page_response_json
        .replace(r#"&lt;span class=&quot;colorful-text-4&quot; &gt;"#, "") // 赤色タグ除去
        .replace(r#"&lt;span class=&quot;colorful-text-2&quot; &gt;"#, "") // 青色タグ除去
        .replace(r#"&lt;span class=&quot;colorful-text-3&quot; &gt;"#, "") // 緑色タグ除去
        .replace(r#"&lt;span class=&quot;colorful-text-1&quot; &gt;"#, "") // 紫色タグ除去
        .replace(r#"&lt;/span&gt;"#, "")                               // 閉じタグ除去
        .replace(r#"&lt;br&gt;"#, ", ")                                // 改行タグ置換
        .replace(r#"&lt;br/&gt;"#, ", ")                               // 改行タグ置換
        .replace(
            r#" &lt;span class=&quot;tooltip&quot; &gt;Low Profile&lt;span class=&quot;tooltiptext&quot; style=&quot;&quot;&gt;When a character's hurtbox is entirely beneath an opponent's attack. This can be caused by crouching, certain moves, and being short.&lt;/span&gt;&lt;/span&gt;"#,
            "",
        ) // tooltipタグ除去
        .replace(r#"&#039;"#, "'")    // アポストロフィ置換
        .replace(r#"&amp;#32;"#, "")   // 不要文字除去
        .replace(r#"'''"#, "")         // 重複引用符除去
        .replace(r#"; "#, r#"\n"#)      // セミコロン置換
        .replace(';', r#"\n"#)       // セミコロン置換
        .replace(r#"\\"#, ""); // バックスラッシュ除去
    char_page_response_json // 除去後の文字列返却
}

/// 特殊な入力または技名称の場合にスキップするかどうかを判断する関数
fn should_skip_move(move_data: &Data) -> bool {
    // 特定の入力 ("j.XX during Homing Jump") の場合、処理スキップ
    if move_data.title.input.is_some()
        && *move_data.title.input.as_ref().unwrap() == "j.XX during Homing Jump"
    {
        return true;
    }

    // 特定の技名称の場合、処理スキップ
    if move_data.title.name.is_some() {
        let name = move_data.title.name.as_ref().unwrap();
        if name == "Dash Cancel"
            || name == "Hoverdash"
            || name == "Finish Blow"
            || name == "Flight"
            || name == "Escape"
        {
            return true;
        }
    }

    false
}

/// 移動データを前処理する関数（入力と名前の補完、キャプションの正規化）
fn preprocess_move_data(move_data: &mut Data) {
    // 入力情報が未定義の場合、プレースホルダー "-" を設定
    if move_data.title.input.is_none() {
        move_data.title.input = Some("-".to_string());
    }

    // 技名称が未定義の場合、入力情報を技名称として設定
    if move_data.title.name.is_none() {
        move_data.title.name = Some(move_data.title.input.as_ref().unwrap().to_string());
    }

    // キャプションが "Ground" または "Air" の場合、空文字に置換
    if move_data.title.caption.is_some()
        && (move_data.title.caption.as_ref().unwrap() == "Ground"
            || move_data.title.caption.as_ref().unwrap() == "Air")
    {
        move_data.title.caption = Some(String::new());
    }
}

/// 入力名を整形する関数
fn format_input_name(input: &str, name: &str) -> String {
    if [
        "Shooting Time Stretch (Accelerate)",
        "Shooting Time Stretch (Decelerate)",
        "Terra Metron",
        "ステイン",
    ]
    .contains(&input)
    {
        input.to_string()
    } else {
        format!("{name}({input})")
    }
}

// 型エイリアス定義
/// 文字列フィールドのタプル型
type StringFields = (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
);
/// 整数フィールドのタプル型
type IntegerFields = (Option<i32>, Option<i32>, Option<i32>, Option<i32>);
/// 浮動小数点フィールドのタプル型
type FloatFields = (Option<f64>, Option<f64>, Option<f64>, Option<f64>);

/// `文字列型フィールドをMoveInfoに設定する補助関数`
fn set_string_fields(move_data: &Data, empty: &str) -> StringFields {
    (
        // 名前
        move_data.title.name.as_deref().unwrap_or(empty).to_string(),
        // ガード
        move_data
            .title
            .guard
            .as_deref()
            .unwrap_or(empty)
            .to_string(),
        // アクティブフレーム
        move_data
            .title
            .active
            .as_deref()
            .unwrap_or(empty)
            .to_string(),
        // ヒット効果
        move_data
            .title
            .on_hit
            .as_deref()
            .unwrap_or(empty)
            .to_string(),
        // ブロック効果
        move_data
            .title
            .on_block
            .as_deref()
            .unwrap_or(empty)
            .to_string(),
        // 技レベル
        move_data
            .title
            .level
            .as_deref()
            .unwrap_or(empty)
            .to_string(),
        // カウンター
        move_data
            .title
            .counter
            .as_deref()
            .unwrap_or(empty)
            .to_string(),
        // 技種別
        move_data
            .title
            .move_type
            .as_deref()
            .unwrap_or(empty)
            .to_string(),
        // 無敵フレーム
        move_data
            .title
            .invincibility
            .as_deref()
            .unwrap_or(empty)
            .to_string(),
        // キャンセル情報
        move_data
            .title
            .cancel
            .as_deref()
            .unwrap_or(empty)
            .to_string(),
    )
}

/// 数値型フィールドをMoveInfoに設定する補助関数（整数型）
fn set_integer_fields(move_data: &Data) -> IntegerFields {
    (
        // ダメージ
        move_data
            .title
            .damage
            .as_ref()
            .and_then(|s| s.parse::<i32>().ok()),
        // 始動フレーム
        move_data
            .title
            .startup
            .as_ref()
            .and_then(|s| s.parse::<i32>().ok()),
        // リカバリーフレーム
        move_data
            .title
            .recovery
            .as_ref()
            .and_then(|s| s.parse::<i32>().ok()),
        // 壁ダメージ
        move_data
            .title
            .wall_damage
            .as_ref()
            .and_then(|s| s.parse::<i32>().ok()),
    )
}

/// 数値型フィールドをMoveInfoに設定する補助関数（浮動小数点型）
fn set_float_fields(move_data: &Data) -> FloatFields {
    (
        // リスクゲイン
        move_data
            .title
            .risc_gain
            .as_ref()
            .and_then(|s| s.parse::<f64>().ok()),
        // リスクロス
        move_data
            .title
            .risc_loss
            .as_ref()
            .and_then(|s| s.parse::<f64>().ok()),
        // 入力緊張度
        move_data
            .title
            .input_tension
            .as_ref()
            .and_then(|s| s.parse::<f64>().ok()),
        // チップ比率
        move_data
            .title
            .chip_ratio
            .as_ref()
            .and_then(|s| s.parse::<f64>().ok()),
    )
}

/// `移動データからMoveInfo構造体を作成する関数`
fn create_move_info(move_data: &Data, empty: &str) -> MoveInfo {
    // 入力名の整形
    let input_str = move_data.title.input.as_deref().unwrap_or("");
    let name_str = move_data.title.name.as_deref().unwrap_or("");
    let input_name = format_input_name(input_str, name_str);

    // 文字列フィールドをまとめて取得
    let (name, guard, active, on_hit, on_block, level, counter, move_type, invincibility, cancel) =
        set_string_fields(move_data, empty);

    // 整数型フィールドをまとめて取得
    let (damage, startup, recovery, wall_damage) = set_integer_fields(move_data);

    // 浮動小数点型フィールドをまとめて取得
    let (risc_gain, risc_loss, input_tension, chip_ratio) = set_float_fields(move_data);

    // スケーリングはユースケースが単独なので個別に取得
    let scaling = move_data
        .title
        .scaling
        .as_ref()
        .and_then(|s| s.parse::<f64>().ok());

    // キャプションと備考は既定値が異なるので個別に取得
    let caption = move_data.title.caption.as_deref().unwrap_or("").to_string();
    let notes = move_data.title.notes.as_deref().unwrap_or("").to_string();

    // MoveInfo 構造体へ変換
    MoveInfo {
        input: input_name,
        name,
        damage,
        guard,
        startup,
        active,
        recovery,
        on_hit,
        on_block,
        level,
        counter,
        move_type,
        risc_gain,
        risc_loss,
        wall_damage,
        input_tension,
        chip_ratio,
        scaling,
        invincibility,
        cancel,
        caption,
        notes,
    }
}

/// フレームデータをJSON形式に変換するメイン関数
pub async fn frames_to_json(
    mut char_page_response_json: String,
    mut file: &File,
    char_count: usize,
) {
    // "-" をプレースホルダーとして設定
    let empty = String::from("-");

    // 不要タグ除去処理実施　結果：クリーンな JSON 文字列
    char_page_response_json = remove_tags(char_page_response_json).await;

    // JSON 文字列を Response 構造体へデシリアライズ　結果：move_data_response 取得
    let mut move_data_response: Response = serde_json::from_str(&char_page_response_json).unwrap();
    // 技情報群の可変参照取得　結果：char_move_data
    let char_move_data = &mut move_data_response.cargoquery;
    // MoveInfo 変換済みデータ格納用ベクター初期化　結果：vec_processed_moves_info
    let mut vec_processed_moves_info = Vec::new();

    // 各技情報処理ループ　結果：各技情報の補完と変換
    for move_data in char_move_data {
        // 前処理
        preprocess_move_data(move_data);

        // スキップすべき技かどうかを確認
        if should_skip_move(move_data) {
            continue;
        }

        // MoveInfo構造体の作成と追加
        let processed_moves_info = create_move_info(move_data, &empty);
        vec_processed_moves_info.push(processed_moves_info);
    }

    // 変換済み MoveInfo ベクターを整形済み JSON としてファイルへ書き込み
    file.write_all(&(serde_json::to_vec_pretty(&vec_processed_moves_info).unwrap()))
        .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count] + ".json'."));
}
