use bevy::asset::StrongHandle;
use bevy::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Resource)]
pub(crate) struct AssetStore<K, V>
where
    K: Eq + Hash + Clone,
    V: Asset,
{
    map: HashMap<K, Handle<V>>,
}

impl<K, V> AssetStore<K, V>
where
    K: Eq + Hash + Clone,
    V: Asset,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V, assets: &mut Assets<V>) -> Handle<V> {
        if let Some(handle) = self.map.get(&key) {
            return handle.clone();
        }

        let handle = assets.add(value);
        self.map.insert(key, handle.clone());
        handle
    }

    pub fn get(&mut self, key: &K) -> Option<Handle<V>> {
        self.map.get(key).cloned()
    }

    /// Removes all assets from the store that are no longer strongly referenced
    /// outside of the store. (i.e., if the only strong handle is in `map`.)
    pub fn clean_unused(&mut self, assets: &mut Assets<V>) {
        // Collect all keys whose handle has no other strong references
        // (strong_count == 1 means only the `map` owns the strong ref).
        let keys_to_remove: Vec<K> = self
            .map
            .iter()
            .filter_map(|(key, handle)| {
                let Handle::Strong(arc) = handle else {
                    return None;
                };
                if Arc::<StrongHandle>::strong_count(&arc) > 1 {
                    None
                } else {
                    Some(key.clone())
                }
            })
            .collect();

        // Remove them from both our store and the Assets resource.
        for key in keys_to_remove {
            if let Some(handle) = self.map.remove(&key) {
                assets.remove(handle.id());
            }
        }
    }
}
