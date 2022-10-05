use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::database::entities;

#[derive(Default)]
pub struct Caches {
    pub user: Cache<entities::user::Entity>,
    pub group: Cache<entities::group::Entity>,
    pub group_member: Cache<entities::group_member::Entity>,
    pub platform_role: Cache<entities::platform_role::Entity>,
    pub group_role: Cache<entities::group_role::Entity>,
    pub group_file_share: Cache<entities::group_file_share::Entity>,
    pub user_file_share: Cache<entities::user_file_share::Entity>,
}

pub type CacheMap<E> = Arc<Mutex<HashMap<Uuid, Arc<E>>>>;

pub struct Cache<E> {
    inner: CacheMap<E>,
}

impl<E> Clone for Cache<E> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<E> Default for Cache<E> {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<E> Cache<E>
where
    Self: std::marker::Send,
    E: std::marker::Send + Sync,
{
    pub fn cache_map(&self) -> CacheMap<E> {
        self.inner.clone()
    }

    pub async fn all_cached(&self) -> Vec<Uuid> {
        self.inner
            .lock()
            .await
            .keys()
            .into_iter()
            .cloned()
            .collect()
    }

    pub async fn get_all(&self, ids: &[Uuid]) -> Vec<Arc<E>> {
        let _cache_map = self.cache_map();
        let cache = _cache_map.lock().await;
        ids.iter().fold(Vec::new(), |mut acc, id| {
            if let Some(item) = cache.get(id) {
                acc.push(item.clone());
            }

            acc
        })
    }
}
