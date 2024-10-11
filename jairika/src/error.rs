/// A custom error type for the library.
/// It includes various variants to represent different types of errors that can occur.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    InvalidModelAddress(String),
    LayerAdditionFailed(String),
    LayerRemovalFailed(String),
    ModelAddressNotFound(String),
    ModelCreationFailed(String),
    ModelInstantiationFailed(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidModelAddress(addr) => {
                write!(f, "The model's address is not a directory: \"{}\".", addr)
            }
            Error::LayerAdditionFailed(msg) => {
                write!(
                    f,
                    "An error occurred when adding a layer to the model: {}",
                    msg
                )
            }
            Error::LayerRemovalFailed(msg) => {
                write!(
                    f,
                    "An error occurred when removing a layer from the model: {}",
                    msg
                )
            }
            Error::ModelAddressNotFound(addr) => {
                write!(
                    f,
                    "The model's address does not exist or cannot be accessed: \"{}\".",
                    addr
                )
            }
            Error::ModelInstantiationFailed(msg) => {
                write!(f, "An error occurred when instantiating the model: {}", msg)
            }
            Error::ModelCreationFailed(msg) => {
                write!(f, "An error occurred when creating the model: {}", msg)
            }
        }
    }
}

impl std::error::Error for Error {}

/// A type alias for `std::result::Result` with the `Error` type in the library.
pub type Result<T> = std::result::Result<T, Error>;
