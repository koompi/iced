use super::error::Result;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub trait TableData:
    'static + Default + Clone + Ord + Serialize + DeserializeOwned
{
    fn get_field_value(&self, field_name: &str) -> Result<Value>;
}

// #[derive(Debug)]
// enum TableEvent {
//    SortColumn(usize),
// }
