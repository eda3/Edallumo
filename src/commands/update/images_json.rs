#![allow(clippy::needless_raw_string_hashes)]

//! `images_json.rs`
//!
//! このファイルは、Dustloop Wiki から取得した画像関連の JSON データを前処理し、
//! ImageLinks 構造体の形式に変換するための関数群を定義する。

extern crate ureq;
use crate::common::preprocess; // JSON 前処理関数群の利用
use crate::{ImageLinks, CHARS}; // ImageLinks 構造体およびキャラクター定数群の利用
use md5::{Digest, Md5}; // MD5 ハッシュ計算用
use serde::Deserialize; // JSON デシリアライズ用
use std::{fs::File, io::Write}; // ファイル操作および書き込み用

// ======================================================================
// JSON デシリアライズ用構造体定義
// ======================================================================

#[derive(Deserialize, Debug)]
struct ImageResponse {
    #[serde(rename = "cargoquery")]
    cargoquery: Vec<ImageData>, // 複数の画像データエントリ
}

#[derive(Deserialize, Debug)]
struct ImageData {
    #[serde(rename = "title")]
    title: ImageTitle, // 画像データのタイトル情報
}

#[derive(Deserialize, Debug)]
struct ImageTitle {
    input: Option<String>,    // 入力文字列　未定義時は None
    name: Option<String>,     // 画像名称　未定義時は None
    images: Option<String>,   // 画像ファイル名　未定義時は None
    hitboxes: Option<String>, // ヒットボックス情報　未定義時は None
}

// ======================================================================
// 定数定義
// ======================================================================

/// 画像の基本URL  
/// Dustloop Wiki の画像が配置されている基本パス
const IMAGE_HALF: &str = "https://www.dustloop.com/wiki/images";

// ======================================================================
// 画像リンク生成および JSON 変換関数群
// ======================================================================

/// キャラクター画像JSONデータを前処理する関数
async fn preprocess_images_json(mut json_str: String) -> String {
    // 共通前処理適用
    // JSON 文字列を前処理関数で整形　前処理結果：整形済み JSON 文字列
    json_str = preprocess::preprocess_json(json_str);

    // アポストロフィのエンティティを置換　置換結果：アポストロフィに変換
    json_str = json_str.replace(r#"&#039;"#, "'");

    json_str
}

/// ヒットボックスリンクを生成する関数
async fn generate_hitbox_links(hitboxes: &Option<String>) -> Vec<String> {
    let mut hitbox_links: Vec<String> = Vec::new();

    // ヒットボックス情報が未定義の場合は空文字をベクターに追加
    if hitboxes.is_none() {
        hitbox_links.push(String::new());
    } else {
        // ヒットボックス情報をセミコロンで分割
        let hitbox_str: Vec<&str> = hitboxes.as_ref().unwrap().split(';').collect();

        // 各ヒットボックス名に対して画像リンク生成関数を呼び出し
        for hitbox_string in &hitbox_str {
            hitbox_links
                .push(make_link((*hitbox_string).to_string().trim().replace(' ', "_")).await);
        }
    }

    hitbox_links
}

/// 画像リンクを生成する関数
async fn generate_image_link(images: &Option<String>) -> String {
    // 画像ファイル名が未定義の場合は空文字とする
    if images.is_none() {
        return String::new();
    }

    // 画像ファイル名が空白のみの場合は空文字に設定
    if images.as_ref().unwrap().trim() == "" {
        return String::new();
    }

    // 複数の画像ファイル名が存在する場合の処理
    if images.as_ref().unwrap().contains(';') {
        // セミコロンで分割し、先頭要素を使用
        let split_image: Vec<&str> = images.as_ref().unwrap().split(';').collect();
        // 画像リンク生成関数でリンク形式に整形
        make_link(split_image[0].to_string().trim().replace(' ', "_")).await
    } else {
        // 単一の画像ファイル名の場合の処理
        make_link(images.as_ref().unwrap().to_string()).await
    }
}

/// 入力名を整形する関数
fn format_input_name(input: &Option<String>, name: &Option<String>) -> String {
    let input_str = input.as_deref().unwrap_or("");

    // 特定の入力文字列の場合はそのまま使用
    if [
        "2D",
        "2HS",
        "2K",
        "2P",
        "2S",
        "3K",
        "5D",
        "5HS",
        "5K",
        "5P",
        "5S",
        "6K",
        "6P",
        "Fスタート",
        "Jスタート",
        "JD",
        "JHS",
        "JK",
        "JP",
        "JS",
        "Pスタート",
        "Sスタート",
        "エアダッシュ",
        "ガトリング",
        "クローゼライン",
        "ジャンプ",
        "スタンプ",
        "ダッシュ",
        "ダブルジャンプ",
        "バックステップ",
        "フォルト",
        "ロマンキャンセル",
        "入力猶予",
        "前ジャンプ",
        "後ジャンプ",
        "攻撃判定",
        "特殊技",
        "置換表",
        "通常技",
        "必殺技",
        "打撃無敵",
        "投げ",
        "投げ無敵",
        "歩き",
        "殴られ判定",
        "空中ダッシュ",
        "空中投げ",
        "立ち",
        "起き上がり",
        "走り",
        "通常投げ",
        "ステイン",
    ]
    .contains(&input_str)
    {
        input_str.to_string()
    } else {
        let name_str = name.as_deref().unwrap_or("");
        format!("{name_str}({input_str})")
    }
}

/// 画像データを処理する関数
async fn process_image_data(image_data: &mut ImageData) -> ImageLinks {
    // 入力文字列が未定義の場合は空文字に置換
    if image_data.title.input.is_none() {
        image_data.title.input = Some(String::new());
    } else if *image_data.title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
        // 特定の入力（"j.XX during Homing Jump"）の場合は空のデータを返す
        return ImageLinks {
            input: String::new(),
            move_img: String::new(),
            hitbox_img: Vec::new(),
        };
    }

    // 画像名称が未定義の場合は入力文字列を名称として設定
    if image_data.title.name.is_none() {
        image_data.title.name = image_data.title.input.clone();
    }

    // 画像リンクの生成
    let image_link = generate_image_link(&image_data.title.images).await;

    // ヒットボックスリンクの生成
    let hitbox_links = generate_hitbox_links(&image_data.title.hitboxes).await;

    // 入力名の整形
    let input_name = format_input_name(&image_data.title.input, &image_data.title.name);

    // ImageLinks 構造体へ変換
    ImageLinks {
        input: input_name,
        move_img: image_link,
        hitbox_img: hitbox_links,
    }
}

/// メイン処理：キャラクター画像データをJSONファイルに変換する関数
pub async fn images_to_json(char_images_response_json: String, mut file: &File, char_count: usize) {
    // JSONデータを前処理
    let preprocessed_json = preprocess_images_json(char_images_response_json).await;

    // JSON 文字列を ImageResponse 構造体にデシリアライズ　結果：image_data_response
    let mut image_data_response: ImageResponse = serde_json::from_str(&preprocessed_json).unwrap();

    // 画像データエントリの可変参照取得　結果：char_image_data
    let char_image_data = &mut image_data_response.cargoquery;

    // 変換後の ImageLinks 構造体を格納するベクターを初期化　結果：vec_processed_imagedata
    let mut vec_processed_imagedata = Vec::new();

    // 各画像データエントリに対するループ処理
    for image_data in char_image_data {
        // 特殊なケースをスキップする処理（"j.XX during Homing Jump"）
        if image_data.title.input.is_some()
            && *image_data.title.input.as_ref().unwrap() == "j.XX during Homing Jump"
        {
            continue;
        }

        // 各画像データを処理してImageLinks構造体に変換
        let processed_imagedata = process_image_data(image_data).await;

        // 空のデータ（スキップするべきデータ）を除外
        if processed_imagedata.input.is_empty()
            && processed_imagedata.move_img.is_empty()
            && processed_imagedata.hitbox_img.is_empty()
        {
            continue;
        }

        // 処理済み画像データをベクターへ追加
        vec_processed_imagedata.push(processed_imagedata);
    }

    // 変換済みの ImageLinks ベクターを整形済み JSON としてファイルへ書き込み
    file.write_all(&(serde_json::to_vec_pretty(&vec_processed_imagedata).unwrap()))
        .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count] + ".json'."));
}

/// 画像ファイル名から MD5 ハッシュを利用して画像リンクを生成する非同期関数
///
/// # 引数
/// * `image_name` - 画像ファイル名（例："example.png"）
///
/// # 戻り値
/// 生成された画像リンク（例："https://www.dustloop.com/wiki/images/e/e1/example.png"）
async fn make_link(image_name: String) -> String {
    // 画像名をバイト列に変換　結果：画像名のバイト列
    let image_bytes = image_name.as_bytes();

    // MD5 ハッシュ計算器を生成　結果：MD5 ハッシュ計算器
    let mut hasher = Md5::new();

    // 画像のバイト列をハッシュ計算器に投入　結果：ハッシュ計算器に画像情報を反映
    hasher.update(image_bytes);

    // ハッシュ計算結果を 16 進数文字列に変換　結果：MD5 ハッシュ値（16 進数文字列）
    let hash_result = hasher.finalize();
    let result = format!("{hash_result:x}");

    // 16 進数文字列の先頭 1 文字を取得　結果：ハッシュ値の先頭文字
    let char1 = result.chars().next().unwrap();

    // 16 進数文字列の 2 文字目を取得　結果：ハッシュ値の 2 文字目
    let char2 = result.chars().nth(1).unwrap();

    // 画像リンクを組み立て　組み立て方法：基本 URL / 先頭文字 + 先頭文字 + 2文字目 / 画像名
    // 組み立て結果：生成された画像リンク
    let image_link = format!("{IMAGE_HALF}/{char1}/{char1}{char2}/{image_name}");

    // 生成された画像リンクを返却　返却結果：最終画像リンク
    image_link
}
