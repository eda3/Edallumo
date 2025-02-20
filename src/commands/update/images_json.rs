use crate::{ImageLinks, CHARS};
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
extern crate ureq;
use crate::common::link_utils;
use crate::common::preprocess;

#[derive(Deserialize, Debug)]
struct Imageresponse {
    #[serde(rename = "cargoquery")]
    cargoquery: Vec<Imagedata>,
}

#[derive(Deserialize, Debug)]
struct Imagedata {
    #[serde(rename = "title")]
    title: Imagetitle,
}

#[derive(Deserialize, Debug)]
struct Imagetitle {
    input: Option<String>,
    name: Option<String>,
    images: Option<String>,
    hitboxes: Option<String>,
}

/// 画像データ JSON を前処理後、パースしてファイルへシリアライズする関数
pub async fn images_to_json(mut chara_response_json: String, mut file: &File, char_count: usize) {
    // 共通前処理を適用
    chara_response_json = preprocess::preprocess_json(chara_response_json);

    let mut imagedata: Imageresponse = serde_json::from_str(&chara_response_json).unwrap();

    for x in 0..imagedata.cargoquery.len() {
        let mut hitboxes_link: Vec<String> = Vec::new();
        let image_link;
        if imagedata.cargoquery[x].title.input.is_none() {
            imagedata.cargoquery[x].title.input = Some("".to_string());
        } else if *imagedata.cargoquery[x].title.input.as_ref().unwrap()
            == "j.XX during Homing Jump"
        {
            continue;
        }
        if imagedata.cargoquery[x].title.name.is_none() {
            imagedata.cargoquery[x].title.name = Some(
                imagedata.cargoquery[x]
                    .title
                    .input
                    .as_ref()
                    .unwrap()
                    .to_string(),
            );
        } else if *imagedata.cargoquery[x].title.name.as_ref().unwrap() == "Dash Cancel"
            || *imagedata.cargoquery[x].title.name.as_ref().unwrap() == "Hoverdash"
            || *imagedata.cargoquery[x].title.name.as_ref().unwrap() == "Finish Blow"
            || *imagedata.cargoquery[x].title.name.as_ref().unwrap() == "Flight"
            || *imagedata.cargoquery[x].title.name.as_ref().unwrap() == "Escape"
        {
            continue;
        }
        if imagedata.cargoquery[x].title.images.is_none()
            || imagedata.cargoquery[x]
                .title
                .images
                .as_ref()
                .unwrap()
                .trim()
                == ""
        {
            image_link = "".to_string();
        } else if imagedata.cargoquery[x]
            .title
            .images
            .as_mut()
            .unwrap()
            .contains(';')
        {
            let split_image: Vec<&str> = imagedata.cargoquery[x]
                .title
                .images
                .as_mut()
                .unwrap()
                .split(';')
                .collect();
            imagedata.cargoquery[x].title.images =
                Some(split_image[0].to_string().replace(' ', "_"));
            image_link = link_utils::make_link(
                imagedata.cargoquery[x]
                    .title
                    .images
                    .as_ref()
                    .unwrap()
                    .to_string(),
            )
            .await;
        } else {
            imagedata.cargoquery[x].title.images = Some(
                imagedata.cargoquery[x]
                    .title
                    .images
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .replace(' ', "_"),
            );
            image_link = link_utils::make_link(
                imagedata.cargoquery[x]
                    .title
                    .images
                    .as_ref()
                    .unwrap()
                    .to_string(),
            )
            .await;
        }
        if imagedata.cargoquery[x].title.hitboxes.is_none()
            || imagedata.cargoquery[x]
                .title
                .hitboxes
                .as_ref()
                .unwrap()
                .trim()
                .to_lowercase()
                .contains("6d")
        {
            hitboxes_link.push("".to_string());
        } else {
            let hitbox_str: Vec<&str> = imagedata.cargoquery[x]
                .title
                .hitboxes
                .as_ref()
                .unwrap()
                .split(';')
                .collect();
            for hitbox_string in &hitbox_str {
                hitboxes_link.push(
                    link_utils::make_link(hitbox_string.to_string().trim().replace(' ', "_")).await,
                );
            }
        }
        let input_str = imagedata.cargoquery[x]
            .title
            .input
            .as_ref()
            .unwrap()
            .to_string();
        let mut _input_name = String::new();
        println!("{}", &input_str.as_str());
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
        .contains(&input_str.as_str())
        {
            _input_name = input_str;
        } else {
            let name_str = imagedata.cargoquery[x]
                .title
                .name
                .as_ref()
                .unwrap()
                .to_string();
            _input_name = format!("{}({})", name_str, input_str);
        }
        let processed_imagedata = serde_json::to_string_pretty(&ImageLinks {
            input: _input_name.to_string(),
            move_img: image_link,
            hitbox_img: hitboxes_link,
        })
        .unwrap();
        write!(file, "{}", processed_imagedata)
            .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count] + ".json'."));
        if x == imagedata.cargoquery.len() - 2
            && *imagedata.cargoquery[x + 1].title.input.as_ref().unwrap()
                == "j.XX during Homing Jump"
        {
            continue;
        } else if x != imagedata.cargoquery.len() - 1 {
            (&mut file).write_all(b",\n\t").expect(
                &("\nFailed to write ',\\n\\t' while serializing '".to_owned()
                    + CHARS[char_count]
                    + ".json'."),
            );
        }
    }
}
