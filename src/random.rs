//! Functions for iterating over the cache.
use std::path::Path;

use crate::errors::Result;
use crate::index;

/// Returns a synchronous random entry.
pub fn random_sync<P: AsRef<Path>>(cache: P) -> Result<Option<index::Metadata>> {
    index::random(cache.as_ref())
}

/// Returns a random entry.
pub async fn random<P: AsRef<Path>>(cache: P) -> Result<Option<index::Metadata>> {
    index::random_async(cache.as_ref()).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "async-std")]
    use async_attributes::test as async_test;
    #[cfg(feature = "tokio")]
    use tokio::test as async_test;

    #[test]
    fn test_random_sync() {
        // check that the public interface to random element
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().to_owned();
        let metadata = random_sync(dir).unwrap();

        assert!(metadata.is_none());
    }

    #[async_test]
    async fn test_random() {
        // check that the public interface to random element
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().to_owned();

        let metadata = random(dir).await.unwrap();

        assert!(metadata.is_none());
    }
}
