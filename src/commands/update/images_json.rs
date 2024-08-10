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
        .replace(r#""j236H""#, r#""j236HS""#)
        .replace(r#""623H""#, r#""623HS""#)
        .replace(r#""214H""#, r#""214HS""#)
        .replace(r#""41236H""#, r#""41236HS""#)
        .replace(r#""632146H""#, r#""632146HS""#)
        .replace(r#""63214H""#, r#""63214HS""#)
        .replace(r#""j632146H""#, r#""j632146HS""#)
        .replace(r#""236236H""#, r#""236236HS""#)
        .replace(r#""Wild Assault""#, r#""ワイルドアサルト""#)
        .replace(r#""Charged Wild Assault""#, r#""溜めワイルドアサルト""#)
        .replace(r#""Wild Assault (Hold)""#, r#""溜めワイルドアサルト""#)
        .replace(r#""Ground Throw""#, r#""投げ""#)
        .replace(r#""Air Throw""#, r#""空投げ""#)
        // A.B.A
        .replace(r#""JR 2H""#, r#""JR2HS""#)
        .replace(r#""JR 2K""#, r#""JR2K""#)
        .replace(r#""JR 2P""#, r#""JR2P""#)
        .replace(r#""JR 2S""#, r#""JR2S""#)
        .replace(r#""JR 5H""#, r#""JR5HS""#)
        .replace(r#""JR 5K""#, r#""JR5K""#)
        .replace(r#""JR 5P""#, r#""JR5P""#)
        .replace(r#""JR 6H""#, r#""JR6HS""#)
        .replace(r#""JR 6P""#, r#""JR6P""#)
        .replace(r#""JR c.S""#, r#""JR近S""#)
        .replace(r#""JR f.S""#, r#""JR遠S""#)
        .replace(r#""JR j.D""#, r#""JRjD""#)
        .replace(r#""JR j.H""#, r#""JRjHS""#)
        .replace(r#""JR j.K""#, r#""JRjK""#)
        .replace(r#""JR j.P""#, r#""JRjP""#)
        .replace(r#""JR j.S""#, r#""JRjS""#)
        .replace(r#""236S~6S""#, r#""236S6S""#)
        .replace(r#""JR 214H""#, r#""JR214HS""#)
        .replace(r#""JR 214K""#, r#""JR214K""#)
        .replace(r#""JR 236K""#, r#""JR236K""#)
        .replace(r#""JR 236S""#, r#""JR236S""#)
        .replace(r#""JR 236S~6S""#, r#""JR236S6S""#)
        .replace(r#""JR 63214P""#, r#""JR63214P""#)
        .replace(r#""JR Deactivation""#, r#""JR解除""#)
        .replace(r#""JR 632146H""#, r#""JR632146HS""#)
        .replace(r#""JR 632146K""#, r#""JR632146K""#)
        .replace(r#""Bonding and Dissolving""#, r#""結合と変性""#)
        .replace(r#""Haul and Heed""#, r#""牽引と随順""#)
        .replace(r#""Frenzy and Astonishment""#, r#""逆上と驚愕""#)
        .replace(r#""Intertwine and Tilt""#, r#""戮力と傾動""#)
        .replace(r#""Menace and Groan""#, r#""威喝と嗚咽""#)
        .replace(r#""Restriction and Constraint""#, r#""抑圧と束縛""#)
        .replace(r#""Judgment and Sentiment""#, r#""断罪と情動""#)
        .replace(r#""Changing and Swaying""#, r#""変転と感化""#)
        .replace(r#""JR Bonding and Dissolving""#, r#""JR結合と変性""#)
        .replace(r#""JR Haul and Heed""#, r#""JR牽引と随順""#)
        .replace(r#""JR Intertwine and Tilt""#, r#""JR戮力と傾動""#)
        .replace(r#""JR Menace and Groan""#, r#""JR威喝と嗚咽""#)
        .replace(r#""JR Restriction and Constraint""#, r#""JR抑圧と束縛""#)
        .replace(r#""JR Changing and Swaying""#, r#""JR変転と感化""#)
        .replace(r#""Jealous Rage Deactivation""#, r#""JR解除""#)
        .replace(r#""The Law is Key, Key is King.""#, r#""鍵の支配""#)
        .replace(r#""Keeper of the Key""#, r#""鍵の守護者""#)
        .replace(r#""JR The Law is Key, Key is King.""#, r#""JR鍵の支配""#)
        .replace(r#""JR Keeper of the Key""#, r#""JR鍵の守護者""#)
        .replace(r#""Shitsu""#, r#""疾""#) // 闇慈
        .replace(r#""Suigetsu No Hakobi""#, r#""水月のハコビ""#) // 闇慈
        .replace(r#""Kou""#, r#""紅""#) // 闇慈
        .replace(r#""Fuujin""#, r#""風神""#) // 闇慈
        .replace(r#""Shin: Ichishiki""#, r#""針・壱式""#) // 闇慈
        .replace(r#""Issokutobi""#, r#""一足飛び""#) // 闇慈
        .replace(r#""Nagiha""#, r#""凪刃""#) // 闇慈
        .replace(r#""Rin""#, r#""臨""#) // 闇慈
        .replace(r#""Midare""#, r#""乱""#) // 闇慈
        .replace(r#""Issei Ougi: Sai""#, r#""一誠奥義「彩」""#) // 闇慈
        .replace(r#""Kachoufuugetsu Kai""#, r#""花鳥風月改""#) // 闇慈
        .replace(r#""Near Kachoufuugetsu Kai""#, r#""花鳥風月近""#) // 闇慈
        .replace(r#""Far Kachoufuugetsu Kai""#, r#""花鳥風月遠""#) // 闇慈
        .replace(r#""Draw""#, r#""ブックマーク(ドロー)""#) // 飛鳥
        .replace(r#""Discard""#, r#""ブックマーク(破棄)""#) // 飛鳥
        .replace(r#""Snail""#, r#""蝸牛""#) // アクセル
        .replace(r#""Whistling Wind""#, r#""虎落笛""#) // アクセル
        .replace(r#""Rainwater""#, r#""潦""#) // アクセル
        .replace(r#""Whistling Wind (Charged)""#, r#""溜め虎落笛""#) // アクセル
        .replace(r#""Winter Mantis""#, r#""冬蟷螂""#) // アクセル
        .replace(r#""Air Snail""#, r#""空中蝸牛""#) // アクセル
        .replace(r#""Axl Bomber""#, r#""アクセルボンバー""#) // アクセル
        .replace(r#""Sickle Flash""#, r#""鎌閃撃""#) // アクセル
        .replace(r#""Spinning Chain Strike""#, r#""旋鎖撃""#) // アクセル
        .replace(r#""Soaring Chain Strike""#, r#""曲鎖撃""#) // アクセル
        .replace(r#""Winter Cherry""#, r#""鬼灯""#) // アクセル
        .replace(r#""Sickle Storm""#, r#""百重鎌焼""#) // アクセル
        .replace(r#""One Vision""#, r#""ワンヴィジョン""#) // アクセル
        .replace("23S", "236S") // 梅喧
        .replace(r#""Ground Throw (Tether)""#, r#""投げ""#) // 梅喧
        .replace(r#""Ground Throw (Knockback)""#, r#""溜め投げ""#) // 梅喧
        .replace(r#""41236HS~HS""#, r#""41236HSHS""#) // 梅喧
        .replace(r#""Tatami Gaeshi""#, r#""畳返し""#) // 梅喧
        .replace(r#""Air Tatami Gaeshi""#, r#""空中畳返し""#) // 梅喧
        .replace(r#""H Kabari""#, r#""HS蚊鉤""#) // 梅喧
        .replace(r#""S Kabari""#, r#""S蚊鉤""#) // 梅喧
        .replace(r#""Kabari""#, r#""蚊鉤""#) // 梅喧
        .replace(r#""41236HH""#, r#""HS蚊鉤追撃""#) // 梅喧
        .replace(r#""41236H~H""#, r#""41236HSHS""#) // 梅喧
        .replace(r#""Kabari Followup""#, r#""HS蚊鉤追撃""#) // 梅喧
        .replace(r#""Youzansen""#, r#""妖斬扇""#) // 梅喧
        .replace(r#""Hiiragi""#, r#""柊""#) // 梅喧
        .replace(r#""Tsurane Sanzu-watashi""#, r#""連ね三途渡し""#) // 梅喧
        .replace(r#""Kenjyu""#, r#""拳銃""#) // 梅喧
        .replace("Regular Throw", "溜め投げ") // 梅喧
        .replace("Air Kenjyu", "空中拳銃") // 梅喧
        .replace(r#""Rolling Movement""#, r#""ローリング移動""#) // ブリジット
        .replace(r#""Stop and Dash""#, r#""ストップアンドダッシュ""#) // ブリジット
        .replace(r#""Kick Start My Heart""#, r#""キックスタートマイハート""#) // ブリジット
        .replace(r#""Shoot""#, r#""発射""#) // ブリジット
        .replace(r#""Brake""#, r#""停止""#) // ブリジット
        .replace(r#""Starship""#, r#""スターシップ""#) // ブリジット
        .replace(r#""Roger Dive""#, r#""ロジャーダイブ""#) // ブリジット
        .replace(r#""Rock the Baby""#, r#""ロックザベイビー""#) // ブリジット
        .replace(r#""Air Rock the Baby""#, r#""空中ロックザベイビー""#) // ブリジット
        .replace(
            r#""Return of the Killing Machine""#,
            r#""帰ってきたキルマシーン""#,
        ) // ブリジット
        .replace(r#""Loop the Loop""#, r#""ループザループ""#) // ブリジット
        .replace(r#""Wall Run""#, r#""壁走り""#) // チップ
        .replace(r#""Wall Run "#, r#""壁走り"#) // チップ
        .replace(r#""壁走りH""#, r#""壁走りHS""#) // チップ
        .replace(r#""壁走り6H""#, r#""壁走り6HS""#) // チップ
        .replace(r#""Gamma Blade""#, r#""γブレード""#) // チップ
        .replace(r#""Alpha Blade (Diagonal)""#, r#""αブレード・斜め""#) // チップ
        .replace(r#""Alpha Blade (Horizontal)""#, r#""αブレード・横""#) // チップ
        .replace(r#""Resshou""#, r#""冽掌""#) // チップ
        .replace(r#""Rokusai""#, r#""麓砕""#) // チップ
        .replace(r#""Senshuu""#, r#""穿踵""#) // チップ
        .replace(r#""Beta Blade""#, r#""βブレード""#) // チップ
        .replace(r#""Genrouzan""#, r#""幻朧斬""#) // チップ
        .replace(r#""Shuriken""#, r#""手裏剣""#) // チップ
        .replace(
            r#""Air Alpha Blade (Diagonal)""#,
            r#""空中αブレード・斜め""#,
        ) // チップ
        .replace(
            r#""Air Alpha Blade (Horizontal)""#,
            r#""空中αブレード・横""#,
        ) // チップ
        .replace(r#""Air Beta Blade""#, r#""空中βブレード""#) // チップ
        .replace(r#""Banki Messai""#, r#""万鬼滅砕""#) // チップ
        .replace(r#""Zansei Rouga""#, r#""斬星狼牙""#) // チップ
        .replace(r#""Air Zansei Rouga""#, r#""空中斬星狼牙""#) // チップ
                // エルフェルト
        .replace(r#""214S~H""#, r#""214SHS""#)
        .replace(r#""214S~K""#, r#""214SK""#)
        .replace(r#""214S~P""#, r#""214SP""#)
        .replace(r#""214S~P/K~K""#, r#""214SP/KK""#)
        .replace(r#""214S~P/K~P""#, r#""214SP/KP""#)
        .replace(r#""236S/H""#, r#""236S/HS""#)
        .replace(r#""j236S/H""#, r#""j236S/HS""#)
        .replace(r#""236236K Explosion""#, r#""236236K爆発""#)
        .replace(r#""Bomb-Bomb Chocolat""#, r#""ボンボン・ショコラ""#)
        .replace(
            r#""Miss Charlotte (Out of Repair)""#,
            r#""Missシャルロット（お手入れ不足）""#,
        )
        .replace(r#""Here I Go!""#, r#""やります！""#)
        .replace(r#""Nailed It!""#, r#""決めます！""#)
        .replace(r#""Down Low!""#, r#""下から！""#)
        .replace(r#""Up High!""#, r#""上から！""#)
        .replace(r#""Down Low! (Finisher)""#, r#""下から！（フィニッシュ）""#)
        .replace(r#""Up High! (Finisher)""#, r#""上から！（フィニッシュ）""#)
        .replace(r#""Miss Charlotte""#, r#""Missシャルロット""#)
        .replace(r#""Air Miss Charlotte""#, r#""空中Missシャルロット""#)
        .replace(r#""Bomb-Bombnnière""#, r#""ボンボニエール""#)
        .replace(r#""Bomb-Bombnnière Explosion""#, r#""ボンボニエール爆発""#)
        .replace(r#""Juganto Da Parfeo""#, r#""ジュガント ダ パルフェーオ""#)
        .replace(r#""Thrust""#, r#""突きます。""#) // ファウスト
        .replace(r#""Thrust (Charged)""#, r#""溜め突きます。""#) // ファウスト
        .replace(r#""Pull Back""#, r#""引き戻し""#) // ファウスト
        .replace(r#""Home Run!""#, r#""ナイスショット""#) // ファウスト
        .replace(
            r#""What Could This Be? (Eat)""#,
            r#""何が出るかな？（食べる）""#,
        ) // ファウスト
        .replace(r#""What Could This Be?""#, r#""何が出るかな？""#) // ファウスト
        .replace(r#""Mix Mix Mix""#, r#""涅和混練""#) // ファウスト
        .replace(r#""Air Mix Mix Mix""#, r#""空中涅和混練""#) // ファウスト
        .replace(r#""Snip Snip Snip""#, r#""メッタ刈り""#) // ファウスト
        .replace(r#""Love""#, r#""愛""#) // ファウスト
        .replace(r#""Scarecrow""#, r#""P久延毘古""#) // ファウスト
        .replace(r#""P Scarecrow""#, r#""P久延毘古""#) // ファウスト
        .replace(r#""S Scarecrow""#, r#""S久延毘古""#) // ファウスト
        .replace(r#""K Scarecrow""#, r#""K久延毘古""#) // ファウスト
        .replace(r#""Bone-crushing Excitement""#, r#""エキサイティング骨折""#) // ファウスト
        .replace(
            r#""W-W-What Could This Be?""#,
            r#""な・な・な・なにがでるかな？""#,
        ) // ファウスト
        .replace(
            r#""W-W-W-W-W-W-W-W-W-What Could This Be?""#,
            r#""な・な・な・な・な・な・な・な・な・なにがでるかな？""#,
        ) // ファウスト
        .replace(r#""Bomb""#, r#""爆弾""#) // ファウスト
        .replace(r#""Banana""#, r#""バナナ""#) // ファウスト
        .replace(r#""Donut""#, r#""ドーナツ""#) // ファウスト
        .replace(r#""Afro""#, r#""アフロ""#) // ファウスト
        .replace(r#""Hammer""#, r#""ハンマー""#) // ファウスト
        .replace(r#""Mini Faust""#, r#""ちびファウスト""#) // ファウスト
        .replace(r#""Horn""#, r#""ラッパ""#) // ファウスト
        .replace(r#""100T Weight""#, r#""100t重り""#) // ファウスト
        .replace(r#""Earthquake""#, r#""100t重り""#) // ファウスト
        .replace(r#""Meteors""#, r#""メテオ""#) // ファウスト
        .replace(r#""Love Afro""#, r#""愛アフロ""#) // ファウスト
        .replace(r#""Tandem Top""#, r#""Sタンデム""#) // ミリア
        .replace(r#""H Tandem Top""#, r#""HSタンデム""#) // ミリア
        .replace(r#""Lust Shaker""#, r#""ラストシェイカー""#) // ミリア
        .replace(r#""Iron Savior""#, r#""アイアンセイバー""#) // ミリア
        .replace(r#""Bad Moon""#, r#""バッドムーン""#) // ミリア
        .replace(r#""Turbo Fall""#, r#""高速落下""#) // ミリア
        .replace(r#""Mirazh""#, r#""ミラーシュ""#) // ミリア
        .replace(r#""Kapel""#, r#""カピエル""#) // ミリア
        .replace(r#""Septem Voices""#, r#""セプテムヴォイシズ""#) // ミリア
        .replace(r#""Winger""#, r#""ヴィンガー""#) // ミリア
        .replace(r#""Artemis""#, r#""アルテミス""#) // ミリア
        .replace(r#""Hammer Fall""#, r#""ハンマーフォール""#) // ポチョムキン
        .replace(r#""Hammer Fall Break""#, r#""ハンマーフォールブレーキ""#) // ポチョムキン
        .replace(r#""Potemkin Buster""#, r#""ポチョムキンバスター""#) // ポチョムキン
        .replace(r#""Heat Knuckle""#, r#""ヒートナックル""#) // ポチョムキン
        .replace(r#""Mega Fist""#, r#""メガフィスト・前方""#) // ポチョムキン
        .replace(r#""B Mega Fist""#, r#""メガフィスト・後方""#) // ポチョムキン
        .replace(r#""Forward Mega Fist""#, r#""メガフィスト・前方""#) // ポチョムキン
        .replace(r#""Backward Mega Fist""#, r#""メガフィスト・後方""#) // ポチョムキン
        .replace(r#""Slide Head""#, r#""スライドヘッド""#) // ポチョムキン
        .replace(r#""Garuda Impact""#, r#""ガルダインパクト""#) // ポチョムキン
        .replace(
            r#""Heavenly Potemkin Buster""#,
            r#""ヘブンリーポチョムキンバスター""#,
        ) // ポチョムキン
        .replace(r#""Giganter Kai""#, r#""ガイガンダー改""#) // ポチョムキン
        .replace(r#""Giganter Kai Barrier""#, r#""ガイガンダー改バリア""#) // ポチョムキン
        .replace(r#""Giganter Kai (Barrier)""#, r#""ガイガンダー改バリア""#) // ポチョムキン
        .replace(r#""[4]6H P""#, r#""[4]6HS P""#) // ポチョムキン
        .replace(r#""[4]6H""#, r#""[4]6HS""#) // ポチョムキン
        .replace(r#""Heat Tackle""#, r#""ヒートタックル""#) // ポチョムキン
        .replace(r#""Heat Tackle""#, r#""ヒートタックル""#) // ポチョムキン
        .replace(r#""Dandy Step K""#, r#""ダンディーステップK""#) // スレイヤー
        .replace(r#""Dandy Step P""#, r#""ダンディーステップP""#) // スレイヤー
        .replace(r#""Master's Hammer""#, r#""マスターズハンマー""#) // スレイヤー
        .replace(r#""Bump Ahead""#, r#""バンプアヘッド""#) // スレイヤー
        .replace(r#""Pilebunker""#, r#""パイルバンカー""#) // スレイヤー
        .replace(r#""It's Late""#, r#""イッツレイト""#) // スレイヤー
        .replace(r#""Last Horizon""#, r#""ラスト・ホライズン""#) // スレイヤー
        .replace(r#""Mappa Hunch K""#, r#""マッパハンチK""#) // スレイヤー
        .replace(r#""Mappa Hunch P""#, r#""マッパハンチP""#) // スレイヤー
        .replace(r#""Hand of Doom""#, r#""Hand of Doom""#) // スレイヤー
        .replace(r#""Super Mappa Hunch""#, r#""スーパーマッパハンチ""#) // スレイヤー
        .replace(r#""Bloodsucking Universe""#, r#""血を吸う宇宙""#); // スレイヤー

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
            "2D",
            "2HS",
            "2K",
            "2P",
            "2S",
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
            "H",
            "jD",
            "jHS",
            "jK",
            "jP",
            "jS",
            "j2K",
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
