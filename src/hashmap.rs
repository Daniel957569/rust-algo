use std::ops::Deref;

#[derive(Debug, Clone)]
pub enum ReturnParent<'a, T> {
    String(String),
    Hash(&'a Hash<T>),
}

#[derive(Debug, Clone)]
enum ReturnChild<'a, T> {
    String(String),
    Hash(&'a Hash<T>),
}

#[derive(Debug, Clone)]

pub struct HashMap<T> {
    hashs: Option<Vec<Hash<T>>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Hash<T> {
    key: i32,
    value: T,
}

impl<T> HashMap<T> {
    pub fn new() -> Self {
        HashMap { hashs: None }
    }

    pub fn add(&mut self, key: i32, value: T) {
        match &mut self.hashs {
            None => {
                let list = vec![Hash::new(key, value)];
                self.hashs = Option::Some(list);
            }
            Some(n) => n.push(Hash::new(key, value)),
        }
    }

    pub fn remove(&mut self, index: usize) {
        match &mut self.hashs {
            None => {}
            Some(n) => {
                n.remove(index - 1);
                return;
            }
        }
    }

    pub fn find_by_key(&self, key: i32) -> ReturnParent<T> {
        match &self.hashs {
            None => ReturnParent::String(String::from("hash wasn't found")),
            Some(n) => match HashMap::find_recusively(n, 0, key) {
                ReturnChild::String(message) => ReturnParent::String(message),
                ReturnChild::Hash(hash) => ReturnParent::Hash(hash),
            },
        }
    }

    fn find_recusively(hash: &Vec<Hash<T>>, index: usize, key: i32) -> ReturnChild<T> {
        // println!("{}", index);
        match hash.get(index) {
            None => ReturnChild::String(String::from("value wasn't found")),
            Some(n) => {
                if n.key == key {
                    return ReturnChild::Hash(n);
                } else {
                    HashMap::find_recusively(hash, index + 1, key)
                }
            }
        }
    }
}

impl<T> Hash<T> {
    fn new(key: i32, value: T) -> Hash<T> {
        Hash { key, value }
    }
}
