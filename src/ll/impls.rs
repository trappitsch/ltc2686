//! This module mainly impls additional functionality on enums and structs that are define in the
//! low-level device-driver definition.

use crate::Span;

/// The total span range.
///
/// The span on this chip is either unipolar (0 to x V) or bipolar, (-x to +x V). This enum provides
/// the type as well as the range (in case of the unipolar) or half-range in the case of the bipolar.
/// We specifically select range and half-range as this enum is mainly used for DAC code
/// calculation.
pub enum SpanRangeVolts {
    Unipolar { range_v: f64 },
    Bipolar { range_v: f64 },
}

impl Span {
    /// Get the total span in Volts.
    pub fn get_range_volts(&self) -> SpanRangeVolts {
        match self {
            Span::Unip5 => SpanRangeVolts::Unipolar { range_v: 5.0 },
            Span::Unip10 => SpanRangeVolts::Unipolar { range_v: 10.0 },
            Span::Bip5 => SpanRangeVolts::Bipolar { range_v: 10.0 },
            Span::Bip10 => SpanRangeVolts::Bipolar { range_v: 20.0 },
            Span::Bip15 => SpanRangeVolts::Bipolar { range_v: 30.0 },
            Span::OvrUnip5 => SpanRangeVolts::Unipolar { range_v: 5.25 },
            Span::OvrUnip10 => SpanRangeVolts::Unipolar { range_v: 10.5 },
            Span::OvrBip5 => SpanRangeVolts::Bipolar { range_v: 10.5 },
            Span::OvrBip10 => SpanRangeVolts::Bipolar { range_v: 21.0 },
            Self::OvrBip15 => SpanRangeVolts::Bipolar { range_v: 31.5 },
        }
    }
}
