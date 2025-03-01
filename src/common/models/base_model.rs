use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait BaseModel: DeserializeOwned + Serialize + Unpin + Send + Sync + Clone {}
