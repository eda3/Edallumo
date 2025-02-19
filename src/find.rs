// 必要な型や構造体を他モジュールからインポート
use crate::{Error, MoveAliases, MoveInfo, Nicknames};
use std::{fs, path::Path};

// キャラクター名やニックネーム、技情報から検索を行う非同期関数
pub async fn find_character(character: &String) -> Result<String, Error> {
    // 検索結果が見つかったかどうかの判定用フラグ
    // ※以降の処理ではフラグ自体は更新されず、常にfalseのままですが、論理上の説明用として記述
    let character_found = false;

    let error_message = "\n'nicknames.json' ファイルの読み込みに失敗しました。";
    // data/nicknames.json ファイルを文字列として読み込み、失敗した場合はエラーメッセージを出力してプログラムを終了する
    let data_from_file = fs::read_to_string("data/nicknames.json").expect(error_message);

    // 読み込んだJSON文字列を、Nicknames構造体のベクターにデシリアライズする
    let vec_nicknames = serde_json::from_str::<Vec<Nicknames>>(&data_from_file).unwrap();

    // まず、ニックネームから該当キャラクターを探す処理
    if !character_found {
        // JSON内の各キャラクターエントリをループ処理
        for x_nicknames in &vec_nicknames {
            // 各キャラクターに紐付くニックネームを順にチェック
            for y_nicknames in &x_nicknames.nicknames {
                // ユーザー入力とニックネームを大文字小文字無視で比較し、一致する場合
                if y_nicknames.to_lowercase() == character.to_lowercase().trim() {
                    // 該当するキャラクターの正式な名前を返す
                    return Ok(x_nicknames.character.to_owned());
                }
            }
        }
    }

    // 次に、キャラクターの正式な名前から部分一致で検索する処理
    if !character_found {
        // 各キャラクターエントリをループ処理
        for x_nicknames in &vec_nicknames {
            // ユーザー入力が、キャラクター名全体または一部に含まれているかチェック
            // '-' は取り除いて比較する
            if x_nicknames
                .character
                .to_lowercase()
                .replace('-', "")
                .contains(&character.to_lowercase())
                || x_nicknames
                    .character
                    .to_lowercase()
                    .contains(&character.to_lowercase())
            {
                // 一致したキャラクターの正式な名前を返す
                return Ok(x_nicknames.character.to_owned());
            }
        }
    }

    // エッジケース：ユーザーが "all" と入力した場合（update.rs用の特別処理）
    if !character_found && character.trim().to_lowercase() == "all".to_lowercase() {
        return Ok("".into());
    }

    // 上記のいずれの方法でもキャラクターが見つからなかった場合は、エラーを返す
    if !character_found {
        let error_msg = "Character `".to_owned() + &character + "` was not found!";
        Err(error_msg.into())
    } else {
        // 本来ここには到達しないはずの論理エラー（念のためのエラー）
        Err("Weird logic error in find_character".into())
    }
}

// ユーザーの入力に基づき、キャラクターの技情報から該当する技のインデックスと技名を返す非同期関数
pub async fn find_move_index(
    character_arg_altered: &String, // 正式なキャラクター名（または調整済みの名前）
    mut character_move: String,     // ユーザーが入力した技名（またはエイリアス）
    moves_info: &[MoveInfo],        // キャラクターの技情報が格納されたスライス
) -> Result<(usize, String), Error> {
    // 技が見つかったかどうかの判定用フラグ（以降の処理で変更は行われない）
    let move_found = false;

    // 対象キャラクターのエイリアス情報が格納されているJSONファイルのパスを生成
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
    // エイリアスファイルが存在するかチェック
    if Path::new(&aliases_path).exists() {
        // エイリアスファイルを読み込む
        let aliases_data = fs::read_to_string(&aliases_path)
            .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));

        // 読み込んだJSON文字列を、MoveAliases構造体のベクターにデシリアライズする
        let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

        // エイリアス情報を探索するための外側のループにラベルを付与
        'outer: for alias_data in aliases_data {
            // 各エイリアスリスト内の各エイリアスをチェック
            for x_aliases in alias_data.aliases {
                // ユーザーの入力（不要な記号を除去し小文字に変換）とエイリアスを比較
                if x_aliases.to_lowercase().trim().replace(['.', ' '], "")
                    == character_move.to_lowercase().trim().replace(['.', ' '], "")
                {
                    // 一致した場合、ユーザー入力を実際の技名に置き換える
                    character_move = alias_data.input.to_string();
                    // エイリアスが見つかったので、外側のループを抜ける
                    break 'outer;
                }
            }
        }
    }

    // moves_info内の各技について、入力と正確に一致するかチェック
    for (x, moves) in moves_info.iter().enumerate() {
        if moves.input.to_string().to_lowercase().replace('.', "")
            == character_move.to_string().to_lowercase().replace('.', "")
        {
            // 一致した技が見つかった場合、技のインデックスと技名を返す
            return Ok((x, character_move));
        }
    }

    // 正確な一致が見つからなかった場合、技名に部分一致するかどうかチェックする処理
    if !move_found {
        for (x, moves) in moves_info.iter().enumerate() {
            // ユーザー入力が、技の名称の一部に含まれているかチェック
            if moves
                .name
                .to_string()
                .to_lowercase()
                .contains(&character_move.to_string().to_lowercase())
            {
                // 部分一致した場合、該当する技のインデックスと技名を返す
                return Ok((x, character_move));
            }
        }
    }

    // 上記のいずれの方法でも技が見つからなかった場合、エラーメッセージを返す
    if !move_found {
        let error_msg = "Move `".to_owned() + &character_move + "` was not found!";
        Err(error_msg.into())
    } else {
        // 本来ここには到達しないはずの論理エラー（念のためのエラー）
        Err("Weird logic error in find_move".into())
    }
}
