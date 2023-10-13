use std::convert::Into;
use std::fmt::Display;
use std::mem;
use std::ops::{Add, BitAnd, BitOr, Not, Shl};

pub enum Bit {
    Off,
    On,
}

pub trait BitSet
where
    Self: Sized
        + Copy
        + From<u8>
        + BitOr<Output = Self>
        + Shl<Output = Self>
        + BitAnd<Output = Self>
        + Not<Output = Self>
        + Ord
        + Display,
{
    fn check(bit: &u8) -> Result<(), &'static str> {
        if (mem::size_of::<Self>() * 8) > *bit as usize {
            Ok(())
        } else {
            Err("Invalid bit position for type")
        }
    }
    fn update(&mut self, bit: u8, flag: Bit) -> Result<(), &'static str> {
        match flag {
            Bit::Off => Self::unset(self, bit),
            Bit::On => Self::set(self, bit),
        }
    }
    fn set(&mut self, bit: u8) -> Result<(), &'static str> {
        Self::check(&bit)?;
        let mut mask = Self::one();
        mask = mask << bit.into();
        *self = *self | mask;
        Ok(())
    }
    fn unset(&mut self, bit: u8) -> Result<(), &'static str> {
        Self::check(&bit)?;
        let mask: Self = Self::one();
        let mask = !(mask << bit.into());
        *self = *self & mask;
        Ok(())
    }
    fn reset(&mut self) {
        *self = Self::zero();
    }

    fn initialise() -> Self {
        Self::zero()
    }

    fn contains(&self, mask: Self) -> bool {
        (*self & mask) > Self::zero()
    }

    fn mask_for(i: Self) -> Self {
        Self::one() << i
    }

    fn one() -> Self {
        1.into()
    }

    fn zero() -> Self {
        0.into()
    }
}

impl BitSet for u128 {}
impl BitSet for u64 {}
impl BitSet for u32 {}
impl BitSet for u16 {}
impl BitSet for u8 {}

pub trait Increment
where
    Self: Add<Output = Self> + Sized + From<u8> + Copy,
{
    fn increment(&mut self) -> Self {
        *self = *self + Self::from(1);
        *self
    }
}

impl Increment for u128 {}
impl Increment for u64 {}
impl Increment for u32 {}
impl Increment for u16 {}
impl Increment for u8 {}
#[cfg(test)]
mod tests {
    use crate::math::{BitSet, Increment};

    #[test]
    fn increment_values_u32() {
        let mut x = 0_u32;
        x.increment();
        assert_eq!(1, x);
        x.increment();
        assert_eq!(2, x);
        x.increment();
        assert_eq!(3, x);
    }

    #[test]
    fn increment_values_u64() {
        let mut x = 0_u64;
        x.increment();
        assert_eq!(1, x);
        x.increment();
        assert_eq!(2, x);
        x.increment();
        assert_eq!(3, x);
    }

    #[test]
    fn increment_values_u128() {
        let mut x = 0_u128;
        x.increment();
        assert_eq!(1, x);
        x.increment();
        assert_eq!(2, x);
        x.increment();
        assert_eq!(3, x);
    }

    #[test]
    fn set_bits() {
        let mut flags = 0_u64;
        let _ = flags.set(0);
        assert_eq!(1, flags);
        let _ = flags.unset(0);
        assert_eq!(0, flags);
        let bit_position = 27;
        let mut test = 1_u64 << bit_position;
        let _ = test.unset(bit_position);
        assert_eq!(0, flags);
        let _ = test.set(bit_position);
        assert_eq!(1_u64 << bit_position, test);
    }

    #[test]
    fn contains() {
        let flags = 2_u64;
        assert!(flags.contains(2));
    }
}
