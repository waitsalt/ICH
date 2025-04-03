pub mod ich;
pub mod post;
pub mod user;

use crate::util::error::AppError;

type SqlResult<T> = Result<T, AppError>;
