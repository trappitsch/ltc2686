//! Module to deal with representing voltages.
//!
//! If the "measurements" feature is activated, channel values can be set by in DAC values (as
//! `u16`) or as [`measurements::Voltage`]. This module helps with implementing the voltages.

static U16MAX_FLOAT: f64 = u16::MAX as f64;

use measurements::Voltage;

use crate::{
    AsDacValue,
    Span::{self},
    ll::SpanRangeVolts,
};

/// *Only available with feature "measurements"*
impl AsDacValue for Voltage {
    /// Convert a [`measurements::Voltage`] into a DAC value.
    ///
    /// This is only available if the "measurements" feature is activated.
    fn as_dac_value(&self, span: Span) -> u16 {
        let input_volts = self.as_volts();
        let span_range = span.get_range_volts();

        match span_range {
            SpanRangeVolts::Unipolar { range_v } => {
                let dac_value_float =
                    libm::round(U16MAX_FLOAT / range_v * input_volts).clamp(0.0, U16MAX_FLOAT);
                dac_value_float as u16
            }
            SpanRangeVolts::Bipolar { range_v } => {
                let dac_value_float =
                    libm::round(U16MAX_FLOAT / range_v * (input_volts + range_v / 2.0))
                        .clamp(0.0, U16MAX_FLOAT);
                dac_value_float as u16
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use measurements::Voltage;
    use rstest::rstest;

    #[rstest]
    #[case(Voltage::from_volts(0.0), 0)]
    #[case(Voltage::from_volts(10.0), u16::MAX)]
    #[case(Voltage::from_volts(1.0), 6554)]
    #[case(Voltage::from_volts(4./3.), 8738)]
    #[case(Voltage::from_volts(0.00015), 1)]
    fn test_dac_value_conversion_for_unipolar_10_v_range_for_various_values(
        #[case] volts: Voltage,
        #[case] dac_exp: u16,
    ) {
        let span = Span::Unip10;
        let dac_res = volts.as_dac_value(span);
        assert_eq!(dac_res, dac_exp);
    }

    #[rstest]
    #[case(Voltage::from_volts(0.0), 0x8000)]
    #[case(Voltage::from_volts(10.0), u16::MAX)]
    #[case(Voltage::from_volts(-10.0), 0x0000)]
    #[case(Voltage::from_volts(2.0), 0x9999)]
    #[case(Voltage::from_volts(8./3.), 0xA222)]
    #[case(Voltage::from_volts(0.0004), 0x8001)]
    #[case(Voltage::from_volts(-0.0003), 0x7FFF)]
    fn test_dac_value_conversion_for_bipolar_10_v_range_for_various_values(
        #[case] volts: Voltage,
        #[case] dac_exp: u16,
    ) {
        let span = Span::Bip10;
        let dac_res = volts.as_dac_value(span);
        assert_eq!(dac_res, dac_exp);
    }
}
