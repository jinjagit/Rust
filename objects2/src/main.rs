// How to change value in Struct instance using a function

pub struct Value {
    value: i32,
}

impl Value {
    pub fn add(&mut self, x: i32) {
        self.value = self.value + x;
    }
}

fn main() {
    let mut a = Value {
        value: 0,
    };

    change(&mut a);

    println!("a.value = {}", a.value);
}

// Change the value in the struct. Note how we pass an &mut Struct and don't return anything :-)
fn change(v: &mut Value) {
    v.value = 3;
}
