use std::env::args;
use std::fs::read_to_string;
use std::process::exit;


struct GrepArgs{
    pattern: String, 
    file_path: String,
}

impl GrepArgs{
    fn new(pattern: String, file_path: String) -> Self{
        GrepArgs{
            pattern: pattern, 
            file_path: file_path,
        }
    }
}

fn grep(args: GrepArgs){
    // 指定されたファイルパスにあるファイルを読み込む
    let content = read_to_string(args.file_path);
    //指定された検索パターンを含む行を探し出して表示する
    match content {
        Ok(content) => {
            for line in content.lines(){
                if line.contains(args.pattern.as_str()){
                    println!("{}", line);
                } 
            }
        }
        Err(why) => eprintln!("{}", why)
    }
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
    let args = GrepArgs::new(
        pattern.to_string(),
        file_path.to_string()
    );
    // grep関数を呼び出す
    grep(args)
}