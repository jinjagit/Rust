use my_iterator::{MyIterator, MyIteratorExt};

fn main() {
    let vec = vec![1, 2, 3];

    let mut iter = vec.my_iter();

    while let Some(item) = iter.next() {
        println!("item: {}", item);
    }


    let mut iter = vec.my_iter().map(|x: &usize| (x * 2).to_string() + "y");

        while let Some(item) = iter.next() {
            println!("item: {}", item);
        }
}
