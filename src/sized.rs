//! This is an experimental API that allows for a dynamically sized decimal
use crate::decimal::DecimalLike;
use crate::Decimal;

// This is internal, but it'd be nice to add constraints on this. e.g. > 3. The alternative is to artificially
// constraint by ensuring SIZE is always SIZE + 3
pub(crate) struct VariableSizedDecimal<const SIZE: usize> {
    flags: u32,
    mantissa: [u32; SIZE],
}

impl<const SIZE: usize> From<&crate::Decimal> for VariableSizedDecimal<SIZE> {
    fn from(value: &Decimal) -> Self {
        VariableSizedDecimal {
            flags: value.flags(),
            mantissa: init_array(value.mantissa_array3()),
        }
    }
}

impl<const SIZE: usize> From<VariableSizedDecimal<SIZE>> for Decimal {
    fn from(value: VariableSizedDecimal<SIZE>) -> Self {
        // TODO: Rounding for underflow / overflow
        Decimal::from_parts_raw(value.lo(), value.mid(), value.hi(), value.flags)
    }
}

impl<const SIZE: usize> DecimalLike for VariableSizedDecimal<SIZE> {
    const ZERO: Self = VariableSizedDecimal {
        flags: 0,
        mantissa: [0u32; SIZE],
    };

    fn is_zero(&self) -> bool {
        self.mantissa.iter().all(|i| *i == 0)
    }

    // TODO: These could be shared
    fn is_sign_negative(&self) -> bool {
        self.flags & crate::constants::SIGN_MASK > 0
    }
    fn is_sign_positive(&self) -> bool {
        self.flags & crate::constants::SIGN_MASK == 0
    }
    fn scale(&self) -> u32 {
        ((self.flags & crate::constants::SCALE_MASK) >> crate::constants::SCALE_SHIFT) as u32
    }

    // TODO: Refactor these
    fn from_parts(lo: u32, mid: u32, hi: u32, negative: bool, scale: u32) -> Self {
        VariableSizedDecimal {
            flags: (scale << crate::constants::SCALE_SHIFT) | ((negative as u32) << crate::constants::SIGN_SHIFT),
            mantissa: init_array([lo, mid, hi]),
        }
    }
    fn lo(&self) -> u32 {
        self.mantissa[0]
    }
    fn mid(&self) -> u32 {
        self.mantissa[1]
    }
    fn hi(&self) -> u32 {
        self.mantissa[2]
    }
}

fn init_array<const SIZE: usize>(init: [u32; 3]) -> [u32; SIZE] {
    let mut mantissa = [0u32; SIZE];
    mantissa[0] = init[0];
    mantissa[1] = init[1];
    mantissa[2] = init[2];
    mantissa
}
