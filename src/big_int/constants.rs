pub(crate) const MIN_REPRESENTABLE_BASE: u8 = 2;
pub(crate) const MAX_REPRESENTABLE_BASE: u8 = 36;
pub(super) const MIDDLE_BYTE: u8 = 1u8 << (u8::BITS - 1);
pub(super) const WINDOW_CUTOFF: usize = 8;
pub(super) const WINDOW_SHIFT: usize = 5;
pub(super) const WINDOW_BASE: usize = 1 << WINDOW_SHIFT;
