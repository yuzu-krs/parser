use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // コマンドライン引数からファイル名を取得
    let args: Vec<String> = env::args().collect();

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
    charsイテレータを作する．*/
    let mut chars = contents.chars().peekable();

    //行番号
    let mut line_number=1;

    //Someはイテレータに次の要素があるか,ないならNoneとなる
    //判定だけなので&cを使っている cだけだと，所有権を渡してしまう
    //chars.peek()は次の要素を見るだけ
    while let Some(&c) = chars.peek() {
        //charはスタック領域なので，&がなくてもよい．(コピーされる)
        //すぐ取り出せるようにメモリ上の近い位置にある
        if is_white_space(c){ 
            //白空白かどうか
            if c=='\n'{
                line_number+=1;
            }
            chars.next(); //イテレータを進める
        }else if c == '#' {
            chars.next();
            // '#'が見つかったら\nを見つけるまでスキップ
            while let Some(next_char) = chars.next() { //イテレータを進める
                if next_char == '\n' {
                    line_number+=1;
                    break;
                }
            }
        }else if c=='"'{
            chars.next();
            let mut string_content=String::new();
            //2つ目の'"'が見つけるまで
            while let Some(next_char)=chars.next(){
                if next_char == '"' {
                    println!("{}\t11\tTK_STRING\t{}",line_number,string_content);
                    break;
                }else{
                    string_content.push(next_char);
                }
            }
        }else if c == ':' {
            chars.next(); // ':'を消費する

            // 次が'='かどうかを覗き見
            if let Some(&next_char) = chars.peek() {
                if next_char == '=' {
                    chars.next(); //'='を消費する
                    println!("{}\t22\tTK_COLON_EQUAL",line_number);
                }
            }
        }else if is_special_character(c,line_number){
            chars.next();
        } else if c.is_digit(10) { //10進数の数字であれば
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
                if number.matches('.').count()>1||number.ends_with('.'){
                    panic!("エラー {}行目:小数点が2つ以上か末尾に含まれています.",line_number);
                }
                println!("{}\t10\tTK_FLOAT\t{}",line_number,number); // 浮動小数点数
            } else {
                println!("{}\t9\tTK_INTEGER\t{}",line_number,number); // 整数
            }

        }else if is_alphabet_or_underscore(c){
            let mut letters=String::new();
            while let Some(&next_char)=chars.peek(){
                if is_alphabet_or_underscore(next_char) || next_char.is_digit(10){
                    letters.push(next_char);
                    chars.next();
                }else{
                    break;
                }
            }

            match letters.as_str(){
                "var"=>println!("{}\t2\tTK_VAR",line_number),
                "read"=>println!("{}\t3\tTK_READ",line_number),
                "print"=>println!("{}\t4\tTK_PRINT",line_number),
                "println"=>println!("{}\t5\tTK_PRINTLN",line_number),
                "div"=>println!("{}\t6\tTK_DIV",line_number),
                "repeat"=>println!("{}\t7\tTK_REPEAT",line_number),
                _=>println!("{}\t1\tTK_IDENTIFIER\t{}",line_number,letters),
            }


        }else {
            // 通常の処理
            println!("{}", c);
            chars.next();
        }
    }
}



//英字かどうか
fn is_alphabet_or_underscore(c:char)->bool{
    //アスキー内のアルファベットか_
    c.is_ascii_alphabetic()||c=='_'
}


//白空白かどうか
fn is_white_space(c:char)->bool{
    c=='\n'||c.is_whitespace()||c=='\t'
}

// 記号
fn is_special_character(c: char, line_number: usize) -> bool {
    match c {
        '+' => { println!("{}\t12\tTK_PLUS",line_number); true },
        '-' => { println!("{}\t13\tTK_MINUS",line_number); true },
        '*' => { println!("{}\t14\tTK_MULTIPLY",line_number); true },
        '/' => { println!("{}\t15\tTK_DIVIDE",line_number); true },
        '%' => { println!("{}\t16\tTK_MODULUS",line_number); true },
        '(' => { println!("{}\t17\tTK_LEFTPAREN",line_number); true },
        ')' => { println!("{}\t18\tTK_RIGHTPAREN",line_number); true },
        ';' => { println!("{}\t19\tTK_SEMICOLON",line_number); true },
        ',' => { println!("{}\t20\tTK_COMMA",line_number); true },
        '@' => { println!("{}\t21\tTK_AT",line_number); true },
        _ => {
            false
        }
    }
}