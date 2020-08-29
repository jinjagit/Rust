// Specify the code to run when a value goes out of scope by implementing the
// Drop trait. The Drop trait requires you to implement one method named drop
// that takes a mutable reference to self.

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Call std::mem::drop to explicitly drop a value before it goes out of scope
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
