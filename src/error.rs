use crate::status;
use core::convert::TryInto;
use r_efi::efi;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Error<T> {
    status: status::NotSuccess,
    value: T,
}
impl<T> Error<T> {
    pub fn status(&self) -> &status::NotSuccess {
        &self.status
    }

    pub fn value(&self) -> &T {
        &self.value
    }

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
