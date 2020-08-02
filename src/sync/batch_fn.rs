use std::collections::HashMap;

pub trait BatchFn<K, V> {
    fn load(&self, keys: &[K]) -> HashMap<K, V>;
}
