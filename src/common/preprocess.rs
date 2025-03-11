#![allow(clippy::needless_raw_string_hashes)]

//! `preprocess.rs`
//!
//! JSON置換処理モジュール。  
//! このモジュールは、入力されたJSON文字列に対して、HTMLエンティティや特定の英語表記を日本語表記へ変換する共通の置換処理を行う。

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

// 正規表現パターンを事前にコンパイルして再利用
lazy_static! {
    // 基本的な正規表現パターン
    static ref RE_C_S: Regex = Regex::new(r#""c.S""#).unwrap();
    static ref RE_F_S: Regex = Regex::new(r#""f.S""#).unwrap();
    static ref RE_J_MOVE: Regex = Regex::new(r#""j\.(.+?)""#).unwrap();
    static ref RE_HKD_MOVE: Regex = Regex::new(r#"HKD "#).unwrap();
    static ref RE_KD_MOVE: Regex = Regex::new(r#"KD "#).unwrap();

    // 置換マップ（通常の文字列置換用）
    static ref REPLACEMENT_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();

        // HTMLエンティティ
        // map.insert(r#"&lt;br&gt;"#, ", ");
        // map.insert(r#"&lt;br/&gt;"#, ", ");
        map.insert(r#""All""#, r#""上段""#);
        map.insert(r#""All (Guard Crush)""#, r#""上段(ガードクラッシュ)""#);
        map.insert(r#""High""#, r#""中段""#);
        map.insert(r#""Low""#, r#""下段""#);
        map.insert(r#""w.""#, r#""w""#);
        map.insert(r#""w6H""#, r#""w6HS""#);
        map.insert(r#""wH""#, r#""wHS""#);
        // map.insert(r#"j.4D"#, r#"j4D"#);
        map.insert(r#"j.6D"#, r#"j6D"#);
        map.insert(r#""Sweep""#, r#""足払い""#);
        map.insert(r#""Uncharged""#, r#""ダスト""#);
        map.insert(r#""Dust Attack""#, r#""ダスト""#);
        map.insert(r#""Charged Dust Attack""#, r#""溜めダスト""#);
        map.insert(r#""2H""#, r#""2HS""#);
        map.insert(r#""5H""#, r#""5HS""#);
        map.insert(r#""6H""#, r#""6HS""#);
        map.insert(r#""jH""#, r#""jHS""#);
        map.insert(r#""236H""#, r#""236HS""#);
        map.insert(r#""j236H""#, r#""j236HS""#);
        map.insert(r#""623H""#, r#""623HS""#);
        map.insert(r#""214H""#, r#""214HS""#);
        map.insert(r#""41236H""#, r#""41236HS""#);
        map.insert(r#""632146H""#, r#""632146HS""#);
        map.insert(r#""63214H""#, r#""63214HS""#);
        map.insert(r#""j632146H""#, r#""j632146HS""#);
        map.insert(r#""236236H""#, r#""236236HS""#);
        map.insert(r#""j236236H""#, r#""j236236HS""#);
        map.insert(r#""214214H""#, r#""214214HS""#);
        map.insert(r#""j623H""#, r#""j623HS""#);
        map.insert(r#""Wild Assault""#, r#""ワイルドアサルト""#);
        map.insert(r#""Charged Wild Assault""#, r#""溜めワイルドアサルト""#);
        map.insert(r#""Wild Assault (Hold)""#, r#""溜めワイルドアサルト""#);
        map.insert(r#""Ground Throw""#, r#""投げ""#);
        map.insert(r#""Air Throw""#, r#""空投げ""#);
        // A.B.A部分の共通置換
        map.insert(r#""JR 2H""#, r#""JR2HS""#);
        map.insert(r#""JR 2K""#, r#""JR2K""#);
        map.insert(r#""JR 2P""#, r#""JR2P""#);
        map.insert(r#""JR 2S""#, r#""JR2S""#);
        map.insert(r#""JR 5H""#, r#""JR5HS""#);
        map.insert(r#""JR 5K""#, r#""JR5K""#);
        map.insert(r#""JR 5P""#, r#""JR5P""#);
        map.insert(r#""JR 6H""#, r#""JR6HS""#);
        map.insert(r#""JR 6P""#, r#""JR6P""#);
        map.insert(r#""JR c.S""#, r#""JR近S""#);
        map.insert(r#""JR f.S""#, r#""JR遠S""#);
        map.insert(r#""JR j.D""#, r#""JRjD""#);
        map.insert(r#""JR j.H""#, r#""JRjHS""#);
        map.insert(r#""JR j.K""#, r#""JRjK""#);
        map.insert(r#""JR j.P""#, r#""JRjP""#);
        map.insert(r#""JR j.S""#, r#""JRjS""#);
        map.insert(r#""236S~6S""#, r#""236S6S""#);
        map.insert(r#""JR 214H""#, r#""JR214HS""#);
        map.insert(r#""JR 214K""#, r#""JR214K""#);
        map.insert(r#""JR 236K""#, r#""JR236K""#);
        map.insert(r#""JR 236S""#, r#""JR236S""#);
        map.insert(r#""JR 236S~6S""#, r#""JR236S6S""#);
        map.insert(r#""JR 63214P""#, r#""JR63214P""#);
        map.insert(r#""JR Deactivation""#, r#""JR解除""#);
        map.insert(r#""JR 632146H""#, r#""JR632146HS""#);
        map.insert(r#""JR 632146K""#, r#""JR632146K""#);
        map.insert(r#""Bonding and Dissolving""#, r#""結合と変性""#);
        map.insert(r#""Haul and Heed""#, r#""牽引と随順""#);
        map.insert(r#""Frenzy and Astonishment""#, r#""逆上と驚愕""#);
        map.insert(r#""Intertwine and Tilt""#, r#""戮力と傾動""#);
        map.insert(r#""Menace and Groan""#, r#""威喝と嗚咽""#);
        map.insert(r#""Restriction and Constraint""#, r#""抑圧と束縛""#);
        map.insert(r#""Judgment and Sentiment""#, r#""断罪と情動""#);
        map.insert(r#""Changing and Swaying""#, r#""変転と感化""#);
        map.insert(r#""JR Bonding and Dissolving""#, r#""JR結合と変性""#);
        map.insert(r#""JR Haul and Heed""#, r#""JR牽引と随順""#);
        map.insert(r#""JR Intertwine and Tilt""#, r#""JR戮力と傾動""#);
        map.insert(r#""JR Menace and Groan""#, r#""JR威喝と嗚咽""#);
        map.insert(r#""JR Restriction and Constraint""#, r#""JR抑圧と束縛""#);
        map.insert(r#""JR Changing and Swaying""#, r#""JR変転と感化""#);
        map.insert(r#""Jealous Rage Deactivation""#, r#""JR解除""#);
        map.insert(r#""The Law is Key, Key is King.""#, r#""鍵の支配""#);
        map.insert(r#""Keeper of the Key""#, r#""鍵の守護者""#);
        map.insert(r#""JR The Law is Key, Key is King.""#, r#""JR鍵の支配""#);
        map.insert(r#""JR Keeper of the Key""#, r#""JR鍵の守護者""#);

        // 闇慈
        map.insert(r#""Shitsu""#, r#""疾""#);
        map.insert(r#""Suigetsu No Hakobi""#, r#""水月のハコビ""#);
        map.insert(r#""Kou""#, r#""紅""#);
        map.insert(r#""Fuujin""#, r#""風神""#);
        map.insert(r#""Shin: Ichishiki""#, r#""針・壱式""#);
        map.insert(r#""Issokutobi""#, r#""一足飛び""#);
        map.insert(r#""Nagiha""#, r#""凪刃""#);
        map.insert(r#""Rin""#, r#""臨""#);
        map.insert(r#""Midare""#, r#""乱""#);
        map.insert(r#""Issei Ougi: Sai""#, r#""一誠奥義「彩」""#);
        map.insert(r#""Kachoufuugetsu Kai""#, r#""花鳥風月改""#);
        map.insert(r#""Near Kachoufuugetsu Kai""#, r#""花鳥風月近""#);
        map.insert(r#""Far Kachoufuugetsu Kai""#, r#""花鳥風月遠""#);
        map.insert(r#""Draw""#, r#""ブックマーク(ドロー)""#);
        map.insert(r#""Discard""#, r#""ブックマーク(破棄)""#);

        // アクセル
        map.insert(r#""Snail""#, r#""蝸牛""#);
        map.insert(r#""Whistling Wind""#, r#""虎落笛""#);
        map.insert(r#""Rainwater""#, r#""潦""#);
        map.insert(r#""Whistling Wind (Charged)""#, r#""溜め虎落笛""#);
        map.insert(r#""Winter Mantis""#, r#""冬蟷螂""#);
        map.insert(r#""Air Snail""#, r#""空中蝸牛""#);
        map.insert(r#""Axl Bomber""#, r#""アクセルボンバー""#);
        map.insert(r#""Sickle Flash""#, r#""鎌閃撃""#);
        map.insert(r#""Spinning Chain Strike""#, r#""旋鎖撃""#);
        map.insert(r#""Soaring Chain Strike""#, r#""曲鎖撃""#);
        map.insert(r#""Winter Cherry""#, r#""鬼灯""#);
        map.insert(r#""Sickle Storm""#, r#""百重鎌焼""#);
        map.insert(r#""One Vision""#, r#""ワンヴィジョン""#);

        // 梅喧
        // map.insert(r#"23S"#, r#"236S"#);
        map.insert(r#""Ground Throw (Tether)""#, r#""投げ""#);
        map.insert(r#""Ground Throw (Knockback)""#, r#""溜め投げ""#);
        map.insert(r#""41236HS~HS""#, r#""41236HSHS""#);
        map.insert(r#""Tatami Gaeshi""#, r#""畳返し""#);
        map.insert(r#""Air Tatami Gaeshi""#, r#""空中畳返し""#);
        map.insert(r#""H Kabari""#, r#""HS蚊鉤""#);
        map.insert(r#""S Kabari""#, r#""S蚊鉤""#);
        map.insert(r#""Kabari""#, r#""蚊鉤""#);
        map.insert(r#""41236HH""#, r#""HS蚊鉤追撃""#);
        map.insert(r#""41236H~H""#, r#""41236HSHS""#);
        map.insert(r#""Kabari Followup""#, r#""HS蚊鉤追撃""#);
        map.insert(r#""Youzansen""#, r#""妖斬扇""#);
        map.insert(r#""Hiiragi""#, r#""柊""#);
        map.insert(r#""Tsurane Sanzu-watashi""#, r#""連ね三途渡し""#);
        map.insert(r#""Kenjyu""#, r#""拳銃""#);
        map.insert(r#"Regular Throw"#, r#"溜め投げ"#);
        map.insert(r#"Air Kenjyu"#, r#"空中拳銃"#);
        map.insert(r#""Rolling Movement""#, r#""ローリング移動""#);
        map.insert(r#""Stop and Dash""#, r#""ストップアンドダッシュ""#);
        map.insert(r#""Kick Start My Heart""#, r#""キックスタートマイハート""#);
        map.insert(r#""Shoot""#, r#""発射""#);
        map.insert(r#""Brake""#, r#""停止""#);
        map.insert(r#""Starship""#, r#""スターシップ""#);
        map.insert(r#""Roger Dive""#, r#""ロジャーダイブ""#);
        map.insert(r#""Rock the Baby""#, r#""ロックザベイビー""#);
        map.insert(r#""Air Rock the Baby""#, r#""空中ロックザベイビー""#);
        map.insert(r#""Return of the Killing Machine""#, r#""帰ってきたキルマシーン""#);
        map.insert(r#""214S/H""#, r#""214S/HS""#);
        map.insert(r#""Loop the Loop""#, r#""ループザループ""#);

        // チップ
        map.insert(r#""j6236S""#, r#""j623S""#);
        map.insert(r#""Wall Run""#, r#""壁走り""#);
        map.insert(r#""Wall Run ""#, r#""壁走り""#);
        map.insert(r#""壁走りH""#, r#""壁走りHS""#);
        map.insert(r#""壁走り6H""#, r#""壁走り6HS""#);
        // map.insert(r#""input":"214[H]","name":"Tightrope""#, r#""input":"214[H]","name":"綱渡り""#);
        map.insert(r#""Tightrope""#, r#""綱張り""#);
        map.insert(r#""214[H]""#, r#""214[HS]""#);
        map.insert(r#""Gamma Blade""#, r#""γブレード""#);
        map.insert(r#""Alpha Blade (Diagonal)""#, r#""αブレード・斜め""#);
        map.insert(r#""Alpha Blade (Horizontal)""#, r#""αブレード・横""#);
        map.insert(r#""Resshou""#, r#""冽掌""#);
        map.insert(r#""Rokusai""#, r#""麓砕""#);
        map.insert(r#""Senshuu""#, r#""穿踵""#);
        map.insert(r#""Beta Blade""#, r#""βブレード""#);
        map.insert(r#""Genrouzan""#, r#""幻朧斬""#);
        map.insert(r#""Shuriken""#, r#""手裏剣""#);
        map.insert(r#""Air Alpha Blade (Diagonal)""#, r#""空中αブレード・斜め""#);
        map.insert(r#""Air Alpha Blade (Horizontal)""#, r#""空中αブレード・横""#);
        map.insert(r#""Air Beta Blade""#, r#""空中βブレード""#);
        map.insert(r#""Banki Messai""#, r#""万鬼滅砕""#);
        map.insert(r#""Zansei Rouga""#, r#""斬星狼牙""#);
        map.insert(r#""Air Zansei Rouga""#, r#""空中斬星狼牙""#);

        // エルフェルト
        map.insert(r#""214S~H""#, r#""214SHS""#);
        map.insert(r#""214S~K""#, r#""214SK""#);
        map.insert(r#""214S~P""#, r#""214SP""#);
        map.insert(r#""214S~P/K~K""#, r#""214SP/KK""#);
        map.insert(r#""214S~P/K~P""#, r#""214SP/KP""#);
        map.insert(r#""236S/H""#, r#""236S/HS""#);
        map.insert(r#""j236S/H""#, r#""j236S/HS""#);
        map.insert(r#""236236K Explosion""#, r#""236236K爆発""#);
        map.insert(r#""Bomb-Bomb Chocolat""#, r#""ボンボン・ショコラ""#);
        map.insert(r#""Miss Charlotte (Out of Repair)""#, r#""Missシャルロット（お手入れ不足）""#);
        map.insert(r#""Here I Go!""#, r#""やります！""#);
        map.insert(r#""Nailed It!""#, r#""決めます！""#);
        map.insert(r#""Down Low!""#, r#""下から！""#);
        map.insert(r#""Up High!""#, r#""上から！""#);
        map.insert(r#""Down Low! (Finisher)""#, r#""下から！（フィニッシュ）""#);
        map.insert(r#""Up High! (Finisher)""#, r#""上から！（フィニッシュ）""#);
        map.insert(r#""Miss Charlotte""#, r#""Missシャルロット""#);
        map.insert(r#""Air Miss Charlotte""#, r#""空中Missシャルロット""#);
        map.insert(r#""Bomb-Bombnnière""#, r#""ボンボニエール""#);
        map.insert(r#""Bomb-Bombnnière Explosion""#, r#""ボンボニエール爆発""#);
        map.insert(r#""Juganto Da Parfeo""#, r#""ジュガント ダ パルフェーオ""#);

        // ファウスト
        map.insert(r#""Thrust""#, r#""突きます。""#);
        map.insert(r#""Thrust (Charged)""#, r#""溜め突きます。""#);
        map.insert(r#""Pull Back""#, r#""引き戻し""#);
        map.insert(r#""Home Run!""#, r#""ナイスショット""#);
        map.insert(r#""Hole In One""#, r#""ナイスショット""#);
        map.insert(r#""Hole in One!""#, r#""ナイスショット""#);
        map.insert(r#""What Could This Be? (Eat)""#, r#""何が出るかな？（食べる）""#);
        map.insert(r#""What Could This Be? (Spit)""#, r#""何が出るかな？（射出）""#);
        map.insert(r#""What Could This Be?""#, r#""何が出るかな？""#);
        map.insert(r#""Mix Mix Mix""#, r#""涅和混練""#);
        map.insert(r#""Air Mix Mix Mix""#, r#""空中涅和混練""#);
        map.insert(r#""Snip Snip Snip""#, r#""メッタ刈り""#);
        map.insert(r#""Love""#, r#""愛""#);
        map.insert(r#""Scarecrow""#, r#""P久延毘古""#);
        map.insert(r#""P Scarecrow""#, r#""P久延毘古""#);
        map.insert(r#""S Scarecrow""#, r#""S久延毘古""#);
        map.insert(r#""K Scarecrow""#, r#""K久延毘古""#);
        map.insert(r#""Bone-crushing Excitement""#, r#""エキサイティング骨折""#);
        map.insert(r#""W-W-What Could This Be?""#, r#""な・な・な・なにがでるかな？""#);
        map.insert(r#""W-W-W-W-W-W-W-W-W-What Could This Be?""#, r#""な・な・な・な・な・な・な・な・な・なにがでるかな？""#);
        map.insert(r#""Bomb""#, r#""爆弾""#);
        map.insert(r#""Banana""#, r#""バナナ""#);
        map.insert(r#""Donut""#, r#""ドーナツ""#);
        map.insert(r#""Afro""#, r#""アフロ""#);
        map.insert(r#""Hammer""#, r#""ハンマー""#);
        map.insert(r#""Mini Faust""#, r#""ちびファウスト""#);
        map.insert(r#""Horn""#, r#""ラッパ""#);
        map.insert(r#""100T Weight""#, r#""100t重り""#);
        map.insert(r#""Earthquake""#, r#""100t重り""#);
        map.insert(r#""Meteors""#, r#""メテオ""#);
        map.insert(r#""Love Afro""#, r#""愛アフロ""#);

        // ジオヴァーナ
        map.insert(r#""6236S""#, r#""623S""#);
        map.insert(r#""6HH""#, r#""6HSHS""#);
        map.insert(r#""6HHH""#, r#""6HSHSHS""#);
        map.insert(r#""Chave""#, r#""シャーヴィ""#);
        map.insert(r#""Sepultura""#, r#""セパルトゥラ""#);
        map.insert(r#""Sol Poente""#, r#""ソウ・ポエンチ""#);
        map.insert(r#""Trovao""#, r#""トロヴァォン""#);
        map.insert(r#""Trovão""#, r#""トロヴァォン""#);
        map.insert(r#""Enhanced Trovão""#, r#""シャーヴィトロヴァォン""#);
        map.insert(r#""Sol Nascente""#, r#""ソウ・ナセンテ""#);
        map.insert(r#""Air Sol Poente""#, r#""空中ソウ・ナセンテ""#);
        map.insert(r#""Ventania""#, r#""ヴェンターニア""#);
        map.insert(r#""Tempestade""#, r#""テンペスターヂ""#);
        map.insert(r#""214H~6K""#, r#""214HS6K""#);

        // ゴールドルイス・ディキンソン
        map.insert(r#""Thunderbird (Level 1)""#, r#""サンダーバード""#);
        map.insert(r#""Thunderbird (Level 2)""#, r#""サンダーバード2""#);
        map.insert(r#""Thunderbird (Level 3)""#, r#""サンダーバード3""#);
        map.insert(r#""Skyfish (Level 1)""#, r#""スカイフィッシュ""#);
        map.insert(r#""Skyfish (Level 2)""#, r#""スカイフィッシュ2""#);
        map.insert(r#""Skyfish (Level 3)""#, r#""スカイフィッシュ3""#);
        map.insert(r#""Behemoth Typhoon (248)""#, r#""ベヒーモスタイフーン""#);
        map.insert(r#""Behemoth Typhoon (268)""#, r#""ベヒーモスタイフーン""#);
        map.insert(r#""Behemoth Typhoon (426)""#, r#""ベヒーモスタイフーン""#);
        map.insert(r#""Behemoth Typhoon (486)""#, r#""ベヒーモスタイフーン""#);
        map.insert(r#""Behemoth Typhoon (624)""#, r#""ベヒーモスタイフーン""#);
        map.insert(r#""Behemoth Typhoon (684)""#, r#""ベヒーモスタイフーン""#);
        map.insert(r#""Behemoth Typhoon (842)""#, r#""ベヒーモスタイフーン""#);
        map.insert(r#""Behemoth Typhoon (862)""#, r#""ベヒーモスタイフーン""#);
        map.insert(r#""Air Behemoth Typhoon (248)""#, r#""空中ベヒーモスタイフーン""#);
        map.insert(r#""Air Behemoth Typhoon (268)""#, r#""空中ベヒーモスタイフーン""#);
        map.insert(r#""Air Behemoth Typhoon (426)""#, r#""空中ベヒーモスタイフーン""#);
        map.insert(r#""Air Behemoth Typhoon (486)""#, r#""空中ベヒーモスタイフーン""#);
        map.insert(r#""Air Behemoth Typhoon (624)""#, r#""空中ベヒーモスタイフーン""#);
        map.insert(r#""Air Behemoth Typhoon (684)""#, r#""空中ベヒーモスタイフーン""#);
        map.insert(r#""Air Behemoth Typhoon (842)""#, r#""空中ベヒーモスタイフーン""#);
        map.insert(r#""Air Behemoth Typhoon (862)""#, r#""空中ベヒーモスタイフーン""#);
        map.insert(r#""Burn It Down (Level 1)""#, r#""バーン・イット・ダウン""#);
        map.insert(r#""Burn It Down (Level 2)""#, r#""バーン・イット・ダウン2""#);
        map.insert(r#""Burn It Down (Level 3)""#, r#""バーン・イット・ダウン3""#);
        map.insert(r#""Down With The System""#, r#""ダウン・ウィズ・ザ・システム""#);
        map.insert(r#""Down With The System (720)""#, r#""ダウン・ウィズ・ザ・システム""#);
        map.insert(r#""Down With The System (1080)""#, r#""ダウン・ウィズ・ザ・システム""#);
        map.insert(r#""214S Level 1""#, r#""214S""#);
        map.insert(r#""214S Level 2""#, r#""214S2""#);
        map.insert(r#""214S Level 3""#, r#""214S3""#);
        map.insert(r#""236S Level 1""#, r#""236S""#);
        map.insert(r#""236S Level 2""#, r#""236S2""#);
        map.insert(r#""236S Level 3""#, r#""236S3""#);
        map.insert(r#""236236K Level 1""#, r#""236236K""#);
        map.insert(r#""236236K Level 2""#, r#""236236K2""#);
        map.insert(r#""236236K Level 3""#, r#""236236K3""#);
        map.insert(r#""21478H""#, r#""21478HS""#);
        map.insert(r#""23698H""#, r#""23698HS""#);
        map.insert(r#""47896H""#, r#""47896HS""#);
        map.insert(r#""69874H""#, r#""69874HS""#);
        map.insert(r#""87412H""#, r#""87412HS""#);
        map.insert(r#""89632H""#, r#""89632HS""#);
        map.insert(r#""j21478H""#, r#""j21478HS""#);
        map.insert(r#""j23698H""#, r#""j23698HS""#);
        map.insert(r#""j41236H""#, r#""j41236HS""#);
        map.insert(r#""j47896H""#, r#""j47896HS""#);
        map.insert(r#""j63214H""#, r#""j63214HS""#);
        map.insert(r#""j69874H""#, r#""j69874HS""#);
        map.insert(r#""j87412H""#, r#""j87412HS""#);
        map.insert(r#""j89632H""#, r#""j89632HS""#);

        // ハッピー・ケイオス
        map.insert(r#""214S 214S""#, r#""214S214S""#);
        map.insert(r#""236S 2H""#, r#""236S2HS""#);
        map.insert(r#""236S H""#, r#""236SHS""#);
        map.insert(r#""H""#, r#""銃を構える(HS)""#);
        map.insert(r#""Roll""#, r#""前転""#);
        map.insert(r#""Focus""#, r#""フォーカス""#);
        map.insert(r#""Steady Aim""#, r#""しっかり狙いを定める""#);
        map.insert(r#""Cancel Aim""#, r#""構え解除""#);
        map.insert(r#""Fire""#, r#""射撃""#);
        map.insert(r#""Reload""#, r#""リロード""#);
        map.insert(r#""Scapegoat""#, r#""スケープゴート""#);
        map.insert(r#""Curse""#, r#""カース""#);
        map.insert(r#""At the Ready""#, r#""銃を構える""#);
        map.insert(r#""Super Focus""#, r#""超フォーカス""#);
        map.insert(r#""Deus Ex Machina""#, r#""デウス・エクス・マキナ""#);

        // イノ
        map.insert(r#""Chemical Love""#, r#""ケミカル愛情""#);
        map.insert(r#""Antidepressant Scale""#, r#""抗鬱音階""#);
        map.insert(r#""Mad Love Agitato""#, r#""狂愛アジタート""#);
        map.insert(r#""H Stroke the Big Tree""#, r#""HS大木をさする手""#);
        map.insert(r#""S Stroke the Big Tree""#, r#""S大木をさする手""#);
        map.insert(r#""Air Chemical Love""#, r#""空中ケミカル愛情""#);
        map.insert(r#""Air Antidepressant Scale""#, r#""空中抗鬱音階""#);
        map.insert(r#""H Sultry Performance""#, r#""HS狂言実行""#);
        map.insert(r#""H Leap""#, r#""H跳躍""#);
        map.insert(r#""K Sultry Performance""#, r#""K狂言実行""#);
        map.insert(r#""K Leap""#, r#""K跳躍""#);
        map.insert(r#""S Sultry Performance""#, r#""S狂言実行""#);
        map.insert(r#""S Leap""#, r#""S跳躍""#);
        map.insert(r#""H Sultry Performance (charged)""#, r#""溜めHS狂言実行""#);
        map.insert(r#""K Sultry Performance (charged)""#, r#""溜めK狂言実行""#);
        map.insert(r#""S Sultry Performance (charged)""#, r#""溜めS狂言実行""#);
        map.insert(r#""Megalomania""#, r#""メガロマニア""#);
        map.insert(r#""Ultimate Fortissimo""#, r#""限界フォルテッシモ""#);
        map.insert(r#""Air Ultimate Fortissimo""#, r#""空中限界フォルテッシモ""#);

        // ジャックオー
        map.insert(r#""Countdown""#, r#""カウントダウン""#);
        map.insert(r#""Attack Command""#, r#""攻撃指示""#);
        map.insert(r#""Recover Servant""#, r#""回収""#);
        map.insert(r#""Defend Command""#, r#""防御指示""#);
        map.insert(r#""Servant Shoot""#, r#""サーヴァントシュート""#);
        map.insert(r#""Summon Servant""#, r#""サーヴァント召喚""#);
        map.insert(r#""Summon Servant""#, r#""サーヴァント召喚""#);
        map.insert(r#""Pick Up Servant""#, r#""サーヴァントを持ち上げる""#);
        map.insert(r#""Throw Servant""#, r#""サーヴァントを投げる""#);
        map.insert(r#""Servant""#, r#""サーヴァント""#);
        map.insert(r#""Release Servant""#, r#""サーヴァントを放す""#);
        map.insert(r#""Air Servant Shoot""#, r#""空中サーヴァントシュート""#);
        map.insert(r#""Held Attack Command""#, r#""防御指示""#);
        map.insert(r#""Servant""#, r#""サーヴァント""#);
        map.insert(r#""Held Defend Command""#, r#""ディフェンスコマンド""#);
        map.insert(r#""Cheer Servant On (H)""#, r#""HSサーヴァントを激励する""#);
        map.insert(r#""Cheer Servant On (S)""#, r#""Sサーヴァントを激励すす""#);
        map.insert(r#""Forever Elysion Driver""#, r#""フォーエヴァーエリシオンドライバー""#);

        // ジョニー
        map.insert(r#""236HH""#, r#""236HSHS""#);
        map.insert(r#""Ensenga""#, r#""燕穿牙""#);
        map.insert(r#""Mist Finer Stance""#, r#""ミストファイナー構え""#);
        map.insert(r#""Mist Finer (Horizontal)""#, r#""ミストファイナー（横）""#);
        map.insert(r#""Mist Finer (Upward)""#, r#""ミストファイナー（上）""#);
        map.insert(r#""Mist Finer (Downward)""#, r#""ミストファイナー（下）""#);
        map.insert(r#""Vault""#, r#""跳躍""#);
        map.insert(r#""Vault Deal""#, r#""跳躍ディール""#);
        map.insert(r#""Deal""#, r#""ディール""#);
        map.insert(r#""Turn Up""#, r#""ミストファイナー（カードヒット時）""#);
        map.insert(r#""Mist Finer Cancel""#, r#""ミストファイナーキャンセル""#);
        map.insert(r#""Air Mist Finer (Horizontal)""#, r#""空中ミストファイナー（横）""#);
        map.insert(r#""Air Mist Finer (Upward)""#, r#""空中ミストファイナー（上）""#);
        map.insert(r#""Air Mist Finer (Downward)""#, r#""空中ミストファイナー（下）""#);
        map.insert(r#""Air Deal""#, r#""空中ディール""#);
        map.insert(r#""Mist Finer Dash (Backward)""#, r#""ミストファイナーバックステップ""#);
        map.insert(r#""Mist Finer Dash (Forward)""#, r#""ミストファイナー前ステップ""#);
        map.insert(r#""Joker Trick""#, r#""ジョーカートリック""#);
        map.insert(r#""That&#039;s My Name""#, r#""それが俺の名だ""#);

        // カイ
        map.insert(r#""Foudre Arc""#, r#""フードゥルアルク""#);
        map.insert(r#""Dire Eclat""#, r#""ダイアエクラ""#);
        map.insert(r#""Charged Stun Edge""#, r#""スタンエッジ・チャージアタック""#);
        map.insert(r#""Stun Dipper""#, r#""スタンディッパー""#);
        map.insert(r#""Stun Edge""#, r#""スタンエッジ""#);
        map.insert(r#""H Vapor Thrust""#, r#""HSヴェイパースラスト""#);
        map.insert(r#""S Vapor Thrust""#, r#""ヴェイパースラスト""#);
        map.insert(r#""DI Foudre Arc""#, r#""ドラゴンインストールフードゥルアルク""#);
        map.insert(r#""DI Dire Eclat""#, r#""ドラゴンインストールダイアエクラ""#);
        map.insert(r#""DI Charged Stun Edge""#, r#""ドラゴンインストールスタンエッジ・チャージアタック""#);
        map.insert(r#""DI Stun Dipper""#, r#""ドラゴンインストールスタンディッパー""#);
        map.insert(r#""DI Stun Edge""#, r#""ドラゴンインストールスタンエッジ""#);
        map.insert(r#""DI H Vapor Thrust""#, r#""ドラゴンインストールH ヴェイパースラスト""#);
        map.insert(r#""DI S Vapor Thrust""#, r#""ドラゴンインストールヴェイパースラスト""#);
        map.insert(r#""DI Aerial H Stun Edge""#, r#""ドラゴンインストール空中HSスタンエッジ""#);
        map.insert(r#""DI Aerial S Stun Edge""#, r#""ドラゴンインストール空中スタンエッジ""#);
        map.insert(r#""DI Air H Vapor Thrust""#, r#""ドラゴンインストール空中HSヴェイパースラスト""#);
        map.insert(r#""DI Air S Vapor Thrust""#, r#""ドラゴンインストール空中ヴェイパースラスト""#);
        map.insert(r#""Aerial H Stun Edge""#, r#""空中HSスタンエッジ""#);
        map.insert(r#""Aerial S Stun Edge""#, r#""空中スタンエッジ""#);
        map.insert(r#""Air H Vapor Thrust""#, r#""空中HSヴェイパースラスト""#);
        map.insert(r#""Air S Vapor Thrust""#, r#""空中ヴェイパースラスト""#);
        map.insert(r#""Dragon Install""#, r#""ドラゴンインストール""#);
        map.insert(r#""Sacred Edge""#, r#""セイクリッドエッジ""#);
        map.insert(r#""Ride the Lightning""#, r#""ライドザライトニング""#);
        map.insert(r#""DI Sacred Edge""#, r#""ドラゴンインストールセイクリッドエッジ""#);
        map.insert(r#""DI Ride the Lightning""#, r#""ドラゴンインストールライドザライトニング""#);
        map.insert(r#""DI Air Ride the Lightning""#, r#""ドラゴンインストール空中ライドザライトニング""#);
        map.insert(r#""Air Ride the Lightning""#, r#""空中ライドザライトニング""#);

        // レオ
        map.insert(r#""Brynhildr Cancel""#, r#""ブリュンヒルドの構え解除""#);
        map.insert(r#""Kahn Schild""#, r#""カーンシルト""#);
        map.insert(r#""Turbulenz""#, r#""トゥルブレンツ""#);
        map.insert(r#""Kaltes Gestöber Zweit""#, r#""ツヴァイト・カルタスゲシュトゥーバー""#);
        map.insert(r#""Kaltes Gestöber Erst""#, r#""エアースト・カルタスゲシュトゥーバー""#);
        map.insert(r#""Blitzschlag""#, r#""ブリッツシュラーク""#);
        map.insert(r#""Gländzendes Dunkel""#, r#""グレンツェンドゥンケル""#);
        map.insert(r#""H Eisen Sturm""#, r#""HSアイゼンシュトルム""#);
        map.insert(r#""S Eisen Sturm""#, r#""Sアイゼンシュトルム""#);
        map.insert(r#""H Graviert Wurde""#, r#""HSグラヴィエットヴァーダ""#);
        map.insert(r#""S Graviert Wurde""#, r#""Sグラヴィエットヴァーダ""#);
        map.insert(r#""Leidenschaft des Dirigenten""#, r#""ライデンシャフトディリガント""#);
        map.insert(r#""Stahl Wirbel""#, r#""シュタイルヴァービル""#);
        map.insert(r#""Stahl Wirbel""#, r#""シュタイルヴァービル""#);
        map.insert(r#""input":"bt.66","name":"bt.66""#, r#""input":"bt.66","name":"ブリュンヒルデの構え66""#);
        map.insert(r#""input":"bt.44","name":"bt.44""#, r#""input":"bt.44","name":"ブリュンヒルデの構え44""#);
        map.insert(r#""input":"bt.K","name":"bt.K""#, r#""input":"bt.K","name":"ブリュンヒルデの構えK""#);
        map.insert(r#""input":"bt.P","name":"bt.P""#, r#""input":"bt.P","name":"ブリュンヒルデの構えP""#);
        map.insert(r#""input":"bt.S","name":"bt.S""#, r#""input":"bt.S","name":"ブリュンヒルデの構えS""#);
        map.insert(r#""input":"bt.HS","name":"bt.HS""#, r#""input":"bt.HS","name":"ブリュンヒルデの構えHS""#);
        map.insert(r#""bt."#, r#""bt"#);

        // メイ
        map.insert(r#""K Arisugawa Sparkle""#, r#""K有栖川""#);
        map.insert(r#""P Arisugawa Sparkle""#, r#""P有栖川""#);
        map.insert(r#""Overhead Kiss""#, r#""オーバーヘッドキッス""#);
        map.insert(r#""H Mr. Dolphin Vertical""#, r#""HS縦イルカ""#);
        map.insert(r#""S Mr. Dolphin Vertical""#, r#""S縦イルカ""#);
        map.insert(r#""H Mr. Dolphin Horizontal""#, r#""HS横イルカ""#);
        map.insert(r#""S Mr. Dolphin Horizontal""#, r#""S横イルカ""#);
        map.insert(r#""Split""#, r#""分離""#);
        map.insert(r#""Whiff""#, r#""停止""#);
        map.insert(r#""Great Yamada Attack""#, r#""グレート山田アタック""#);
        map.insert(r#""The Wonderful and Dynamic Goshogawara""#, r#""ワンダフル五所川原ダイナミック""#);
        map.insert(r#""Air The Wonderful and Dynamic Goshogawara""#, r#""空中ワンダフル五所川原ダイナミック""#);
        map.insert(r#""[2]8H""#, r#""[2]8HS""#);
        map.insert(r#""[4]6S/H~K""#, r#""[4]6SK/[4]6HSK""#);
        map.insert(r#""[4]6S/H~P""#, r#""[4]6SP/[4]6HSP""#);

        // ミリア
        map.insert(r#""Tandem Top""#, r#""Sタンデム""#);
        map.insert(r#""H Tandem Top""#, r#""HSタンデム""#);
        map.insert(r#""Lust Shaker""#, r#""ラストシェイカー""#);
        map.insert(r#""Iron Savior""#, r#""アイアンセイバー""#);
        map.insert(r#""Bad Moon""#, r#""バッドムーン""#);
        map.insert(r#""Turbo Fall""#, r#""高速落下""#);
        map.insert(r#""Mirazh""#, r#""ミラーシュ""#);
        map.insert(r#""Kapel""#, r#""カピエル""#);
        map.insert(r#""Septem Voices""#, r#""セプテムヴォイシズ""#);
        map.insert(r#""Winger""#, r#""ヴィンガー""#);
        map.insert(r#""Artemis""#, r#""アルテミス""#);

        // 名残雪
        map.insert(r#""input":"2H Level 1","name":"Level 1""#, r#""input":"2HS1","name":"2HS Level 1""#);
        map.insert(r#""input":"2H Level 2","name":"Level 2""#, r#""input":"2HS2","name":"2HS Level 2""#);
        map.insert(r#""input":"2H Level 3","name":"Level 3""#, r#""input":"2HS3","name":"2HS Level 3""#);
        map.insert(r#""input":"2H Level 1","name":"Level 1""#, r#""input":"2HS1","name":"2HS Level 1""#);
        map.insert(r#""input":"2H Level 2,"name":"Level 2""#, r#""input":"2HS2","name":"2HS Level 2""#);
        map.insert(r#""input":"2H Level 3","name":"Level 3""#, r#""input":"2HS3","name":"2HS Level 3""#);
        map.insert(r#""input":"2S Level 1","name":"Level 1""#, r#""input":"2S1","name":"2S Level 1""#);
        map.insert(r#""input":"2S Level 2","name":"Level 2""#, r#""input":"2S2","name":"2S Level 2""#);
        map.insert(r#""input":"2S Level 3","name":"Level 3""#, r#""input":"2S3","name":"2S Level 3""#);
        map.insert(r#""input":"5H Level 1","name":"Level 1""#, r#""input":"5HS1","name":"5HS Level 1""#);
        map.insert(r#""input":"5H Level 2","name":"Level 2""#, r#""input":"5HS2","name":"5HS Level 2""#);
        map.insert(r#""input":"5H Level 3","name":"Level 3""#, r#""input":"5HS3","name":"5HS Level 3""#);
        map.insert(r#""input":"6H Level 1","name":"Level 1""#, r#""input":"6HS1","name":"6HS Level 1""#);
        map.insert(r#""input":"6H Level 2","name":"Level 2""#, r#""input":"6HS2","name":"6HS Level 2""#);
        map.insert(r#""input":"6H Level 3","name":"Level 3""#, r#""input":"6HS3","name":"6HS Level 3""#);
        map.insert(r#""input":"f.S Level 1","name":"Level 1""#, r#""input":"遠S1","name":"遠S Level 1""#);
        map.insert(r#""input":"f.S Level 2","name":"Level 2""#, r#""input":"遠S2","name":"遠S Level 2""#);
        map.insert(r#""input":"f.S Level 3","name":"Level 3""#, r#""input":"遠S3","name":"遠S Level 3""#);
        map.insert(r#""input":"f.SS Level 1","name":"Level 1""#, r#""input":"遠SS1","name":"遠SS Level 1""#);
        map.insert(r#""input":"f.SS Level 2","name":"Level 2""#, r#""input":"遠SS2","name":"遠SS Level 2""#);
        map.insert(r#""input":"f.SS Level 3","name":"Level 3""#, r#""input":"遠SS3","name":"遠SS Level 3""#);
        map.insert(r#""input":"f.SSS Level 1","name":"Level 1""#, r#""input":"遠SSS1","name":"遠SSS Level 1""#);
        map.insert(r#""input":"f.SSS Level 2","name":"Level 2""#, r#""input":"遠SSS2","name":"遠SSS Level 2""#);
        map.insert(r#""input":"f.SSS Level 3","name":"Level 3""#, r#""input":"遠SSS3","name":"遠SSS Level 3""#);
        map.insert(r#""input":"jS Level 1","name":"Level 1""#, r#""input":"jS1","name":"jS Level 1""#);
        map.insert(r#""input":"jS Level 2","name":"Level 2""#, r#""input":"jS2","name":"jS Level 2""#);
        map.insert(r#""input":"jS Level 3","name":"Level 3""#, r#""input":"jS3","name":"jS Level 3""#);
        map.insert(r#""input":"jD Level 1","name":"Level 1""#, r#""input":"jD1","name":"jD Level 1""#);
        map.insert(r#""input":"jD Level 2","name":"Level 2""#, r#""input":"jD2","name":"jD Level 2""#);
        map.insert(r#""input":"jD Level 3","name":"Level 3""#, r#""input":"jD3","name":"jD Level 3""#);
        map.insert(r#""input":"jH Level 1","name":"Level 1""#, r#""input":"jHS1","name":"jHS Level 1""#);
        map.insert(r#""input":"jH Level 2","name":"Level 2""#, r#""input":"jHS2","name":"jHS Level 2""#);
        map.insert(r#""input":"jH Level 3","name":"Level 3""#, r#""input":"jHS3","name":"jHS Level 3""#);
        map.insert(r#""input":"2H Level BR","name":"Blood Rage""#, r#""input":"2SBR","name":"2S Level BR""#);
        map.insert(r#""input":"2S Level BR","name":"Blood Rage""#, r#""input":"2SBR","name":"2S Level BR""#);
        map.insert(r#""input":"5S Level BR","name":"Blood Rage""#, r#""input":"5HSBR","name":"5S Level BR""#);
        map.insert(r#""input":"5H Level BR","name":"Blood Rage""#, r#""input":"5HSBR","name":"5S Level BR""#);
        map.insert(r#""input":"6S Level BR","name":"Blood Rage""#, r#""input":"6HSBR","name":"6HS Level BR""#);
        map.insert(r#""input":"f.S Level BR","name":"Blood Rage""#, r#""input":"遠SBR","name":"遠S Level BR""#);
        map.insert(r#""input":"f.SS Level BR","name":"Blood Rage""#, r#""input":"遠SSBR","name":"遠SS Level BR""#);
        map.insert(r#""input":"f.SSS Level BR","name":"Blood Rage""#, r#""input":"遠SSSBR","name":"遠SSS Level BR""#);
        map.insert(r#""input":"jD Level BR","name":"Blood Rage""#, r#""input":"jDBR","name":"jD Level BR""#);
        map.insert(r#""input":"jH Level BR","name":"Blood Rage""#, r#""input":"jHSBR","name":"jHS Level BR""#);
        map.insert(r#""input":"jS Level BR","name":"Blood Rage""#, r#""input":"jSBR","name":"jS Level BR""#);
        map.insert(r#""input":"6H Level BR","name":"Blood Rage""#, r#""input":"6HSBR","name":"6HS Level BR""#);
        map.insert(r#""Kamuriyuki""#, r#""冠雪""#);
        map.insert(r#""Backward Fukyo""#, r#""後ろ不香""#);
        map.insert(r#""Forward Fukyo""#, r#""不香""#);
        map.insert(r#""Zarameyuki""#, r#""粒雪""#);
        map.insert(r#""Shizuriyuki (1)""#, r#""垂雪""#);
        map.insert(r#""Shizuriyuki (2)""#, r#""垂雪追撃""#);
        map.insert(r#""Zansetsu""#, r#""残雪""#);
        map.insert(r#""Wasureyuki""#, r#""忘れ雪""#);
        map.insert(r#""623HH""#, r#""623HSHS""#);

        // ラムレザル
        map.insert(r#""H Bajoneto""#, r#""HSバヨネート""#);
        map.insert(r#""S Bajoneto""#, r#""Sバヨネート""#);
        map.insert(r#""Dauro""#, r#""ダウロ""#);
        map.insert(r#""Sildo Detruo""#, r#""シルド""#);
        map.insert(r#""Air Sildo Detruo""#, r#""空中シルド""#);
        map.insert(r#""Sabrubato""#, r#""サブロバート""#);
        map.insert(r#""Erarlumo (3)""#, r#""エラルルーモ3""#);
        map.insert(r#""Erarlumo (2)""#, r#""エラルルーモ2""#);
        map.insert(r#""Erarlumo (1)""#, r#""エラルルーモ1""#);
        map.insert(r#""Agressa Ordono""#, r#""アグレーサ""#);
        map.insert(r#""Ondo""#, r#""オンド""#);
        map.insert(r#""Calvados""#, r#""カルヴァドス""#);
        map.insert(r#""Mortobato""#, r#""モルトバート""#);
        map.insert(r#""214P 214P 214P""#, r#""214P214P214P""#);
        map.insert(r#""214P 214P""#, r#""214P214PP""#);
        map.insert(r#""R.T.L Follow-up""#, r#""R.T.L.派生""#);

        // ディズィー
        map.insert(r#""H Michael Sword""#, r#""HSミカエルソード""#);
        map.insert(r#""We talked a lot together""#, r#""よく話し相手になってくれました""#);
        map.insert(r#""S Michael Sword""#, r#""Sミカエルソード""#);
        map.insert(r#""Wings of Light""#, r#""光の翼""#);
        map.insert(r#""For roasting chestnuts""#, r#""焼き栗が欲しい時に使ってたんです""#);
        map.insert(r#""I used this to catch fish""#, r#""魚を捕る時に使ってたんです""#);
        map.insert(r#""Ice Field""#, r#""氷原""#);
        map.insert(r#""Gamma Ray""#, r#""ガンマレイ""#);
        map.insert(r#""Imperial Ray""#, r#""インペリアルレイ""#);
        map.insert(r#""236S~6S/236H~6H""#, r#""236S6S/236HS6HS""#);

        // ポチョムキン
        map.insert(r#""Hammer Fall""#, r#""ハンマーフォール""#);
        map.insert(r#""Hammer Fall Break""#, r#""ハンマーフォールブレーキ""#);
        map.insert(r#""Potemkin Buster""#, r#""ポチョムキンバスター""#);
        map.insert(r#""Heat Knuckle""#, r#""ヒートナックル""#);
        map.insert(r#""Mega Fist""#, r#""メガフィスト・前方""#);
        map.insert(r#""B Mega Fist""#, r#""メガフィスト・後方""#);
        map.insert(r#""Forward Mega Fist""#, r#""メガフィスト・前方""#);
        map.insert(r#""Backward Mega Fist""#, r#""メガフィスト・後方""#);
        map.insert(r#""Slide Head""#, r#""スライドヘッド""#);
        map.insert(r#""Garuda Impact""#, r#""ガルダインパクト""#);
        map.insert(r#""Heavenly Potemkin Buster""#, r#""ヘブンリーポチョムキンバスター""#);
        map.insert(r#""Giganter Kai""#, r#""ガイガンダー改""#);
        map.insert(r#""Giganter Kai Barrier""#, r#""ガイガンダー改バリア""#);
        map.insert(r#""Giganter Kai (Barrier)""#, r#""ガイガンダー改バリア""#);
        map.insert(r#""[4]6H P""#, r#""[4]6HS P""#);
        map.insert(r#""[4]6H""#, r#""[4]6HS""#);
        map.insert(r#""Heat Tackle""#, r#""ヒートタックル""#);
        map.insert(r#""F.D.B. (Charged)""#, r#""溜めF.D.B.""#);

        // シン
        map.insert(r#""Hoof Stomp""#, r#""フーフスタンプ""#);
        map.insert(r#""Hoof Stomp Follow-Up""#, r#""フーフスタンプ派生""#);
        map.insert(r#""Beak Driver""#, r#""ビークドライバー""#);
        map.insert(r#""Beak Driver Follow-Up""#, r#""ビークドライバー派生""#);
        map.insert(r#""Elk Hunt""#, r#""エルクハント""#);
        map.insert(r#""Elk Hunt Follow-Up""#, r#""エルクハント派生""#);
        map.insert(r#""Hawk Baker""#, r#""ホークベイカー""#);
        map.insert(r#""Hawk Baker Follow-Up""#, r#""ホークベイカー派生""#);
        map.insert(r#""Still Growing""#, r#""育ち盛りだからな。""#);
        map.insert(r#""Gazelle Step""#, r#""ガゼルステップ""#);
        map.insert(r#""Gazelle Step ""#, r#""ガゼルステップ""#);
        map.insert(r#""Gazelle Step Cancel""#, r#""ガゼルステップキャンセル""#);
        map.insert(r#""Tyrant Barrel""#, r#""タイランバレル""#);
        map.insert(r#""Tyrant Barrel Follow-up""#, r#""タイランレイブ""#);
        map.insert(r#""R.T.L""#, r#""R.T.L.""#);
        map.insert(r#""R.T.L Follow-up""#, r#""R.T.L.派生""#);
        map.insert(r#""214S~S""#, r#""214SS""#);
        map.insert(r#""236H~H""#, r#""236HSHS""#);
        map.insert(r#""236K~K""#, r#""236KK""#);
        map.insert(r#""6236S~S""#, r#""623SS""#);
        map.insert(r#""236236P~]P[""#, r#""236236P]P[""#);
        map.insert(r#""632146HH""#, r#""632146HSHS""#);

        // スレイヤー
        map.insert(r#""Dandy Step K""#, r#""ダンディーステップK""#);
        map.insert(r#""Dandy Step P""#, r#""ダンディーステップP""#);
        map.insert(r#""Master's Hammer""#, r#""マスターズハンマー""#);
        map.insert(r#""Master&#039;s Hammer""#, r#""マスターズハンマー""#);
        map.insert(r#""Bump Ahead""#, r#""バンプアヘッド""#);
        map.insert(r#""Pilebunker""#, r#""パイルバンカー""#);
        map.insert(r#""It's Late""#, r#""イッツレイト""#);
        map.insert(r#""It&#039;s Late""#, r#""イッツレイト""#);
        map.insert(r#""Last Horizon""#, r#""ラスト・ホライズン""#);
        map.insert(r#""Mappa Hunch K""#, r#""マッパハンチK""#);
        map.insert(r#""Mappa Hunch P""#, r#""マッパハンチP""#);
        map.insert(r#""Hand of Doom""#, r#""Hand of Doom""#);
        map.insert(r#""Super Mappa Hunch""#, r#""スーパーマッパハンチ""#);

        // ソル
        map.insert(r#""Bandit Bringer""#, r#""バンディットブリンガー""#);
        map.insert(r#""Gun Flame (Feint)""#, r#""ガンフレイムフェイント""#);
        map.insert(r#""Night Raid Vortex""#, r#""ヴォルテックス""#);
        map.insert(r#""Bandit Revolver (1)""#, r#""バンディットリボルバー""#);
        map.insert(r#""Bandit Revolver (2)""#, r#""バンディットリボルバー2""#);
        map.insert(r#""Gun Flame""#, r#""ガンフレイム""#);
        map.insert(r#""Fafnir""#, r#""ファフニール""#);
        map.insert(r#""H Volcanic Viper""#, r#""HSヴォルカニックヴァイパー""#);
        map.insert(r#""Wild Throw""#, r#""ぶっきらぼうに投げる""#);
        map.insert(r#""S Volcanic Viper""#, r#""ヴォルカニックファイパー""#);
        map.insert(r#""Aerial Bandit Bringer""#, r#""空中バンディットブリンガー""#);
        map.insert(r#""Aerial Bandit Revolver (1)""#, r#""空中バンディットリボルバー""#);
        map.insert(r#""Aerial Bandit Revolver (2)""#, r#""空中バンディットリボルバー2""#);
        map.insert(r#""Aerial H Volcanic Viper""#, r#""空中HSヴォルカニックヴァイパー""#);
        map.insert(r#""Aerial S Volcanic Viper""#, r#""空中ヴォルカニックヴァイパー""#);
        map.insert(r#""Heavy Mob Cemetery""#, r#""ヘヴィモブセメタリー""#);
        map.insert(r#""Tyrant Rave""#, r#""タイランレイブ""#);

        // テスタメント
        map.insert(r#""Possession""#, r#""ポゼッション""#);
        map.insert(r#""Unholy Diver""#, r#""アンホーリーダイバー""#);
        map.insert(r#""S Arbiter Sign""#, r#""Sアービターサイン""#);
        map.insert(r#""H Arbiter Sign""#, r#""HSアービターサイン""#);
        map.insert(r#""S Grave Reaper""#, r#""Sグレイヴリーパー""#);
        map.insert(r#""H Grave Reaper""#, r#""HSグレイヴリーパー""#);
        map.insert(r#""Stain""#, r#""ステイン""#);
        map.insert(r#""Calamity One""#, r#""カラミティ・ワン""#);
        map.insert(r#""Nostrovia""#, r#""ノストロヴィア""#);
        map.insert(r#""236[H]""#, r#""236[HS]""#);
        map.insert(r#""236{H}""#, r#""236{HS}""#);

        // ザトー
        map.insert(r#""22H""#, r#""22HS""#);
        map.insert(r#""]H[""#, r#""]HS[""#);
        map.insert(r#""Unsummon Eddie""#, r#""エディ召喚""#);
        map.insert(r#""Summon Eddie""#, r#""エディ収納""#);
        map.insert(r#""Break The Law""#, r#""ブレイク・ザ・ロウ""#);
        map.insert(r#""Drunkard Shade""#, r#""ドランカーシェイド""#);

        map.insert(r#""Eddie Dash""#, r#""エディダッシュ""#);
        map.insert(r#""Eddie Teleport""#, r#""エディスワップ""#);
        map.insert(r#""Invite Hell""#, r#""インヴァイトヘル""#);
        map.insert(r#""Oppose""#, r#""「張り合う」""#);
        map.insert(r#""That's a lot""#, r#""「多い」""#);
        map.insert(r#""Pierce""#, r#""「突く」""#);
        map.insert(r#""Leap""#, r#""「跳ねる」""#);
        map.insert(r#""Damned Fang""#, r#""ダムドファング""#);
        map.insert(r#""Amorphous""#, r#""アモルファス""#);
        map.insert(r#""That&#039;s a lot""#, r#""「多い」""#);
        map.insert(r#""Sun Void""#, r#""サンヴォイド""#);

        map.insert(r#""6236S""#, r#""623S""#);
        map.insert(r#""j6236S""#, r#""j623S""#);

        map
    };

    // キャラクター別の置換マップ
    // 必要に応じて追加のキャラクター別マップを作成
}

/// 与えられたJSON文字列に対して共通の置換処理を実施する関数
///
/// # 概要
/// 入力されたJSON文字列内の特定パターンを、所定の日本語表記へ変換する。  
/// フレームデータ等の文字列の正規化に利用する。
///
/// # 引数
/// * `json` - 置換前のJSON文字列
///
/// # 戻り値
/// 置換処理後のJSON文字列
///
/// # 例
/// ```rust,no_run
/// use your_crate::preprocess_json;
/// let input = r#"{"key": "c.S", "other": "f.S"}"#.to_string();
/// let output = preprocess_json(input);
/// println!("{}", output);
/// ```
pub fn preprocess_json(json: String) -> String {
    let mut result = json;

    // 正規表現による置換
    result = RE_C_S.replace_all(&result, r#""近S""#).to_string();
    result = RE_F_S.replace_all(&result, r#""遠S""#).to_string();
    result = RE_J_MOVE.replace_all(&result, r#""j$1""#).to_string();
    result = RE_HKD_MOVE
        .replace_all(&result, r#"強制ダウン"#)
        .to_string();
    result = RE_KD_MOVE.replace_all(&result, r#"ダウン"#).to_string();

    // マップを使用した置換
    for (pattern, replacement) in REPLACEMENT_MAP.iter() {
        result = result.replace(pattern, replacement);
    }

    result
}
