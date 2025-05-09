//! simple.rs  
//!
//! シンプル表示コマンド実装ファイル  
//! キャラクターの技のフレームデータなどの情報を読み取り、  
//! 画像付きの埋め込みメッセージとして表示する。
//!
//! # 概要
//! ユーザーからのキャラクター名と技名（またはエイリアス）を受け取り、  
//! JSONファイルから対応するデータを取得し、  
//! Discordの埋め込みメッセージとして出力する。
//!
//! # 注意
//! コマンド実行前に必要なデータファイル（dataフォルダ内のJSONファイル）が存在していること。

use crate::{check, error::AppError, find, Context, ImageLinks, MoveInfo, EMBED_COLOR};
use colored::Colorize;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use std::{fs, string::String};

/// デフォルト画像URL
const IMAGE_DEFAULT: &str = "https://www.dustloop.com/wiki/images/5/54/GGST_Logo_Sparkly.png";

/// キャラクターデータを読み込む関数
///
/// # 引数
/// * `character` - ユーザーが入力したキャラクター名
/// * `ctx` - コマンドコンテキスト
///
/// # 戻り値
/// 成功時は正式なキャラクター名、失敗時はエラー
async fn load_character_data(character: &str, ctx: &Context<'_>) -> Result<String, AppError> {
    // キャラクター検索　完全名取得
    let character_arg_altered = match find::find_character(&character.to_string()).await {
        Ok(name) => name, // キャラクター正式名取得
        Err(err) => {
            // エラー表示　メッセージ送信
            ctx.say(err.to_string()).await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Err(AppError::CharacterNotFound(err.to_string()));
        }
    };

    Ok(character_arg_altered)
}

/// 入力文字列から括弧を除去した技名と括弧内のコンテンツを抽出する
///
/// # 引数
/// * `input` - 入力文字列（例：「2HS(2HS)」）
///
/// # 戻り値
/// (括弧を除去した技名, 括弧内のコンテンツ) のタプル
fn extract_move_name_parts(input: &str) -> (String, String) {
    // 括弧を除去した技名を作成（例：「2HS(2HS)」→「2HS」）
    let cleaned_input = if input.contains('(') {
        input.split('(').next().unwrap_or("").trim().to_string()
    } else {
        input.to_string()
    };

    // 括弧内のコマンドを取得（例：「2HS(2HS)」→「2HS」）
    let bracket_content = if input.contains('(') && input.contains(')') {
        let start = input.find('(').unwrap_or(0) + 1;
        let end = input.find(')').unwrap_or(input.len());
        if start < end {
            input[start..end].to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    (cleaned_input, bracket_content)
}

/// 技情報と画像リンクを照合して適切な画像URLを見つける
///
/// # 引数
/// * `move_data` - 技情報
/// * `image_links` - 画像リンク情報の配列
/// * `default_image` - デフォルト画像URL
///
/// # 戻り値
/// 画像のURL（見つからない場合はデフォルト画像）
fn find_matching_image(
    move_data: &MoveInfo,
    image_links: &[ImageLinks],
    default_image: &str,
) -> String {
    let mut embed_image = default_image.to_string();

    // 括弧除去と括弧内コンテンツの抽出
    let (cleaned_input, bracket_content) = extract_move_name_parts(&move_data.input);

    println!(
        "デバッグ - 検索する技: '{}'（大文字小文字変換: '{}'）",
        move_data.input,
        move_data.input.to_lowercase()
    );
    println!("デバッグ - 括弧除去後の技名: '{cleaned_input}', 括弧内容: '{bracket_content}'");
    println!("デバッグ - 画像リンク配列の要素数: {}", image_links.len());

    for (i, img_links) in image_links.iter().enumerate() {
        println!(
            "デバッグ - [{}]: img_links.input='{}' (小文字: '{}'), img_links.move_img='{}'",
            i,
            img_links.input,
            img_links.input.to_lowercase(),
            img_links.move_img
        );

        // 元の入力との完全一致
        if move_data.input.to_lowercase() == img_links.input.to_lowercase()
            && !img_links.move_img.is_empty()
        {
            println!(
                "デバッグ - 完全一致！ 入力: '{}' == '{}'",
                move_data.input.to_lowercase(),
                img_links.input.to_lowercase()
            );
            embed_image = img_links.move_img.to_string(); // 画像リンク更新
            break; // ループ抜け
        }
        // 括弧を除去した技名との一致
        else if !cleaned_input.is_empty()
            && cleaned_input.to_lowercase() == img_links.input.to_lowercase()
            && !img_links.move_img.is_empty()
        {
            println!(
                "デバッグ - 括弧除去後の一致！ 除去後: '{}' == '{}'",
                cleaned_input.to_lowercase(),
                img_links.input.to_lowercase()
            );
            embed_image = img_links.move_img.to_string(); // 画像リンク更新
            break; // ループ抜け
        }
        // 括弧内のコマンドとの一致
        else if !bracket_content.is_empty()
            && bracket_content.to_lowercase() == img_links.input.to_lowercase()
            && !img_links.move_img.is_empty()
        {
            println!(
                "デバッグ - 括弧内コマンド一致！ 括弧内: '{}' == '{}'",
                bracket_content.to_lowercase(),
                img_links.input.to_lowercase()
            );
            embed_image = img_links.move_img.to_string(); // 画像リンク更新
            break; // ループ抜け
        }
        // 部分一致の場合 - 上記のいずれにも該当しない場合のバックアップ
        else if img_links
            .input
            .to_lowercase()
            .contains(&move_data.input.to_lowercase())
            && !img_links.move_img.is_empty()
        {
            println!(
                "デバッグ - 部分一致！ JSON入力: '{}', 技入力: '{}'",
                img_links.input, move_data.input
            );
            embed_image = img_links.move_img.to_string(); // 画像リンク更新
        }
    }

    embed_image
}

/// 技情報と画像データを読み込む関数
///
/// # 引数
/// * `character_arg_altered` - 正式なキャラクター名
/// * `character_move` - ユーザーが入力した技名
/// * `ctx` - コマンドコンテキスト
///
/// # 戻り値
/// 成功時は (技情報, 画像リンク情報) のタプル、失敗時はエラー
async fn find_move_data(
    character_arg_altered: &str,
    character_move: &str,
    ctx: &Context<'_>,
) -> Result<(MoveInfo, String), AppError> {
    // JSONファイルパス組み立て　対象キャラクターのデータファイル
    let char_file_path =
        "data/".to_owned() + character_arg_altered + "/" + character_arg_altered + ".json";
    // JSONファイル読み込み　ファイル内容取得
    let char_file_data = fs::read_to_string(&char_file_path)
        .expect(&("\nFailed to read '".to_owned() + character_arg_altered + ".json" + "' file."));

    // JSONデータデシリアライズ　技データ配列取得
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    // 読み込み成功表示
    println!(
        "{}",
        ("Successfully read '".to_owned() + character_arg_altered + ".json' file.").green()
    );

    // 技インデックス検索　指定技の位置特定
    let index = match find::find_move_index(
        &character_arg_altered.to_string(),
        character_move.to_string(),
        &moves_info,
    )
    .await
    {
        Ok(idx) => idx, // 技インデックス取得
        Err(err) => {
            // エラー表示　案内メッセージ送信
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.")
                .await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Err(AppError::MoveNotFound(err.to_string()));
        }
    };

    // 画像JSONファイル読み込み　対象キャラクターの画像データ取得
    let image_links = fs::read_to_string(
        "data/".to_owned() + character_arg_altered + "/images.json",
    )
    .expect(
        &("\nFailed to read 'data/".to_owned() + character_arg_altered + "'/images.json' file."),
    );

    // 画像データデシリアライズ　画像リンク配列取得
    let image_links = serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();
    // 対象技情報取得
    let move_data = moves_info[index].clone();

    // 技読み込み成功表示
    println!(
        "{}",
        ("Successfully read move '".to_owned()
            + &move_data.input
            + "' in '"
            + character_arg_altered
            + ".json' file.")
            .green()
    );

    // 画像マッチング処理
    let embed_image = find_matching_image(&move_data, &image_links, IMAGE_DEFAULT);

    Ok((move_data, embed_image))
}

/// 技情報の埋め込みメッセージを作成する関数
///
/// # 引数
/// * `move_data` - 技情報
/// * `embed_image` - 埋め込む画像のURL
/// * `character_arg_altered` - 正式なキャラクター名
///
/// # 戻り値
/// 埋め込みメッセージ
fn create_move_embed(
    move_data: &MoveInfo,
    embed_image: &str,
    character_arg_altered: &str,
) -> CreateEmbed {
    // 埋め込みタイトル組み立て　キャラクター名と技情報を連結
    let embed_title = format!("__**{}：{}**__", character_arg_altered, move_data.input);

    // 埋め込みURL組み立て　Dustloop Wiki の対象キャラクターページ
    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + character_arg_altered + "#Overview";
    // 埋め込みフッター作成　キャプション利用
    let embed_footer = CreateEmbedFooter::new(&move_data.caption);

    // 埋め込みメッセージ作成　各種フィールド追加
    CreateEmbed::new()
        .color(EMBED_COLOR) // 埋め込みカラー設定
        .title(embed_title) // タイトル設定
        .url(embed_url) // URL設定
        .image(embed_image) // 画像設定
        .fields(vec![
            (
                "ダメージ",
                &move_data.damage.map_or("-".to_string(), |v| v.to_string()),
                true,
            ),
            ("ガード", &move_data.guard, true),
            ("無敵", &move_data.invincibility, true),
            (
                "始動",
                &move_data.startup.map_or("-".to_string(), |v| v.to_string()),
                true,
            ),
            ("持続", &move_data.active, true),
            (
                "硬直",
                &move_data
                    .recovery
                    .map_or("-".to_string(), |v| v.to_string()),
                true,
            ),
            ("ヒット時", &move_data.on_hit, true),
            ("ガード時", &move_data.on_block, true),
            ("カウンター", &move_data.counter.to_string(), true),
        ])
        .footer(embed_footer) // フッター設定
}

/// キャラクターの技情報を埋め込み表示する指定
#[poise::command(prefix_command, slash_command)]
pub async fn simple(
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = "キャラクター名または愛称"]
    character: String,
    #[min_length = 2]
    #[rename = "move"]
    #[description = "技名、入力、またはエイリアス"]
    character_move: String,
) -> Result<(), AppError> {
    // コマンド引数表示
    println!(
        "{}",
        ("Command Args: '".to_owned() + &character + ", " + &character_move + "'").purple()
    );

    // 必要チェック実施　データ整合性確認
    if (check::adaptive_check(
        ctx,
        check::CheckOptions::DATA_FOLDER
            | check::CheckOptions::NICKNAMES_JSON
            | check::CheckOptions::CHARACTER_FOLDERS
            | check::CheckOptions::CHARACTER_JSONS
            | check::CheckOptions::CHARACTER_IMAGES,
    )
    .await)
        .is_err()
    {
        return Ok(());
    }

    // キャラクターデータ読み込み
    let Ok(character_arg_altered) = load_character_data(&character, &ctx).await else {
        return Ok(());
    };

    // 技情報と画像データ読み込み
    let Ok((move_data, embed_image)) =
        find_move_data(&character_arg_altered, &character_move, &ctx).await
    else {
        return Ok(());
    };

    // 埋め込みメッセージ作成
    let embed = create_move_embed(&move_data, &embed_image, &character_arg_altered);

    // 埋め込みメッセージ送信　Discordへ出力
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    // 新バージョン通知（コメントアウト）
    // ctx.channel_id().say(ctx, r"[__**Patch.**__](<https://github.com/yakiimoninja/baiken/releases>)").await?;
    Ok(())
}
