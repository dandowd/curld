#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Storage {
    fn write(&self, content: &str);
    fn get(&self) -> Option<String>;
}
