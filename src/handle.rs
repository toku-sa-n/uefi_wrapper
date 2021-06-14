//! A module containing the definition of [`Handle`].

use core::fmt;
use r_efi::efi;

/// The Handler type.
#[repr(transparent)]
#[allow(missing_copy_implementations)]
pub struct Handle(efi::Handle);
impl Handle {
    pub(crate) fn get_ptr(&self) -> efi::Handle {
        self.0
    }
}
impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle")
    }
}
