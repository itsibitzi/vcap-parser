use crate::{
    fixed_width_data::{FixedWidthData, FixedWidthResult},
    slice_cursor::SliceCursor,
    types::{
        AbstractCode, AbstractNumber3, AbstractNumber4, CancellationReason, CaseFileNumber,
        CostInterestPrincipalSequence, CountyNumber, CourtType, Date, EntryType,
        LastModifiedReasonCode, PrioritySequence, ProgramName, RecordType, SequenceNumber, Status,
        Timestamp, UnsureAbstractType, UpdateIndicator, User, Year,
    },
};

//
// Abstract record with headers
//

#[derive(Debug)]
pub struct AbstractRecord {
    pub county_number: CountyNumber,
    pub record_type: RecordType,
    pub case_file_number: CaseFileNumber,
    pub abstract_number: AbstractNumber4,
    pub update_indicator: UpdateIndicator,
    pub extract_data: Option<AbstractExtractData>,
}

pub fn process_abstract_record(line_bytes: &[u8]) -> FixedWidthResult<AbstractRecord> {
    let mut cursor = SliceCursor::new(line_bytes);

    let county_number = cursor.read_map::<CountyNumber>()?;
    let record_type = cursor.read_map::<RecordType>()?;
    let case_file_number = cursor.read_map::<CaseFileNumber>()?;
    let abstract_number = cursor.read_map::<AbstractNumber4>()?;
    let update_indicator = cursor.read_map::<UpdateIndicator>()?;

    let extract_data = match record_type {
        RecordType(4) => Some(AbstractExtractData::AbstractRecordInformation(
            cursor.read_map::<AbstractRecordInformation>()?,
        )),
        RecordType(number) => {
            eprintln!("UNHANDLED RECORD TYPE: {}", number);
            None
        }
    };

    Ok(AbstractRecord {
        county_number,
        record_type,
        case_file_number,
        abstract_number,
        update_indicator,
        extract_data,
    })
}

//
// Macros for generating abstract extract data
//

macro_rules! expand_field_read {
    ($cursor: tt, $field_name: ident, Option<$field_type: ty>>) => {
        let $field_name = $cursor.read_map_optional::<$field_type>()?;
    };
    ($cursor: tt, $field_name: ident, $field_type: ty) => {
        let $field_name = $cursor.read_map::<$field_type>()?;
    };
}

#[macro_export]
macro_rules! impl_abstract_record_struct {
    ($name: ident, $($field_name: ident: $field_type: ty,) +) => {
        #[derive(Debug, PartialEq, Eq)]
        pub struct $name {
            $($field_name: $field_type), *
        }

        impl FixedWidthData for $name {
            const FIELD_NAME:  &'static str = stringify!($name);
            const LEN: usize = 325;

            fn from_bytes(bytes: &[u8]) -> FixedWidthResult<Self> {
                let mut cursor = SliceCursor::new(bytes);

                $(
                    expand_field_read!{cursor, $field_name, $field_type}
                ) *

                Ok(Self {
                    $($field_name), *
                })
            }
        }
    };
}

//
// Structures defining extract data types go below here
//

#[derive(Debug)]
pub enum AbstractExtractData {
    AbstractRecordInformation(AbstractRecordInformation),
}

// Record type = 04
// This record is used to record information regarding an abstract. This may be both judgment and non-judgment type of abstracts.
impl_abstract_record_struct! {
    AbstractRecordInformation,
    county_number: CountyNumber,
    case_year: Year,
    case_court_type: CourtType,
    case_sequence_number: SequenceNumber,
    abstract_code: AbstractCode,
    abstract_number: AbstractNumber3,
    inserted_timestamp: Timestamp,
    abstract_type: UnsureAbstractType,
    clock_in_timestamp: Timestamp,
    docket_timestamp: Option<Timestamp>,
    entry_type: EntryType,
    status: Status,
    priority_sequence: PrioritySequence,
    last_modification_reason_code: LastModifiedReasonCode,
    cost_interest_principal_sequence: CostInterestPrincipalSequence,
    cancel_date: Option<Date>,
    vacated_date: Option<Date>,
    last_updated_timestamp: Timestamp,
    last_updated_user: User,
    last_updated_program_name: ProgramName,
    cancellation_reason: CancellationReason,
}

// Record type = 19
// This record is used to define assignments from one creditor to another. This assignor is the creditor and the assignee is an individual that is not a party on the case.
// Record type = 03
// This record is used to store all the information related to the cost a particular individual has paid.
// Record type = 36
// This record is used to store the various types of electronic service documents that are send from the clerks office
// Record type = 21
// This record is used to store a connection between the service row and either the party liability rows when the service type is an execution, or to the abstract row when the service type is a notice of rights.
// Record type = 05
// This record identifies the parties included on the judgment order. This is where an alternate name or address is defined.
// Record type = 37
// This record is used to store all parts of an address. This is a generic record and used for all addresses. The address on this record is related to the prior record in the extract.
// Record type = 06
// This record is used to define the details of the abstract that relates to costs an attorney fees.
// Record type = 07
// This record is used to define the details of the abstract that relates to principal an attorney fees.
// Record type = 08
// This record is used to store the date intervals and interest rates as defined in the judgment order.
// Record type = 09
// This record is used to define the details of the abstract that relates to property, both real and personnel.
// Record type = 10
// This record is used to store the free text line items entered by the user for either a property abstract or a special abstract.
// Record type = 11
// This record is used to define the details of the abstract that does not conform to the rules of money abstracts. Any details recorded in this record will forfeit the calculation of payoff information by the VCAP system.
// Record type = 12
// This record is used to record the liability of a party on a particular abstract. The two types of liability include debtor and creditor.
// Record type = 13
// This record is used to store all the information related to the payment a particular individual has paid.
// Record type = 14
// This record is used to identify the party to which a payment was applied to.
// Record type = 20
// This record is used to tie payment distributed to indigent costs.
// Record type = 15
// This record is used to store various abstract events.
// Record type = 16
// This record is used to record the details for a particular event and is used for certain types of changes that require we save the original value and the changed to value.
// Record type = 18
// This record is used for informational purposes to record relationship between cases. These include abstracts, historical docketing, or historical transcripts.
