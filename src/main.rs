// 各種モジュールをインポート（別ファイルに分割された機能群を読み込む）
mod check; // ディレクトリやファイルの存在チェックなどを行うモジュール
mod commands; // 各種Discordコマンドの実装が含まれるモジュール
mod find; // 特定の情報を検索するための機能を提供するモジュール

// 外部クレートのインポート
use colored::Colorize; // ターミナル出力に色付けを行うためのクレート
use commands::*; // commandsモジュール内の全ての項目をインポート
use poise::serenity_prelude as serenity; // poise内のserenity関連の型や関数を利用するためのエイリアス
use serde::{Deserialize, Serialize}; // JSON等のシリアライズ/デシリアライズ用に必要な機能
use std::time::Duration; // 時間計測やタイムアウト設定に使用

// 全てのコマンド関数で使用する型の定義
type Error = Box<dyn std::error::Error + Send + Sync>; // 送信可能かつ同期可能な汎用エラー型
type Context<'a> = poise::Context<'a, Data, Error>; // poiseフレームワーク用のコンテキスト型

// 全てのコマンド関数に渡されるカスタムユーザーデータ
pub struct Data {
    // 必要に応じてフィールドを追加可能
}

// キャラクター情報を保持するための構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct CharInfo {
    page: String, // キャラクターに関する詳細情報が記載されたページのURLや識別子
}

// 各技（ムーブ）の詳細情報を保持する構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct MoveInfo {
    input: String,         // 入力コマンド（例：コマンド入力文字列）
    name: String,          // 技の名称
    damage: String,        // ダメージ量
    guard: String,         // ガード時の情報
    startup: String,       // 技の発生（スタートアップ）フレーム
    active: String,        // 技が当たり判定を持つ（アクティブ）フレーム
    recovery: String,      // 技のリカバリーフレーム
    hit: String,           // ヒット時の効果やダメージ
    block: String,         // ブロック時の効果やダメージ
    level: String,         // 技のレベルやランク
    counter: String,       // カウンター対応情報
    scaling: String,       // ダメージのスケーリング情報
    riscgain: String,      // リスクゲインに関する情報
    invincibility: String, // 無敵フレームの情報
}

// 技の画像リンクやヒットボックス画像のリンク情報を保持する構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageLinks {
    input: String,           // 技の入力情報（識別キーとして利用）
    move_img: String,        // 技の画像URL
    hitbox_img: Vec<String>, // 複数のヒットボックス画像のURLを格納するベクター
}

// 技の別名（エイリアス）情報を保持する構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct MoveAliases {
    input: String,        // 技の入力情報（識別キー）
    aliases: Vec<String>, // 別名のリスト
}

// キャラクターの愛称（ニックネーム）情報を保持する構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct Nicknames {
    character: String,      // キャラクター名
    nicknames: Vec<String>, // 複数の愛称を保持するベクター
}

// 28体のキャラクター名を定数として定義
pub const CHARS: [&str; 28] = [
    "A.B.A",
    "Anji_Mito",
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

// 画像が存在しない場合のデフォルトURL
const IMAGE_DEFAULT: &str =
    "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/no_image.png";
// ヒットボックス画像が存在しない場合のデフォルトURL
const HITBOX_DEFAULT: &str =
    "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/no_hitbox.png";

// 非同期関数として定義されたエラーハンドラー
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // 発生したエラーに対して、カスタムなエラー処理を実施
    match error {
        // Botのセットアップ中にエラーが発生した場合
        poise::FrameworkError::Setup { error, .. } => panic!(
            "{}",
            // エラーメッセージを赤色で出力し、プログラムを強制終了
            ("Failed to start bot: ".to_owned() + &error.to_string() + ".").red()
        ),
        // コマンド実行時にエラーが発生した場合
        poise::FrameworkError::Command { error, ctx } => {
            println!(
                "{}",
                // コマンド名とエラー内容を赤色で出力
                ("Error in command `".to_owned()
                    + &ctx.command().name
                    + "`: "
                    + &error.to_string()
                    + ".")
                    .red()
            );
        }
        // その他のエラーに対して、poiseの標準エラーハンドラーを利用
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

// Tokioの非同期ランタイムを利用してメイン関数を定義
#[tokio::main]
async fn main() {
    println!("メイン！");

    // Bot起動前の初期チェック（必要なフォルダやファイルが存在するかを確認）
    println!();
    check::data_folder_exists(true).await;
    check::nicknames_json_exists(true).await;
    check::character_folders_exist(true).await;
    check::character_jsons_exist(true).await;
    check::character_images_exist(true).await;

    // poiseフレームワークのオプションを設定
    let options = poise::FrameworkOptions {
        // 利用するコマンド群をベクターで指定（各モジュールの関数を呼び出して登録）
        commands: vec![
            feedback::feedback(),
            fmeter::fmeter(),
            frames::frames(),
            help::help(),
            hitboxes::hitboxes(),
            nicknames::nicknames(),
            moves::moves(),
            register::register(),
            update::update(),
        ],
        // コマンドの呼び出しに利用するプレフィックスの設定
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()), // メインのプレフィックスは "!"
            // コマンド編集の有効期限を1時間に設定
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            // 追加のプレフィックスとして "b." も利用可能
            additional_prefixes: vec![poise::Prefix::Literal("b.")],
            ..Default::default()
        },
        // 全体のエラーハンドラとして先ほど定義した on_error を指定
        on_error: |error| Box::pin(on_error(error)),
        // 各コマンド実行前に実行されるフック
        pre_command: |ctx| {
            Box::pin(async move {
                println!(
                    "{}",
                    // コマンド開始時のログをシアン色で出力
                    ("\nExecuting command ".to_owned() + &ctx.command().qualified_name + "...")
                        .cyan()
                );
            })
        },
        // 各コマンド実行後（正常終了時）に実行されるフック
        post_command: |ctx| {
            Box::pin(async move {
                println!(
                    "{}",
                    // コマンド完了時のログをシアン色で出力
                    ("Executed command ".to_owned() + &ctx.command().qualified_name + "!").cyan()
                );
            })
        },
        // 各コマンド実行前に実行されるチェック関数
        // 特定のユーザーID（ここでは例として123456789）を拒否する処理
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false); // このユーザーの場合、コマンドの実行を中止
                }
                Ok(true) // それ以外は正常に実行
            })
        }),
        // 以下はデバッグ用リスナーの例（コメントアウトされている）
        // listener: |_ctx, event, _framework, _data| {
        //     Box::pin(async move {
        //         println!("Got an event in listener: {:?}", event.name());
        //         Ok(())
        //     })
        // },
        ..Default::default()
    };

    // .envファイルから環境変数を読み込む
    dotenv::dotenv().expect("Failed to load .env file.");

    // poiseフレームワークのbuilderを利用してBotを起動
    poise::Framework::builder()
        // DiscordのBotトークンを環境変数から取得
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN."))
        // Bot起動時のセットアップ処理（ここでは空のData構造体を返す）
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {})
            })
        })
        .options(options) // 先ほど定義したオプションを設定
        .intents(
            // 非特権のゲートウェイインテントを指定（必要に応じて変更可能）
            serenity::GatewayIntents::non_privileged() /*| serenity::GatewayIntents::MESSAGE_CONTENT*/,
        )
        .run()    // Botを実行開始
        .await
        .unwrap(); // エラーが発生した場合はパニックさせる
}
