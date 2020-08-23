fn main() {
    let dish = ("Ham", "Eggs");

    // this body will be skipped because the pattern is refuted
    if let ("Bacon", b) = dish {
        println!("Bacon is served with {}", b);
    } else {
        // This block is evaluated instead.
        println!("No bacon will be served");
    }

    // this body will execute
    if let ("Ham", b) = dish {
        println!("Ham is served with {}", b);
    }

    if let (a, "Eggs") = dish {
        println!("Eggs can also be good without {}", a);
    }

    if let _ = 5 {
        println!("Irrefutable patterns are always true");
    }

    
    let x: u8 = 3;
    let some_u8_value = Some(&x);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
