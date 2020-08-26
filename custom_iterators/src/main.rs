// You can create iterators from the collection types in the standard library,
// such as hash map. You can also create iterators that do anything you want by
// implementing the Iterator trait on your own types.

// To demonstrate, let’s create an iterator that will only ever count from
// 1 to 5, (for some reason):

// Defining the Counter struct and a new function that creates instances of
// Counter with an initial value of 0 for count.
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Implementing the Iterator trait for our Counter type by defining the body of
// the next method to specify what we want to happen when this iterator is used.
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();

    loop {
        let c = counter.next();
        println!("counter = {:?}   counter.count = {}", c, counter.count);

        if c == None {
            break;
        }
    }
}


// if for some reason we wanted to take the values produced by an instance of
// Counter, pair them with values produced by another Counter instance after
// skipping the first value, multiply each pair together, keep only those
// results that are divisible by 3, and add all the resulting values together,
// we could do so:

#[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
