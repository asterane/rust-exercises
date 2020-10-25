use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    pub fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: U) -> V {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn multiple_types() {
        let mut c = Cacher::new(|a: &str| a.len());

        let v1 = c.value("abcd");
        let v2 = c.value("xyz");

        assert_eq!(v2, 3);
    }
}
