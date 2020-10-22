// https://www.youtube.com/watch?v=lQt0adYPdfQ

pub trait MyIterator: Sized {
    type Item; // Associated type

    fn next(&mut self) -> Option<Self::Item>;

    fn map<F, B>(self, f: F) -> MyMap<Self, F>
    where
        F: FnMut(Self::Item) -> B,
    {
        MyMap {
            iterator: self,
            func: f,
        }
    }
}

pub struct MyMap<I, F> {
    iterator: I,
    func: F,
}

impl <I: MyIterator, F: FnMut(I::Item) -> B, B> MyIterator for MyMap<I, F> {
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iterator.next()?;
        
        Some((self.func)(item))
    }
}

// 'a lifetime is defined, here, as the lifetime of our reference to the vector.
// This lifetime is tied to however long our vector lives for.
pub struct MyVecIterator<'a, T> {
    pub vec: &'a Vec<T>,
    pub current_index: usize,
}

// Note: Vectors do not have an iterator trait by default.
impl<'a, T> MyIterator for MyVecIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // which current item?
        let current = self.current_index;

        // get & return a reference to the current item
        // get will return an option if there is no item @ 'current' index.
        let current_item = self.vec.get(current);
        self.current_index += 1;
        current_item
    }
}

// Extension trait for vector
pub trait MyIteratorExt<T> {
    fn my_iter<'a>(&'a self) -> MyVecIterator<'a, T>;
}

impl<T> MyIteratorExt<T> for Vec<T> {
    fn my_iter<'a>(&'a self) -> MyVecIterator<'a, T> {
        MyVecIterator {
            vec: &self,
            current_index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let vec = vec![1usize, 2, 3];

        let mut iter = vec.my_iter();

        while let Some(item) = iter.next() {
            println!("item: {}", item);
        }

        let mut iter = vec.my_iter().map(|x: &usize| x.to_string() + "x");

        while let Some(item) = iter.next() {
            println!("item: {}", item);
        }
    }
}
