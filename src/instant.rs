use crate::duration::Duration;
use crate::helpers::{self, Helpers};
use core::cmp::Ordering;
use core::ops;

#[derive(Clone, Copy, Debug)]
pub struct Instant<T, const NOM: u32, const DENOM: u32> {
    ticks: T,
}

macro_rules! impl_instant_for_integer {
    ($i:ty) => {
        impl<const NOM: u32, const DENOM: u32> Instant<$i, NOM, DENOM> {
            pub const fn from_ticks(ticks: $i) -> Self {
                helpers::greater_than_0::<NOM>();
                helpers::greater_than_0::<DENOM>();

                Instant { ticks }
            }

            pub const fn ticks(&self) -> $i {
                self.ticks
            }

            pub fn checked_duration_since(self, other: Self) -> Option<Duration<$i, NOM, DENOM>> {
                if self >= other {
                    Some(Duration::<$i, NOM, DENOM>::from_ticks(
                        self.ticks.wrapping_sub(other.ticks),
                    ))
                } else {
                    None
                }
            }

            pub const fn checked_sub_duration<const O_NOM: u32, const O_DENOM: u32>(
                self,
                other: Duration<$i, O_NOM, O_DENOM>,
            ) -> Option<Self> {
                if Helpers::<NOM, DENOM, O_NOM, O_DENOM>::SAME_BASE {
                    Some(Instant::<$i, NOM, DENOM>::from_ticks(
                        self.ticks.wrapping_sub(other.ticks()),
                    ))
                } else {
                    if let Some(lh) = other
                        .ticks()
                        .checked_mul(Helpers::<NOM, DENOM, O_NOM, O_DENOM>::LD_TIMES_RN as $i)
                    {
                        let ticks = lh / Helpers::<NOM, DENOM, O_NOM, O_DENOM>::RD_TIMES_LN as $i;

                        Some(Instant::<$i, NOM, DENOM>::from_ticks(
                            self.ticks.wrapping_sub(ticks),
                        ))
                    } else {
                        None
                    }
                }
            }

            pub const fn checked_add_duration<const O_NOM: u32, const O_DENOM: u32>(
                self,
                other: Duration<$i, O_NOM, O_DENOM>,
            ) -> Option<Self> {
                if Helpers::<NOM, DENOM, O_NOM, O_DENOM>::SAME_BASE {
                    Some(Instant::<$i, NOM, DENOM>::from_ticks(
                        self.ticks.wrapping_add(other.ticks()),
                    ))
                } else {
                    if let Some(lh) = other
                        .ticks()
                        .checked_mul(Helpers::<NOM, DENOM, O_NOM, O_DENOM>::LD_TIMES_RN as $i)
                    {
                        let ticks = lh / Helpers::<NOM, DENOM, O_NOM, O_DENOM>::RD_TIMES_LN as $i;

                        Some(Instant::<$i, NOM, DENOM>::from_ticks(
                            self.ticks.wrapping_add(ticks),
                        ))
                    } else {
                        None
                    }
                }
            }
        }

        impl<const NOM: u32, const DENOM: u32> PartialOrd for Instant<$i, NOM, DENOM> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<const NOM: u32, const DENOM: u32> Ord for Instant<$i, NOM, DENOM> {
            fn cmp(&self, other: &Self) -> Ordering {
                if self.ticks == other.ticks {
                    Ordering::Equal
                } else {
                    self.ticks
                        .wrapping_sub(other.ticks)
                        .cmp(&(<$i>::MAX / 2))
                        .reverse()
                }
            }
        }

        impl<const NOM: u32, const DENOM: u32> PartialEq for Instant<$i, NOM, DENOM> {
            fn eq(&self, other: &Self) -> bool {
                self.ticks.eq(&other.ticks)
            }
        }

        impl<const NOM: u32, const DENOM: u32> Eq for Instant<$i, NOM, DENOM> {}

        // Instant - Instant = Duration
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // `checked_duration_since`.
        impl<const NOM: u32, const DENOM: u32> ops::Sub<Instant<$i, NOM, DENOM>>
            for Instant<$i, NOM, DENOM>
        {
            type Output = Duration<$i, NOM, DENOM>;

            fn sub(self, other: Self) -> Self::Output {
                if let Some(v) = self.checked_duration_since(other) {
                    v
                } else {
                    panic!("Sub failed! Other > self");
                }
            }
        }

        // Instant - Duration = Instant
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // `checked_sub_duration`.
        impl<const NOM: u32, const DENOM: u32> ops::Sub<Duration<$i, NOM, DENOM>>
            for Instant<$i, NOM, DENOM>
        {
            type Output = Instant<$i, NOM, DENOM>;

            fn sub(self, other: Duration<$i, NOM, DENOM>) -> Self::Output {
                if let Some(v) = self.checked_sub_duration(other) {
                    v
                } else {
                    panic!("Sub failed! Overflow");
                }
            }
        }

        // Instant + Duration = Instant
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // `checked_add_duration`.
        impl<const NOM: u32, const DENOM: u32> ops::Add<Duration<$i, NOM, DENOM>>
            for Instant<$i, NOM, DENOM>
        {
            type Output = Instant<$i, NOM, DENOM>;

            fn add(self, other: Duration<$i, NOM, DENOM>) -> Self::Output {
                if let Some(v) = self.checked_add_duration(other) {
                    v
                } else {
                    panic!("Add failed! Overflow");
                }
            }
        }
    };
}

impl_instant_for_integer!(u32);
impl_instant_for_integer!(u64);

//
// Operations between u32 Duration and u64 Instant
//

// Instant - Duration = Instant
// We have limited this to use same numerator and denominator in both left and right hand sides,
// this allows for the extension traits to work. For usage with different fraction, use
// `checked_sub_duration`.
impl<const NOM: u32, const DENOM: u32> ops::Sub<Duration<u32, NOM, DENOM>>
    for Instant<u64, NOM, DENOM>
{
    type Output = Instant<u64, NOM, DENOM>;

    fn sub(self, other: Duration<u32, NOM, DENOM>) -> Self::Output {
        self.sub(Duration::<u64, NOM, DENOM>::from_ticks(other.ticks() as u64))
    }
}

// Instant + Duration = Instant
// We have limited this to use same numerator and denominator in both left and right hand sides,
// this allows for the extension traits to work. For usage with different fraction, use
// `checked_add_duration`.
impl<const NOM: u32, const DENOM: u32> ops::Add<Duration<u32, NOM, DENOM>>
    for Instant<u64, NOM, DENOM>
{
    type Output = Instant<u64, NOM, DENOM>;

    fn add(self, other: Duration<u32, NOM, DENOM>) -> Self::Output {
        self.add(Duration::<u64, NOM, DENOM>::from_ticks(other.ticks() as u64))
    }
}
