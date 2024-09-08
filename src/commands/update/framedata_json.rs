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
        .replace(r#""63214H""#, r#""63214HS""#)
        .replace(r#""j632146H""#, r#""j632146HS""#)
        .replace(r#""236236H""#, r#""236236HS""#)
        .replace(r#""j236236H""#, r#""j236236HS""#)
        .replace(r#""Wild Assault""#, r#""ワイルドアサルト""#)
        .replace(r#""Wild Assault (Hold)""#, r#""溜めワイルドアサルト""#)
        .replace(r#""Charged Wild Assault""#, r#""溜めワイルドアサルト""#)
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
        // チップ
        .replace(r#""Loop the Loop""#, r#""ループザループ""#) // ブリジット
        // チップ
        .replace(r#""j6236S""#, r#""j623S""#) // チップ
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
        // ファウスト
        .replace(r#""Thrust""#, r#""突きます。""#) // ファウスト
        .replace(r#""Thrust (Charged)""#, r#""溜め突きます。""#) // ファウスト
        .replace(r#""Pull Back""#, r#""引き戻し""#) // ファウスト
        .replace(r#""Home Run!""#, r#""ナイスショット""#) // ファウスト
        .replace(r#""Hole in One!""#, r#""ナイスショット""#) // ファウスト
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
        // ジオヴァーナ
        .replace(r#""6236S""#, r#""623S""#)
        .replace(r#""6HH""#, r#""6HSHS""#)
        .replace(r#""6HHH""#, r#""6HSHSHS""#)
        .replace(r#""Chave""#, r#""シャーヴィ""#)
        .replace(r#""Sepultura""#, r#""セパルトゥラ""#)
        .replace(r#""Sol Poente""#, r#""ソウ・ポエンチ""#)
        .replace(r#""Trovao""#, r#""トロヴァォン""#)
        .replace(r#""Sol Nascente""#, r#""ソウ・ナセンテ""#)
        .replace(r#""Air Sol Poente""#, r#""空中ソウ・ナセンテ""#)
        .replace(r#""Ventania""#, r#""ヴェンターニア""#)
        .replace(r#""Tempestade""#, r#""テンペスターヂ""#)
        // ゴールドルイス・ディキンソン
        .replace(r#""Thunderbird (Level 1)""#, r#""サンダーバード""#)
        .replace(r#""Thunderbird (Level 2)""#, r#""サンダーバード2""#)
        .replace(r#""Thunderbird (Level 3)""#, r#""サンダーバード3""#)
        .replace(r#""Skyfish (Level 1)""#, r#""スカイフィッシュ""#)
        .replace(r#""Skyfish (Level 2)""#, r#""スカイフィッシュ2""#)
        .replace(r#""Skyfish (Level 3)""#, r#""スカイフィッシュ3""#)
        .replace(r#""Behemoth Typhoon (248)""#, r#""ベヒーモスタイフーン""#)
        .replace(r#""Behemoth Typhoon (268)""#, r#""ベヒーモスタイフーン""#)
        .replace(r#""Behemoth Typhoon (426)""#, r#""ベヒーモスタイフーン""#)
        .replace(r#""Behemoth Typhoon (486)""#, r#""ベヒーモスタイフーン""#)
        .replace(r#""Behemoth Typhoon (624)""#, r#""ベヒーモスタイフーン""#)
        .replace(r#""Behemoth Typhoon (684)""#, r#""ベヒーモスタイフーン""#)
        .replace(r#""Behemoth Typhoon (842)""#, r#""ベヒーモスタイフーン""#)
        .replace(r#""Behemoth Typhoon (862)""#, r#""ベヒーモスタイフーン""#)
        .replace(
            r#""Air Behemoth Typhoon (248)""#,
            r#""空中ベヒーモスタイフーン""#,
        )
        .replace(
            r#""Air Behemoth Typhoon (268)""#,
            r#""空中ベヒーモスタイフーン""#,
        )
        .replace(
            r#""Air Behemoth Typhoon (426)""#,
            r#""空中ベヒーモスタイフーン""#,
        )
        .replace(
            r#""Air Behemoth Typhoon (486)""#,
            r#""空中ベヒーモスタイフーン""#,
        )
        .replace(
            r#""Air Behemoth Typhoon (624)""#,
            r#""空中ベヒーモスタイフーン""#,
        )
        .replace(
            r#""Air Behemoth Typhoon (684)""#,
            r#""空中ベヒーモスタイフーン""#,
        )
        .replace(
            r#""Air Behemoth Typhoon (842)""#,
            r#""空中ベヒーモスタイフーン""#,
        )
        .replace(
            r#""Air Behemoth Typhoon (862)""#,
            r#""空中ベヒーモスタイフーン""#,
        )
        .replace(r#""Burn It Down (Level 1)""#, r#""バーン・イット・ダウン""#)
        .replace(
            r#""Burn It Down (Level 2)""#,
            r#""バーン・イット・ダウン2""#,
        )
        .replace(
            r#""Burn It Down (Level 3)""#,
            r#""バーン・イット・ダウン3""#,
        )
        .replace(
            r#""Down With The System""#,
            r#""ダウン・ウィズ・ザ・システム""#,
        )
        .replace(
            r#""Down With The System (720)""#,
            r#""ダウン・ウィズ・ザ・システム""#,
        )
        .replace(
            r#""Down With The System (1080)""#,
            r#""ダウン・ウィズ・ザ・システム""#,
        )
        .replace(r#""214S Level 1""#, r#""214S""#)
        .replace(r#""214S Level 2""#, r#""214S2""#)
        .replace(r#""214S Level 3""#, r#""214S3""#)
        .replace(r#""236S Level 1""#, r#""236S""#)
        .replace(r#""236S Level 2""#, r#""236S2""#)
        .replace(r#""236S Level 3""#, r#""236S3""#)
        .replace(r#""236236K Level 1""#, r#""236236K""#)
        .replace(r#""236236K Level 2""#, r#""236236K2""#)
        .replace(r#""236236K Level 3""#, r#""236236K3""#)
        .replace(r#""21478H""#, r#""21478HS""#)
        .replace(r#""23698H""#, r#""23698HS""#)
        .replace(r#""47896H""#, r#""47896HS""#)
        .replace(r#""69874H""#, r#""69874HS""#)
        .replace(r#""87412H""#, r#""87412HS""#)
        .replace(r#""89632H""#, r#""89632HS""#)
        .replace(r#""j21478H""#, r#""j21478HS""#)
        .replace(r#""j23698H""#, r#""j23698HS""#)
        .replace(r#""j41236H""#, r#""j41236HS""#)
        .replace(r#""j47896H""#, r#""j47896HS""#)
        .replace(r#""j63214H""#, r#""j63214HS""#)
        .replace(r#""j69874H""#, r#""j69874HS""#)
        .replace(r#""j87412H""#, r#""j87412HS""#)
        .replace(r#""j89632H""#, r#""j89632HS""#)
        // ハッピー・ケイオス
        .replace(r#""214S 214S""#, r#""214S214S""#)
        .replace(r#""236S 2H""#, r#""236S2HS""#)
        .replace(r#""236S H""#, r#""236SHS""#)
        .replace(r#""H""#, r#""銃を構える(HS)""#)
        .replace(r#""Roll""#, r#""前転""#)
        .replace(r#""Focus""#, r#""フォーカス""#)
        .replace(r#""Steady Aim""#, r#""しっかり狙いを定める""#)
        .replace(r#""Cancel Aim""#, r#""構え解除""#)
        .replace(r#""Fire""#, r#""射撃""#)
        .replace(r#""Reload""#, r#""リロード""#)
        .replace(r#""Scapegoat""#, r#""スケープゴート""#)
        .replace(r#""Curse""#, r#""カース""#)
        .replace(r#""At the Ready""#, r#""銃を構える""#)
        .replace(r#""Super Focus""#, r#""超フォーカス""#)
        .replace(r#""Deus Ex Machina""#, r#""デウス・エクス・マキナ""#)
        // ジョニー
        .replace(r#""236HH""#, r#""236HSHS""#)
        .replace(r#""Ensenga""#, r#""燕穿牙""#)
        .replace(r#""Mist Finer Stance""#, r#""ミストファイナー構え""#)
        .replace(
            r#""Mist Finer (Horizontal)""#,
            r#""ミストファイナー（横）""#,
        )
        .replace(r#""Mist Finer (Upward)""#, r#""ミストファイナー（上）""#)
        .replace(r#""Mist Finer (Downward)""#, r#""ミストファイナー（下）""#)
        .replace(r#""Vault""#, r#""跳躍""#)
        .replace(r#""Vault Deal""#, r#""跳躍ディール""#)
        .replace(r#""Deal""#, r#""ディール""#)
        .replace(r#""Turn Up""#, r#""ミストファイナー（カードヒット時）""#)
        .replace(r#""Mist Finer Cancel""#, r#""ミストファイナーキャンセル""#)
        .replace(
            r#""Air Mist Finer (Horizontal)""#,
            r#""空中ミストファイナー（横）""#,
        )
        .replace(
            r#""Air Mist Finer (Upward)""#,
            r#""空中ミストファイナー（上）""#,
        )
        .replace(
            r#""Air Mist Finer (Downward)""#,
            r#""空中ミストファイナー（下）""#,
        )
        .replace(r#""Air Deal""#, r#""空中ディール""#)
        .replace(
            r#""Mist Finer Dash (Backward)""#,
            r#""ミストファイナーバックステップ""#,
        )
        .replace(
            r#""Mist Finer Dash (Forward)""#,
            r#""ミストファイナー前ステップ""#,
        )
        .replace(r#""Joker Trick""#, r#""ジョーカートリック""#)
        .replace(r#""That&#039;s My Name""#, r#""それが俺の名だ""#)
        // ミリア
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
        println!("{}", &input_str.as_str());

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
            "6S",
            "H",
            "jD",
            "jHS",
            "jK",
            "jP",
            "jS",
            "j2K",
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
