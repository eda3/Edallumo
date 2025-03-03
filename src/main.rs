//! `main.rs`
//!
//! このファイルは、Discord Bot の起動基盤および各種機能モジュールの読み込み・初期設定を行う。
//! poise フレームワークを利用し、コマンドの登録、エラーハンドリング、定期更新タスクなどを構成する。

// モジュール読み込み
mod async_utils; // 非同期処理ユーティリティ
mod check; // 初期チェック機能
mod commands; // コマンド群実装
mod common; // 共通処理群
mod error; // エラー処理
mod find; // 情報検索機能
mod models; // データモデル
mod test_utils; // テスト用ユーティリティ
mod utils; // 共通ユーティリティ関数

// Re-export important modules and types
pub use error::{AppError, Result};
pub use find::Nicknames;
pub use models::{CharInfo, GuardType, MoveAliases, MoveInfo};

// 外部クレート読み込み
use colored::Colorize; // 文字色変換用
use commands::{feedback, frames, help, hitboxes, moves, nicknames, register, update};
#[allow(unused_imports)]
use poise::serenity_prelude as serenity; // Serenity 用エイリアス

/// コンテキスト型定義
///
/// # 型説明
/// * `Context<'a>` - poise コマンド実行時に渡されるコンテキスト型
pub type Context<'a> = poise::Context<'a, Data, AppError>;

/// 各コマンドに共通して渡されるユーザーデータ
#[derive(Debug, Clone)]
pub struct Data {
    /// データディレクトリのパス
    pub data_dir: String,
}

/// 画像リンク構造体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ImageLinks {
    pub input: String,           // 入力コマンド
    pub move_img: String,        // 技画像リンク
    pub hitbox_img: Vec<String>, // ヒットボックス画像リンク群
}

/// サーバーID情報構造体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Gids {
    id: Vec<String>, // 識別子群
}

/// 埋め込みメッセージのカラーコード
pub const EMBED_COLOR: u32 = 0x00FF_FF99;

/// キャラクター名定数配列
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

/// エラーハンドリング関数
///
/// コマンド実行中に発生したエラーを処理し、適切なメッセージを表示する。
///
/// # 引数
/// * `error` - 発生したエラー
///
/// # 戻り値
/// `Result<(), AppError>` - エラー処理の結果
async fn on_error(error: poise::FrameworkError<'_, Data, AppError>) -> Result<()> {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            eprintln!(
                "{}",
                format!("フレームワークのセットアップエラー: {error}").red()
            );
            Err(AppError::Discord(format!("セットアップエラー: {error}")))
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            eprintln!("{}", format!("コマンド実行エラー: {error}").red());
            let embed = poise::CreateReply::default().embed(
                serenity::CreateEmbed::new()
                    .title("エラーが発生しました")
                    .description(format!("```{error:?}```"))
                    .color(0x00FF_0000),
            );
            if let Err(e) = ctx.send(embed).await {
                eprintln!("{}", format!("エラーメッセージ送信失敗: {e}").red());
            }
            Ok(())
        }
        poise::FrameworkError::CommandPanic { payload, ctx, .. } => {
            eprintln!("{}", format!("コマンドパニック: {payload:?}").red());
            let embed = poise::CreateReply::default().embed(
                serenity::CreateEmbed::new()
                    .title("内部エラーが発生しました")
                    .description(
                        "開発者に問題が報告されました。ご迷惑をおかけして申し訳ありません。",
                    )
                    .color(0x00FF_0000),
            );
            if let Err(e) = ctx.send(embed).await {
                eprintln!("{}", format!("エラーメッセージ送信失敗: {e}").red());
            }
            Ok(())
        }
        _ => {
            eprintln!("{}", format!("その他のエラー: {error}").red());
            Ok(())
        }
    }
}

/// メイン関数
///
/// アプリケーションのエントリーポイント。
/// 環境変数の読み込み、Botの初期化、コマンド登録、イベントハンドラの設定などを行う。
#[tokio::main]
async fn main() -> Result<()> {
    // .envファイルを読み込む
    dotenv::dotenv().map_err(|e| {
        eprintln!(
            "{}",
            format!(".envファイルの読み込みに失敗しました: {e}").red()
        );
        AppError::Config(format!(".envファイルの読み込みに失敗しました: {e}"))
    })?;

    // 環境変数からトークンを読み込む（.envから設定された変数）
    let token = std::env::var("DISCORD_TOKEN").map_err(|_| {
        eprintln!("{}", "環境変数 DISCORD_TOKEN が見つかりません".red());
        AppError::Config("環境変数 DISCORD_TOKEN が見つかりません".to_string())
    })?;

    // データディレクトリのパスを設定
    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| {
        println!(
            "{}",
            "環境変数 DATA_DIR が見つからないため、デフォルト値「data」を使用します".yellow()
        );
        "data".to_string()
    });

    // ユーザーデータを初期化
    let user_data = Data {
        data_dir: data_dir.clone(),
    };

    // 初期化時の確認
    if let Err(e) = check::validate_data_dir(&data_dir) {
        eprintln!(
            "{}",
            format!("エラー: データディレクトリの確認に失敗しました: {e}").red()
        );
        return Err(AppError::Config(format!(
            "データディレクトリの確認に失敗しました: {e}"
        )));
    }

    // フレームワークの設定
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                help::help(),
                frames::frames(),
                hitboxes::hitboxes(),
                moves::moves(),
                nicknames::nicknames(),
                feedback::feedback(),
                update::update(),
                register::register(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                additional_prefixes: vec![poise::Prefix::Literal("！")],
                mention_as_prefix: true,
                ..Default::default()
            },
            // エラーハンドラ
            on_error: |error| {
                Box::pin(async move {
                    if let Err(e) = on_error(error).await {
                        eprintln!(
                            "{}",
                            format!("エラー処理中にエラーが発生しました: {e}").red()
                        );
                    }
                })
            },
            // イベントハンドラ
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(async move {
                    if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
                        println!(
                            "{}",
                            format!("ログイン: {name}", name = data_about_bot.user.name).green()
                        );
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(user_data)
            })
        })
        .build();

    // Botを実行
    let mut client =
        serenity::ClientBuilder::new(token, serenity::GatewayIntents::non_privileged())
            .framework(framework)
            .await
            .map_err(|e| AppError::Discord(e.to_string()))?;

    // クライアントを起動
    client
        .start()
        .await
        .map_err(|e| AppError::Discord(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_imagelinks_serialization() {
        let image_links = ImageLinks {
            input: "5P".to_string(),
            move_img: "http://example.com/5p.png".to_string(),
            hitbox_img: vec![
                "http://example.com/5p_hitbox1.png".to_string(),
                "http://example.com/5p_hitbox2.png".to_string(),
            ],
        };

        let serialized = serde_json::to_string(&image_links).unwrap();
        let deserialized: ImageLinks = serde_json::from_str(&serialized).unwrap();

        assert_eq!(image_links.input, deserialized.input);
        assert_eq!(image_links.move_img, deserialized.move_img);
        assert_eq!(image_links.hitbox_img, deserialized.hitbox_img);
    }

    #[test]
    fn test_chars_constant() {
        // これは単純に配列の長さが正しいことを確認するテスト
        assert_eq!(CHARS.len(), 29);
        // キャラクター名が正しく含まれていることを確認
        assert!(CHARS.contains(&"Sol_Badguy"));
        assert!(CHARS.contains(&"Ky_Kiske"));
        assert!(CHARS.contains(&"May"));
    }

    #[test]
    fn test_embed_color() {
        // カラーコードが正しいことを確認
        assert_eq!(EMBED_COLOR, 0x00FF_FF99);
    }
}
