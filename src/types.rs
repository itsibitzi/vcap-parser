//! This module contains types used in both case and abstract records.

use crate::{fixed_width_data::FixedWidthData, impl_fixed_width_numeric, impl_fixed_width_string};

// A load of the data types we want to just have as a "string" of bytes
// This is because they're basically just tokens representing something
// like an ID, and don't need any further validation.

// Some things here *might* require some custom parsing to be useful but I've put
// them here as a default.
impl_fixed_width_string!(CountyNumber, 6); // Docs say 3, but we think that's wrong because it would leave a gap in the parsed line
impl_fixed_width_string!(CourtType, 3);
impl_fixed_width_string!(SoundEx, 4);
impl_fixed_width_string!(DomesticType, 1);
impl_fixed_width_string!(AbstractCode, 1);
impl_fixed_width_string!(DispositionCode, 4);
impl_fixed_width_string!(PresidingOfficialId, 7);
impl_fixed_width_string!(User, 8);
impl_fixed_width_string!(ProgramName, 8);

impl_fixed_width_string!(CaseFileNumber, 11);
impl_fixed_width_string!(AbstractNumber4, 4);
impl_fixed_width_string!(AbstractNumber3, 3);
impl_fixed_width_string!(EntryType, 4);
impl_fixed_width_string!(Status, 4);
impl_fixed_width_string!(PrioritySequence, 9);
impl_fixed_width_string!(LastModifiedReasonCode, 4);
impl_fixed_width_string!(CostInterestPrincipalSequence, 3);

// Valid values are:
//   PSTC - Paid and Satisfied in Full to Clerk and
//   PSTP - Paid and Satisfied in Full to Party.
impl_fixed_width_string!(CancellationReason, 4);

impl_fixed_width_string!(UpdateIndicator, 1); // TODO migrate to custom
impl_fixed_width_string!(Date, 4); // TODO migrate to custom
impl_fixed_width_string!(Timestamp, 26); // TODO migrate to custom

// unsure what these mean - the docs arent super clear
impl_fixed_width_string!(UnsureAbstractType, 4);

// Numeric types, e.g. years, sequence numbers

impl_fixed_width_numeric!(RecordType, 2, u8);
impl_fixed_width_numeric!(Year, 4, u32);
impl_fixed_width_numeric!(SequenceNumber, 6, u32);
impl_fixed_width_numeric!(ReelNumber, 3, u32);
impl_fixed_width_numeric!(FrameNumber, 4, u32);
