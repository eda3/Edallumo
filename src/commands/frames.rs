//! frames.rs
//!
//! フレームデータ表示モジュール  
//! Discord コマンド /frames 実装モジュール  
//! 指定されたキャラクターの技のフレームデータを取得し、画像付きで表示する処理を提供  

// 必要なモジュールや型をインポートする
use crate::{check, find, IMAGE_DEFAULT}; // 入力チェック、キャラクター検索、デフォルト画像定数を利用
use crate::{Context, Error, ImageLinks, MoveInfo}; // コマンド実行用コンテキスト、エラー型、画像リンク構造体、技情報構造体
use colored::Colorize; // コンソール出力に色を付けるための拡張メソッド
use std::{fs, string::String}; // ファイル操作と文字列操作を行うための標準ライブラリ

/// 指定された技のフレームデータを画像付きで表示する処理
///
/// キャラクター名（またはニックネーム）と技名（またはエイリアス）を元に、
/// 対象のキャラクター JSON ファイルから技情報を取得し、
/// 対応する画像があればそれも併せて Discord に埋め込み形式で送信する。
#[allow(unused_assignments)]
#[poise::command(prefix_command, slash_command, aliases("f"))]
pub async fn frames(
    ctx: Context<'_>,
    #[description = "Character name or nickname."] character: String, // キャラクター名またはニックネーム
    #[description = "Move name, input or alias."] mut character_move: String, // 技名、入力、またはエイリアス（必要に応じて変更可能）
) -> Result<(), Error> {
    // コマンド実行時の引数をログ出力（紫色で表示）
    println!(
        "{}",
        ("Command Args: '".to_owned() + &character + ", " + &character_move + "'").purple()
    );

    // ユーザー入力がエイリアスの場合に、正式なキャラクター名を保持するための変数（初期値は空文字）
    let mut character_arg_altered = String::new();
    // 埋め込み画像用の変数を初期化
    // 空の場合、Discord 送信用の embed が送信できなくなるため、デフォルト画像を設定
    let mut image_embed = IMAGE_DEFAULT.to_string();

    // 各種入力チェックおよび環境整合性チェックを実施する
    // check::adaptive_check で、キャラクター名、技名、データファイルの存在などを確認する
    if (check::adaptive_check(
        ctx,
        (true, &character),
        (true, &character_move),
        true, // データフォルダ存在チェック
        true, // nicknames.json 存在チェック
        true, // キャラクターフォルダ存在チェック
        true, // キャラクター JSON 存在チェック
        true, // 画像 JSON 存在チェック
    )
    .await)
        .is_err()
    {
        // チェックに失敗した場合は、以降の処理をせず正常終了
        return Ok(());
    }

    // キャラクター検索処理
    // 入力された文字列から正式なキャラクター名を取得する
    character_arg_altered = match find::find_character(&character).await {
        Ok(name) => name, // 検索成功時は正式名を代入
        Err(err) => {
            // 検索に失敗した場合は、Discord にエラーメッセージを送信し、コンソールにも出力
            ctx.say(err.to_string()).await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(());
        }
    };

    // キャラクター JSON ファイルのパスを組み立て、内容を読み込む
    let char_file_path =
        "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character_arg_altered + ".json" + "' file."));

    // 読み込んだ JSON 文字列を、MoveInfo のベクターにデシリアライズする
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    // JSON の読み込みが成功した旨をコンソールに緑色で出力
    println!(
        "{}",
        ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green()
    );

    // 入力された技名またはエイリアスに該当する技のインデックスを検索する
    let mframes_index =
        find::find_move_index(&character_arg_altered, character_move, &moves_info).await;
    let mframes_index = match mframes_index {
        Ok(index) => index, // 検索成功時はインデックスと正規化後の技名を返す
        Err(err) => {
            // 検索に失敗した場合は、Discord にエラーメッセージを送信し、コンソールにも出力
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.")
                .await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(());
        }
    };

    // TODO: 現在の処理では character_move を上書きしているため、改善の余地あり
    character_move = mframes_index.1;

    // 画像データが格納された JSON ファイルのパスを組み立て、内容を読み込む
    let image_links = fs::read_to_string(
        "data/".to_owned() + &character_arg_altered + "/images.json",
    )
    .expect(
        &("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."),
    );

    // 読み込んだ画像 JSON を、ImageLinks のベクターにデシリアライズする
    let image_links = serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();

    // 取得した技情報から、対象の技情報を抽出する
    let mframes = &moves_info[mframes_index.0];

    // 対象技の読み込み成功をコンソールに出力（緑色）
    println!(
        "{}",
        ("Successfully read move '".to_owned()
            + &mframes.input.to_string()
            + "' in '"
            + &character_arg_altered
            + ".json' file.")
            .green()
    );

    // Discord 埋め込み送信用のコンテンツ URL とタイトルを組み立てる
    let content_embed = "https://dustloop.com/wiki/index.php?title=GGST/".to_owned()
        + &character_arg_altered
        + "/Frame_Data";
    let title_embed = "Move: ".to_owned() + &mframes.input.to_string();

    // 画像データのチェック処理
    // JSON 内の各画像エントリを順次確認し、対象技に対応する画像リンクが存在する場合は image_embed を更新する
    for img_links in image_links {
        if mframes.input == img_links.input && !img_links.move_img.is_empty() {
            image_embed = img_links.move_img.to_string();
            break; // 対応する画像リンクが見つかったらループを抜ける
        }
    }

    // Discord のチャネルに、組み立てた埋め込みメッセージを送信する
    let _msg = ctx
        .send(|m| {
            // 埋め込みメッセージの内容として、Dustloop のページ URL をコンテンツとして送信
            m.content(&content_embed);
            m.embed(|e| {
                e.color((140, 75, 64)); // 埋め込み枠のカラー設定
                e.title(&title_embed); // 埋め込みタイトル設定
                                       // e.description("This is a description"); // 説明文（必要に応じて追加）
                e.image(&image_embed); // 画像リンク設定
                                       // 各種フィールド（技の各フレーム情報）を追加
                e.fields(vec![
                    ("ダメージ", &mframes.damage.to_string(), true),
                    ("ガード", &mframes.guard.to_string(), true),
                    ("無敵", &mframes.invincibility.to_string(), true),
                    ("発生", &mframes.startup.to_string(), true),
                    ("持続", &mframes.active.to_string(), true),
                    ("硬直", &mframes.recovery.to_string(), true),
                    ("ヒット時", &mframes.hit.to_string(), true),
                    ("ガード時", &mframes.block.to_string(), true),
                    ("攻撃レベル", &mframes.level.to_string(), true),
                    ("リスクゲージ", &mframes.riscgain.to_string(), true),
                    ("補正", &mframes.scaling.to_string(), true),
                    ("カウンター", &mframes.counter.to_string(), true),
                ]);
                e // 結果として embed を返す
            });
            m // 結果としてメッセージビルダーを返す
        })
        .await;

    Ok(()) // 正常終了を返す
}
