use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait BaseEntity: DeserializeOwned + Serialize + Unpin + Send + Sync + Clone {}
