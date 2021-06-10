//! Protocol definitions.

pub mod console;
pub mod media;

use r_efi::efi;

/// Types that represent UEFI protocols.
///
/// # Safety
///
/// Types that implement this trait must have the same structure as the protocol interface.
pub unsafe trait Protocol {
    /// The GUID of the protocol.
    const GUID: efi::Guid;
}
