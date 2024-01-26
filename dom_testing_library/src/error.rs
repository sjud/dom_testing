use super::*;

#[derive(Error, Debug, PartialEq)]
pub enum GetOneError {
    #[error("Not Found:Attempting to find: {ident} by method {method}")]
    NotFound { method: &'static str, ident: String },
    #[error("Found more than one element by method of get_{method} with input of {ident}, if you were expecting more than one match see the get_all_{method} version of this method instead.")]
    MoreThanOne { method: &'static str, ident: String },
}

impl GetOneError {
    pub(crate) fn more_than_one(method: &'static str, ident: String) -> Self {
        Self::MoreThanOne { method, ident }
    }
    pub(crate) fn not_found(method: &'static str, ident: String) -> Self {
        Self::NotFound { method, ident }
    }
}
pub trait GetOneErrorTrait {
    fn is_not_found(&self) -> bool;
    fn is_more_than_one(&self) -> bool;
}

impl GetOneErrorTrait for GetOneError {
    fn is_not_found(&self) -> bool {
        matches!(self, GetOneError::NotFound { .. })
    }
    fn is_more_than_one(&self) -> bool {
        matches!(self, GetOneError::MoreThanOne { .. })
    }
}

impl<T> GetOneErrorTrait for Result<T, GetOneError> {
    fn is_not_found(&self) -> bool {
        match &self {
            Ok(_) => false,
            Err(err) => matches!(err, GetOneError::NotFound { .. }),
        }
    }
    fn is_more_than_one(&self) -> bool {
        match &self {
            Ok(_) => false,
            Err(err) => matches!(err, GetOneError::MoreThanOne { .. }),
        }
    }
}
