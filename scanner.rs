use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // コマンドライン引数からファイル名を取得
    let args: Vec<String> = env::args().collect();

    // プログラム名と引数が足りない場合はエラーメッセージを表示して終了
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    // 引数からファイル名を取得
    let filename = &args[1];

    // ファイルが見つかりませんでした
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");

    // テキストは\n{}です
    for c in contents.chars() {
        println!("{}", c);
    }
}
