use std::io;
use std::io::Cursor;
use rw_utils::num_read::NumRead;
use rw_utils::num_write::NumWrite;

#[test]
fn test_negative_signed() -> io::Result<()>{
    let mut buffer: Vec<u8> = vec![];
    buffer.write_i8(-5)?;
    buffer.write_i16_ne(-5)?;
    buffer.write_i32_ne(-5)?;
    buffer.write_i64_ne(-5)?;
    buffer.write_i128_ne(-5)?;
    let mut cursor = Cursor::new(&buffer);
    assert_eq!(cursor.read_i8()?, -5);
    assert_eq!(cursor.read_i16_ne()?, -5);
    assert_eq!(cursor.read_i32_ne()?, -5);
    assert_eq!(cursor.read_i64_ne()?, -5);
    assert_eq!(cursor.read_i128_ne()?, -5);

    return Ok(());
}

#[test]
fn test_u8() -> io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    buffer.write_u8(5)?;
    buffer.write_u8(0xff)?;
    buffer.write_u8(2)?;
    assert_eq!(buffer, vec![5, 0xff, 2]);

    let mut cursor = Cursor::new(&buffer);
    assert_eq!(cursor.read_u8()?, 5);
    assert_eq!(cursor.read_u8()?, 0xff);
    assert_eq!(cursor.read_u8()?, 2);

    return Ok(());
}

#[test]
fn test_u16() -> io::Result<()>{
    let mut buffer: Vec<u8> = vec![];
    buffer.write_u16_le(0x4418)?;
    buffer.write_u16_be(0x4418)?;
    buffer.write_u16_ne(0x4418)?;

    if cfg!(target_endian = "little") {
        assert_eq!(buffer, vec![0x18, 0x44, 0x44, 0x18, 0x18, 0x44]);
    } else {
        assert_eq!(buffer, vec![0x18, 0x44, 0x44, 0x18, 0x44, 0x18]);
    }

    let mut cursor = Cursor::new(&buffer);
    assert_eq!(cursor.read_u16_le()?, 0x4418);
    assert_eq!(cursor.read_u16_be()?, 0x4418);
    assert_eq!(cursor.read_u16_ne()?, 0x4418);

    return Ok(());
}

#[test]
fn test_u32() -> io::Result<()>{
    let mut buffer: Vec<u8> = vec![];
    buffer.write_u32_le(0x77665544)?;
    buffer.write_u32_be(0x77665544)?;
    buffer.write_u32_ne(0x77665544)?;
    if cfg!(target_endian = "little") {
        assert_eq!(buffer, vec![0x44, 0x55, 0x66, 0x77, 0x77, 0x66, 0x55, 0x44, 0x44, 0x55, 0x66, 0x77]);
    } else {
        assert_eq!(buffer, vec![0x77, 0x66, 0x55, 0x44, 0x77, 0x66, 0x55, 0x44, 0x77, 0x66, 0x55, 0x44]);
    }
    let mut cursor = Cursor::new(&buffer);
    assert_eq!(cursor.read_u32_le()?, 0x77665544);
    assert_eq!(cursor.read_u32_be()?, 0x77665544);
    assert_eq!(cursor.read_u32_ne()?, 0x77665544);
    return Ok(());
}


#[test]
fn test_u64() -> io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    buffer.write_u64_le(0x8877665544332211)?;
    buffer.write_u64_be(0x8877665544332211)?;
    buffer.write_u64_ne(0x8877665544332211)?;
    if cfg!(target_endian = "little") {
        assert_eq!(buffer, vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
    } else {
        assert_eq!(buffer, vec![0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]);
    }
    let mut cursor = Cursor::new(&buffer);
    assert_eq!(cursor.read_u64_le()?, 0x8877665544332211);
    assert_eq!(cursor.read_u64_be()?, 0x8877665544332211);
    assert_eq!(cursor.read_u64_ne()?, 0x8877665544332211);
    return Ok(());
}

#[test]
fn test_u128() -> io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    buffer.write_u128_le(0x1100ffe500ddaabbaabbccdd11223344)?;
    buffer.write_u128_be(0x1100ffe500ddaabbaabbccdd11223344)?;


    buffer.write_u128_ne(0x1100ffe500ddaabbaabbccdd11223344)?;

    if cfg!(target_endian = "little") {
        assert_eq!(buffer, vec![0x44, 0x33, 0x22, 0x11, 0xdd, 0xcc, 0xbb, 0xaa,
                                0xbb, 0xaa, 0xdd, 0x00, 0xe5, 0xff, 0x00, 0x11, //le

                                0x11, 0x00, 0xff, 0xe5, 0x00, 0xdd, 0xaa, 0xbb,
                                0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x22, 0x33, 0x44, //be

                                0x44, 0x33, 0x22, 0x11, 0xdd, 0xcc, 0xbb, 0xaa,
                                0xbb, 0xaa, 0xdd, 0x00, 0xe5, 0xff, 0x00, 0x11, //le
        ]);
    } else {
        assert_eq!(buffer, vec![0x44, 0x33, 0x22, 0x11, 0xdd, 0xcc, 0xbb, 0xaa,
                                0xbb, 0xaa, 0xdd, 0x00, 0xe5, 0xff, 0x00, 0x11, //le

                                0x11, 0x00, 0xff, 0xe5, 0x00, 0xdd, 0xaa, 0xbb,
                                0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x22, 0x33, 0x44, //be

                                0x11, 0x00, 0xff, 0xe5, 0x00, 0xdd, 0xaa, 0xbb,
                                0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x22, 0x33, 0x44, //be
        ]);
    }
    let mut cursor = Cursor::new(&buffer);
    assert_eq!(cursor.read_u128_le()?, 0x1100ffe500ddaabbaabbccdd11223344);
    assert_eq!(cursor.read_u128_be()?, 0x1100ffe500ddaabbaabbccdd11223344);
    assert_eq!(cursor.read_u128_ne()?, 0x1100ffe500ddaabbaabbccdd11223344);
    return Ok(());
}


#[test]
fn test_i8() -> io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    buffer.write_i8(5)?;
    buffer.write_i8(-34)?;
    buffer.write_i8(2)?;
    assert_eq!(buffer, vec![5, -34i8 as u8, 2]);

    let mut cursor = Cursor::new(&buffer);
    assert_eq!(cursor.read_i8()?, 5);
    assert_eq!(cursor.read_i8()?, -34);
    assert_eq!(cursor.read_i8()?, 2);

    return Ok(());
}

#[test]
fn test_i16() -> io::Result<()>{
    let mut buffer: Vec<u8> = vec![];
    buffer.write_i16_le(0x4418)?;
    buffer.write_i16_be(0x4418)?;
    buffer.write_i16_ne(0x4418)?;

    if cfg!(target_endian = "little") {
        assert_eq!(buffer, vec![0x18, 0x44, 0x44, 0x18, 0x18, 0x44]);
    } else {
        assert_eq!(buffer, vec![0x18, 0x44, 0x44, 0x18, 0x44, 0x18]);
    }

    let mut cursor = Cursor::new(&buffer);
    assert_eq!(cursor.read_i16_le()?, 0x4418);
    assert_eq!(cursor.read_i16_be()?, 0x4418);
    assert_eq!(cursor.read_i16_ne()?, 0x4418);

    return Ok(());
}

#[test]
fn test_i32() -> io::Result<()>{
    let mut buffer: Vec<u8> = vec![];
    buffer.write_i32_le(0x77665544)?;
    buffer.write_i32_be(0x77665544)?;
    buffer.write_i32_ne(0x77665544)?;
    if cfg!(target_endian = "little") {
        assert_eq!(buffer, vec![0x44, 0x55, 0x66, 0x77, 0x77, 0x66, 0x55, 0x44, 0x44, 0x55, 0x66, 0x77]);
    } else {
        assert_eq!(buffer, vec![0x77, 0x66, 0x55, 0x44, 0x77, 0x66, 0x55, 0x44, 0x77, 0x66, 0x55, 0x44]);
    }
    let mut cursor = Cursor::new(&buffer);
    assert_eq!(cursor.read_i32_le()?, 0x77665544);
    assert_eq!(cursor.read_i32_be()?, 0x77665544);
    assert_eq!(cursor.read_i32_ne()?, 0x77665544);
    return Ok(());
}

#[test]
fn test_i64() -> io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    buffer.write_i64_le(0x1877665544332211)?;
    buffer.write_i64_be(0x1877665544332211)?;
    buffer.write_i64_ne(0x1877665544332211)?;
    if cfg!(target_endian = "little") {
        assert_eq!(buffer, vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x18, 0x18, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x18]);
    } else {
        assert_eq!(buffer, vec![0x18, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x18, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x18, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]);
    }
    let mut cursor = Cursor::new(&buffer);
    assert_eq!(cursor.read_i64_le()?, 0x1877665544332211);
    assert_eq!(cursor.read_i64_be()?, 0x1877665544332211);
    assert_eq!(cursor.read_i64_ne()?, 0x1877665544332211);
    return Ok(());
}

#[test]
fn test_i128() -> io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    buffer.write_i128_le(0x1100ffe500ddaabbaabbccdd11223344)?;
    buffer.write_i128_be(0x1100ffe500ddaabbaabbccdd11223344)?;
    buffer.write_i128_ne(0x1100ffe500ddaabbaabbccdd11223344)?;

    if cfg!(target_endian = "little") {
        assert_eq!(buffer, vec![0x44, 0x33, 0x22, 0x11, 0xdd, 0xcc, 0xbb, 0xaa,
                                0xbb, 0xaa, 0xdd, 0x00, 0xe5, 0xff, 0x00, 0x11, //le

                                0x11, 0x00, 0xff, 0xe5, 0x00, 0xdd, 0xaa, 0xbb,
                                0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x22, 0x33, 0x44, //be

                                0x44, 0x33, 0x22, 0x11, 0xdd, 0xcc, 0xbb, 0xaa,
                                0xbb, 0xaa, 0xdd, 0x00, 0xe5, 0xff, 0x00, 0x11, //le
        ]);
    } else {
        assert_eq!(buffer, vec![0x44, 0x33, 0x22, 0x11, 0xdd, 0xcc, 0xbb, 0xaa,
                                0xbb, 0xaa, 0xdd, 0x00, 0xe5, 0xff, 0x00, 0x11, //le

                                0x11, 0x00, 0xff, 0xe5, 0x00, 0xdd, 0xaa, 0xbb,
                                0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x22, 0x33, 0x44, //be

                                0x11, 0x00, 0xff, 0xe5, 0x00, 0xdd, 0xaa, 0xbb,
                                0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x22, 0x33, 0x44, //be
        ]);
    }
    let mut cursor = Cursor::new(&buffer);
    assert_eq!(cursor.read_i128_le()?, 0x1100ffe500ddaabbaabbccdd11223344);
    assert_eq!(cursor.read_i128_be()?, 0x1100ffe500ddaabbaabbccdd11223344);
    assert_eq!(cursor.read_i128_ne()?, 0x1100ffe500ddaabbaabbccdd11223344);
    return Ok(());
}


