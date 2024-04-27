//Implements pascal ANSIString in rust for dll function call

//From https://www.freepascal.org/docs-html/current/prog/progsu160.html#x204-2150008.2.7

// Table 8.3: AnsiString memory structure (32-bit model)

// Offset	Contains

// -12	Code page indicator (2 bytes).
// -10	Character size (2 bytes)
// -8	Longint with reference count.
// -4	Longint with actual string size.
// 0	Actual array of char, null-terminated.

// Table 8.4: AnsiString memory structure (64-bit model)

// Offset	Contains

// -24	Code page indicator (2 bytes).
// -22	Character size (2 bytes)
// -16	Sizeint with reference count.
// -8	Sizeint with actual string size.
// 0	Actual array of char, null-terminated.

use codepage::{from_encoding, to_encoding};
use encoding_rs::{Encoding, UTF_8};
use std::convert::{TryFrom, TryInto};

#[repr(C)]
pub struct PascalAnsiString {
    codepage: u16,
    char_size: u16,
    #[cfg(target_pointer_width = "64")]
    padding: u32,
    reference_count: i64, //-1 for not reference counted (constant), or any other value for number of references
    #[cfg(target_pointer_width = "64")]
    actual_string_size: u64,
    #[cfg(target_pointer_width = "32")]
    actual_string_size: u32,
    actual_array_of_char: *mut u8,
}

//actual_array_of_char arg is kept as reference arg here despite complicating provided functions (ptr->ref conversion)
//To allow for more standard usage of the struct compared to other Rust libraries
impl PascalAnsiString {
    pub fn new(
        codepage: u16,
        char_size: u16,
        reference_count: i64,
        #[cfg(target_pointer_width = "64")] actual_string_size: u64,
        #[cfg(target_pointer_width = "32")] actual_string_size: u32,
        actual_array_of_char: &mut [u8],
    ) -> Self {
        Self {
            codepage,
            char_size,
            #[cfg(target_pointer_width = "64")]
            padding: 0u32,
            reference_count,
            actual_string_size,
            actual_array_of_char: (*actual_array_of_char).as_mut_ptr(),
        }
    }
}

impl TryFrom<&str> for PascalAnsiString {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        static ENCODING: &Encoding = UTF_8; // Actually USA ASCII, but UTF-8 is backwards-compatible
        let codepage: u16 = from_encoding(ENCODING).unwrap();
        let char_size = 1;
        let reference_count = -1; //Not reference counted - constant value

        let mut value = value.to_string();

        let actual_array_of_char = unsafe { value.as_mut_vec() };
        actual_array_of_char.push(0u8); //Null-terminated (0u8 == '\0')

        #[cfg(target_pointer_width = "64")]
        let actual_string_size = actual_array_of_char.len() as u64; //Length in pascal is 1-indexed - including null terminator simplifies this
        #[cfg(target_pointer_width = "32")]
        let actual_string_size = actual_array_of_char.len() as u32; //Length in pascal is 1-indexed - including null terminator simplifies this

        let actual_array_of_char = actual_array_of_char.as_mut_slice();
        Ok(Self::new(
            codepage,
            char_size,
            reference_count,
            actual_string_size,
            actual_array_of_char,
        ))
    }
}

impl TryInto<String> for PascalAnsiString {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        let encoding: &Encoding = to_encoding(self.codepage.into()).unwrap();

        let array_slice = unsafe {
            std::slice::from_raw_parts(self.actual_array_of_char, self.actual_string_size as usize)
        };

        let string = encoding.decode(array_slice).0.to_string();
        Ok(string)
    }
}

impl TryFrom<Vec<u8>> for PascalAnsiString {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        static ENCODING: &Encoding = UTF_8; // Actually USA ASCII, but UTF-8 is backwards-compatible
        let codepage: u16 = from_encoding(ENCODING).unwrap();
        let char_size = 1;
        let reference_count = -1; //Not reference counted - constant value
        let mut actual_array_of_char = value;
        actual_array_of_char.push(0u8); //Null-terminated (0u8 == '\0')

        #[cfg(target_pointer_width = "64")]
        let actual_string_size = actual_array_of_char.len() as u64; //Length in pascal is 1-indexed - including null terminator simplifies this
        #[cfg(target_pointer_width = "32")]
        let actual_string_size = actual_array_of_char.len() as u32; //Length in pascal is 1-indexed - including null terminator simplifies this

        let actual_array_of_char = actual_array_of_char.as_mut_slice();
        Ok(Self::new(
            codepage,
            char_size,
            reference_count,
            actual_string_size,
            actual_array_of_char,
        ))
    }
}

impl TryInto<Vec<u8>> for PascalAnsiString {
    type Error = ();

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut vec = Vec::with_capacity(self.actual_string_size as usize);
        for i in 0..(self.actual_string_size as usize) {
            let char = unsafe { *(self.actual_array_of_char.wrapping_add(i as usize)) };
            vec.push(char);
        }
        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_ansistring() {
        let ansistring = PascalAnsiString::try_from("Hello, world!").unwrap();
        assert_eq!(ansistring.codepage, 1252);
        assert_eq!(ansistring.char_size, 1);
        assert_eq!(ansistring.reference_count, -1);
        assert_eq!(ansistring.actual_string_size, 14);
        assert_eq!(
            ansistring.actual_array_of_char,
            "Hello, world!\0".as_bytes().as_ptr() as *mut u8
        );
    }

    #[test]
    fn test_pascal_ansistring_try_into() {
        let ansistring = PascalAnsiString::try_from("Hello, world!").unwrap();
        let string: String = ansistring.try_into().unwrap();
        assert_eq!(string, "Hello, world!");
    }

    #[test]
    fn test_pascal_ansistring_try_from() {
        let string = "Hello, world!";
        let ansistring: PascalAnsiString = PascalAnsiString::try_from(string).unwrap();
        assert_eq!(ansistring.codepage, 1252);
        assert_eq!(ansistring.char_size, 1);
        assert_eq!(ansistring.reference_count, -1);
        assert_eq!(ansistring.actual_string_size, 14);
        assert_eq!(
            ansistring.actual_array_of_char,
            "Hello, world!\0".as_bytes().as_ptr() as *mut u8
        );
    }

    #[test]
    fn test_pascal_ansistring_round_trip() {
        let string = "Hello, world!";
        let ansistring = PascalAnsiString::try_from(string).unwrap();
        let string2: String = ansistring.try_into().unwrap();
        assert_eq!(string, string2);
    }
}
