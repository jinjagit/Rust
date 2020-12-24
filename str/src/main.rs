fn main() {
    let s = my_str();

    for i in 0.. s.iter().count() {
        println!("{}", s[i]);
    }
}

fn my_str() -> Vec<&'static str> {
    let s: Vec<&str> = vec!["yo", "hello"];

    s
}