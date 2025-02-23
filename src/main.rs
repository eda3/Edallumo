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

mod check;
mod commands;
mod common;
mod find;

use colored::Colorize;
use commands::*;
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    time::{Duration, Instant},
};
use tokio::{task, time};

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharInfo {
    defense: String,
    guts: String,
    guard_balance: String,
    prejump: String,
    umo: String,
    forward_dash: String,
    backdash: String,
    backdash_duration: String,
    backdash_invincibility: String,
    backdash_airborne: String,
    backdash_distance: String,
    jump_duration: String,
    jump_height: String,
    high_jump_duration: String,
    high_jump_height: String,
    earliest_iad: String,
    ad_duration: String,
    ad_distance: String,
    abd_duration: String,
    abd_distance: String,
    movement_tension: String,
    jump_tension: String,
    airdash_tension: String,
    walk_speed: String,
    back_walk_speed: String,
    dash_initial_speed: String,
    dash_acceleration: String,
    dash_friction: String,
    jump_gravity: String,
    high_jump_gravity: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveInfo {
    input: String,
    name: String,
    damage: String,
    guard: String,
    startup: String,
    active: String,
    recovery: String,
    on_hit: String,
    on_block: String,
    level: String,
    counter: String,
    move_type: String,
    risc_gain: String,
    risc_loss: String,
    wall_damage: String,
    input_tension: String,
    chip_ratio: String,
    otg_ratio: String,
    scaling: String,
    invincibility: String,
    cancel: String,
    caption: String,
    notes: String,
    //hitbox_caption: String,
    //images: String,
    //hitboxes: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageLinks {
    input: String,
    move_img: String,
    hitbox_img: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveAliases {
    input: String,
    aliases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nicknames {
    character: String,
    nicknames: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Gids {
    id: Vec<String>,
}

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

const IMAGE_DEFAULT: &str =
    "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/no_image.png";
const HITBOX_DEFAULT: &str =
    "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/no_hitbox.png";
const EMBED_COLOR: (u8, u8, u8) = (140, 75, 64);

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!(
            "{}",
            ("Failed to start bot: ".to_owned() + &error.to_string() + ".").red()
        ),
        poise::FrameworkError::Command { error, ctx, .. } => {
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
            if let Err(e) = poise::builtins::on_error(error).await {
                println!(
                    "{}",
                    ("Error while handling error: ".to_owned() + &e.to_string() + ".").red()
                )
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Running initial checks
    println!();
    check::data_folder_exists(true).await;
    check::character_folders_exist(true).await;
    check::character_jsons_exist(true).await;
    check::character_images_exist(true).await;
    check::nicknames_json_exists(true).await;

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![
            frames::frames(),
            help::help(),
            hitboxes::hitboxes(),
            nicknames::nicknames(),
            moves::moves(),
            register::register(),
            update::update(),
        ],
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!(
                    "{}",
                    ("\nExecuting command ".to_owned() + &ctx.command().qualified_name + ".")
                        .cyan()
                );
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
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
        // Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                ctx.set_invocation_data(Instant::now()).await;
                Ok(true)
            })
        }),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        // On ready event start the task of auto updating
        // the character data every 24 hours
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                match event {
                    serenity::FullEvent::Ready { data_about_bot: _ } => {
                        let forever = task::spawn(async {
                            let mut interval = time::interval(Duration::from_secs(86400));
                            loop {
                                // Runs update_all_char_data every 24h
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
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    dotenv::dotenv().expect("Failed to load .env file.");
    let token = std::env::var("DISCORD_TOKEN").expect("Failed to load `DISCORD_TOKEN` env var.");
    let intents = serenity::GatewayIntents::non_privileged() /*| serenity::GatewayIntents::MESSAGE_CONTENT */;
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
