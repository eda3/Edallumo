//! images_json.rs
//!
//! このファイルは、Dustloop Wiki から取得した画像関連の JSON データを前処理し、
//! ImageLinks 構造体の形式に変換するための関数群を定義する。

extern crate ureq;
use crate::common::preprocess; // JSON 前処理関数群の利用
use crate::{ImageLinks, CHARS}; // ImageLinks 構造体およびキャラクター定数群の利用
use md5::{Digest, Md5}; // MD5 ハッシュ計算ライブラリの利用
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

/// 与えられた画像関連 JSON 文字列に対して前処理を実施し、
/// ImageLinks 構造体の形式に変換した整形済み JSON を指定ファイルに書き込む非同期関数
///
/// # 引数
/// * `char_images_response_json` - 取得した画像データの JSON 文字列（前処理前）
/// * `file` - 書き込み対象のファイルハンドル
/// * `char_count` - CHARS 配列内の対象キャラクターのインデックス
///
/// # 動作
/// 1. 前処理関数 `preprocess_json` により、不要なタグやエンティティを除去する。  
/// 2. 除去後の JSON を `ImageResponse` 構造体にデシリアライズする。  
/// 3. 各画像データを処理し、ImageLinks 構造体へ変換する。  
/// 4. 変換済みのデータを整形済み JSON としてファイルへ書き込む。
pub async fn images_to_json(
    mut char_images_response_json: String,
    mut file: &File,
    char_count: usize,
) {
    // 共通前処理適用
    // JSON 文字列を前処理関数で整形　前処理結果：整形済み JSON 文字列
    char_images_response_json = preprocess::preprocess_json(char_images_response_json);

    // アポストロフィのエンティティを置換　置換結果：アポストロフィに変換
    char_images_response_json = char_images_response_json.replace(r#"&#039;"#, "'");

    // JSON 文字列を ImageResponse 構造体にデシリアライズ　結果：image_data_response
    let mut image_data_response: ImageResponse =
        serde_json::from_str(&char_images_response_json).unwrap();
    // 画像データエントリの可変参照取得　結果：char_image_data
    let char_image_data = &mut image_data_response.cargoquery;
    // 変換後の ImageLinks 構造体を格納するベクターを初期化　結果：vec_processed_imagedata
    let mut vec_processed_imagedata = Vec::new();

    // 各画像データエントリに対するループ処理
    for image_data in char_image_data {
        // ヒットボックスリンク格納用ベクターを初期化　結果：hitbox_links
        let mut hitbox_links: Vec<String> = Vec::new();
        // 画像リンク格納用変数の宣言　結果：image_link
        let image_link;

        // 入力文字列が未定義の場合は空文字に置換　結果：image_data.title.input が定義される
        if image_data.title.input.is_none() {
            image_data.title.input = Some("".to_string());
        } else {
            // 特定の入力（"j.XX during Homing Jump"）の場合はスキップ　動作：continue
            if *image_data.title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
                continue;
            }
        }
        // 画像名称が未定義の場合は入力文字列を名称として設定　結果：image_data.title.name に値が入る
        if image_data.title.name.is_none() {
            image_data.title.name = Some(image_data.title.input.as_ref().unwrap().to_string());
        } else {
            // 特定の画像名称（"Dash Cancel", "Hoverdash", "Finish Blow", "Flight", "Escape"）の場合はスキップ
            if image_data.title.name.as_ref().unwrap().to_string().trim() == "Dash Cancel"
                || image_data.title.name.as_ref().unwrap().to_string().trim() == "Hoverdash"
                || image_data.title.name.as_ref().unwrap().to_string().trim() == "Finish Blow"
                || image_data.title.name.as_ref().unwrap().to_string().trim() == "Flight"
                || image_data.title.name.as_ref().unwrap().to_string().trim() == "Escape"
            {
                continue;
            }
        }
        // 画像ファイル名が未定義の場合は空文字とする　結果：image_link に空文字が設定
        if image_data.title.images.is_none() {
            image_link = "".to_string();
        } else {
            // 画像ファイル名が空白のみの場合は空文字に設定　結果：image_link に空文字が設定
            if image_data.title.images.as_ref().unwrap().trim() == "" {
                image_link = "".to_string();
            } else {
                // 複数の画像ファイル名が存在する場合の処理
                if image_data.title.images.as_mut().unwrap().contains(';') {
                    // セミコロンで分割し、先頭要素を使用　結果：split_image[0] に設定
                    let split_image: Vec<&str> = image_data
                        .title
                        .images
                        .as_mut()
                        .unwrap()
                        .split(';')
                        .collect();

                    image_data.title.images = Some(split_image[0].to_string().replace(' ', "_"));

                    // 画像リンク生成関数を呼び出し　結果：image_link に生成されたリンクを設定
                    image_link =
                        make_link(image_data.title.images.as_ref().unwrap().to_string()).await;
                } else {
                    // 単一の画像ファイル名の場合　結果：画像ファイル名の空白をアンダースコアに置換
                    image_data.title.images = Some(
                        image_data
                            .title
                            .images
                            .as_ref()
                            .unwrap()
                            .to_string()
                            .replace(' ', "_"),
                    );
                    // 画像リンク生成関数を呼び出し　結果：image_link に生成されたリンクを設定
                    image_link =
                        make_link(image_data.title.images.as_ref().unwrap().to_string()).await;
                }
            }
        }

        // ヒットボックス情報が未定義の場合は空文字をベクターに追加　結果：hitbox_links に空文字追加
        if image_data.title.hitboxes.is_none() {
            hitbox_links.push("".to_string());
        } else {
            // ヒットボックス情報をセミコロンで分割　結果：hitbox_str に分割された各ヒットボックス名を格納
            let hitbox_str: Vec<&str> = image_data
                .title
                .hitboxes
                .as_ref()
                .unwrap()
                .split(';')
                .collect();

            // 各ヒットボックス名に対して画像リンク生成関数を呼び出し　結果：hitbox_links に生成されたリンクを追加
            for hitbox_string in &hitbox_str {
                hitbox_links
                    .push(make_link(hitbox_string.to_string().trim().replace(' ', "_")).await);
            }
        }

        let input_str = &image_data.title.input.as_deref().unwrap_or("");
        let mut _input_name = String::new();
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
            "5[D]",
            "6HS",
            "6K",
            "6P",
            "近S",
            "遠S",
            "S",
            "6S",
            "H",
            "jD",
            "jHS",
            "jK",
            "jP",
            "jS",
            "j2K",
            "j2H",
            "JR2HS",
            "JR2K",
            "JR2P",
            "JR2S",
            "JR5HS",
            "JR5K",
            "JR5P",
            "JR6HS",
            "JR6P",
            "JR近S",
            "JR遠S",
            "JRjD",
            "JRjHS",
            "JRjK",
            "JRjP",
            "JRjS",
            "JR解除",
            "6HSHS",
            "6HSHSHS",
            "銃を構える(HS)",
            "BR Activation",
            "BR Deactivation",
            "214X (Discard)",
            "214X (Draw)",
            "Accipiter Metron",
            "Aquila Metron",
            "Bit Shift Metron",
            "Bookmark (Auto Import)",
            "Bookmark (Full Import)",
            "Bookmark (Random Import)",
            "Chaotic Option",
            "Delayed Howling Metron",
            "Delayed Tardus Metron",
            "Go to Marker",
            "Gravity Rod (Shooting)",
            "High-Pass Filter Gravity",
            "Howling Metron",
            "Howling Metron MS Processing",
            "Low-Pass Filter Gravity",
            "Metron Arpeggio",
            "Metron Screamer 808",
            "Recover Mana (Continuous)",
            "Recover Mana (Instant)",
            "Reduce Mana Cost",
            "Repulsive Rod (Shooting)",
            "RMS Boost Metron",
            "Sampler 404",
            "Shooting Time Stretch (Accelerate)",
            "Shooting Time Stretch (Decelerate)",
            "Terra Metron",
            "ステイン",
        ]
        .contains(&input_str)
        {
            _input_name = input_str.to_string();
        } else {
            let name_str = image_data.title.name.as_deref().unwrap_or("");
            _input_name = format!("{}({})", name_str, input_str);
        }

        // ImageLinks 構造体へ変換　各フィールドは Option::unwrap で取得、未定義の場合は既定値
        let processed_imagedata = ImageLinks {
            input: _input_name.to_string(),
            // input: image_data.title.input.as_ref().unwrap().to_string(),
            move_img: image_link,
            hitbox_img: hitbox_links,
        };

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
    let result = format!("{:x}", hasher.finalize());

    // 16 進数文字列の先頭 1 文字を取得　結果：ハッシュ値の先頭文字
    let char1 = result.chars().next().unwrap();

    // 16 進数文字列の 2 文字目を取得　結果：ハッシュ値の 2 文字目
    let char2 = result.chars().nth(1).unwrap();

    // 画像リンクを組み立て　組み立て方法：基本 URL / 先頭文字 + 先頭文字 + 2文字目 / 画像名
    // 組み立て結果：生成された画像リンク
    let image_link = format!("{}/{}/{}{}/{}", IMAGE_HALF, char1, char1, char2, image_name);

    // 生成された画像リンクを返却　返却結果：最終画像リンク
    image_link
}
