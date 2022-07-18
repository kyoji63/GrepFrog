use encoding_rs;
use std::fs;
use std::io::Write;

fn main() {
    let args: Vec<String> = args().collect();
    let path = &args[1]; // 読み込むファイルパスの設定
    let file = fs::File::create(&args[2]).unwrap();  // 出力先ファイルパスの設定
    // 読み込み
    let s = fs::read(path).unwrap();
    // Shift_JISのバイト列(Vec<u8>) を UTF-8の文字列(String) に変換
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&s);
    let text = res.into_owned();
    // 出力
    file.write(text.as_bytes()).unwrap();
}

fn main(){
    // 標準ライブラリーで実行時引数を解析しデータを取り出す
    let args: Vec<String> = args().collect();
    if args.len() > 3{
        println!("argument must be two.");
    }
    // 受け取ったデータをGrepArgs構造体に詰める
    let pattern = &args[1];
    let file_path = &args[2];

}