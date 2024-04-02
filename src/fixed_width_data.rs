pub enum FixedWidthError {
    // Field which we expect to be numeric contains non-numeric characters
    BadNumeric {
        field_name: &'static str,
        range: FixedWidthErrorRange,
    },
    // The provided number of bytes is too short
    RangeTooShort {
        field_name: &'static str,
        range: FixedWidthErrorRange,
    },
}

// We want to print the range that failed to parse - ideally as a human readable string
// but if the files are corrupt they might not parse to UTF-8 so we have to deal with that
#[derive(Debug)]
pub enum FixedWidthErrorRange {
    Utf8(String),
    ParseFail(Vec<u8>),
}

impl FixedWidthErrorRange {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let Ok(line_str) = std::str::from_utf8(bytes) else {
            return FixedWidthErrorRange::ParseFail(bytes.to_owned());
        };
        return FixedWidthErrorRange::Utf8(line_str.to_string());
    }
}

pub type FixedWidthResult<T> = Result<T, FixedWidthError>;

pub trait FixedWidthData: Sized {
    const FIELD_NAME: &'static str;
    const LEN: usize;

    fn from_bytes(bytes: &[u8]) -> FixedWidthResult<Self>;

    fn to_sqlite_row(&self) {
        todo!()
    }
}

impl<T: FixedWidthData> FixedWidthData for Option<T> {
    const FIELD_NAME: &'static str = T::FIELD_NAME;
    const LEN: usize = T::LEN;

    fn from_bytes(bytes: &[u8]) -> FixedWidthResult<Self> {
        if bytes.iter().all(|b| *b == b'\0') {
            Ok(None)
        } else {
            T::from_bytes(bytes).map(|t| Some(t))
        }
    }
}

//
// Helpful macros for producing rich types from the byte arrays
//

#[macro_export]
macro_rules! impl_fixed_width_string {
    ($name: ident, $len: tt) => {
        #[derive(PartialEq, Eq)]
        pub struct $name(pub [u8; $len]);

        impl FixedWidthData for $name {
            const FIELD_NAME: &'static str = stringify!($name);
            const LEN: usize = $len;

            fn from_bytes(bytes_in: &[u8]) -> $crate::fixed_width_data::FixedWidthResult<Self> {
                if bytes_in.len() < $name::LEN {
                    return Err($crate::fixed_width_data::FixedWidthError::RangeTooShort {
                        field_name: Self::FIELD_NAME,
                        range: $crate::fixed_width_data::FixedWidthErrorRange::from_bytes(bytes_in),
                    });
                }

                let mut bytes = [0_u8; $name::LEN];
                bytes.copy_from_slice(bytes_in);
                Ok($name(bytes))
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_fixed_width_numeric {
    ($name: ident, $len: tt, $numeric_type: ty) => {
        #[derive(Debug, PartialEq, Eq)]
        pub struct $name(pub $numeric_type);

        // Slightly lazy to do an implementation for every newtype, even if they're
        // the same number of bytes wide, but whatever.
        impl FixedWidthData for $name {
            const FIELD_NAME: &'static str = stringify!($name);
            const LEN: usize = $len;

            fn from_bytes(bytes_in: &[u8]) -> $crate::fixed_width_data::FixedWidthResult<Self> {
                // println!(
                //     "Parsing {} - {:?}",
                //     stringify!($name),
                //     String::from_utf8_lossy(&bytes_in)
                // );

                if bytes_in.len() < $name::LEN {
                    return Err($crate::fixed_width_data::FixedWidthError::RangeTooShort {
                        field_name: Self::FIELD_NAME,
                        range: $crate::fixed_width_data::FixedWidthErrorRange::from_bytes(bytes_in),
                    });
                }

                if bytes_in.iter().any(|b| !b.is_ascii_digit()) {
                    return Err($crate::fixed_width_data::FixedWidthError::BadNumeric {
                        field_name: Self::FIELD_NAME,
                        range: $crate::fixed_width_data::FixedWidthErrorRange::from_bytes(bytes_in),
                    });
                }

                let mut num: $numeric_type = 0;

                for i in 0..bytes_in.len() {
                    let digit = bytes_in[i] - b'0';
                    num = num * 10 + digit as $numeric_type;
                }

                Ok($name(num))
            }
        }
    };
}
