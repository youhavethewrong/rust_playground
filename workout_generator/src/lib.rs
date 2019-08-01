use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq,
    U: Hash,
    U: Copy,
    V: Copy,
{
    calculation: T,
    value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq,
    U: Hash,
    U: Copy,
    V: Copy,
{
    pub fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: U) -> V {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_cache_different_values() {
        let mut c = Cacher::new(|a: u32| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn can_cache_more_than_numbers() {
        let mut c = Cacher::new(|a: &str| a);
        let v = c.value("Pizza");
        assert_eq!(v, "Pizza");
    }
}
