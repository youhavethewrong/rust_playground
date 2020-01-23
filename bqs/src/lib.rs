pub struct Marble {
    pub name: &'static str,
    pub value: i32,
}

pub struct Bag<T> {
    contents: Vec<T>,
}

impl<T> Bag<T> {
    fn add(&mut self, item: T) {
        self.contents.push(item)
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn size(&self) -> usize {
        self.contents.len()
    }
}

impl<T> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_empty_bag() {
        let contents: Vec<Box<Marble>> = vec![];
        let bag = Bag { contents };
        assert_eq!(0, bag.size());
        assert_eq!(true, bag.is_empty());
    }

    #[test]
    fn add_one_marble() {
        let contents: Vec<Box<Marble>> = vec![];
        let mut bag = Bag { contents };
        assert_eq!(0, bag.size());
        assert_eq!(true, bag.is_empty());

        let marble = Marble {
            name: "ok",
            value: 77,
        };
        bag.add(Box::from(marble));
        assert_eq!(1, bag.size());
        assert_eq!(false, bag.is_empty());
    }

    #[test]
    fn value_sum() {
        let contents: Vec<Box<Marble>> = vec![];
        let mut bag = Bag { contents };
        let marble_a = Marble {
            name: "ok",
            value: 99,
        };

        let marble_b = Marble {
            name: "ok",
            value: 1,
        };

        bag.add(Box::from(marble_a));
        bag.add(Box::from(marble_b));
        assert_eq!(100, bag.into_iter().fold(0, |acc, m| acc + m.value))
    }
}
