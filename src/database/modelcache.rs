#[allow(dead_code)]
pub trait ModelCache {
    type Id;

    fn unused_id() -> Self::Id;

    fn use_id(id: Self::Id);
}
