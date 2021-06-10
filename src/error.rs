/// A module containing [`Error`] type definition.
use crate::status;
use core::convert::TryInto;
use r_efi::efi;

/// An error type returned by functions of this crate.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Error<T> {
    status: status::NotSuccess,
    value: T,
}
impl<T> Error<T> {
    /// Returns the status value.
    pub fn status(&self) -> &status::NotSuccess {
        &self.status
    }

    /// Returns the value which is included when the instance of [`Error`] is created.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Maps the value included in this [`Error`].
    pub fn map_value<U>(self, f: impl FnOnce(T) -> U) -> Error<U> {
        Error {
            status: self.status,
            value: f(self.value),
        }
    }

    pub(crate) fn from_status_and_value(status: efi::Status, value: T) -> Self {
        Self {
            status: status.try_into().expect("`SUCCESS` is passed."),
            value,
        }
    }
}
impl From<efi::Status> for Error<()> {
    fn from(s: efi::Status) -> Self {
        Self::from_status_and_value(s, ())
    }
}
