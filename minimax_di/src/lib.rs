use service_collection::ServiceCollectionImpl;
use service_traits::{ServiceCollection, ServiceProviderBuilder};

pub mod service_collection;
pub mod service_provider;
pub mod service_traits;

pub fn new_service_collection() -> Box<dyn ServiceCollection> {
    Box::new(ServiceCollectionImpl::new())
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(5, add(2, 3));
    }
}
