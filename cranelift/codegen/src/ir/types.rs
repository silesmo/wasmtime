//! Common types for the Cranelift code generator.

use core::default::Default;
use core::fmt::{self, Debug, Display, Formatter};
use target_lexicon::{PointerWidth, Triple};

/// The type of an SSA value.
///
/// The `INVALID` type isn't a real type, and is used as a placeholder in the IR where a type
/// field is present put no type is needed, such as the controlling type variable for a
/// non-polymorphic instruction.
///
/// Basic integer types: `I8`, `I16`, `I32`, `I64`, and `I128`. These types are sign-agnostic.
///
/// Basic floating point types: `F32` and `F64`. IEEE single and double precision.
///
/// Boolean types: `B1`, `B8`, `B16`, `B32`, `B64`, and `B128`. These all encode 'true' or 'false'. The
/// larger types use redundant bits.
///
/// SIMD vector types have power-of-two lanes, up to 256. Lanes can be any int/float/bool type.
///
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Type(u8);

/// Not a valid type. Can't be loaded or stored. Can't be part of a SIMD vector.
pub const INVALID: Type = Type(0);

/// Start of the lane types. See also `meta/src/cdsl/types.rs`.
const LANE_BASE: u8 = 0x70;

/// Start of the 2-lane vector types.
const VECTOR_BASE: u8 = LANE_BASE + 16;

// Include code generated by `cranelift-codegen/meta/gen_types.rs`. This file contains constant
// definitions for all the scalar types as well as common vector types for 64, 128, 256, and
// 512-bit SIMD vectors.
include!(concat!(env!("OUT_DIR"), "/types.rs"));

impl Type {
    /// Get the lane type of this SIMD vector type.
    ///
    /// A lane type is the same as a SIMD vector type with one lane, so it returns itself.
    pub fn lane_type(self) -> Self {
        if self.0 < VECTOR_BASE {
            self
        } else {
            Self(LANE_BASE | (self.0 & 0x0f))
        }
    }

    /// The type transformation that returns the lane type of a type variable; it is just a
    /// renaming of lane_type() to be used in context where we think in terms of type variable
    /// transformations.
    pub fn lane_of(self) -> Self {
        self.lane_type()
    }

    /// Get log_2 of the number of bits in a lane.
    pub fn log2_lane_bits(self) -> u8 {
        match self.lane_type() {
            B1 => 0,
            B8 | I8 => 3,
            B16 | I16 => 4,
            B32 | I32 | F32 | R32 => 5,
            B64 | I64 | F64 | R64 => 6,
            B128 | I128 => 7,
            _ => 0,
        }
    }

    /// Get the number of bits in a lane.
    pub fn lane_bits(self) -> u8 {
        match self.lane_type() {
            B1 => 1,
            B8 | I8 => 8,
            B16 | I16 => 16,
            B32 | I32 | F32 | R32 => 32,
            B64 | I64 | F64 | R64 => 64,
            B128 | I128 => 128,
            _ => 0,
        }
    }

    /// Get an integer type with the requested number of bits.
    pub fn int(bits: u16) -> Option<Self> {
        match bits {
            8 => Some(I8),
            16 => Some(I16),
            32 => Some(I32),
            64 => Some(I64),
            128 => Some(I128),
            _ => None,
        }
    }

    /// Get a type with the same number of lanes as `self`, but using `lane` as the lane type.
    fn replace_lanes(self, lane: Self) -> Self {
        debug_assert!(lane.is_lane() && !self.is_special());
        Type((lane.0 & 0x0f) | (self.0 & 0xf0))
    }

    /// Get a type with the same number of lanes as this type, but with the lanes replaced by
    /// booleans of the same size.
    ///
    /// Lane types are treated as vectors with one lane, so they are converted to the multi-bit
    /// boolean types.
    pub fn as_bool_pedantic(self) -> Self {
        // Replace the low 4 bits with the boolean version, preserve the high 4 bits.
        self.replace_lanes(match self.lane_type() {
            B8 | I8 => B8,
            B16 | I16 => B16,
            B32 | I32 | F32 => B32,
            B64 | I64 | F64 => B64,
            R32 | R64 => panic!("Reference types should not convert to bool"),
            B128 | I128 => B128,
            _ => B1,
        })
    }

    /// Get a type with the same number of lanes as this type, but with the lanes replaced by
    /// booleans of the same size.
    ///
    /// Scalar types are all converted to `b1` which is usually what you want.
    pub fn as_bool(self) -> Self {
        if !self.is_vector() {
            B1
        } else {
            self.as_bool_pedantic()
        }
    }

    /// Get a type with the same number of lanes as this type, but with lanes that are half the
    /// number of bits.
    pub fn half_width(self) -> Option<Self> {
        Some(self.replace_lanes(match self.lane_type() {
            I16 => I8,
            I32 => I16,
            I64 => I32,
            I128 => I64,
            F64 => F32,
            B16 => B8,
            B32 => B16,
            B64 => B32,
            B128 => B64,
            _ => return None,
        }))
    }

    /// Get a type with the same number of lanes as this type, but with lanes that are twice the
    /// number of bits.
    pub fn double_width(self) -> Option<Self> {
        Some(self.replace_lanes(match self.lane_type() {
            I8 => I16,
            I16 => I32,
            I32 => I64,
            I64 => I128,
            F32 => F64,
            B8 => B16,
            B16 => B32,
            B32 => B64,
            B64 => B128,
            _ => return None,
        }))
    }

    /// Is this the INVALID type?
    pub fn is_invalid(self) -> bool {
        self == INVALID
    }

    /// Is this a special type?
    pub fn is_special(self) -> bool {
        self.0 < LANE_BASE
    }

    /// Is this a lane type?
    ///
    /// This is a scalar type that can also appear as the lane type of a SIMD vector.
    pub fn is_lane(self) -> bool {
        LANE_BASE <= self.0 && self.0 < VECTOR_BASE
    }

    /// Is this a SIMD vector type?
    ///
    /// A vector type has 2 or more lanes.
    pub fn is_vector(self) -> bool {
        self.0 >= VECTOR_BASE
    }

    /// Is this a scalar boolean type?
    pub fn is_bool(self) -> bool {
        match self {
            B1 | B8 | B16 | B32 | B64 | B128 => true,
            _ => false,
        }
    }

    /// Is this a scalar integer type?
    pub fn is_int(self) -> bool {
        match self {
            I8 | I16 | I32 | I64 | I128 => true,
            _ => false,
        }
    }

    /// Is this a scalar floating point type?
    pub fn is_float(self) -> bool {
        match self {
            F32 | F64 => true,
            _ => false,
        }
    }

    /// Is this a CPU flags type?
    pub fn is_flags(self) -> bool {
        match self {
            IFLAGS | FFLAGS => true,
            _ => false,
        }
    }

    /// Is this a ref type?
    pub fn is_ref(self) -> bool {
        match self {
            R32 | R64 => true,
            _ => false,
        }
    }

    /// Get log_2 of the number of lanes in this SIMD vector type.
    ///
    /// All SIMD types have a lane count that is a power of two and no larger than 256, so this
    /// will be a number in the range 0-8.
    ///
    /// A scalar type is the same as a SIMD vector type with one lane, so it returns 0.
    pub fn log2_lane_count(self) -> u8 {
        self.0.saturating_sub(LANE_BASE) >> 4
    }

    /// Get the number of lanes in this SIMD vector type.
    ///
    /// A scalar type is the same as a SIMD vector type with one lane, so it returns 1.
    pub fn lane_count(self) -> u16 {
        1 << self.log2_lane_count()
    }

    /// Get the total number of bits used to represent this type.
    pub fn bits(self) -> u16 {
        u16::from(self.lane_bits()) * self.lane_count()
    }

    /// Get the number of bytes used to store this type in memory.
    pub fn bytes(self) -> u32 {
        (u32::from(self.bits()) + 7) / 8
    }

    /// Get a SIMD vector type with `n` times more lanes than this one.
    ///
    /// If this is a scalar type, this produces a SIMD type with this as a lane type and `n` lanes.
    ///
    /// If this is already a SIMD vector type, this produces a SIMD vector type with `n *
    /// self.lane_count()` lanes.
    pub fn by(self, n: u16) -> Option<Self> {
        if self.lane_bits() == 0 || !n.is_power_of_two() {
            return None;
        }
        let log2_lanes: u32 = n.trailing_zeros();
        let new_type = u32::from(self.0) + (log2_lanes << 4);
        if new_type < 0x100 {
            Some(Type(new_type as u8))
        } else {
            None
        }
    }

    /// Get a SIMD vector with half the number of lanes.
    ///
    /// There is no `double_vector()` method. Use `t.by(2)` instead.
    pub fn half_vector(self) -> Option<Self> {
        if self.is_vector() {
            Some(Type(self.0 - 0x10))
        } else {
            None
        }
    }

    /// Index of this type, for use with hash tables etc.
    pub fn index(self) -> usize {
        usize::from(self.0)
    }

    /// True iff:
    ///
    /// 1. `self.lane_count() == other.lane_count()` and
    /// 2. `self.lane_bits() >= other.lane_bits()`
    pub fn wider_or_equal(self, other: Self) -> bool {
        self.lane_count() == other.lane_count() && self.lane_bits() >= other.lane_bits()
    }

    /// Return the pointer type for the given target triple.
    pub fn triple_pointer_type(triple: &Triple) -> Self {
        match triple.pointer_width() {
            Ok(PointerWidth::U16) => I16,
            Ok(PointerWidth::U32) => I32,
            Ok(PointerWidth::U64) => I64,
            Err(()) => panic!("unable to determine architecture pointer width"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_bool() {
            write!(f, "b{}", self.lane_bits())
        } else if self.is_int() {
            write!(f, "i{}", self.lane_bits())
        } else if self.is_float() {
            write!(f, "f{}", self.lane_bits())
        } else if self.is_vector() {
            write!(f, "{}x{}", self.lane_type(), self.lane_count())
        } else if self.is_ref() {
            write!(f, "r{}", self.lane_bits())
        } else {
            f.write_str(match *self {
                IFLAGS => "iflags",
                FFLAGS => "fflags",
                INVALID => panic!("INVALID encountered"),
                _ => panic!("Unknown Type(0x{:x})", self.0),
            })
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_bool() {
            write!(f, "types::B{}", self.lane_bits())
        } else if self.is_int() {
            write!(f, "types::I{}", self.lane_bits())
        } else if self.is_float() {
            write!(f, "types::F{}", self.lane_bits())
        } else if self.is_vector() {
            write!(f, "{:?}X{}", self.lane_type(), self.lane_count())
        } else if self.is_ref() {
            write!(f, "types::R{}", self.lane_bits())
        } else {
            match *self {
                INVALID => write!(f, "types::INVALID"),
                IFLAGS => write!(f, "types::IFLAGS"),
                FFLAGS => write!(f, "types::FFLAGS"),
                _ => write!(f, "Type(0x{:x})", self.0),
            }
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        INVALID
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn basic_scalars() {
        assert_eq!(INVALID, INVALID.lane_type());
        assert_eq!(0, INVALID.bits());
        assert_eq!(IFLAGS, IFLAGS.lane_type());
        assert_eq!(0, IFLAGS.bits());
        assert_eq!(FFLAGS, FFLAGS.lane_type());
        assert_eq!(0, FFLAGS.bits());
        assert_eq!(B1, B1.lane_type());
        assert_eq!(B8, B8.lane_type());
        assert_eq!(B16, B16.lane_type());
        assert_eq!(B32, B32.lane_type());
        assert_eq!(B64, B64.lane_type());
        assert_eq!(B128, B128.lane_type());
        assert_eq!(I8, I8.lane_type());
        assert_eq!(I16, I16.lane_type());
        assert_eq!(I32, I32.lane_type());
        assert_eq!(I64, I64.lane_type());
        assert_eq!(I128, I128.lane_type());
        assert_eq!(F32, F32.lane_type());
        assert_eq!(F64, F64.lane_type());
        assert_eq!(B1, B1.by(8).unwrap().lane_type());
        assert_eq!(I32, I32X4.lane_type());
        assert_eq!(F64, F64X2.lane_type());
        assert_eq!(R32, R32.lane_type());
        assert_eq!(R64, R64.lane_type());

        assert_eq!(INVALID.lane_bits(), 0);
        assert_eq!(IFLAGS.lane_bits(), 0);
        assert_eq!(FFLAGS.lane_bits(), 0);
        assert_eq!(B1.lane_bits(), 1);
        assert_eq!(B8.lane_bits(), 8);
        assert_eq!(B16.lane_bits(), 16);
        assert_eq!(B32.lane_bits(), 32);
        assert_eq!(B64.lane_bits(), 64);
        assert_eq!(B128.lane_bits(), 128);
        assert_eq!(I8.lane_bits(), 8);
        assert_eq!(I16.lane_bits(), 16);
        assert_eq!(I32.lane_bits(), 32);
        assert_eq!(I64.lane_bits(), 64);
        assert_eq!(I128.lane_bits(), 128);
        assert_eq!(F32.lane_bits(), 32);
        assert_eq!(F64.lane_bits(), 64);
        assert_eq!(R32.lane_bits(), 32);
        assert_eq!(R64.lane_bits(), 64);
    }

    #[test]
    fn typevar_functions() {
        assert_eq!(INVALID.half_width(), None);
        assert_eq!(INVALID.half_width(), None);
        assert_eq!(FFLAGS.half_width(), None);
        assert_eq!(B1.half_width(), None);
        assert_eq!(B8.half_width(), None);
        assert_eq!(B16.half_width(), Some(B8));
        assert_eq!(B32.half_width(), Some(B16));
        assert_eq!(B64.half_width(), Some(B32));
        assert_eq!(B128.half_width(), Some(B64));
        assert_eq!(I8.half_width(), None);
        assert_eq!(I16.half_width(), Some(I8));
        assert_eq!(I32.half_width(), Some(I16));
        assert_eq!(I32X4.half_width(), Some(I16X4));
        assert_eq!(I64.half_width(), Some(I32));
        assert_eq!(I128.half_width(), Some(I64));
        assert_eq!(F32.half_width(), None);
        assert_eq!(F64.half_width(), Some(F32));

        assert_eq!(INVALID.double_width(), None);
        assert_eq!(IFLAGS.double_width(), None);
        assert_eq!(FFLAGS.double_width(), None);
        assert_eq!(B1.double_width(), None);
        assert_eq!(B8.double_width(), Some(B16));
        assert_eq!(B16.double_width(), Some(B32));
        assert_eq!(B32.double_width(), Some(B64));
        assert_eq!(B64.double_width(), Some(B128));
        assert_eq!(B128.double_width(), None);
        assert_eq!(I8.double_width(), Some(I16));
        assert_eq!(I16.double_width(), Some(I32));
        assert_eq!(I32.double_width(), Some(I64));
        assert_eq!(I32X4.double_width(), Some(I64X4));
        assert_eq!(I64.double_width(), Some(I128));
        assert_eq!(I128.double_width(), None);
        assert_eq!(F32.double_width(), Some(F64));
        assert_eq!(F64.double_width(), None);
    }

    #[test]
    fn vectors() {
        let big = F64.by(256).unwrap();
        assert_eq!(big.lane_bits(), 64);
        assert_eq!(big.lane_count(), 256);
        assert_eq!(big.bits(), 64 * 256);

        assert_eq!(big.half_vector().unwrap().to_string(), "f64x128");
        assert_eq!(B1.by(2).unwrap().half_vector().unwrap().to_string(), "b1");
        assert_eq!(I32.half_vector(), None);
        assert_eq!(INVALID.half_vector(), None);

        // Check that the generated constants match the computed vector types.
        assert_eq!(I32.by(4), Some(I32X4));
        assert_eq!(F64.by(8), Some(F64X8));
    }

    #[test]
    fn format_scalars() {
        assert_eq!(IFLAGS.to_string(), "iflags");
        assert_eq!(FFLAGS.to_string(), "fflags");
        assert_eq!(B1.to_string(), "b1");
        assert_eq!(B8.to_string(), "b8");
        assert_eq!(B16.to_string(), "b16");
        assert_eq!(B32.to_string(), "b32");
        assert_eq!(B64.to_string(), "b64");
        assert_eq!(B128.to_string(), "b128");
        assert_eq!(I8.to_string(), "i8");
        assert_eq!(I16.to_string(), "i16");
        assert_eq!(I32.to_string(), "i32");
        assert_eq!(I64.to_string(), "i64");
        assert_eq!(I128.to_string(), "i128");
        assert_eq!(F32.to_string(), "f32");
        assert_eq!(F64.to_string(), "f64");
        assert_eq!(R32.to_string(), "r32");
        assert_eq!(R64.to_string(), "r64");
    }

    #[test]
    fn format_vectors() {
        assert_eq!(B1.by(8).unwrap().to_string(), "b1x8");
        assert_eq!(B8.by(1).unwrap().to_string(), "b8");
        assert_eq!(B16.by(256).unwrap().to_string(), "b16x256");
        assert_eq!(B32.by(4).unwrap().by(2).unwrap().to_string(), "b32x8");
        assert_eq!(B64.by(8).unwrap().to_string(), "b64x8");
        assert_eq!(I8.by(64).unwrap().to_string(), "i8x64");
        assert_eq!(F64.by(2).unwrap().to_string(), "f64x2");
        assert_eq!(I8.by(3), None);
        assert_eq!(I8.by(512), None);
        assert_eq!(INVALID.by(4), None);
    }

    #[test]
    fn as_bool() {
        assert_eq!(I32X4.as_bool(), B32X4);
        assert_eq!(I32.as_bool(), B1);
        assert_eq!(I32X4.as_bool_pedantic(), B32X4);
        assert_eq!(I32.as_bool_pedantic(), B32);
    }
}
