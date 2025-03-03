//! `help.rs`
//!
//! ヘルプコマンドモジュール  
//! Discordコマンド /help 実装モジュール  
//! ユーザーが指定したコマンドのヘルプ情報を表示する処理を提供  
//! 自動補完機能を使用し、ユーザーが入力したコマンド名に対応するヘルプメッセージを送信する  

// futures クレートから、Stream 型や拡張メソッド StreamExt をインポートする。
// これにより、非同期ストリームの操作が可能になる。
use crate::serenity::futures::{self, Stream, StreamExt};

// Discord コマンドの実行コンテキスト (Context) とエラー型 (AppError) を定義しているモジュールをインポート
use crate::{error::AppError, Context};

// colored クレートを利用して、コンソール出力に色付けするための拡張メソッドを使用する
use colored::Colorize;

/// ヘルプコマンドの自動補完候補を生成する非同期関数
///
/// # 引数
/// * `_` - コマンド実行時のコンテキスト（今回は使用していない）
/// * `partial` - ユーザーが入力した部分文字列
///
/// # 戻り値
/// ユーザーの入力にマッチする候補文字列の非同期ストリームを返す
async fn autocomplete_help<'a>(
    _: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    // ヘルプ候補の一覧を列挙したストリームを作成する
    futures::stream::iter(&[
        "general",
        "frames",
        "hitboxes",
        "fmeter",
        "moves",
        "nicknames",
        "notes",
        "specifics",
        "register",
        "update",
        "feedback",
    ])
    // ユーザー入力 (partial) にマッチする候補のみフィルタリングする
    .filter(move |name| {
        // すべて小文字に変換して比較（大文字小文字を区別しない）
        futures::future::ready(name.to_lowercase().contains(&partial.to_lowercase()))
    })
    // 各候補を String 型に変換して返す
    .map(|name| (*name).to_string())
}

/// ヘルプメッセージを表示するコマンド
///
/// ユーザーが指定したオプションに応じて、対応するヘルプメッセージを送信する。
/// オプションに該当するヘルプが存在しない場合は、エラーメッセージを出力する。
#[poise::command(prefix_command, slash_command, aliases("?"))]
pub async fn help(
    ctx: Context<'_>, // コマンド実行時のコンテキスト。ユーザー情報やチャンネル情報を含む。
    #[description = "Pick a command to display help for."]
    #[autocomplete = "autocomplete_help"] // オートコンプリートに先ほど定義した関数を使用
    option: String, // ユーザーが表示したいヘルプの対象コマンドを指定する文字列
) -> Result<(), AppError> {
    // コマンド実行時の引数を紫色でログ出力
    println!(
        "{}",
        ("Command Args: '".to_owned() + &option + "'").purple()
    );

    // ヘルプメッセージの一時格納用変数
    let help_message;

    // ユーザーの入力に応じて、対応するヘルプ関数を呼び出す
    match option.trim() {
        "feedback" => help_feedback(ctx).await,
        "fmeter" => help_fmeter(ctx).await,
        "frames" => help_frames(ctx).await,
        "general" => help_general(ctx).await,
        "hitboxes" => help_hitboxes(ctx).await,
        "moves" => help_moves(ctx).await,
        "nicknames" => help_nicknames(ctx).await,
        "notes" => help_notes(ctx).await,
        "register" => help_register(ctx).await,
        "specifics" => help_specifics(ctx).await,
        "update" => help_update(ctx).await,
        _ => {
            // 入力に該当するヘルプがない場合、エラーメッセージを生成
            help_message = "Help for `".to_owned() + &option + "` not found!";
            // Discord にエラーメッセージを送信
            ctx.say(&help_message).await?;
            // エラー内容を赤色でコンソール出力
            println!("{}", ("Error: ".to_owned() + &help_message).red());
            return Ok(());
        }
    }

    Ok(())
}

/// 一般的なヘルプメッセージを送信する関数
async fn help_general(ctx: Context<'_>) {
    // ヘルプメッセージ（Markdown フォーマット）を定義
    let help_msg = r#"
__**コマンドリスト**__
```frames``````
hitboxes``````
fmeter``````
moves``````
nicknames``````
feedback``````
help```

__**参考にさせてもらったソースコード:**__
__<https://github.com/yakiimoninja/baiken>__

"#;

    // Discord にヘルプメッセージを送信する
    let _ = ctx.say(help_msg).await;
}

/// フィードバック用ヘルプメッセージを送信する関数
async fn help_feedback(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/feedback`.

__**text**__: Any text. Cannot be empty.

Sends feedback or a request to the dev."#;

    // ヘルプメッセージ送信
    let _ = ctx.say(help_msg).await;
    // さらに、関連画像の URL を Discord チャンネルに送信する
    let _ = ctx
        .channel_id()
        .say(
            ctx,
            "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/feedback.png",
        )
        .await;
}

/// フレームメーター用ヘルプメッセージを送信する関数
async fn help_fmeter(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/fmeter`.
__**Example**__: `/fmeter cz super`.

__**character_arg**__: キャラクター名は空欄に出来ないよ！
__**character_move_arg**__: キャラクターのコマンド名や技名は空欄に出来ないよ！

キャラクターの技の始動フレーム、持続フレーム、後隙フレームを視覚的に表示するよ！"#;

    let _ = ctx.say(help_msg).await;
    // 関連画像の URL を送信
    let _ = ctx
        .channel_id()
        .say(
            ctx,
            "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/fmeter.png",
        )
        .await;
}

/// フレームデータ表示用ヘルプメッセージを送信する関数
async fn help_frames(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/frames`.
__**Example**__: `/frames baiken 236K`.

__**character_arg**__: Character name or nickname. Cannot be empty.
__**character_move_arg**__: Character move name, input or alias. Cannot be empty.

Displays the frame data of a move along with an image."#;

    let _ = ctx.say(help_msg).await;
    // 関連画像の URL を送信
    let _ = ctx
        .channel_id()
        .say(
            ctx,
            "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/frames.png",
        )
        .await;
}

/// ヒットボックス表示用ヘルプメッセージを送信する関数
async fn help_hitboxes(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/hitboxes`. 
__**Example**__: `/hitboxes goldlewis 426H`.

__**character_arg**__: Character name or nickname. Cannot be empty.
__**character_move_arg**__: Character move name, input or alias. Cannot be empty.

Displays the hitbox images of a character's move."#;

    let _ = ctx.say(help_msg).await;
    // ヒットボックス画像の URL を送信
    let _ = ctx
        .channel_id()
        .say(
            ctx,
            "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/hitboxes.png",
        )
        .await;
}

/// 技一覧表示用ヘルプメッセージを送信する関数
async fn help_moves(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/moves`.
__**Example**__: `/moves sol`.

__**character_arg**__: Character name or nickname. Cannot be empty.

Displays all the moves, inputs and move aliases of a character."#;

    let _ = ctx.say(help_msg).await;
    // 技一覧画像の URL を送信
    let _ = ctx
        .channel_id()
        .say(
            ctx,
            "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/moves.png",
        )
        .await;
}

/// キャラクター愛称表示用ヘルプメッセージを送信する関数
async fn help_nicknames(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/nicknames`.

Displays all the nicknames for each character."#;

    let _ = ctx.say(help_msg).await;
    // 愛称一覧画像の URL を送信
    let _ = ctx
        .channel_id()
        .say(
            ctx,
            "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/nicknames.png",
        )
        .await;
}

/// 使用上の注意点を説明するヘルプメッセージを送信する関数
async fn help_notes(ctx: Context<'_>) {
    let help_msg = r#"
__**Usage notes.**__

- **`/` commands can be substituted with direct mentions if preferred.**
    - Doing so will enable the use of shorthand commands.
        - Example: `@Baiken f sol 2k` same as `/frames sol 2k`.
        - Example: `@Baiken h ky 6p` same as `/hitboxes ky 6p`.
        - Example: `@Baiken m leo` same as `/moves leo`.
        - Example: `@Baiken a chipp` same as `/aliases chipp`.

- **All searching is case insensitive.**
    - All names, nicknames, moves and aliases are case agnostic.
    - Example: `/hitboxes ky dp` = `/hitboxes KY dP`.

- **Character searching.**
    - Characters can be found either using a part of their name, or any of their nicknames.
    - Example: `/frames Happy Chaos cs` = `/frames happy cs` = `/frames hc cs`.

- **Move searching.**
    - Moves can be found either using a part of their name, their input, or any of the aliases that exist.
        - Example: `/frames Anji Needles` = `/frames Anji 236HP` = `/frames Anji ichi`.
    - Charged moves can be found with or without the use of `[]`.
        - Example `/frames may 46S` = `/frames may [4]6S`.
    - For a fully charged dust attack the alias `5D!` can be used instead.
        - Example: `/frames chipp 5D!`."#;

    let _ = ctx.say(help_msg).await;
}

/// コマンド登録用ヘルプメッセージを送信する関数
async fn help_register(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/register`.

**This command only works for owners.**
Registers or removes all slash commands in the current server or every server the bot is in."#;

    let _ = ctx.say(help_msg).await;
    // 登録画面の画像 URL を送信
    let _ = ctx
        .channel_id()
        .say(
            ctx,
            "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/register.png",
        )
        .await;
}

/// キャラクター固有の仕様について説明するヘルプメッセージを送信する関数
async fn help_specifics(ctx: Context<'_>) {
    let help_msg = r#"
__**Character specifics.**__

- **For normals that have levels like Nagoriyuki.**
  - Add the level number next to the normal.
  - For Level 1 `fS`: `/frames nago fs`. 
  - For Level 2 `fS`: `/frames nago fs2`.
  - For Level 3 `fS`: `/frames nago fs3`.
  - If it's a level 1 normal nothing needs to be added since it's the default state.

- **For specials that have levels like Goldlewis.**
  - Add the level number next to the special.
  - For Level 1 `Thunderbird`: `/frames gold Drone`.
  - For Level 2 `Thunderbird`: `/frames gold Drone 2`.
  - For Level 3 `Thunderbird`: `/frames gold Drone 3`.
  - The above is not always the case depending on the special move and alias used.
  - For Level 1 `Thunderbird`: `/frames gold D1`.
  - For Level 2 `Thunderbird`: `/frames gold D2`.
  - For Level 3 `Thunderbird`: `/frames gold D3`.
  - See `/aliases gold` for more info on his aliases.

- **For Testament's different Grave Reaper versions use as shown.**
  - Regular version: `/frames testament 236S`.
  - Partially charged version: `/frames testament 236S!`.
  - Fully charged version: `/frames testament 236S!!`.
"#;

    let _ = ctx.say(help_msg).await;
}

/// 更新コマンド用ヘルプメッセージを送信する関数
async fn help_update(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**: `/update`.
__**Example**__: `/update frames all`.

__**frames_or_images**__: `frames`, `images` or `all`. Cannot be empty.
__**character_arg**__: Character name or nickname. Cannot be empty.

**This command only works for owners.**
Meaning that it requires an instance of the source code to use it.
Updates the frame data and or image links for all or a specific character according to dustloop."#;

    let _ = ctx.say(help_msg).await;
    // 更新画面の画像 URL を送信
    let _ = ctx
        .channel_id()
        .say(
            ctx,
            "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/update.png",
        )
        .await;
}
