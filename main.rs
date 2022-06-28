use std::io;
fn main(){
    println!("{}", input());
}

fn input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    input
}