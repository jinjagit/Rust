fn main() {
    let s = "First line
Second line


Third line.";

    println!("{}", s);

    let split: Vec<&str> = s.split("\n").collect();

    println!();

    for i in 0..split.iter().count() {
        if split[i] != "" {
            println!("{}", split[i]);
        }
    }
}
