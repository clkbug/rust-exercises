#![allow(unused_variables, dead_code)]

/* 
ゴール：
1. cargo run -- Cargo.toml と実行して、このプログラムの動作を確認してください
2. 存在しないファイル名を指定して実行すると、panicがおきます。修正してください
3. コンビネータを使って、read_file2を実装してください
*/

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut args = env::args();
    if let Some(file) = args.nth(1) {
        match read_file(&file){
            Err(e)=> println!("{} read中に{}", file, e),
            Ok(t)=>println!("{} {}", file, t),
        }
        println!("{}", read_file2(&file));
        
    }
}

fn read_file(filename: &String) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn read_file2(filename: &String) -> String {
    let mut content = String::new();
    let x = File::open(filename).and_then(|mut x| x.read_to_string(&mut content));
    match x { // これどうやってコンビネータに？
        Err(e) => e.to_string(),
        Ok(x)  => content
    }
}

