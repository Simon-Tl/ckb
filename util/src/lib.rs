mod linked_hash_set;

use std::time::Duration;

pub use fnv::{FnvBuildHasher, FnvHashMap, FnvHashSet};
pub use linked_hash_map::{Entries as LinkedHashMapEntries, LinkedHashMap};
pub use linked_hash_set::LinkedHashSet;

pub type LinkedFnvHashMap<K, V> = LinkedHashMap<K, V, FnvBuildHasher>;
pub type LinkedFnvHashMapEntries<'a, K, V> = LinkedHashMapEntries<'a, K, V, FnvBuildHasher>;
pub type LinkedFnvHashSet<T> = LinkedHashSet<T, FnvBuildHasher>;

pub use parking_lot::{
    self, Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};

const TRY_LOCK_TIMEOUT: Duration = Duration::from_secs(300);

pub fn lock_or_panic<T>(data: &Mutex<T>) -> MutexGuard<T> {
    data.try_lock_for(TRY_LOCK_TIMEOUT)
        .expect("please check if reach a deadlock")
}
