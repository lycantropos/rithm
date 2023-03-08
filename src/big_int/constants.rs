pub(crate) const MAX_REPRESENTABLE_BASE: u8 = 36;
pub(super) const MIDDLE_BYTE: u8 = 1u8 << (u8::BITS - 1);
pub(crate) const MIN_REPRESENTABLE_BASE: u8 = 2;
pub(super) const WINDOW_BASE: usize = 1 << WINDOW_BITNESS;
pub(super) const WINDOW_BITNESS: usize = 5;
pub(super) const WINDOW_CUTOFF: usize = 8;
