use crate::{MoveInfo, CHARS};
use regex::Regex;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;

#[derive(Deserialize, Debug)]
struct Response {
    cargoquery: Vec<Data>,
}

#[derive(Deserialize, Debug)]
struct Data {
    title: Title,
}

#[derive(Deserialize, Debug)]
struct Title {
    input: Option<String>,
    name: Option<String>,
    damage: Option<String>,
    guard: Option<String>,
    invuln: Option<String>,
    startup: Option<String>,
    active: Option<String>,
    recovery: Option<String>,
    #[serde(rename = "onHit")]
    hit: Option<String>,
    #[serde(rename = "onBlock")]
    block: Option<String>,
    level: Option<String>,
    #[serde(rename = "riscGain")]
    riscgain: Option<String>,
    prorate: Option<String>,
    counter: Option<String>,
}

pub async fn frames_to_json(mut chara_response_json: String, mut file: &File, char_count: usize) {
    let mut re = Regex::new(r#""c.S""#).unwrap();
    chara_response_json = re.replace_all(&chara_response_json, r#""近S""#).to_string();

    re = Regex::new(r#""f.S""#).unwrap();
    chara_response_json = re.replace_all(&chara_response_json, r#""遠S""#).to_string();

    re = Regex::new(r#""j\.(.+?)""#).unwrap();
    chara_response_json = re.replace_all(&chara_response_json, r#""j$1""#).to_string();

    chara_response_json = chara_response_json.replace(r#"&lt;br&gt;"#, ", ");
    chara_response_json = chara_response_json.replace(r#"&lt;br/&gt;"#, ", ");
    // Ino low profile
    chara_response_json = chara_response_json.replace(r#" &lt;span class=&quot;tooltip&quot; &gt;Low Profile&lt;span class=&quot;tooltiptext&quot; style=&quot;&quot;&gt;When a character's hurtbox is entirely beneath an opponent's attack. This can be caused by crouching, certain moves, and being short.&lt;/span&gt;&lt;/span&gt;"#, "");

    chara_response_json = chara_response_json
        .replace(r#""All""#, r#""上段""#)
        .replace(r#""All (Guard Crush)""#, r#""上段(ガードクラッシュ)""#)
        .replace(r#""High""#, r#""中段""#)
        .replace(r#""Low""#, r#""下段""#)
        .replace(r#""HKD "#, r#""強制ダウン"#)
        .replace(r#""KD "#, r#""ダウン"#);

    // ミリア
    chara_response_json = chara_response_json
        .replace(r#""w."#, r#""w"#)
        .replace(r#""w6H"#, r#""w6HS"#)
        .replace(r#""wH"#, r#""wHS"#)
        .replace(r#""j.4D""#, r#""j4D""#)
        .replace(r#""Sweep""#, r#""足払い""#)
        .replace(r#""Uncharged""#, r#""ダスト""#)
        .replace(r#""Dust Attack""#, r#""ダスト""#)
        .replace(r#""Charged Dust Attack""#, r#""溜めダスト""#)
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
        .replace(r#""j632146H""#, r#""j632146HS""#)
        .replace(r#""236236H""#, r#""236236HS""#)
        .replace(r#""Wild Assault""#, r#""ワイルドアサルト""#)
        .replace(r#""Wild Assault (Hold)""#, r#""溜めワイルドアサルト""#)
        .replace(r#""Charged Wild Assault""#, r#""溜めワイルドアサルト""#)
        .replace(r#""Ground Throw""#, r#""投げ""#)
        .replace(r#""Air Throw""#, r#""空投げ""#)
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

    re = Regex::new(r#""j\.(.+?)""#).unwrap();
    chara_response_json = re.replace_all(&chara_response_json, r#""j$1""#).to_string();

    let mut moves_info: Response = serde_json::from_str(&chara_response_json).unwrap();

    for x in 0..moves_info.cargoquery.len() {
        // Replacing None values with a generic '-'
        if moves_info.cargoquery[x].title.input.is_none() {
            moves_info.cargoquery[x].title.input = Some("-".to_string());
        } else {
            // Skips finish blow for sol
            if *moves_info.cargoquery[x].title.input.as_ref().unwrap() == "j.XX during Homing Jump"
            {
                continue;
            }
        }
        if moves_info.cargoquery[x].title.name.is_none() {
            moves_info.cargoquery[x].title.name = Some(
                moves_info.cargoquery[x]
                    .title
                    .input
                    .as_ref()
                    .unwrap()
                    .to_string(),
            );
        } else {
            // Skips dash cancel entry, ino hoverdash chipp escape zato flight and finish blow
            if *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Dash Cancel"
                || *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Hoverdash"
                || *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Finish Blow"
                || *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Flight"
                || *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Escape"
            {
                continue;
            }
        }
        if moves_info.cargoquery[x].title.damage.is_none() {
            moves_info.cargoquery[x].title.damage = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.guard.is_none() {
            moves_info.cargoquery[x].title.guard = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.invuln.is_none() {
            moves_info.cargoquery[x].title.invuln = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.startup.is_none() {
            moves_info.cargoquery[x].title.startup = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.active.is_none() {
            moves_info.cargoquery[x].title.active = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.recovery.is_none() {
            moves_info.cargoquery[x].title.recovery = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.hit.is_none() {
            moves_info.cargoquery[x].title.hit = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.block.is_none() {
            moves_info.cargoquery[x].title.block = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.level.is_none() {
            moves_info.cargoquery[x].title.level = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.riscgain.is_none() {
            moves_info.cargoquery[x].title.riscgain = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.prorate.is_none() {
            moves_info.cargoquery[x].title.prorate = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.counter.is_none() {
            moves_info.cargoquery[x].title.counter = Some("-".to_string());
        }

        let input_str = moves_info.cargoquery[x]
            .title
            .input
            .as_ref()
            .unwrap()
            .to_string();
        let mut input_name = String::new();

        if [
            "2D", "2HS", "2K", "2P", "2S", "5D", "5HS", "5K", "5P", "5[D]", "6HS", "6K", "6P",
            "近S", "遠S", "S", "H", "jD", "jHS", "jK", "jP", "jS",
        ]
        .contains(&input_str.as_str())
        {
            input_name = input_str;
        } else {
            let name_str = moves_info.cargoquery[x]
                .title
                .name
                .as_ref()
                .unwrap()
                .to_string();
            input_name = format!("{}({})", name_str, input_str);
        }

        // Serializing frame data
        let processed_moves_info = serde_json::to_string(&MoveInfo {
            input: input_name.to_string(),
            name: moves_info.cargoquery[x]
                .title
                .name
                .as_ref()
                .unwrap()
                .to_string(),
            damage: moves_info.cargoquery[x]
                .title
                .damage
                .as_ref()
                .unwrap()
                .to_string(),
            guard: moves_info.cargoquery[x]
                .title
                .guard
                .as_ref()
                .unwrap()
                .to_string(),
            invincibility: moves_info.cargoquery[x]
                .title
                .invuln
                .as_ref()
                .unwrap()
                .to_string(),
            startup: moves_info.cargoquery[x]
                .title
                .startup
                .as_ref()
                .unwrap()
                .to_string(),
            active: moves_info.cargoquery[x]
                .title
                .active
                .as_ref()
                .unwrap()
                .to_string(),
            recovery: moves_info.cargoquery[x]
                .title
                .recovery
                .as_ref()
                .unwrap()
                .to_string(),
            hit: moves_info.cargoquery[x]
                .title
                .hit
                .as_ref()
                .unwrap()
                .to_string(),
            block: moves_info.cargoquery[x]
                .title
                .block
                .as_ref()
                .unwrap()
                .to_string(),
            level: moves_info.cargoquery[x]
                .title
                .level
                .as_ref()
                .unwrap()
                .to_string(),
            riscgain: moves_info.cargoquery[x]
                .title
                .riscgain
                .as_ref()
                .unwrap()
                .to_string(),
            scaling: moves_info.cargoquery[x]
                .title
                .prorate
                .as_ref()
                .unwrap()
                .to_string(),
            counter: moves_info.cargoquery[x]
                .title
                .counter
                .as_ref()
                .unwrap()
                .to_string(),
        })
        .unwrap();

        write!(file, "{}", processed_moves_info)
            .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count] + ".json'."));

        // Skip writting comma/tab if next and last iteration
        // contains 'finish blow' in last the input field
        if x == moves_info.cargoquery.len() - 2
            && *moves_info.cargoquery[x + 1].title.input.as_ref().unwrap()
                == "j.XX during Homing Jump"
        {
            continue;
        } else if x != moves_info.cargoquery.len() - 1 {
            // Adding comma/tab
            // file.write(b",\n\t")
            //     .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS[char_count]+ ".json'."));
            (&mut file).write_all(b",\n\t").expect(
                &("\nFailed to write ',\\n\\t' while serializing '".to_owned()
                    + CHARS[char_count]
                    + ".json'."),
            );
        }
    }
}
