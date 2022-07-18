use std::fs;

fn main() -> std::io::Result<()> {
        // 標準ライブラリーで実行時引数を解析しデータを取り出す
        let args: Vec<String> = args().collect();
        if args.len() > 3{
            println!("argument must be two.");
        }        
        let input = &args[1];
        let output = &args[2];
    fs::rename(input, output)?; //a.txtの名前をb.txtに変更します
}