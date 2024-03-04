use std::io;
use std::io::{Cursor};
use rw_utils::leb128_read::Leb128Read;
use rw_utils::leb128_write::Leb128Write;
use rw_utils::num_read::NumRead;
use rw_utils::num_write::NumWrite;

#[test]
fn test_leb_wikipedia() -> io::Result<()>{

    let buffer = vec![0b11000000u8, 0b10111011u8,0b01111000u8];
    let mut cursor = Cursor::new(&buffer);
    let r = cursor.read_leb128_i32()?;
    assert_eq!(-123456, r);

    return Ok(());
}

const MIN_NEXT : u8 = 0b1000_0000u8;
const MAX_NEXT : u8 = 0b1111_1111u8;
const MAX_END : u8 = 0b0111_1111u8;
const MIN_END : u8 = 0b0000_0001u8;
const MIN_END_S : u8 = 0b0100_0000u8;


#[test]
fn test_leb_u128() -> io::Result<()>{
    let buf : Vec<u8> = vec![MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_END];
    let mut cursor = Cursor::new(&buf);
    let x = cursor.read_leb128_u128()?;
    let mut y: u128 = 1;
    y = y << 70;
    assert_eq!(x,y);

    return Ok(());
}

#[test]
fn test_leb_i128() -> io::Result<()>{
    let buf : Vec<u8> = vec![MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_NEXT,MIN_END_S];
    let mut cursor = Cursor::new(&buf);
    let x = cursor.read_leb128_i128()?;
    let mut y: u128 = 1;
    y = y << 76;
    y = y -1;
    y = !y;

    assert_eq!(x,y as i128);

    return Ok(());
}

#[test]
fn test_leb_u64() -> io::Result<()>{
    let testdata = vec![
        vec![0],
        vec![MIN_NEXT, MIN_END],
        vec![MIN_NEXT, MIN_NEXT, MIN_END],
        vec![MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_END],
        vec![MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_END],
        vec![MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_END],
        vec![MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_END],
        vec![MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_END],
        vec![MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_END],

        vec![MAX_END],
        vec![MAX_NEXT, MAX_END],
        vec![MAX_NEXT, MAX_NEXT, MAX_END],
        vec![MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_END],
        vec![MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_END],
        vec![MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_END],
        vec![MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_END],
        vec![MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_END],
        vec![MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_END],
    ];

    for v in testdata {
        let mut cursor = Cursor::new(&v);
        let r = cursor.read_leb128_u64()?;
        let mut cursor = Cursor::new(&v);
        let l = leb128::read::unsigned(&mut cursor).unwrap();
        assert_eq!(r, l);
    }

    return Ok(());
}

#[test]
fn test_leb_u32() -> io::Result<()>{
    let testdata = vec![
        vec![0],
        vec![MIN_NEXT, MIN_END],
        vec![MIN_NEXT, MIN_NEXT, MIN_END],
        vec![MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_END],
        vec![MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_NEXT, MIN_END],

        vec![MAX_END],
        vec![MAX_NEXT, MAX_END],
        vec![MAX_NEXT, MAX_NEXT, MAX_END],
        vec![MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_END],
        vec![MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_NEXT, MAX_END],
    ];

    for v in testdata {
        let mut cursor = Cursor::new(&v);
        let r = cursor.read_leb128_u32()?;
        let mut cursor = Cursor::new(&v);
        let l = leb128::read::unsigned(&mut cursor).unwrap() as u32;
        assert_eq!(r, l);
    }

    return Ok(());
}

#[test]
fn test_read_large_unsigned_leading_zero() -> io::Result<()>{
    let dta = vec![128u8, 128, 1];
    let mut cursor = Cursor::new(&dta);
    let parsed = cursor.read_leb128_large_unsigned(2)?;
    let mut acc = parsed[0] as u16;
    if parsed.len() > 1 {
        acc |= (parsed[1] as u16) << 8;
    }

    assert_eq!(acc, 16384);
    return Ok(());
}

#[test]
fn test_read_large_unsigned_u64() -> io::Result<()> {
    let mut data : Vec<u8> = Vec::new();
    leb128::write::unsigned(&mut data, u64::MAX)?;
    let mut cursor = Cursor::new(&data);
    let r = cursor.read_leb128_large_unsigned(8)?;
    assert_eq!(r.len(), 8);
    for x in r {
        assert_eq!(x, u8::MAX);
    }

    return Ok(());
}

#[test]
fn test_write_large_unsigned() -> io::Result<()> {
    {
        let x = vec![128u8, 0u8];
        let mut b: Vec<u8> = vec![];
        b.write_leb128_large_unsigned(&x)?;
        assert_eq!(b, vec![128, 1]);
    }

    {
        let x = vec![128u8, 244, 222, 12];
        let mut cursor = Cursor::new(&x);
        let n = cursor.read_u32_le()?;
        let mut xt : Vec<u8> = vec![];
        leb128::write::unsigned(&mut xt, n as u64)?;

        let mut b: Vec<u8> = vec![];
        b.write_leb128_large_unsigned(&x)?;
        assert_eq!(b, xt);

        let mut cursor = Cursor::new(&b);
        let x2 = cursor.read_leb128_large_unsigned(x.len())?;

        assert_eq!(x, x2);
    }

    {
        let mut x = vec![];
        x.write_u64_le(u64::MAX-15)?;
        let mut cursor = Cursor::new(&x);
        let n = cursor.read_u64_le()?;
        let mut xt : Vec<u8> = vec![];
        leb128::write::unsigned(&mut xt, n as u64)?;

        let mut b: Vec<u8> = vec![];
        b.write_leb128_large_unsigned(&x)?;
        assert_eq!(b, xt);
        let mut cursor = Cursor::new(&b);
        let x2 = cursor.read_leb128_large_unsigned(x.len())?;
        assert_eq!(x, x2);
    }

    return Ok(());
}





#[test]
fn test_leb_u16() -> io::Result<()>{
    for i in 0 .. u16::MAX {
        let mut buf: Vec<u8> = vec![];
        leb128::write::unsigned(&mut buf, i as u64)?;
        let mut cursor = Cursor::new(&buf);
        let r = cursor.read_leb128_u16()?;
        assert_eq!(r, i);
        let mut cursor = Cursor::new(&buf);
        let parsed = cursor.read_leb128_large_unsigned(2)?;
        let mut acc = parsed[0] as u16;
        if parsed.len() > 1 {
            acc |= (parsed[1] as u16) << 8;
        }
        assert_eq!(acc, i);
        let mut buf2: Vec<u8> = vec![];
        buf2.write_leb128_u16(i)?;
        assert_eq!(buf, buf2, "{}", i);
        let mut buf3: Vec<u8> = vec![];
        buf3.write_leb128_large_unsigned(&parsed)?;
        assert_eq!(buf, buf3, "{}", i);
    }

    return Ok(());
}

#[test]
fn test_write_large_leb_signed() -> io::Result<()>{
    {
        let i = -16384i16;
        let mut expected: Vec<u8> = vec![];
        leb128::write::signed(&mut expected, i as i64)?;

        let mut input: Vec<u8> = vec![];
        input.write_i16_le(i)?;

        let mut result: Vec<u8> = vec![];
        result.write_leb128_large_signed(&input)?;
        assert_eq!(expected, result, "{}", i);
    }

    {
        let i = -8192i16;
        let mut expected: Vec<u8> = vec![];
        leb128::write::signed(&mut expected, i as i64)?;

        let mut input: Vec<u8> = vec![];
        input.write_i16_le(i)?;

        let mut result: Vec<u8> = vec![];
        result.write_leb128_large_signed(&input)?;
        assert_eq!(expected, result, "{}", i);
    }

    return Ok(());
}
#[test]
fn test_leb_i16() -> io::Result<()>{
    for i in i16::MIN .. i16::MAX {
        let mut buf: Vec<u8> = vec![];
        leb128::write::signed(&mut buf, i as i64)?;
        let mut cursor = Cursor::new(&buf);
        let r = cursor.read_leb128_i16()?;
        assert_eq!(r, i);
        let mut cursor = Cursor::new(&buf);
        let parsed = cursor.read_leb128_large_signed(2)?;
        let mut acc = parsed[0] as u16;
        if parsed.len() > 1 {
            acc |= (parsed[1] as u16) << 8;
        } else {
            if acc & 0b1000_0000u16 != 0 {
                acc |= 0b1111_1111_0000_0000u16
            }
        }
        assert_eq!(acc as i16, i);
        let mut buf3: Vec<u8> = vec![];
        buf3.write_leb128_large_signed(&parsed)?;
        assert_eq!(buf, buf3, "{}", i);
    }

    return Ok(());
}


//#[test]
#[allow(dead_code)]
fn test_leb_i32() -> io::Result<()>{

    let mut buf : Vec<u8> = Vec::new();
    for i in i32::MIN .. i32::MAX {
        if (i as u32) % 4096 == 0 {
            dbg!(i);
        }
        buf.clear();
        leb128::write::signed(&mut buf, i as i64)?;

        let mut cursor = Cursor::new(&buf);
        let r = cursor.read_leb128_i32()?;
        assert_eq!(i, r);
    }

    return Ok(());
}
