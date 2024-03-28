use std::future::Future;

use super::Error;

/// The result of a database query or operation. Contains either the result of
/// the successful operation, or a database error.
pub type DBResult<T> = Result<T, Error>;

/// Trait alias for a future that returns a `DBResult<T>`.
pub trait DBFut<T>: Future<Output = DBResult<T>> {}
impl<T, U: Future<Output = DBResult<T>>> DBFut<T> for U {}
