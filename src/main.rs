use std::io;

mod lib;

fn main() {
    let mut input = String::new();

    println!("please input any charctors.");
    io::stdin().read_line(&mut input).expect("no charactors.");

    // 借用値
    str(&mut input);
    println!("returned: {}", input);
    println!("{} Tom!", lib::tmp())

    // lib::calc();    
}

fn str(message : &mut String) -> () {
    let mut a1 : &mut String = message;
    println!("{}", a1.to_string());

    // a1の借用を剥奪する。
    let a2 = &mut a1;
    println!("{a2_me}", a2_me=a2);
    *a1 = String::from("hello! ".to_owned() + a2);

    // bbb = String::from("good morning");
    let a3  = String::from("afternoon");
    println!("a3: {}", a3);
    println!("modified *a1: {}", a1);
    // println!("{}", *a3);
    // return a1;
}
