use std::{sync::Arc, collections::HashMap};

use tokio::sync::Mutex;
use uuid::Uuid;


pub type Cache<E> = Arc<Mutex<HashMap<Uuid, Arc<E>>>>;

#[async_trait::async_trait]
pub trait HasCache<E>
where
    Self: std::marker::Send,
    E: std::marker::Send + Sync,
{
    fn cache(&mut self) -> Cache<E>;

    async fn all_cached(&mut self) -> Vec<Uuid> {
        self.cache().lock().await
            .keys()
            .into_iter().cloned()
            .collect()
    }

    async fn get_all(&mut self, ids: &[Uuid]) -> Vec<Arc<E>> {
        let _cache = self.cache();
        let cache = _cache.lock().await;
        ids.iter().fold(Vec::new(), |mut acc, id| {
            if let Some(item) = cache.get(id) {
                acc.push(item.clone());
            }

            acc
        })
    }
}
