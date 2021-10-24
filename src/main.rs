use std::env;
use std::fs::File;
use std::io::prelude::*;

/*
    引数からファイル名を第二引数で取って、読み込み、描画する
*/

fn main(){
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("in file {}", filename);
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("sometihng went wrong reading the file");
    println!("with text:\n{}", contents);
}