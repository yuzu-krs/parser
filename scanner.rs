use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // コマンドライン引数からファイル名を取得
    let args: Vec<String> = env::args().collect();

    //構文解析に使うベクタ
    let mut parser: Vec<i32> = Vec::new();

    // プログラム名と引数が足りない場合は標準エラーメッセージを表示して終了
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    // 引数からファイル名を取得
    let filename = &args[1];

    //expectはｴﾗｰの時に表示
    let mut f = File::open(filename).expect("file not found");

    //ファイルの内容を格納するための文字列を作成
    let mut contents = String::new(); //可変

    // ファイルの内容を文字列として読み取るexpectはエラーメッセージ
    //read_to_stringはファイルの内容を文字列として読み取ってcontentsに渡している
    f.read_to_string(&mut contents) //可変の参照
        .expect("something went wrong reading the file");

    /*読み取った文字列の各文字にアクセスするための
    charsイテレータを作成する．*/
    let mut chars = contents.chars().peekable();

    //行番号
    let mut line_number = 1;

    //Someはイテレータに次の要素があるか,ないならNoneとなる
    //判定だけなので&cを使っている cだけだと，所有権を渡してしまう
    //chars.peek()は次の要素を見るだけ
    while let Some(&c) = chars.peek() {
        //charはスタック領域なので，&がなくてもよい．(コピーされる)
        //すぐ取り出せるようにメモリ上の近い位置にある
        if is_white_space(c) {
            //白空白かどうか
            if c == '\n' {
                line_number += 1;
            }
            chars.next(); //イテレータを進める
        } else if c == '#' {
            chars.next();
            // '#'が見つかったら\nを見つけるまでスキップ
            while let Some(next_char) = chars.next() {
                //イテレータを進める
                if next_char == '\n' {
                    line_number += 1;
                    break;
                }
            }
        } else if c == '"' {
            chars.next();
            let mut string_content = String::new();
            //2つ目の'"'が見つけるまで
            while let Some(next_char) = chars.next() {
                if next_char == '"' {
                    parser.push(11);
                    break;
                } else {
                    string_content.push(next_char);
                }
            }

            //2つ目の'"'が見つからないまま終端に達した場合のエラー処理
            if chars.peek().is_none() {
                panic!(
                    "エラー {}行目:ダブルクォートが閉じられていません",
                    line_number
                );
            }
        } else if c == ':' {
            chars.next(); // ':'を消費する

            // 次が'='かどうかを覗き見
            if let Some(&next_char) = chars.peek() {
                if next_char == '=' {
                    chars.next(); //'='を消費する
                    parser.push(22);
                } else {
                    panic!("エラー {}行目:':'の後に'='がありません", line_number);
                }
            }
        } else if is_special_character(c, &mut parser) {
            chars.next();
        } else if c.is_digit(10) {
            //10進数の数字であれば
            let mut number = String::new();
            while let Some(&next_char) = chars.peek() {
                if next_char.is_digit(10) || next_char == '.' {
                    number.push(next_char);
                    chars.next(); // 文字を消費
                } else {
                    break;
                }
            }

            if number.contains('.') {
                if number.matches('.').count() > 1 || number.ends_with('.') {
                    panic!(
                        "エラー {}行目:小数点が2つ以上か末尾に含まれています",
                        line_number
                    );
                }
                parser.push(10); // 浮動小数点数
            } else {
                parser.push(9); // 整数
            }
        } else if is_alphabet_or_underscore(c) {
            let mut letters = String::new();
            while let Some(&next_char) = chars.peek() {
                if is_alphabet_or_underscore(next_char) || next_char.is_digit(10) {
                    letters.push(next_char);
                    chars.next();
                } else {
                    break;
                }
            }

            match letters.as_str() {
                "var" => parser.push(2),
                "read" => parser.push(3),
                "print" => parser.push(4),
                "println" => parser.push(5),
                "div" => parser.push(6),
                "repeat" => parser.push(7),
                _ => parser.push(1),
            }
        } else {
            panic!("エラー {}行目:文法が間違っています", line_number);
        }
    }

    /*###############*/
    /*ここから構文解析*/
    /*###############*/

    // ベクタの最初から最後まで表示
    let mut iterator = parser.iter();

    //構文解析のスタート
    program(&mut iterator);
}

//英字かどうか
fn is_alphabet_or_underscore(c: char) -> bool {
    //アスキー内のアルファベットか_
    c.is_ascii_alphabetic() || c == '_'
}

//白空白かどうか
fn is_white_space(c: char) -> bool {
    c == '\n' || c.is_whitespace() || c == '\t'
}

// 記号
fn is_special_character(c: char, parser: &mut Vec<i32>) -> bool {
    match c {
        '+' => {
            parser.push(12);
            true
        }
        '-' => {
            parser.push(13);
            true
        }
        '*' => {
            parser.push(14);
            true
        }
        '/' => {
            parser.push(15);
            true
        }
        '%' => {
            parser.push(16);
            true
        }
        '(' => {
            parser.push(17);
            true
        }
        ')' => {
            parser.push(18);
            true
        }
        ';' => {
            parser.push(19);
            true
        }
        ',' => {
            parser.push(20);
            true
        }
        '@' => {
            parser.push(21);
            true
        }
        _ => false,
    }
}

//iter.next() で次のトークンへ

/*プログラム*/
// {<解釈単位>“;”}
fn program(tokens: &mut std::slice::Iter<i32>) {
    // イテレータをpeekableに変換
    let mut token = tokens.peekable();

    // 先読みが可能かチェックして表示
    if let Some(&next_token) = token.peek() {
        println!("{}", next_token);
        token.next();
    }

    unit_of_interpretation(&mut token);
}

/*解釈単位*/
// <変数代入> | <変数宣言> | <変数入力> | <出力指定> | <repeat 文>
fn unit_of_interpretation(tokens: &mut std::iter::Peekable<&mut std::slice::Iter<i32>>) {
    // ここで最後の部分を実行
    if let Some(&next_token) = tokens.peek() {
        println!("{}", next_token);
        tokens.next();
    }
    variable_assignment(tokens);
}

/*変数代入*/
// <変数名> “:=” <式>
fn variable_assignment(tokens: &mut std::iter::Peekable<&mut std::slice::Iter<i32>>) {
    // 同様に次のトークンも表示
    if let Some(&next_token) = tokens.peek() {
        println!("{}", next_token);
        tokens.next();
    }
}

/*
/*変数名*/
// "識別子"
// “識別子”
fn variable_name(parser: &mut Vec<i32>) {}

/*式*/
//  [“+” | “-”] <項> {“+” <項> | “-” <項> }
// “+”, “-”, “(”, “整数”, “実数”, “識別子”, “@”
fn formula(parser: &mut Vec<i32>) {}

/*項*/
// <因子> {“*” <因子> | “/” <因子> | “div” <因子> | “%” <因子>}
// “(”, “整数”, “実数”, “識別子”, “@”
fn term(parser: &mut Vec<i32>) {}

/*因子*/
// “(” <式>“)” | “整数” | “実数” | <変数名> | <関数呼出>
// “(”, “整数”, “実数”, “識別子”, “@”
fn factor(parser: &mut Vec<i32>) {}

/*変数宣言*/
//  “var” <変数名> [“:=” <式>]
// “var”
fn variable_declaration(parser: &mut Vec<i32>) {}

/*変数入力*/
// “read” “(” <変数名> “)”
// “read”
fn variable_input(parser: &mut Vec<i32>) {}

/*出力指定*/
// “print” “(”<出力単位の並び> “)” | “println” “(”<出力単位の並び> “)”
// “print”, “println”
fn output_specification(parser: &mut Vec<i32>) {}

/*出力単位の並び*/
// ε | <出力単位> {“,” <出力単位>}
fn output_unit_sequence(parser: &mut Vec<i32>) {}

/*出力単位*/
// <式> | “文字列”
//  “文字列”, “+”, “-”, “(”, “整数”, “実数”, “識別子”, “@”
fn output_unit(parser: &mut Vec<i32>) {}

/*repeat*/
// “repeat” <式> <変数代入>
// “repeat”
fn repeat(parser: &mut Vec<i32>) {}

/*関数呼出*/
//  “@” <関数名> “(” <式の並び> “)”
// "@"
fn function_call(parser: &mut Vec<i32>) {}

/*関数名*/
// "識別子"
// "識別子"
fn function_name(parser: &mut Vec<i32>) {}

/*式の並び*/
// ε | <式> {“,” <式>}
fn sequence_of_expressions(parser: &mut Vec<i32>) {}
*/
