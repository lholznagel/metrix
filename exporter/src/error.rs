use cachem::CachemError;

/// All errors that can be thrown in this module
#[derive(Debug)]
pub enum MetrixError {
    // There was an error with the database connection pool
    DbConnectionPoolError(cachem::CachemError),
    // There was an error with the database protocol
    DbProtocolError(cachem::CachemError),
}
impl std::error::Error for MetrixError {}

impl std::fmt::Display for MetrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) 
    }
}

impl From<cachem::CachemError> for MetrixError {
    fn from(x: cachem::CachemError) -> Self {
        match x {
            CachemError::ConnectionPoolError(_) => Self::DbConnectionPoolError(x),
            _ => Self::DbProtocolError(x),
        }
    }
}
