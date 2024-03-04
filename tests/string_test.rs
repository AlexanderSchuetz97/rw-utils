use std::{io, panic};
use std::io::{Cursor, ErrorKind};
use static_assertions::const_assert;
use rw_utils::string_read::StringRead;
use rw_utils::string_write::StringWrite;

const_assert!(std::mem::size_of::<char>() == std::mem::size_of::<u32>());
#[test]
fn test_bom_utf32_le() -> io::Result<()>{
    let mut vec: Vec<u8> = vec![];
    vec.write_string_utf32_le("\u{FEFF}A")?;
    assert_eq!(vec, vec![0xFFu8, 0xFE, 0x00, 0x00, b'A', 0,0,0]);
    let mut cursor = Cursor::new(&vec);
    let read = cursor.read_string_utf32_le(2)?;
    assert_eq!(read, "\u{FEFF}A");
    let mut cursor = Cursor::new(&vec);
    let read_err = cursor.read_string_utf32_be(2).unwrap_err();
    match read_err.kind() {
        ErrorKind::InvalidData => {
            assert_eq!(read_err.to_string(), "Encountered byte order mark 0xFFFE. This indicates a wrong byte order.");
        }
        _ => panic!("{}", read_err)
    }

    return Ok(());
}

#[test]
fn test_bom_utf32_be() -> io::Result<()>{
    let mut vec: Vec<u8> = vec![];
    vec.write_string_utf32_be("\u{FEFF}A")?;
    assert_eq!(vec, vec![0x00u8, 0x00, 0xFE, 0xFF, 0, 0, 0, b'A']);
    let mut cursor = Cursor::new(&vec);
    let read = cursor.read_string_utf32_be(2)?;
    assert_eq!(read, "\u{FEFF}A");
    let mut cursor = Cursor::new(&vec);
    let read_err = cursor.read_string_utf32_le(2).unwrap_err();
    match read_err.kind() {
        ErrorKind::InvalidData => {
            assert_eq!(read_err.to_string(), "Encountered byte order mark 0xFFFE. This indicates a wrong byte order.");
        }
        _ => panic!("{}", read_err)
    }

    return Ok(());
}

#[test]
fn test_bom_utf16_le() -> io::Result<()>{
    let mut vec: Vec<u8> = vec![];
    vec.write_string_utf16_le("\u{FEFF}A")?;
    assert_eq!(vec, vec![0xFFu8, 0xFE, b'A', 0]);
    let mut cursor = Cursor::new(&vec);
    let read = cursor.read_string_utf16_le(2)?;
    assert_eq!(read, "\u{FEFF}A");
    let mut cursor = Cursor::new(&vec);
    let read_err = cursor.read_string_utf16_be(2).unwrap_err();
    match read_err.kind() {
        ErrorKind::InvalidData => {
            assert_eq!(read_err.to_string(), "Encountered byte order mark 0xFFFE. This indicates a wrong byte order.");
        }
        _ => panic!("{}", read_err)
    }

    return Ok(());
}

#[test]
fn test_bom_utf16_be() -> io::Result<()> {
    let mut vec: Vec<u8> = vec![];
    vec.write_string_utf16_be("\u{FEFF}A")?;
    assert_eq!(vec, vec![0xFE, 0xFF, 0, b'A']);
    let mut cursor = Cursor::new(&vec);
    let read = cursor.read_string_utf16_be(2)?;
    assert_eq!(read, "\u{FEFF}A");
    let mut cursor = Cursor::new(&vec);
    let read_err = cursor.read_string_utf16_le(2).unwrap_err();
    match read_err.kind() {
        ErrorKind::InvalidData => {
            assert_eq!(read_err.to_string(), "Encountered byte order mark 0xFFFE. This indicates a wrong byte order.");
        }
        _ => panic!("{}", read_err)
    }

    return Ok(());
}

#[test]
fn test_jstring() -> io::Result<()> {
    {
        let mut vec: Vec<u8> = vec![];
        vec.write_java_data_output_utf("Hello World")?;
        assert_eq!(vec, vec![0u8, 11, b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd']);
        let mut cursor = Cursor::new(&vec);
        let read = cursor.read_java_data_input_utf()?;
        assert_eq!(read, "Hello World");
    }
    {
        let mut vec: Vec<u8> = vec![];
        vec.write_java_data_output_utf("abcäöüabc")?;

        //Taken from a java program
        assert_eq!(vec, vec![0, 12, 97, 98, 99, 195, 164, 195, 182, 195, 188, 97, 98, 99]);
        let mut cursor = Cursor::new(&vec);
        let read = cursor.read_java_data_input_utf()?;
        assert_eq!(read, "abcäöüabc");
    }

    {
        let mut vec: Vec<u8> = vec![];
        vec.write_java_data_output_utf("abcä\u{20AC}üabc")?;

        //Taken from a java program
        assert_eq!(vec, vec![0, 13, 97, 98, 99, 195, 164, 226, 130, 172, 195, 188, 97, 98, 99]);

        let mut cursor = Cursor::new(&vec);
        let read = cursor.read_java_data_input_utf()?;
        assert_eq!(read, "abcä\u{20AC}üabc");
    }

    {
        let mut vec: Vec<u8> = vec![];
        vec.write_java_data_output_utf("abcä\u{20AC}\u{1F4A9}üabc")?;

        //Taken from a java program
        assert_eq!(vec, vec![0, 19, 97, 98, 99, 195, 164, 226, 130, 172, 237, 160, 189, 237, 178, 169, 195, 188, 97, 98, 99]);

        let mut cursor = Cursor::new(&vec);
        let read = cursor.read_java_data_input_utf()?;
        assert_eq!(read, "abcä\u{20AC}\u{1F4A9}üabc");
    }

    return Ok(());
}