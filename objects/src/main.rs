// aspect commonly associated with OOP is the idea of encapsulation, which means
// that the implementation details of an object aren’t accessible to code using
// that object. Therefore, the only way to interact with an object is through
// its public API:

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut c = AveragedCollection {
        list: Vec::new(),
        average: 0.0,
    };

    c.add(33);
    c.add(57);

    println!("c.list = {:?}, c.average = {}", c.list, c.average);
    // => c.list = [33, 57], c.average = 45
}
