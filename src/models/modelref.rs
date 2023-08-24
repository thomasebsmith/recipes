use serde::{ser::SerializeMap, Serialize, Serializer};
use sqlx::{Any, Transaction};

use super::Model;
use crate::database::DBResult;

/// A reference to a model.
///
/// The model is referenced using its internal ID.
///
/// At any point in time, a `Ref` either holds a cached version of the
/// referenced model (and its ID) or holds only the ID.
pub struct Ref<M: Model> {
    id: M::ID,
    value: Option<M>,
}

impl<M: Model> Ref<M> {
    /// Creates a reference to a model with the ID `id`.
    ///
    /// The reference initially holds only the ID and not a cached version of
    /// the referenced model.
    pub fn new(id: M::ID) -> Self {
        Self { id, value: None }
    }

    /// Attempts to retrieve the referenced model from the database using
    /// `transaction`.
    ///
    /// If successful, the model is cached in this struct and returned.
    pub async fn query(
        &mut self,
        transaction: &mut Transaction<'_, Any>,
    ) -> DBResult<&M> {
        if let Some(ref value) = self.value {
            Ok(value)
        } else {
            let value = M::get(transaction, self.id).await?;
            self.value = Some(value);
            Ok(self.value.as_ref().unwrap())
        }
    }

    pub async fn fill(
        &mut self,
        transaction: &mut Transaction<'_, Any>,
    ) -> DBResult<()> {
        self.query(transaction).await.map(|_| ())
    }
}

impl<M: Model> Serialize for Ref<M>
where
    M::ID: Serialize,
{
    fn serialize<S: Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(ref value) = self.value {
            value.serialize(serializer)
        } else {
            let mut map = serializer.serialize_map(Some(1))?;
            map.serialize_entry("id", &self.id)?;
            map.end()
        }
    }
}
