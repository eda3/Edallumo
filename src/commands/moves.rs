//! `moves.rs`
//!
//! このファイルは、キャラクターの技、入力、エイリアス情報を表示するためのコマンド処理を実装する。
//! キャラクター名および技情報の取得、整形、埋め込みメッセージ生成を行う。

mod utils; // ユーティリティ関数群
use crate::{check, error::AppError, find, Context, MoveAliases, MoveInfo, EMBED_COLOR}; // 必要な型・関数群
use colored::Colorize; // 文字色変換用
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter}; // 埋め込み生成用
use std::{fs, string::String}; // ファイル操作・文字列操作用
use utils::{get_normal_moves, get_special_moves, get_super_moves}; // ユーティリティ関数取得

/// ムーブタイプ選択列挙体
#[derive(Debug, poise::ChoiceParameter)]
pub enum TypeChoice {
    #[name = "all"]
    All, // 全て選択
    #[name = "normals"]
    Normals, // 通常技選択
    #[name = "specials"]
    Specials, // スペシャル技選択
    #[name = "supers"]
    Supers, // 必殺技選択
}

/// キャラクターデータを読み込む関数
///
/// キャラクター名を受け取り、必要なデータファイルを読み込んでパースします
///
/// # 引数
/// * `character` - ユーザーが入力したキャラクター名
/// * `ctx` - コマンドコンテキスト
///
/// # 戻り値
/// 成功時は `(キャラクター名, 技情報, エイリアス情報)` のタプル、失敗時は `AppError`
async fn load_character_data(
    character: &str,
    ctx: &Context<'_>,
) -> Result<(String, Vec<MoveInfo>, Vec<MoveAliases>), AppError> {
    // キャラクター探索処理　find関数呼出
    let character_arg_altered = match find::find_character(&character.to_string()).await {
        Ok(character_arg_altered) => character_arg_altered, // キャラクター名称取得
        Err(err) => {
            ctx.say(err.to_string()).await?; // エラーメッセージ送信
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red()); // エラー出力
            return Err(AppError::CharacterNotFound(err.to_string())); // エラー時早期終了
        }
    };

    // キャラクターファイルパス生成　文字列結合
    let char_file_path =
        "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    // キャラクターファイル読み込み　ファイル読み出し
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + character + ".json" + "' file."));

    // キャラクター情報デシリアライズ　JSONパース
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    println!(
        "{}",
        ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green() // 成功出力
    );

    // エイリアスファイルパス生成　文字列結合
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
    // エイリアスファイル読み込み　ファイル読み出し
    let aliases_data = fs::read_to_string(&aliases_path)
        .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));

    // エイリアス情報デシリアライズ　JSONパース
    let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

    Ok((character_arg_altered, moves_info, aliases_data))
}

/// 指定された技種別に対する埋め込みメッセージを作成する関数
///
/// # 引数
/// * `category` - 技の種別
/// * `moves_info` - 技情報
/// * `aliases_data` - エイリアス情報
/// * `character_arg_altered` - キャラクター名
///
/// # 戻り値
/// 埋め込みメッセージのベクター
async fn create_embeds_for_move_type(
    category: &TypeChoice,
    moves_info: &[MoveInfo],
    aliases_data: &[MoveAliases],
    character_arg_altered: &str,
) -> Vec<CreateEmbed> {
    let mut vec_embeds = Vec::new(); // 埋め込みメッセージ群格納用ベクター

    // 埋め込みタイトル生成　キャラクター名表示
    let embed_title =
        "__**".to_owned() + &character_arg_altered.replace('_', " ") + " Moves / Aliases**__";
    // 埋め込みURL生成　Dustloop Wiki URL構築
    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + character_arg_altered + "#Overview";
    // 埋め込みフッター生成　補足メッセージ
    let embed_footer = CreateEmbedFooter::new(
        "Try the \"/help notes\" command for usage notes and specifics.\nOr \"/report\" to request a new aliases."
    );

    // 技種別に応じた処理分岐
    match category {
        TypeChoice::All => {
            // 通常技取得　ユーティリティ関数呼出
            let normal_moves = get_normal_moves(moves_info, aliases_data).await;
            // スペシャル技取得　ユーティリティ関数呼出
            let special_moves = get_special_moves(moves_info, aliases_data).await;
            // 必殺技取得　ユーティリティ関数呼出
            let super_moves = get_super_moves(moves_info, aliases_data).await;

            // 通常技埋め込み作成　CreateEmbed呼出
            let normals_embed = CreateEmbed::new()
                .color(EMBED_COLOR) // 埋め込み色設定
                .title(embed_title.clone()) // タイトル設定
                .url(embed_url.clone()) // URL設定
                .description(normal_moves); // 説明文設定

            // スペシャル技埋め込み作成
            let specials_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .description(special_moves); // 説明文設定

            // 必殺技埋め込み作成　フッター追加
            let supers_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .description(super_moves)
                .footer(embed_footer);

            vec_embeds.push(normals_embed); // 埋め込み追加
            vec_embeds.push(specials_embed); // 埋め込み追加
            vec_embeds.push(supers_embed); // 埋め込み追加
        }
        TypeChoice::Normals => {
            // 通常技取得　ユーティリティ関数呼出
            let normal_moves = get_normal_moves(moves_info, aliases_data).await;

            let normals_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .title(embed_title)
                .url(embed_url)
                .description(normal_moves)
                .footer(embed_footer);

            vec_embeds.push(normals_embed); // 埋め込み追加
        }
        TypeChoice::Specials => {
            // スペシャル技取得　ユーティリティ関数呼出
            let special_moves = get_special_moves(moves_info, aliases_data).await;

            let specials_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .title(embed_title)
                .url(embed_url)
                .description(special_moves)
                .footer(embed_footer);

            vec_embeds.push(specials_embed); // 埋め込み追加
        }
        TypeChoice::Supers => {
            // 必殺技取得　ユーティリティ関数呼出
            let super_moves = get_super_moves(moves_info, aliases_data).await;

            let supers_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .title(embed_title)
                .url(embed_url)
                .description(super_moves)
                .footer(embed_footer);

            vec_embeds.push(supers_embed); // 埋め込み追加
        }
    };

    vec_embeds
}

/// 技一覧コマンド
///
/// 指定したキャラクターの技一覧とエイリアスを表示する
///
/// # 引数
/// * `ctx` - コマンド実行コンテキスト
/// * `character` - キャラクター指定文字列
/// * `category` - 技種別選択（"all", "normals", "specials", "supers"）
/// # 戻り値
/// `Result<(), Error>` を返す
#[poise::command(prefix_command, slash_command)]
pub async fn moves(
    ctx: Context<'_>, // コマンド実行コンテキスト
    #[min_length = 2]
    #[description = "Character name or nickname."] // キャラクター名またはニックネーム
    character: String, // キャラクター指定文字列
    #[rename = "type"]
    #[description = "Move type."] // 技種別指定
    category: TypeChoice, // 技種別選択値
) -> Result<(), AppError> {
    println!(
        "{}",
        ("Command Args: '".to_owned() + &character + "'").purple() // 引数出力
    );

    // 入力チェック実施　条件確認
    if (check::adaptive_check(
        ctx,
        check::CheckOptions::DATA_FOLDER
            | check::CheckOptions::NICKNAMES_JSON
            | check::CheckOptions::CHARACTER_FOLDERS
            | check::CheckOptions::CHARACTER_JSONS,
    )
    .await)
        .is_err()
    {
        return Ok(()); // チェック失敗時早期終了
    }

    // キャラクターデータ読み込み
    let Ok((character_arg_altered, moves_info, aliases_data)) =
        load_character_data(&character, &ctx).await
    else {
        return Ok(());
    };

    // 埋め込みメッセージ作成
    let vec_embeds = create_embeds_for_move_type(
        &category,
        &moves_info,
        &aliases_data,
        &character_arg_altered,
    )
    .await;

    // 返信オブジェクト初期化　CreateReply生成
    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds); // 埋め込みメッセージ追加

    ctx.send(reply).await?; // 返信送信

    Ok(()) // 正常終了返却
}
