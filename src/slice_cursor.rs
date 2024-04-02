use crate::fixed_width_data::{
    FixedWidthData, FixedWidthError, FixedWidthErrorRange, FixedWidthResult,
};

pub struct SliceCursor<'a> {
    current_index: usize,
    slice: &'a [u8],
    // slice : Vec<u8>
}

impl<'a> SliceCursor<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self {
            current_index: 0,
            slice,
        }
    }

    pub fn read_map_optional<T: FixedWidthData>(&mut self) -> FixedWidthResult<Option<T>> {
        let bytes = self.read(T::FIELD_NAME, T::LEN)?;

        if bytes.iter().all(|b| *b == b'\0') {
            return Ok(None);
        }

        T::from_bytes(bytes).map(|r| Some(r))
    }

    pub fn read_map<T: FixedWidthData>(&mut self) -> FixedWidthResult<T> {
        let bytes = self.read(T::FIELD_NAME, T::LEN)?;

        T::from_bytes(bytes)
    }

    // Internal read function
    fn read(&mut self, field_name: &'static str, len: usize) -> FixedWidthResult<&[u8]> {
        if self.current_index + len > self.slice.len() {
            return Err(FixedWidthError::RangeTooShort {
                field_name,
                range: FixedWidthErrorRange::from_bytes(self.slice),
            });
        }

        let subslice = &self.slice[self.current_index..self.current_index + len];

        self.current_index += len;

        Ok(subslice)
    }
}
