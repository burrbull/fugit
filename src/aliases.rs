//! Type aliases for common uses

use crate::{Duration, Fraction, Instant, Rate};

/// Alias for nanosecond duration
pub type NanosDuration<T> = Duration<T, { Fraction::NANO }>;

/// Alias for nanosecond duration (`u32` backing storage)
pub type NanosDurationU32 = Duration<u32, { Fraction::MICRO }>;

/// Alias for nanosecond duration (`u64` backing storage)
pub type NanosDurationU64 = Duration<u64, { Fraction::NANO }>;

/// Alias for microsecond duration
pub type MicrosDuration<T> = Duration<T, { Fraction::MICRO }>;

/// Alias for microsecond duration (`u32` backing storage)
pub type MicrosDurationU32 = Duration<u32, { Fraction::MICRO }>;

/// Alias for microsecond duration (`u64` backing storage)
pub type MicrosDurationU64 = Duration<u64, { Fraction::MICRO }>;

/// Alias for millisecond duration
pub type MillisDuration<T> = Duration<T, { Fraction::MILLI }>;

/// Alias for millisecond duration (`u32` backing storage)
pub type MillisDurationU32 = Duration<u32, { Fraction::MILLI }>;

/// Alias for millisecond duration (`u64` backing storage)
pub type MillisDurationU64 = Duration<u64, { Fraction::MILLI }>;

/// Alias for second duration
pub type SecsDuration<T> = Duration<T, { Fraction::ONE }>;

/// Alias for second duration (`u32` backing storage)
pub type SecsDurationU32 = Duration<u32, { Fraction::ONE }>;

/// Alias for second duration (`u64` backing storage)
pub type SecsDurationU64 = Duration<u64, { Fraction::ONE }>;

/// Alias for minutes duration
pub type MinutesDuration<T> = Duration<T, { Fraction::new(60, 1) }>;

/// Alias for minutes duration (`u32` backing storage)
pub type MinutesDurationU32 = Duration<u32, { Fraction::new(60, 1) }>;

/// Alias for minutes duration (`u64` backing storage)
pub type MinutesDurationU64 = Duration<u64, { Fraction::new(60, 1) }>;

/// Alias for hours duration
pub type HoursDuration<T> = Duration<T, { Fraction::new(3_600, 1) }>;

/// Alias for hours duration (`u32` backing storage)
pub type HoursDurationU32 = Duration<u32, { Fraction::new(3_600, 1) }>;

/// Alias for hours duration (`u64` backing storage)
pub type HoursDurationU64 = Duration<u64, { Fraction::new(3_600, 1) }>;

/// Alias for durations that come from timers with a specific frequency
pub type TimerDuration<T, const FREQ_HZ: u32> = Duration<T, { Fraction::new(1, FREQ_HZ) }>;

/// Alias for durations that come from timers with a specific frequency (`u32` backing storage)
pub type TimerDurationU32<const FREQ_HZ: u32> = Duration<u32, { Fraction::new(1, FREQ_HZ) }>;

/// Alias for durations that come from timers with a specific frequency (`u64` backing storage)
pub type TimerDurationU64<const FREQ_HZ: u32> = Duration<u64, { Fraction::new(1, FREQ_HZ) }>;

// -------------------------------

/// Alias for instants that come from timers with a specific frequency
pub type TimerInstant<T, const FREQ_HZ: u32> = Instant<T, { Fraction::new(1, FREQ_HZ) }>;

/// Alias for instants that come from timers with a specific frequency (`u32` backing storage)
pub type TimerInstantU32<const FREQ_HZ: u32> = Instant<u32, { Fraction::new(1, FREQ_HZ) }>;

/// Alias for instants that come from timers with a specific frequency (`u64` backing storage)
pub type TimerInstantU64<const FREQ_HZ: u32> = Instant<u64, { Fraction::new(1, FREQ_HZ) }>;

// -------------------------------

/// Alias for hertz rate
pub type Hertz<T> = Rate<T, { Fraction::ONE }>;

/// Alias for hertz rate (`u32` backing storage)
pub type HertzU32 = Rate<u32, { Fraction::ONE }>;

/// Alias for hertz rate (`u64` backing storage)
pub type HertzU64 = Rate<u64, { Fraction::ONE }>;

/// Alias for kilohertz rate
pub type Kilohertz<T> = Rate<T, { Fraction::KILO }>;

/// Alias for kilohertz rate (`u32` backing storage)
pub type KilohertzU32 = Rate<u32, { Fraction::KILO }>;

/// Alias for kilohertz rate (`u64` backing storage)
pub type KilohertzU64 = Rate<u64, { Fraction::KILO }>;

/// Alias for megahertz rate
pub type Megahertz<T> = Rate<T, { Fraction::MEGA }>;

/// Alias for megahertz rate (`u32` backing storage)
pub type MegahertzU32 = Rate<u32, { Fraction::MEGA }>;

/// Alias for megahertz rate (`u64` backing storage)
pub type MegahertzU64 = Rate<u64, { Fraction::MEGA }>;

/// Alias for rate that come from timers with a specific frequency
pub type TimerRate<T, const FREQ_HZ: u32> = Rate<T, { Fraction::new(FREQ_HZ, 1) }>;

/// Alias for rate that come from timers with a specific frequency (`u32` backing storage)
pub type TimerRateU32<const FREQ_HZ: u32> = Rate<u32, { Fraction::new(FREQ_HZ, 1) }>;

/// Alias for rate that come from timers with a specific frequency (`u64` backing storage)
pub type TimerRateU64<const FREQ_HZ: u32> = Rate<u64, { Fraction::new(FREQ_HZ, 1) }>;
