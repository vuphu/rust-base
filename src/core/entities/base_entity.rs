use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait BaseEntity: DeserializeOwned + Serialize + Unpin + Send + Sync + Clone {}
