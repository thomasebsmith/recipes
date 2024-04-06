#[allow(dead_code)]
pub trait ModelCache {
    /// The type of ID that is used as a primary key for a model type.
    type Id;

    /// Retrieve an unused ID from the cache.
    fn unused_id() -> Self::Id;

    /// Mark the specified ID as used in the cache.
    fn use_id(id: Self::Id);
}
