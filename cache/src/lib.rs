//设计一个 hashmap缓存
// new() 关联函数
// get，put 方法
// add ,remove 方法
// clear 方法

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
struct Cache<K, V> {
    cache_map: HashMap<K, V>,
}

impl<K, V> Cache<K, V> {
    fn new() -> Self {
        let mut hash_map = HashMap::new();
        Self {
            cache_map: hash_map,
        }
    }

    fn get_value(&self, key: &K) -> Option<&V>
    where
        K: Eq + Hash,
    {
        self.cache_map.get(key)
    }

    fn set(&mut self, key: K, value: V) -> Option<V>
    where
        K: Eq + Hash,
    {
        self.cache_map.insert(key, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
    #[test]
    fn new_cache() {
        let mut cache: Cache<&str, u32> = Cache::new();
    }

    #[test]
    fn get_from() {
        let mut cache: Cache<&str, u32> = Cache::new();
        cache.cache_map.insert("json", 12);
        assert_eq!(cache.get_value(&"json"), Some(&12u32));
    }

    #[test]
    fn set_test() {
        let mut cache: Cache<&str, u32> = Cache::new();
        cache.set("json", 12);
        assert_eq!(cache.get_value(&"json"), Some(&12u32));
    }
}
