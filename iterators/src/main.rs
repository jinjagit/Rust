fn main() {
    let v1 = vec![1, 2, 3];

    // Create 'lazy' iterator
    let v1_iter = v1.iter();

    // Note no need for index referenced by counter in for loop
    for val in v1_iter {
        println!("Got: {}", val);
    }
} 

// Using the 'next' method from the Iterator Trait (standard library)
#[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        
        // Note the need to set to mutable, since calling the 'next' method
        // on aiterator changes internal state the iterator uses to keep track
        // of where it is in the sequence.
        let mut v1_iter = v1.iter();

        // The values we get from the calls to next are immutable references
        // to the values in the vector.
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }


// Methods that call next are called consuming adaptors, because calling them
// uses up the iterator. One example is the sum method, which takes ownership
// of the iterator and iterates through the items by repeatedly calling 'next'.

#[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }


// Other methods defined on the Iterator trait, known as iterator adaptors,
// allow you to change iterators into different kinds of iterators. You can
// chain multiple calls to iterator adaptors to perform complex actions in a
// readable way. But because all iterators are lazy, you have to call one of
// the consuming adaptor methods to get results from calls to iterator adaptors.

#[test]
    fn iterator_map() {

    let v1: Vec<i32> = vec![1, 2, 3];

    // Note that 'collect' consumes the iterator, resolving the lazy map adaptor
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
    }



// Filter: Here, we use filter with a closure that captures the shoe_size
// variable from its environment to iterate over a collection of Shoe struct
// instances. It will return only shoes that are the specified size.


    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    
    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn filters_by_size() {
            let shoes = vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 13,
                    style: String::from("sandal"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ];
    
            let in_my_size = shoes_in_my_size(shoes, 10);
    
            assert_eq!(
                in_my_size,
                vec![
                    Shoe {
                        size: 10,
                        style: String::from("sneaker")
                    },
                    Shoe {
                        size: 10,
                        style: String::from("boot")
                    },
                ]
            );
        }
    }