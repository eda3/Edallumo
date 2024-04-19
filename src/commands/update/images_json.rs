use crate::{ImageLinks, CHARS};
use md5::{Digest, Md5};
use regex::Regex;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
extern crate ureq;
//use ureq::Error;
//use std::fs::OpenOptions;

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

const IMAGE_HALF: &str = "https://www.dustloop.com/wiki/images";

pub async fn images_to_json(mut chara_response_json: String, mut file: &File, char_count: usize) {
    let mut re = Regex::new(r#""c.S""#).unwrap();
    chara_response_json = re.replace_all(&chara_response_json, r#""近S""#).to_string();

    re = Regex::new(r#""f.S""#).unwrap();
    chara_response_json = re.replace_all(&chara_response_json, r#""遠S""#).to_string();

    // re = Regex::new(r#"H""#).unwrap();
    // chara_response_json = re
    // .replace_all(&chara_response_json, r#"HS""#)
    // .to_string();

    re = Regex::new(r#""j\.(.+?)""#).unwrap();
    chara_response_json = re.replace_all(&chara_response_json, r#""j$1""#).to_string();

    chara_response_json = chara_response_json
        .replace(r#""w."#, r#""w"#)
        .replace(r#""w6H"#, r#""w6HS"#)
        .replace(r#""wH"#, r#""wHS"#)
        .replace(r#"j.4D"#, r#"j4D"#)
        .replace(r#""2H""#, r#""2HS""#)
        .replace(r#""5H""#, r#""5HS""#)
        .replace(r#""6H""#, r#""6HS""#)
        .replace(r#""jH""#, r#""jHS""#)
        .replace(r#""236H""#, r#""236HS""#)
        .replace(r#""623H""#, r#""623HS""#)
        .replace(r#""214H""#, r#""214HS""#)
        .replace(r#""41236H""#, r#""41236HS""#)
        .replace(r#""632146H""#, r#""632146HS""#)
        .replace(r#""j632146H""#, r#""j632146HS""#)
        .replace(r#""236236H""#, r#""236236HS""#)
        .replace(r#""Wild Assault""#, r#""ワイルドアサルト""#)
        .replace(r#""Charged Wild Assault""#, r#""溜めワイルドアサルト""#)
        .replace(r#""Wild Assault (Hold)""#, r#""溜めワイルドアサルト""#)
        .replace(r#""Ground Throw""#, r#""投げ""#)
        .replace(r#""Air Throw""#, r#""空投げ""#)
        .replace(r#""空投げ(j6D or j.4D)""#, r#""空投げ(j6D or j4D)""#)
        .replace(r#""Shitsu""#, r#""疾""#)
        .replace(r#""Suigetsu No Hakobi""#, r#""水月のハコビ""#)
        .replace(r#""Kou""#, r#""紅""#)
        .replace(r#""Fuujin""#, r#""風神""#)
        .replace(r#""Shin: Ichishiki""#, r#""針・壱式""#)
        .replace(r#""Issokutobi""#, r#""一足飛び""#)
        .replace(r#""Nagiha""#, r#""凪刃""#)
        .replace(r#""Rin""#, r#""臨""#)
        .replace(r#""Midare""#, r#""乱""#)
        .replace(r#""Issei Ougi: Sai""#, r#""一誠奥義「彩」""#)
        .replace(r#""Kachoufuugetsu Kai""#, r#""花鳥風月改""#)
        .replace(r#""Near Kachoufuugetsu Kai""#, r#""花鳥風月近""#)
        .replace(r#""Far Kachoufuugetsu Kai""#, r#""花鳥風月遠""#)
        .replace(r#""Rolling Movement""#, r#""ローリング移動""#)
        .replace(r#""Stop and Dash""#, r#""ストップアンドダッシュ""#)
        .replace(r#""Kick Start My Heart""#, r#""キックスタートマイハート""#)
        .replace(r#""Shoot""#, r#""発射""#)
        .replace(r#""Brake""#, r#""停止""#)
        .replace(r#""Starship""#, r#""スターシップ""#)
        .replace(r#""Roger Dive""#, r#""ロジャーダイブ""#)
        .replace(r#""Rock the Baby""#, r#""ロックザベイビー""#)
        .replace(r#""Air Rock the Baby""#, r#""空中ロックザベイビー""#)
        .replace(
            r#""Return of the Killing Machine""#,
            r#""帰ってきたキルマシーン""#,
        )
        .replace(r#""Loop the Loop""#, r#""ループザループ""#)
        .replace(r#""Wall Run""#, r#""壁走り""#)
        .replace(r#""Wall Run "#, r#""壁走り"#)
        .replace(r#""壁走りH""#, r#""壁走りHS""#)
        .replace(r#""壁走り6H""#, r#""壁走り6HS""#)
        .replace(r#""Gamma Blade""#, r#""γブレード""#)
        .replace(r#""Alpha Blade (Diagonal)""#, r#""αブレード・斜め""#)
        .replace(r#""Alpha Blade (Horizontal)""#, r#""αブレード・横""#)
        .replace(r#""Resshou""#, r#""冽掌""#)
        .replace(r#""Rokusai""#, r#""麓砕""#)
        .replace(r#""Senshuu""#, r#""穿踵""#)
        .replace(r#""Beta Blade""#, r#""βブレード""#)
        .replace(r#""Genrouzan""#, r#""幻朧斬""#)
        .replace(r#""Shuriken""#, r#""手裏剣""#)
        .replace(
            r#""Air Alpha Blade (Diagonal)""#,
            r#""空中αブレード・斜め""#,
        )
        .replace(
            r#""Air Alpha Blade (Horizontal)""#,
            r#""空中αブレード・横""#,
        )
        .replace(r#""Air Beta Blade""#, r#""空中βブレード""#)
        .replace(r#""Banki Messai""#, r#""万鬼滅砕""#)
        .replace(r#""Zansei Rouga""#, r#""斬星狼牙""#)
        .replace(r#""Air Zansei Rouga""#, r#""空中斬星狼牙""#)
        .replace(r#""Tandem Top""#, r#""Sタンデム""#)
        .replace(r#""H Tandem Top""#, r#""HSタンデム""#)
        .replace(r#""Lust Shaker""#, r#""ラストシェイカー""#)
        .replace(r#""Iron Savior""#, r#""アイアンセイバー""#)
        .replace(r#""Bad Moon""#, r#""バッドムーン""#)
        .replace(r#""Turbo Fall""#, r#""高速落下""#)
        .replace(r#""Mirazh""#, r#""ミラーシュ""#)
        .replace(r#""Kapel""#, r#""カピエル""#)
        .replace(r#""Septem Voices""#, r#""セプテムヴォイシズ""#)
        .replace(r#""Winger""#, r#""ヴィンガー""#)
        .replace(r#""Artemis""#, r#""アルテミス""#)
        .replace(r#""Hammer Fall""#, r#""ハンマーフォール""#)
        .replace(r#""Hammer Fall Break""#, r#""ハンマーフォールブレーキ""#)
        .replace(r#""Potemkin Buster""#, r#""ポチョムキンバスター""#)
        .replace(r#""Heat Knuckle""#, r#""ヒートナックル""#)
        .replace(r#""Mega Fist""#, r#""メガフィスト・前方""#)
        .replace(r#""B Mega Fist""#, r#""メガフィスト・後方""#)
        .replace(r#""Forward Mega Fist""#, r#""メガフィスト・前方""#)
        .replace(r#""Backward Mega Fist""#, r#""メガフィスト・後方""#)
        .replace(r#""Slide Head""#, r#""スライドヘッド""#)
        .replace(r#""Garuda Impact""#, r#""ガルダインパクト""#)
        .replace(
            r#""Heavenly Potemkin Buster""#,
            r#""ヘブンリーポチョムキンバスター""#,
        )
        .replace(r#""Giganter Kai""#, r#""ガイガンダー改""#)
        .replace(r#""Giganter Kai Barrier""#, r#""ガイガンダー改バリア""#)
        .replace(r#""Giganter Kai (Barrier)""#, r#""ガイガンダー改バリア""#)
        .replace(r#""[4]6H P""#, r#""[4]6HS P""#)
        .replace(r#""[4]6H""#, r#""[4]6HS""#)
        .replace(r#""Heat Tackle""#, r#""ヒートタックル""#);

    let mut imagedata: Imageresponse = serde_json::from_str(&chara_response_json).unwrap();

    for x in 0..imagedata.cargoquery.len() {
        // Variable that the produced hitbox links will reside
        let mut hitboxes_link: Vec<String> = Vec::new();
        // Variable that the produced image link will reside
        let image_link;

        // Replacing None values with a generic '-'
        if imagedata.cargoquery[x].title.input.is_none() {
            imagedata.cargoquery[x].title.input = Some("".to_string());
        } else {
            // Skips finish blow for sol
            if *imagedata.cargoquery[x].title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
                continue;
            }
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
        } else {
            // Skips dash cancel entry ino hoverdash chipp escape zato flight and finish blow
            if imagedata.cargoquery[x]
                .title
                .name
                .as_ref()
                .unwrap()
                .to_string()
                .trim()
                == "Dash Cancel"
                || imagedata.cargoquery[x]
                    .title
                    .name
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .trim()
                    == "Hoverdash"
                || imagedata.cargoquery[x]
                    .title
                    .name
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .trim()
                    == "Finish Blow"
                || imagedata.cargoquery[x]
                    .title
                    .name
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .trim()
                    == "Flight"
                || imagedata.cargoquery[x]
                    .title
                    .name
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .trim()
                    == "Escape"
            {
                continue;
            }
        }
        if imagedata.cargoquery[x].title.images.is_none() {
            image_link = "".to_string();
        } else {
            // If image field contains only spaces
            if imagedata.cargoquery[x]
                .title
                .images
                .as_ref()
                .unwrap()
                .trim()
                == ""
            {
                image_link = "".to_string();
            } else {
                // Multiple image names
                // Removing any subsequent image names from field
                if imagedata.cargoquery[x]
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

                    // Sending image name to make_link to become a link
                    image_link = make_link(
                        imagedata.cargoquery[x]
                            .title
                            .images
                            .as_ref()
                            .unwrap()
                            .to_string(),
                    )
                    .await;
                } else {
                    // Single image name
                    imagedata.cargoquery[x].title.images = Some(
                        imagedata.cargoquery[x]
                            .title
                            .images
                            .as_ref()
                            .unwrap()
                            .to_string()
                            .replace(' ', "_"),
                    );
                    // Sending image name to make_link to become a link
                    image_link = make_link(
                        imagedata.cargoquery[x]
                            .title
                            .images
                            .as_ref()
                            .unwrap()
                            .to_string(),
                    )
                    .await;
                }
            }
        }

        // If hitbox empty
        if imagedata.cargoquery[x].title.hitboxes.is_none() {
            hitboxes_link.push("".to_string());
        } else {
            // // If image field contains only spaces
            // if imagedata.cargoquery[x].title.hitboxes.as_ref().unwrap().trim() == "" {
            //     hitboxes_link.push("".to_string());
            // }
            // Remove any hitbox images for throws cause they dont exist
            if imagedata.cargoquery[x]
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
                // Splitting the hitboxes names into a vector
                let hitbox_str: Vec<&str> = imagedata.cargoquery[x]
                    .title
                    .hitboxes
                    .as_ref()
                    .unwrap()
                    .split(';')
                    .collect();

                for hitbox_string in &hitbox_str {
                    // Sending hitbox names to make_link to become a vector of links
                    hitboxes_link
                        .push(make_link(hitbox_string.to_string().trim().replace(' ', "_")).await);
                }
            }
        }

        let input_str = imagedata.cargoquery[x]
            .title
            .input
            .as_ref()
            .unwrap()
            .to_string();
        let mut input_name = String::new();

        if [
            "2D", "2HS", "2K", "2P", "2S", "5D", "5HS", "5K", "5P", "5[D]", "6HS", "6K", "6P",
            "近S", "遠S", "S", "H", "jD", "jHS", "jK", "jP", "jS","Reflect Projectile",
        ]
        .contains(&input_str.as_str())
        {
            input_name = input_str;
        } else {
            let name_str = imagedata.cargoquery[x]
                .title
                .name
                .as_ref()
                .unwrap()
                .to_string();
            input_name = format!("{}({})", name_str, input_str);
        }

        // Serializing image data
        let processed_imagedata = serde_json::to_string_pretty(&ImageLinks {
            input: input_name.to_string(),
            move_img: image_link,
            hitbox_img: hitboxes_link,
        })
        .unwrap();

        write!(file, "{}", processed_imagedata)
            .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count] + ".json'."));

        // Skip writting comma/tab if next and last iteration
        // contains 'finish blow' in last the input field
        if x == imagedata.cargoquery.len() - 2
            && *imagedata.cargoquery[x + 1].title.input.as_ref().unwrap()
                == "j.XX during Homing Jump"
        {
            continue;
        } else if x != imagedata.cargoquery.len() - 1 {
            // Adding comma/tab
            //file.write(b",\n\t")
            //    .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS[char_count]+ ".json'."));
            (&mut file).write_all(b",\n\t").expect(
                &("\nFailed to write ',\\n\\t' while serializing '".to_owned()
                    + CHARS[char_count]
                    + ".json'."),
            );
        }
    }
}

async fn make_link(image_name: String) -> String {
    let image_bytes = image_name.as_bytes();

    // Creating a Md5 hasher instance
    let mut hasher = Md5::new();
    hasher.update(image_bytes);
    // Converting hex to string
    let result = format!("{:x}", hasher.finalize());
    // Getting the first two hex digits from the md5sum
    // let char1 = result.chars().nth(0).unwrap();
    let char1 = result.chars().next().unwrap();
    let char2 = result.chars().nth(1).unwrap();
    // Making final link by concating
    // https://www.dustloop.com/wiki/images/first hex digit/first hex second hex/image names with underscores instead of spaces
    let image_link = format!("{}/{}/{}{}/{}", IMAGE_HALF, char1, char1, char2, image_name);

    // // Debug testing links and outputting the broken ones in a file
    // match ureq::get(&image_link).call() {
    //     Ok(_) => {},
    //     Err(Error::Status(code, _/*response*/)) => {
    //         // Creating character images json file
    //         let mut file = OpenOptions::new()
    //             .create(true)
    //             .append(true)
    //             .open("broken_links.txt")
    //             .expect("\nFailed to open 'broken_links.txt'");

    //         write!(file, "Code: {}, Link: {}\n", code, image_link)
    //             .expect("\nFailed to write to 'broken_links.txt'");
    //     }
    //     Err(_) => {}
    // }

    image_link
}
