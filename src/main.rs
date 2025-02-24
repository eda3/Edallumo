//! main.rs
//!
//! このファイルは、Discord Bot の起動基盤および各種機能モジュールの読み込み・初期設定を行う。
//! poise フレームワークを利用し、コマンドの登録、エラーハンドリング、定期更新タスクなどを構成する。

// モジュール読み込み
mod check; // 初期チェック機能
mod commands; // コマンド群実装
mod common; // 共通処理群
mod find; // 情報検索機能

// 外部クレート読み込み
use colored::Colorize; // 文字色変換用
use commands::*; // 各コマンド一括読み込み
use poise::serenity_prelude as serenity; // Serenity 用エイリアス
use serde::{Deserialize, Serialize}; // シリアライズ／デシリアライズ用
use std::{
    io::Write,                 // 標準入出力操作用
    time::{Duration, Instant}, // 時間計測用
};
use tokio::{task, time}; // 非同期タスク、タイマー用

/// エラー型およびコンテキスト型定義
///
/// # 型説明
/// * `Error` - 送信可能かつ同期可能な標準エラー型
/// * `Context<'a>` - poise コマンド実行時に渡されるコンテキスト型
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// 各コマンドに共通して渡されるユーザーデータ
pub struct Data {}

/// キャラクター情報構造体
///
/// 各キャラクターの各種ステータスを保持
#[derive(Serialize, Deserialize, Debug)]
pub struct CharInfo {
    defense: String,                // 防御値
    guts: String,                   // ガッツ
    guard_balance: String,          // ガードバランス
    prejump: String,                // ジャンプ前の状態
    umo: String,                    // 未使用（予備）
    forward_dash: String,           // 前方ダッシュ速度
    backdash: String,               // バックダッシュ速度
    backdash_duration: String,      // バックダッシュ持続時間
    backdash_invincibility: String, // バックダッシュ無敵時間
    backdash_airborne: String,      // 空中バックダッシュ
    backdash_distance: String,      // バックダッシュ移動距離
    jump_duration: String,          // ジャンプ持続時間
    jump_height: String,            // ジャンプ高さ
    high_jump_duration: String,     // ハイジャンプ持続時間
    high_jump_height: String,       // ハイジャンプ高さ
    earliest_iad: String,           // 最速IAD（未使用）
    ad_duration: String,            // AD持続時間（未使用）
    ad_distance: String,            // AD移動距離（未使用）
    abd_duration: String,           // ABD持続時間（未使用）
    abd_distance: String,           // ABD移動距離（未使用）
    movement_tension: String,       // 移動緊張度
    jump_tension: String,           // ジャンプ緊張度
    airdash_tension: String,        // エアダッシュ緊張度
    walk_speed: String,             // 歩行速度
    back_walk_speed: String,        // 後ろ歩行速度
    dash_initial_speed: String,     // ダッシュ初速
    dash_acceleration: String,      // ダッシュ加速
    dash_friction: String,          // ダッシュ摩擦
    jump_gravity: String,           // ジャンプ重力
    high_jump_gravity: String,      // ハイジャンプ重力
}

/// 技情報構造体
///
/// 各技の入力、名称、フレームデータなどを保持
#[derive(Serialize, Deserialize, Debug)]
pub struct MoveInfo {
    input: String,         // 入力コマンド
    name: String,          // 技名称
    damage: String,        // ダメージ値
    guard: String,         // ガード値
    startup: String,       // 始動フレーム
    active: String,        // アクティブフレーム
    recovery: String,      // リカバリーフレーム
    on_hit: String,        // ヒット時効果
    on_block: String,      // ブロック時効果
    level: String,         // 技レベル
    counter: String,       // カウンター情報
    move_type: String,     // 技種別
    risc_gain: String,     // リスクゲイン
    risc_loss: String,     // リスクロス
    wall_damage: String,   // 壁ダメージ
    input_tension: String, // 入力緊張度
    chip_ratio: String,    // チップダメージ比率
    otg_ratio: String,     // OTG比率
    scaling: String,       // ダメージスケーリング
    invincibility: String, // 無敵フレーム
    cancel: String,        // キャンセル情報
    caption: String,       // キャプション
    notes: String,         // 備考
                           // hitbox_caption, images, hitboxes は未使用
}

/// 画像リンク構造体
///
/// 各技の画像リンクおよびヒットボックス画像リンクを保持
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageLinks {
    input: String,           // 入力コマンド
    move_img: String,        // 技画像リンク
    hitbox_img: Vec<String>, // ヒットボックス画像リンク群
}

/// 技エイリアス構造体
///
/// 各技の別名を保持
#[derive(Serialize, Deserialize, Debug)]
pub struct MoveAliases {
    input: String,        // 入力コマンド（キー）
    aliases: Vec<String>, // エイリアス群
}

/// キャラクター愛称構造体
///
/// 各キャラクターの愛称を保持
#[derive(Serialize, Deserialize, Debug)]
pub struct Nicknames {
    character: String,      // キャラクター名称
    nicknames: Vec<String>, // 愛称群
}

/// 識別子群構造体（内部利用）
#[derive(Serialize, Deserialize, Debug)]
struct Gids {
    id: Vec<String>, // 識別子群
}

/// キャラクター定数群
///
/// 29体のキャラクター名称定数
pub const CHARS: [&str; 29] = [
    "A.B.A",
    "Anji_Mito",
    "Asuka_R",
    "Axl_Low",
    "Baiken",
    "Bedman",
    "Bridget",
    "Chipp_Zanuff",
    "Elphelt_Valentine",
    "Faust",
    "Giovanna",
    "Goldlewis_Dickinson",
    "Happy_Chaos",
    "I-No",
    "Jack-O",
    "Johnny",
    "Ky_Kiske",
    "Leo_Whitefang",
    "May",
    "Millia_Rage",
    "Nagoriyuki",
    "Potemkin",
    "Queen_Dizzy",
    "Ramlethal_Valentine",
    "Sin_Kiske",
    "Slayer",
    "Sol_Badguy",
    "Testament",
    "Zato-1",
];

/// デフォルト画像リンク
const IMAGE_DEFAULT: &str =
    "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/no_image.png";
/// デフォルトヒットボックス画像リンク
const HITBOX_DEFAULT: &str =
    "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/no_hitbox.png";
/// 埋め込みメッセージ背景色（RGB）
const EMBED_COLOR: (u8, u8, u8) = (140, 75, 64);

/// グローバルエラーハンドラ
///
/// 発生したエラーに対して、カスタム処理を実行する。
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // エラー種別に応じた処理分岐
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!(
            "{}",
            ("Failed to start bot: ".to_owned() + &error.to_string() + ".").red()
        ),
        poise::FrameworkError::Command { error, ctx, .. } => {
            // コマンド実行中のエラーを表示
            println!(
                "{}",
                ("Error in command `".to_owned()
                    + &ctx.command().name
                    + "`: "
                    + &error.to_string()
                    + ".")
                    .red()
            );
        }
        error => {
            // その他のエラーはデフォルトハンドラに委譲
            if let Err(e) = poise::builtins::on_error(error).await {
                println!(
                    "{}",
                    ("Error while handling error: ".to_owned() + &e.to_string() + ".").red()
                )
            }
        }
    }
}

/// メイン関数
///
/// Bot の初期設定および起動を行う。各種初期チェック、コマンド登録、エラーハンドリング、
/// 自動更新タスクの起動を実施する。
#[tokio::main]
async fn main() {
    // 初期チェック実行
    println!(); // 改行出力
    check::data_folder_exists(true).await; // データフォルダ存在確認
    check::character_folders_exist(true).await; // キャラクターフォルダ存在確認
    check::character_jsons_exist(true).await; // キャラクターJSON存在確認
    check::character_images_exist(true).await; // キャラクター画像JSON存在確認
    check::nicknames_json_exists(true).await; // ニックネームJSON存在確認

    // FrameworkOptions 設定
    let options = poise::FrameworkOptions {
        commands: vec![
            frames::frames(),       // フレーム表示コマンド
            help::help(),           // ヘルプコマンド
            hitboxes::hitboxes(),   // ヒットボックス表示コマンド
            nicknames::nicknames(), // ニックネーム表示コマンド
            moves::moves(),         // ムーブ一覧表示コマンド
            register::register(),   // コマンド再登録コマンド
            update::update(),       // データ更新コマンド
        ],
        on_error: |error| Box::pin(on_error(error)), // グローバルエラーハンドラ設定
        pre_command: |ctx| {
            Box::pin(async move {
                // コマンド実行前にコマンド名を表示
                println!(
                    "{}",
                    ("\nExecuting command ".to_owned() + &ctx.command().qualified_name + ".")
                        .cyan()
                );
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                // コマンド実行後、経過時間を表示
                let elapsed_time = ctx
                    .invocation_data::<Instant>()
                    .await
                    .as_deref()
                    .unwrap()
                    .elapsed();
                print!(
                    "{}",
                    ("Executed command ".to_owned() + &ctx.command().qualified_name + " in ")
                        .cyan()
                );
                print!("{}", (elapsed_time.as_millis().to_string() + "ms").yellow());
                print!("{}\n", ".".cyan());
                std::io::stdout().flush().unwrap();
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                // オーナーIDチェック　特定IDの場合はコマンド実行拒否
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                // コマンド実行時刻を記録
                ctx.set_invocation_data(Instant::now()).await;
                Ok(true)
            })
        }),
        skip_checks_for_owners: false, // オーナーにもチェック実施
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                match event {
                    // Ready イベント発生時の処理
                    serenity::FullEvent::Ready { data_about_bot: _ } => {
                        let forever = task::spawn(async {
                            let mut interval = time::interval(Duration::from_secs(86400));
                            loop {
                                // 24時間毎に全キャラクターデータ更新実行
                                interval.tick().await;
                                update::update_all_char_data().await;
                            }
                        });
                        let _ = forever.await;
                        Ok(())
                    }
                    _ => Ok(()),
                }
            })
        },
        ..Default::default() // その他はデフォルト設定
    };

    // poise フレームワークの構築
    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                // グローバルコマンド登録
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    // .env ファイルから環境変数読み込み
    dotenv::dotenv().expect("Failed to load .env file.");
    // Discord トークン取得
    let token = std::env::var("DISCORD_TOKEN").expect("Failed to load `DISCORD_TOKEN` env var.");
    // 非特権インテント設定
    let intents = serenity::GatewayIntents::non_privileged();
    // クライアントビルダー作成
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    // Bot 起動開始
    client.unwrap().start().await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    /// CharInfo のシリアライズ／デシリアライズ検証用テスト
    ///
    /// # 説明
    /// - テスト対象: `CharInfo` 構造体の JSON 変換機能  
    /// - 入力値: 各フィールドに文字列を設定  
    /// - 期待結果: 変換前後で各フィールド値の一致  
    /// 検証項目: defense、guts など  
    #[test]
    fn test_charinfo_serialization() {
        // サンプルデータ生成
        // 入力値: defense "50", guts "30", guard_balance "20", など
        let char_info = CharInfo {
            defense: "50".to_string(),
            guts: "30".to_string(),
            guard_balance: "20".to_string(),
            prejump: "10".to_string(),
            umo: "unused".to_string(),
            forward_dash: "5".to_string(),
            backdash: "5".to_string(),
            backdash_duration: "3".to_string(),
            backdash_invincibility: "2".to_string(),
            backdash_airborne: "yes".to_string(),
            backdash_distance: "15".to_string(),
            jump_duration: "8".to_string(),
            jump_height: "12".to_string(),
            high_jump_duration: "10".to_string(),
            high_jump_height: "18".to_string(),
            earliest_iad: "0".to_string(),
            ad_duration: "0".to_string(),
            ad_distance: "0".to_string(),
            abd_duration: "0".to_string(),
            abd_distance: "0".to_string(),
            movement_tension: "low".to_string(),
            jump_tension: "medium".to_string(),
            airdash_tension: "high".to_string(),
            walk_speed: "4".to_string(),
            back_walk_speed: "3".to_string(),
            dash_initial_speed: "6".to_string(),
            dash_acceleration: "1".to_string(),
            dash_friction: "0.5".to_string(),
            jump_gravity: "9.8".to_string(),
            high_jump_gravity: "9.8".to_string(),
        };
        // シリアライズ処理
        // JSON 文字列生成
        let json = serde_json::to_string(&char_info).unwrap();
        // デシリアライズ処理
        // JSON 文字列から構造体復元
        let deserialized: CharInfo = serde_json::from_str(&json).unwrap();
        // defense フィールド比較検証
        // 入力値と出力値の一致確認
        assert_eq!(char_info.defense, deserialized.defense);
        // guts フィールド比較検証
        // 入力値と出力値の一致確認
        assert_eq!(char_info.guts, deserialized.guts);
        // （その他フィールド検証省略）
    }

    /// MoveInfo のシリアライズ／デシリアライズ検証用テスト
    ///
    /// # 説明
    /// - テスト対象: `MoveInfo` 構造体の JSON 変換機能  
    /// - 入力値: 各フィールドに適切な値設定  
    /// - 期待結果: 変換前後で各フィールド値の一致  
    /// 検証項目: input、name など  
    #[test]
    fn test_moveinfo_serialization() {
        // サンプルデータ生成
        // 入力値: input "236H", name "Dragon Punch", damage "20", など
        let move_info = MoveInfo {
            input: "236H".to_string(),
            name: "Dragon Punch".to_string(),
            damage: "20".to_string(),
            guard: "S".to_string(),
            startup: "5".to_string(),
            active: "3".to_string(),
            recovery: "10".to_string(),
            on_hit: "KO".to_string(),
            on_block: "SD".to_string(),
            level: "1".to_string(),
            counter: "none".to_string(),
            move_type: "normal".to_string(),
            risc_gain: "0".to_string(),
            risc_loss: "0".to_string(),
            wall_damage: "0".to_string(),
            input_tension: "0".to_string(),
            chip_ratio: "1.0".to_string(),
            otg_ratio: "0".to_string(),
            scaling: "1.0".to_string(),
            invincibility: "0".to_string(),
            cancel: "none".to_string(),
            caption: "A powerful uppercut".to_string(),
            notes: "".to_string(),
        };
        // シリアライズ処理
        // JSON 文字列生成
        let json = serde_json::to_string(&move_info).unwrap();
        // デシリアライズ処理
        // JSON 文字列から構造体復元
        let deserialized: MoveInfo = serde_json::from_str(&json).unwrap();
        // input フィールド比較検証
        // 入力値と出力値の一致確認
        assert_eq!(move_info.input, deserialized.input);
        // name フィールド比較検証
        // 入力値と出力値の一致確認
        assert_eq!(move_info.name, deserialized.name);
    }

    /// ImageLinks のシリアライズ／デシリアライズ検証用テスト
    ///
    /// # 説明
    /// - テスト対象: `ImageLinks` 構造体の JSON 変換機能  
    /// - 入力値: 画像 URL およびヒットボックス画像配列の設定  
    /// - 期待結果: 変換前後で各フィールド値の一致  
    /// 検証項目: input、hitbox_img 配列長  
    #[test]
    fn test_image_links_serialization() {
        // サンプルデータ生成
        // 入力値: input "236H", move_img URL, hitbox_img 配列
        let image_links = ImageLinks {
            input: "236H".to_string(),
            move_img: "https://example.com/move.png".to_string(),
            hitbox_img: vec![
                "https://example.com/hitbox1.png".to_string(),
                "https://example.com/hitbox2.png".to_string(),
            ],
        };
        // シリアライズ処理
        // JSON 文字列生成
        let json = serde_json::to_string(&image_links).unwrap();
        // デシリアライズ処理
        // JSON 文字列から構造体復元
        let deserialized: ImageLinks = serde_json::from_str(&json).unwrap();
        // input フィールド比較検証
        // 入力値と出力値の一致確認
        assert_eq!(image_links.input, deserialized.input);
        // hitbox_img 配列長比較検証
        // 入力配列長と出力配列長の一致確認
        assert_eq!(image_links.hitbox_img.len(), deserialized.hitbox_img.len());
    }

    /// MoveAliases のシリアライズ／デシリアライズ検証用テスト
    ///
    /// # 説明
    /// - テスト対象: `MoveAliases` 構造体の JSON 変換機能  
    /// - 入力値: エイリアス情報の設定  
    /// - 期待結果: 変換前後で各フィールド値の一致  
    /// 検証項目: input、aliases の一致  
    #[test]
    fn test_move_aliases_serialization() {
        // サンプルデータ生成
        // 入力値: input "236H", aliases ["Dragon", "Uppercut"]
        let move_aliases = MoveAliases {
            input: "236H".to_string(),
            aliases: vec!["Dragon".to_string(), "Uppercut".to_string()],
        };
        // シリアライズ処理
        // JSON 文字列生成
        let json = serde_json::to_string(&move_aliases).unwrap();
        // デシリアライズ処理
        // JSON 文字列から構造体復元
        let deserialized: MoveAliases = serde_json::from_str(&json).unwrap();
        // input フィールド比較検証
        // 入力値と出力値の一致確認
        assert_eq!(move_aliases.input, deserialized.input);
        // aliases フィールド比較検証
        // 入力値と出力値の一致確認
        assert_eq!(move_aliases.aliases, deserialized.aliases);
    }

    /// Nicknames のシリアライズ／デシリアライズ検証用テスト
    ///
    /// # 説明
    /// - テスト対象: `Nicknames` 構造体の JSON 変換機能  
    /// - 入力値: キャラクター名および愛称リストの設定  
    /// - 期待結果: 変換前後で各フィールド値の一致  
    /// 検証項目: character、nicknames の一致  
    #[test]
    fn test_nicknames_serialization() {
        // サンプルデータ生成
        // 入力値: character "Baiken", nicknames ["Bay", "Baik"]
        let nicknames = Nicknames {
            character: "Baiken".to_string(),
            nicknames: vec!["Bay".to_string(), "Baik".to_string()],
        };
        // シリアライズ処理
        // JSON 文字列生成
        let json = serde_json::to_string(&nicknames).unwrap();
        // デシリアライズ処理
        // JSON 文字列から構造体復元
        let deserialized: Nicknames = serde_json::from_str(&json).unwrap();
        // character フィールド比較検証
        // 入力値と出力値の一致確認
        assert_eq!(nicknames.character, deserialized.character);
        // nicknames フィールド比較検証
        // 入力値と出力値の一致確認
        assert_eq!(nicknames.nicknames, deserialized.nicknames);
    }

    /// CHARS 定数の要素数検証用テスト
    ///
    /// # 説明
    /// - テスト対象: 定数 `CHARS` の要素数  
    /// - 入力値: 定数 `CHARS`  
    /// - 期待結果: 要素数 29 体の確認  
    /// 検証項目: `CHARS.len()` の一致  
    #[test]
    fn test_chars_constant() {
        // 要素数比較検証
        // 期待値: 29 体
        assert_eq!(CHARS.len(), 29);
    }

    /// EMBED_COLOR 定数の値検証用テスト
    ///
    /// # 説明
    /// - テスト対象: 定数 `EMBED_COLOR` の値  
    /// - 入力値: 定数 `EMBED_COLOR`  
    /// - 期待結果: 値 (140, 75, 64) の確認  
    /// 検証項目: `EMBED_COLOR` の値一致  
    #[test]
    fn test_embed_color() {
        // 値比較検証
        // 期待値: (140, 75, 64)
        assert_eq!(EMBED_COLOR, (140, 75, 64));
    }
}
