//! advanced.rs
//!
//! このファイルは、キャラクターの技データの詳細情報をDiscordの埋め込みメッセージで表示するコマンドを実装。
//! 指定されたキャラクター名（または愛称）と技名（入力またはエイリアス）をもとに、
//! JSONファイルから該当データを取得し、画像リンクや各種技パラメータを整形して表示する。

use crate::{check, find, Context, Error, ImageLinks, MoveInfo, EMBED_COLOR, IMAGE_DEFAULT};
use colored::Colorize;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use std::{fs, string::String};

/// 指定されたキャラクターの技データを詳細に表示する非同期コマンド
///
/// # 概要
/// 入力されたキャラクター名（または愛称）と技名（入力またはエイリアス）から、
/// キャラクター情報ファイルおよび画像リンクファイルを読み込み、  
/// 対象の技データを抽出し、Discord用の埋め込みメッセージとして表示する。
///
/// # 引数
/// * `ctx` - コマンド実行コンテキスト（Discordとの通信に利用）
/// * `character` - キャラクター名または愛称（2文字以上必須）
/// * `character_move` - 技名、入力、またはエイリアス（2文字以上必須）
///
/// # 戻り値
/// 正常終了時は `Ok(())` を返し、エラー発生時は `Err(Error)` を返す。
///
/// # 例
/// ```rust,no_run
/// # async fn example(ctx: Context<'_>) -> Result<(), Error> {
/// advanced(ctx, "Sol".to_string(), "236H".to_string()).await?;
/// # Ok(()) }
/// ```
///
/// # 注意
/// この関数は非同期関数であるため、呼び出し時に `.await` が必要。
#[poise::command(prefix_command, slash_command)]
pub async fn advanced(
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = "キャラクター名または愛称"]
    character: String,
    #[min_length = 2]
    #[rename = "move"]
    #[description = "技名、入力、またはエイリアス"]
    character_move: String,
) -> Result<(), Error> {
    // コマンド引数の表示　引数確認用
    println!(
        "{}",
        ("Command Args: '".to_owned() + &character + ", " + &character_move + "'").purple()
    );

    // 埋め込み画像用変数の初期化　初期値はデフォルト画像URL
    let mut embed_image = IMAGE_DEFAULT.to_string();

    // 入力チェックの実施　各種前提条件チェック
    if (check::adaptive_check(ctx, true, true, true, true, true, false, true).await).is_err() {
        return Ok(());
    }

    // キャラクター名の正規化　入力に基づく正式名称の取得
    let character_arg_altered = match find::find_character(&character).await {
        Ok(character_arg_altered) => character_arg_altered, // 正式名称取得
        Err(err) => {
            ctx.say(err.to_string()).await?; // エラーメッセージ送信
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(()); // 処理中断
        }
    };

    // JSONファイルパスの組み立て　キャラクター情報ファイルのパス生成
    let char_file_path =
        "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    // JSONファイルの読み込み　キャラクター情報の取得
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character_arg_altered + ".json" + "' file."));

    // JSON文字列のデシリアライズ　技情報の構造体へ変換
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    // JSON読み込み成功の表示　確認メッセージ出力
    println!(
        "{}",
        ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green()
    );

    // 画像リンク用JSONファイルのパス組み立て　画像情報ファイルの指定
    let image_links = fs::read_to_string(
        "data/".to_owned() + &character_arg_altered + "/images.json",
    )
    .expect(
        &("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."),
    );

    // 画像リンクJSONのデシリアライズ　画像リンク情報の構造体へ変換
    let image_links = serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();
    // 対象技情報の取得　入力に対応する技データの抽出
    let move_info = &moves_info
        [find::find_move_index(&character_arg_altered, character_move, &moves_info).await?];

    // 対象技の読み込み成功の表示　確認メッセージ出力
    println!(
        "{}",
        ("Successfully read move '".to_owned()
            + &move_info.input.to_string()
            + "' in '"
            + &character_arg_altered
            + ".json' file.")
            .green()
    );

    // 画像リンクの探索　対象技の画像リンクを検索
    for img_links in image_links {
        // 対象技の入力と画像情報の入力が一致し、画像リンクが存在する場合
        if move_info.input == img_links.input && !img_links.move_img.is_empty() {
            embed_image = img_links.move_img.to_string(); // 画像リンク更新
            break; // 探索終了
        }
    }

    // 埋め込みメッセージ群生成用ベクターの初期化
    let mut vec_embeds = Vec::new();
    // 埋め込みタイトルの作成　キャラクター名と技名を組み合わせたタイトル
    let embed_title = "__**".to_owned() + &move_info.input + "**__";
    // 埋め込みURLの作成　Dustloop Wiki のキャラクター概要ページURL生成
    let embed_url =
        "https://dustloop.com/w/GGST/".to_owned() + &character_arg_altered + "#Overview";
    // 埋め込みフッターの作成　技に関するキャプションを利用
    let embed_footer = CreateEmbedFooter::new(&move_info.caption);

    // 埋め込みメッセージの生成　技データの各パラメータをフィールドとして追加
    let embed = CreateEmbed::new()
        .color(EMBED_COLOR) // 埋め込みカラー設定
        .title(&embed_title) // タイトル設定
        .url(&embed_url) // URL設定
        .image(&embed_image) // 画像リンク設定
        .fields(vec![
            ("ダメージ", &move_info.damage.to_string(), true),
            ("ガード", &move_info.guard.to_string(), true),
            ("無敵", &move_info.invincibility.to_string(), true),
            ("始動", &move_info.startup.to_string(), true),
            ("持続", &move_info.active.to_string(), true),
            ("硬直", &move_info.recovery.to_string(), true),
            ("ヒット時", &move_info.on_hit.to_string(), true),
            ("ガード時", &move_info.on_block.to_string(), true),
            ("カウンター", &move_info.counter.to_string(), true),
            ("技レベル", &move_info.level.to_string(), true),
            ("リスク増加", &move_info.risc_gain.to_string(), true),
            ("リスク減少", &move_info.risc_loss.to_string(), true),
            ("ダメージ倍率", &move_info.scaling.to_string(), true),
            ("壁割ダメージ", &move_info.wall_damage.to_string(), true),
            (
                "テンションゲージ",
                &move_info.input_tension.to_string(),
                true,
            ),
            ("削り比率", &move_info.chip_ratio.to_string(), true),
            ("ダウン追い打ち比率", &move_info.otg_ratio.to_string(), true),
            ("キャンセル先", &move_info.cancel.to_string(), true),
        ])
        .footer(embed_footer); // フッター設定

    // 生成した埋め込みメッセージをベクターに追加
    vec_embeds.push(embed);

    // 備考（notes）が存在する場合、別の埋め込みメッセージを生成
    if !&move_info.notes.is_empty() {
        let embed2 = CreateEmbed::new()
            .color(EMBED_COLOR) // 埋め込みカラー設定
            .description(&move_info.notes); // 備考記述設定
        vec_embeds.push(embed2); // ベクターに追加
    }

    // 返信メッセージ用オブジェクト生成　送信用オブジェクトの初期化
    let mut reply = poise::CreateReply::default();
    // 生成した埋め込みメッセージ群を返信オブジェクトに追加
    reply.embeds.extend(vec_embeds);
    // 返信メッセージの送信　Discordへ送信
    ctx.send(reply).await?;

    // 正常終了の返却　処理完了
    Ok(())
}
