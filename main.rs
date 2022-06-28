use std::io;
fn main(){
    
}


//My small cp library
fn input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    input.trim().to_string()
}

fn int_from_str(s: String) -> i64{
    s.parse::<i64>().unwrap()
}

fn binary_from_binarystr(s: String) -> i64{
    i64::from_str_radix(&s, 2).unwrap()
}

fn vec_of_ints(s: String) -> Vec<i64> {
    s.split(" ").map(|x| x.parse::<i64>().unwrap()).collect()
}

fn vec_of_chars(s: String) -> Vec<char> {
    s.chars().collect()
}
