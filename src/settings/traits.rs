#[cfg(test)]
use mockall::automock;
use serde::{de, Serialize};

#[cfg_attr(test, automock)]
pub trait Storage {
    fn write(&self, content: &str);
    fn get(&self) -> Option<String>;
}

#[cfg_attr(test, automock)]
pub trait StoredSettings<T>
where
    T: de::DeserializeOwned + Serialize,
{
    fn get_module(&self, module_name: &str) -> Option<T>;
    fn insert_module(&mut self, module_name: &str, settings: &T);
}
