use bevy::asset::StrongHandle;
use bevy::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Resource)]
pub(crate) struct AssetStore<K, V, F>
where
    K: Eq + Hash + Clone,
    V: Asset,
    F: Fn(&K) -> V,
{
    map: HashMap<K, Handle<V>>,
    create: F,
}

impl<K, V, F> AssetStore<K, V, F>
where
    K: Eq + Hash + Clone,
    V: Asset,
    F: Fn(&K) -> V,
{
    pub fn new(create: F) -> Self {
        Self {
            map: HashMap::new(),
            create,
        }
    }

    /// Get or create an asset by key, using the store's creation function.
    ///
    /// - `key`: The parameter describing this asset (e.g. mesh shape, material properties, etc.).
    /// - `assets`: A mutable reference to Bevy's `Assets<T>`.
    ///
    /// Returns a `Handle<V>` to the asset.
    pub fn get_or_create(&mut self, key: K, assets: &mut Assets<V>) -> Handle<V> {
        // If we already have a handle for this key, return it
        if let Some(existing_handle) = self.map.get(&key) {
            return existing_handle.clone_weak();
        }

        // Otherwise, create a new asset, store it in `assets`,
        // and record the handle in our map.
        let new_asset = (self.create)(&key);
        let new_handle = assets.add(new_asset);
        self.map.insert(key, new_handle.clone_weak());

        new_handle
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
