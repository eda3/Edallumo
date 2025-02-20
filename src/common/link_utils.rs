use md5::{Digest, Md5};

/// 画像の基本URL  
/// Dustloop Wiki の画像が配置されている基本パス
pub const IMAGE_HALF: &str = "https://www.dustloop.com/wiki/images";

/// 指定された画像名から MD5 ハッシュを利用して画像リンクを生成する非同期関数  
///
/// # 概要
/// 画像名から MD5 ハッシュを計算し、そのハッシュ値の先頭2文字を用いて画像リンクを組み立てる。  
/// リンクは `IMAGE_HALF` の後にハッシュ値と画像名が連結された形式となる。
///
/// # 引数
/// * `image_name` - 画像ファイル名（例: `"example.png"`）
///
/// # 戻り値
/// 生成された画像リンクの文字列を返す（例: `"https://www.dustloop.com/wiki/images/e/e1/example.png"`）
///
/// # 例
/// ```rust,no_run
/// # async fn example() {
/// use your_crate::make_link;
/// let link = make_link("example.png".to_string()).await;
/// println!("生成されたリンク: {}", link);
/// # }
/// ```
///
/// # 注意
/// この関数は非同期関数であるため、呼び出し時には `.await` が必要。
pub async fn make_link(image_name: String) -> String {
    // 画像名をバイト列に変換
    // 変換結果: 画像名のバイト列
    let image_bytes = image_name.as_bytes();

    // MD5 ハッシュ計算器を生成
    // 生成結果: MD5 ハッシュ計算器
    let mut hasher = Md5::new();

    // 画像のバイト列をハッシュ計算器に投入
    // 投入対象: 画像名のバイト列
    hasher.update(image_bytes);

    // ハッシュ計算結果を16進数文字列に変換
    // 変換結果: MD5 ハッシュ値（16進数文字列）
    let result = format!("{:x}", hasher.finalize());

    // 16進数文字列の先頭1文字を取得
    // 取得結果: ハッシュ値の先頭1文字
    let char1 = result.chars().next().unwrap();

    // 16進数文字列の2文字目を取得
    // 取得結果: ハッシュ値の2文字目
    let char2 = result.chars().nth(1).unwrap();

    // 基本URL、取得文字、画像名を結合して画像リンクを組み立て
    // 組み立て結果: 生成された画像リンク
    let image_link = format!("{}/{}/{}{}/{}", IMAGE_HALF, char1, char1, char2, image_name);

    // 生成された画像リンクを返却
    // 返却結果: 完成した画像リンク
    image_link
}
