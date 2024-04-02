//use crate::types::CountyNumber;

// 001-003 County Number
// 007-002 Record Type
// 009-011 Case File number
// 020-001 Update indicator – “A” for add, “D” for delete and “U” for update
// 021-480 Extract data – see rest of document for field break down of each record type
// struct CaseRecord<'a> {
//     county_number: CountyNumber,
//     record_type: &'a [u8; 2],
//     case_file_number: &'a [u8; 11],
//     update_indicator: u8,
//     // extract_data: Box<T>,
// }

// fn process_case_record(line_bytes: &[u8]) -> CaseRecord {
//     // Sanity check the line we've received
//     assert!(
//         line_bytes.len() == 500,
//         "Line is unexpected length expected: 500, actual: {}",
//         line_bytes.len()
//     );

//     todo!()
// }

// Record type = 28
// This record is used to store the case file number entered within a county
// impl_case_record_struct! {
//     CountyCaseFileNumber,
//     county_num: CountyNumber,
//     year: Year,
//     court_type: CourtType,
//     seq_num: SequenceNumber,
//     dom_type: DomesticType,
//     disp_code: DispositionCode,
//     disposed_date: Option<Date>,
//     presiding_official_id: Option<PresidingOfficialId>,
//     appeal_timestamp: Option<Timestamp>,
//     last_updated_timestamp: Timestamp,
//     last_updated_user: User,
// }

// Record type = 57
// This record is used to store case activity for on estate case.

// Record type = 58
// This record is used to define calendars defined at the case level.

// Record type = 59
// This record is used to store various events done at the case level

// Record type = 53
// This record is used to store the full microfilm number this case is present on. Normally a case is microfilmed only after it has been disposed.

// Record type = 48
// This record is used to define parties on a case

// Record type = 55
// This record is used to store events at the party level

// Record type = 61
// This record is used to store titles for a particular party. It's primarily used for estate cases.

// Record type = 65
// This record is used to store values relating to changes made to party data like name changes.

// Record type = 46
// This record is used to store any alias' a party may also be known as.
// Record type = 47
// This record is used to define a party's address connection, which also contains the resident county number.
// Record type = 37
// This record is used to store all parts of an address. This is a generic record and used for all addresses.
// The address on this record is related to the prior record in the extract.
// Record type = 29
// This record defines attorneys attached to a party.
// Record type = 60
// This record is used to store the information regarding the pleading that was filed in the clerks office
// Record type = 23
// This record is used to store the detail issues related to the pleading
// Record type = 50
// This record is used to store orders as a result of a hearing by a judge or clerk.
// Record type = 27
// This record is used to store the lead case file number and the consolidation order timestamp of other case consolidated together.

// Record type = 38
// This record is used to define the type of response by a party to the clerks office
// Record type = 21
// This record is used to define the parties that have responded to notification by the clerks office
// Record type = 71
// This record is used to store the summary information regarding the judgments that have not been abstracted in the new abstract sub-system.

// Record type = 72
// This record is used to store judgment level events that have not been abstracted in the new abstract sub-system.
// Record type = 73
// This record is used to store the principal amount of this judgment that has not been abstracted in the new abstract sub-system.

// Record type = 74
// This record is used to store the payments applied to this judgment that have not been abstracted in the new abstract sub-system.

// Record type = 17
// This record is used to record the transcriptions of judgments to other counties within North Carolina.
// Record type = 22
// This record is used to define the role of a party on a particular issue
// Record type = 24
// This record is used to store rows when an X was entered for a party on an issue. An even number of records for
// a specific party and issue means there is no longer an X on the party otherwise an odd number of records means
// an X is present on a party. When a party has an X, they will appear on the index to judgments screen.

// Record type = 32
// This record is used to store the parties that appealed either a CVM case or the issue of a CVD case

// Record type = 33
// This record is used to store the details regarding a CVD or CVM appeal

// Record type = 69
// This record is used to tie the calendar session to the bond forfeiture issue. Only used for CR and CRS bond forfeiture cases.

// Record type = 70
// This record is used to store calendar sessions for bond forfeiture cases only
// Record type = 36
// This record is used to store the various types of electronic service documents that are send from the clerks office

// Record type = 03
// This record is used to store all the information related to the cost a particular individual has paid. These are not yet associated to an abstract.
