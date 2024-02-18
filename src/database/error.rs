use std::fmt;

#[derive(Debug)]
pub enum Error {
    BadArguments(String),
    Internal(String),
    Sql(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sql(value)
    }
}

impl fmt::Display for Error {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        match self {
            Self::BadArguments(message) => {
                write!(formatter, "Bad arguments: {message}")
            }
            Self::Internal(message) => {
                write!(formatter, "Internal error: {message}")
            }
            Self::Sql(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for Error {}

pub fn to_internal_db_error<E>(error: E) -> Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    Error::Internal(error.into().to_string())
}
