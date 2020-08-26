// Closures don’t require you to annotate the types of the parameters or the
// return value like fn functions do. We can add these annotations, however.
// Closures are usually short and relevant only within a narrow context rather
// than in any arbitrary scenario.

// Note that calling a closure subsequently with a different type to one
// already used for that closure will raise an error. i.e. The first use of
// a closure will lock that type into the closure.





fn main() {
    // since this closure is a single line, it could also be written as:
    // let my_closure = |num| num * 2;
    // or with types sepecified:
    // let my_closure = |num: i32| -> i32 { num * 2 };
    let my_closure = |num| {
        num * 2
    };

    let result = my_closure(3);

    println!("result = {}", result);


    // Using the 'Cacher' struct to ensure the closure only runs once and stores
    // the result obtained for later retrieval multiple times

    let mut multiply_by_three = Cacher::new(|num| { num * 3});

    println!("result_from_cacher = {}", multiply_by_three.value(2));
    // => result_from_cacher = 6
    println!("result_from_cacher = {}", multiply_by_three.value(7));
    // => result_from_cacher = 6


    // Closures have an additional capability that functions don’t have:
    // they can capture their environment and access variables from the scope
    // in which they’re defined.

    let x = 4;

    let equal_to_x = |z| z == x; // x would not be in a function's scope (if not passed in)

    let y = 4;

    if equal_to_x(y) {
        println!("x == y");
    } else {
        println!("x != y");
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


