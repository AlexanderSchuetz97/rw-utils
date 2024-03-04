use std::io;
use std::io::Read;
use std::mem::size_of;

///
/// Trait that provides various methods to read numbers.
/// Automatically implemented for all implementations of io::Read.
/// This trait is sealed and cannot be implemented manually.
///
pub trait NumRead : private::Sealed {
    ///
    /// Reads one byte and treats 0 as false and any other value as true.
    ///
    fn read_bool(&mut self) -> io::Result<bool>;

    ///
    /// Reads 1 byte and treats it as an u8
    ///
    fn read_u8(&mut self) -> io::Result<u8>;

    ///
    /// Reads 2 bytes and treats them as an u16 in little endian byte order
    ///
    fn read_u16_le(&mut self) -> io::Result<u16>;

    ///
    /// Reads 2 bytes and treats them as an u16 in big endian byte order
    ///
    fn read_u16_be(&mut self) -> io::Result<u16>;

    ///
    /// Reads 2 bytes and treats them as an u16 in native byte order
    ///
    fn read_u16_ne(&mut self) -> io::Result<u16>;

    ///
    /// Reads 4 bytes and treats them as an u32 in little endian byte order
    ///
    fn read_u32_le(&mut self) -> io::Result<u32>;

    ///
    /// Reads 4 bytes and treats them as an u32 in big endian byte order
    ///
    fn read_u32_be(&mut self) -> io::Result<u32>;

    ///
    /// Reads 4 bytes and treats them as an u32 in native byte order
    ///
    fn read_u32_ne(&mut self) -> io::Result<u32>;

    ///
    /// Reads 8 bytes and treats them as an u64 in little endian byte order
    ///
    fn read_u64_le(&mut self) -> io::Result<u64>;

    ///
    /// Reads 8 bytes and treats them as an u64 in big endian byte order
    ///
    fn read_u64_be(&mut self) -> io::Result<u64>;

    ///
    /// Reads 8 bytes and treats them as an u64 in native byte order
    ///
    fn read_u64_ne(&mut self) -> io::Result<u64>;

    ///
    /// Reads 16 bytes and treats them as an u128 in little endian byte order
    ///
    fn read_u128_le(&mut self) -> io::Result<u128>;

    ///
    /// Reads 16 bytes and treats them as an u128 in big endian byte order
    ///
    fn read_u128_be(&mut self) -> io::Result<u128>;

    ///
    /// Reads 16 bytes and treats them as an u128 in native byte order
    ///
    fn read_u128_ne(&mut self) -> io::Result<u128>;

    ///
    /// Reads 1 byte and treats it as an i8
    ///
    fn read_i8(&mut self) -> io::Result<i8>;

    ///
    /// Reads 2 bytes and treats them as an i16 in little endian byte order
    ///
    fn read_i16_le(&mut self) -> io::Result<i16>;

    ///
    /// Reads 2 bytes and treats them as an i16 in big endian byte order
    ///
    fn read_i16_be(&mut self) -> io::Result<i16>;

    ///
    /// Reads 2 bytes and treats them as an u16 in native byte order
    ///
    fn read_i16_ne(&mut self) -> io::Result<i16>;

    ///
    /// Reads 4 bytes and treats them as an i32 in little endian byte order
    ///
    fn read_i32_le(&mut self) -> io::Result<i32>;

    ///
    /// Reads 4 bytes and treats them as an i32 in big endian byte order
    ///
    fn read_i32_be(&mut self) -> io::Result<i32>;

    ///
    /// Reads 4 bytes and treats them as an i32 in native byte order
    ///
    fn read_i32_ne(&mut self) -> io::Result<i32>;

    ///
    /// Reads 8 bytes and treats them as an i64 in little endian byte order
    ///
    fn read_i64_le(&mut self) -> io::Result<i64>;

    ///
    /// Reads 8 bytes and treats them as an i64 in big endian byte order
    ///
    fn read_i64_be(&mut self) -> io::Result<i64>;

    ///
    /// Reads 8 bytes and treats them as an u64 in native byte order
    ///
    fn read_i64_ne(&mut self) -> io::Result<i64>;

    ///
    /// Reads 16 bytes and treats them as an u128 in little endian byte order
    ///
    fn read_i128_le(&mut self) -> io::Result<i128>;

    ///
    /// Reads 16 bytes and treats them as an u128 in big endian byte order
    ///
    fn read_i128_be(&mut self) -> io::Result<i128>;

    ///
    /// Reads 16 bytes and treats them as an u128 in native byte order
    ///
    fn read_i128_ne(&mut self) -> io::Result<i128>;

    ///
    /// Reads 4 bytes and treats them as an f32 in little endian byte order
    ///
    fn read_f32_le(&mut self) -> io::Result<f32>;

    ///
    /// Reads 4 bytes and treats them as an f32 in big endian byte order
    ///
    fn read_f32_be(&mut self) -> io::Result<f32>;

    ///
    /// Reads 4 bytes and treats them as an f32 in native byte order
    ///
    fn read_f32_ne(&mut self) -> io::Result<f32>;

    ///
    /// Reads 8 bytes and treats them as an f64 in little endian byte order
    ///
    fn read_f64_le(&mut self) -> io::Result<f64>;

    ///
    /// Reads 8 bytes and treats them as an f64 in big endian byte order
    ///
    fn read_f64_be(&mut self) -> io::Result<f64>;

    ///
    /// Reads 8 bytes and treats them as an f64 in native byte order
    ///
    fn read_f64_ne(&mut self) -> io::Result<f64>;

    ///
    /// Reads bytes to fill the slice. Each byte is treated as a u8.
    ///
    fn read_u8_slice(&mut self, slice: &mut [u8]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each byte is treated as a i8.
    ///
    fn read_i8_slice(&mut self, slice: &mut [i8]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 2 bytes is treated as an u16 in little endian
    ///
    fn read_u16_slice_le(&mut self, slice: &mut [u16]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 2 bytes is treated as an u16 in big endian
    ///
    fn read_u16_slice_be(&mut self, slice: &mut [u16]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 2 bytes is treated as an u16 in native endian
    ///
    fn read_u16_slice_ne(&mut self, slice: &mut [u16]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 4 bytes is treated as an u32 in little endian
    ///
    fn read_u32_slice_le(&mut self, slice: &mut [u32]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 4 bytes is treated as an u32 in big endian
    ///
    fn read_u32_slice_be(&mut self, slice: &mut [u32]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 4 bytes is treated as an u32 in native endian
    ///
    fn read_u32_slice_ne(&mut self, slice: &mut [u32]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 8 bytes is treated as an u64 in little endian
    ///
    fn read_u64_slice_le(&mut self, slice: &mut [u64]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 8 bytes is treated as an u64 in big endian
    ///
    fn read_u64_slice_be(&mut self, slice: &mut [u64]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 8 bytes is treated as an u64 in native endian
    ///
    fn read_u64_slice_ne(&mut self, slice: &mut [u64]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 16 bytes is treated as an u128 in little endian
    ///
    fn read_u128_slice_le(&mut self, slice: &mut [u128]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 16 bytes is treated as an u128 in big endian
    ///
    fn read_u128_slice_be(&mut self, slice: &mut [u128]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 16 bytes is treated as an u128 in native endian
    ///
    fn read_u128_slice_ne(&mut self, slice: &mut [u128]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 2 bytes is treated as an i16 in little endian
    ///
    fn read_i16_slice_le(&mut self, slice: &mut [i16]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 2 bytes is treated as an i16 in big endian
    ///
    fn read_i16_slice_be(&mut self, slice: &mut [i16]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 2 bytes is treated as an i16 in native endian
    ///
    fn read_i16_slice_ne(&mut self, slice: &mut [i16]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 4 bytes is treated as an i32 in little endian
    ///
    fn read_i32_slice_le(&mut self, slice: &mut [i32]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 4 bytes is treated as an i32 in big endian
    ///
    fn read_i32_slice_be(&mut self, slice: &mut [i32]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 4 bytes is treated as an i32 in native endian
    ///
    fn read_i32_slice_ne(&mut self, slice: &mut [i32]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 8 bytes is treated as an i64 in little endian
    ///
    fn read_i64_slice_le(&mut self, slice: &mut [i64]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 8 bytes is treated as an i64 in big endian
    ///
    fn read_i64_slice_be(&mut self, slice: &mut [i64]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 8 bytes is treated as an i64 in native endian
    ///
    fn read_i64_slice_ne(&mut self, slice: &mut [i64]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 16 bytes is treated as an i128 in little endian
    ///
    fn read_i128_slice_le(&mut self, slice: &mut [i128]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 16 bytes is treated as an i128 in big endian
    ///
    fn read_i128_slice_be(&mut self, slice: &mut [i128]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 16 bytes is treated as an i128 in native endian
    ///
    fn read_i128_slice_ne(&mut self, slice: &mut [i128]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 8 bytes is treated as an f64 in little endian
    ///
    fn read_f64_slice_le(&mut self, slice: &mut [f64]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 8 bytes is treated as an f64 in big endian
    ///
    fn read_f64_slice_be(&mut self, slice: &mut [f64]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 8 bytes is treated as an f64 in native endian
    ///
    fn read_f64_slice_ne(&mut self, slice: &mut [f64]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 4 bytes is treated as an f32 in little endian
    ///
    fn read_f32_slice_le(&mut self, slice: &mut [f32]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 4 bytes is treated as an f32 in big endian
    ///
    fn read_f32_slice_be(&mut self, slice: &mut [f32]) -> io::Result<()>;

    ///
    /// Reads bytes to fill the slice. Each group of 4 bytes is treated as an f32 in native endian
    ///
    fn read_f32_slice_ne(&mut self, slice: &mut [f32]) -> io::Result<()>;

    ///
    /// reads size amount of data into a new Vec<u8>
    ///
    fn read_u8_vec(&mut self, size: usize) -> io::Result<Vec<u8>>;

    ///
    /// reads size amount of data into a new Vec<i8>
    ///
    fn read_i8_vec(&mut self, size: usize) -> io::Result<Vec<i8>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 2 bytes is treated as an u16 in little endian
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_u16_vec_le(&mut self, size: usize) -> io::Result<Vec<u16>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 2 bytes is treated as an u16 in big endian
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_u16_vec_be(&mut self, size: usize) -> io::Result<Vec<u16>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 2 bytes is treated as an u16 in native endian
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_u16_vec_ne(&mut self, size: usize) -> io::Result<Vec<u16>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 4 bytes is treated as an u32 in little endian
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_u32_vec_le(&mut self, size: usize) -> io::Result<Vec<u32>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 4 bytes is treated as an u32 in big endian
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_u32_vec_be(&mut self, size: usize) -> io::Result<Vec<u32>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 4 bytes is treated as an u32 in native endian
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_u32_vec_ne(&mut self, size: usize) -> io::Result<Vec<u32>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 8 bytes is treated as an u64 in little endian
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_u64_vec_le(&mut self, size: usize) -> io::Result<Vec<u64>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 8 bytes is treated as an u64 in big endian
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_u64_vec_be(&mut self, size: usize) -> io::Result<Vec<u64>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 8 bytes is treated as an u64 in native endian
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_u64_vec_ne(&mut self, size: usize) -> io::Result<Vec<u64>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 16 bytes is treated as an u128 in little endian
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_u128_vec_le(&mut self, size: usize) -> io::Result<Vec<u128>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 16 bytes is treated as an u128 in big endian
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_u128_vec_be(&mut self, size: usize) -> io::Result<Vec<u128>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 16 bytes is treated as an u128 in native endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_u128_vec_ne(&mut self, size: usize) -> io::Result<Vec<u128>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 2 bytes is treated as an i16 in little endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_i16_vec_le(&mut self, size: usize) -> io::Result<Vec<i16>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 2 bytes is treated as an i16 in big endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_i16_vec_be(&mut self, size: usize) -> io::Result<Vec<i16>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 2 bytes is treated as an i16 in native endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_i16_vec_ne(&mut self, size: usize) -> io::Result<Vec<i16>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 4 bytes is treated as an i32 in little endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_i32_vec_le(&mut self, size: usize) -> io::Result<Vec<i32>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 4 bytes is treated as an i32 in big endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_i32_vec_be(&mut self, size: usize) -> io::Result<Vec<i32>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 4 bytes is treated as an i32 in native endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_i32_vec_ne(&mut self, size: usize) -> io::Result<Vec<i32>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 8 bytes is treated as an i64 in little endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_i64_vec_le(&mut self, size: usize) -> io::Result<Vec<i64>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 8 bytes is treated as an i64 in big endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_i64_vec_be(&mut self, size: usize) -> io::Result<Vec<i64>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 8 bytes is treated as an i64 in native endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_i64_vec_ne(&mut self, size: usize) -> io::Result<Vec<i64>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 16 bytes is treated as an i128 in little endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_i128_vec_le(&mut self, size: usize) -> io::Result<Vec<i128>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 16 bytes is treated as an i128 in big endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_i128_vec_be(&mut self, size: usize) -> io::Result<Vec<i128>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 16 bytes is treated as an i128 in native endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_i128_vec_ne(&mut self, size: usize) -> io::Result<Vec<i128>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 8 bytes is treated as an f64 in little endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_f64_vec_le(&mut self, size: usize) -> io::Result<Vec<f64>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 8 bytes is treated as an f64 in big endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_f64_vec_be(&mut self, size: usize) -> io::Result<Vec<f64>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 8 bytes is treated as an f64 in native endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_f64_vec_ne(&mut self, size: usize) -> io::Result<Vec<f64>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 8 bytes is treated as an f32 in little endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_f32_vec_le(&mut self, size: usize) -> io::Result<Vec<f32>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 8 bytes is treated as an f32 in big endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_f32_vec_be(&mut self, size: usize) -> io::Result<Vec<f32>>;

    ///
    /// reads size amount of data into a new Vec.
    /// Each group of 8 bytes is treated as an f32 in native endian.
    /// Size refers to the size of the Vec not the amount of bytes.
    ///
    fn read_f32_vec_ne(&mut self, size: usize) -> io::Result<Vec<f32>>;
}

macro_rules! define_endian_numeric_read_functions {
    ($type:ty, $le_name:ident ,$conv_le:expr, $be_name:ident ,$conv_be:expr, $ne_name:ident) => {
        #[cfg(target_endian = "big")]
        fn $le_name(&mut self) -> io::Result<$type> {
            let mut buf = [0u8; size_of::<$type>()];
            self.read_exact(buf.as_mut_slice())?;
            return Ok($conv_le(buf));
        }

        #[cfg(target_endian = "little")]
        fn $le_name(&mut self) -> io::Result<$type> {
            let mut v: $type = 0 as $type;
            self.read_exact(unsafe { std::slice::from_raw_parts_mut::<u8>(((&mut v) as *mut $type).cast(), size_of::<$type>()) })?;
            return Ok(v);
        }

        #[cfg(target_endian = "big")]
        fn $be_name(&mut self) -> io::Result<$type> {
            let mut v: $type = 0 as $type;
            self.read_exact(unsafe { std::slice::from_raw_parts_mut::<u8>(((&mut v) as *mut $type).cast(), size_of::<$type>()) })?;
            return Ok(v);
        }


        #[cfg(target_endian = "little")]
        fn $be_name(&mut self) -> io::Result<$type> {
            let mut buf = [0u8; size_of::<$type>()];
            self.read_exact(buf.as_mut_slice())?;
            return Ok($conv_be(buf));
        }

        fn $ne_name(&mut self) -> io::Result<$type> {
            let mut v: $type = 0 as $type;
            self.read_exact(unsafe { std::slice::from_raw_parts_mut::<u8>(((&mut v) as *mut $type).cast(), size_of::<$type>()) })?;
            return Ok(v);
        }
    };
}

macro_rules! define_endian_read_slice_with_helper_functions {
    ($type:ty, $helper:ty, $le_name:ident, $be_name:ident, $ne_name:ident) => {
        #[cfg(target_endian = "little")]
        fn $le_name(&mut self, slice: &mut [$type]) -> io::Result<()> {
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len() * size_of::<$type>()) };
            return self.read_exact(sl);
        }

        #[cfg(target_endian = "big")]
        fn $le_name(&mut self, slice: &mut [$type]) -> io::Result<()> {
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len() * size_of::<$type>()) };
            let helper :&mut [$helper] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len()) };
            self.read_exact(sl)?;
            for i in 0 .. helper.len() {
                helper[i] = helper[i].to_le();
            }
            return Ok(());
        }

        #[cfg(target_endian = "little")]
        fn $be_name(&mut self, slice: &mut [$type]) -> io::Result<()> {
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len() * size_of::<$type>()) };
            let helper :&mut [$helper] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len()) };
            self.read_exact(sl)?;
            for i in 0 .. helper.len() {
                helper[i] = helper[i].to_be();
            }
            return Ok(());
        }

        #[cfg(target_endian = "big")]
        fn $be_name(&mut self, slice: &mut [$type]) -> io::Result<()> {
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len() * size_of::<$type>()) };
            return self.read_exact(sl);
        }

        fn $ne_name(&mut self, slice: &mut [$type]) -> io::Result<()> {
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len() * size_of::<$type>()) };
            return self.read_exact(sl);
        }
    }
}
macro_rules! define_endian_read_slice_functions {
    ($type:ty, $le_name:ident, $be_name:ident, $ne_name:ident) => {
        #[cfg(target_endian = "little")]
        fn $le_name(&mut self, slice: &mut [$type]) -> io::Result<()> {
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len() * size_of::<$type>()) };
            return self.read_exact(sl);
        }

        #[cfg(target_endian = "big")]
        fn $le_name(&mut self, slice: &mut [$type]) -> io::Result<()> {
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len() * size_of::<$type>()) };
            self.read_exact(sl)?;
            for i in 0 .. slice.len() {
                slice[i] = slice[i].to_le();
            }
            return Ok(());
        }


        #[cfg(target_endian = "little")]
        fn $be_name(&mut self, slice: &mut [$type]) -> io::Result<()> {
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len() * size_of::<$type>()) };
            self.read_exact(sl)?;
            for i in 0 .. slice.len() {
                slice[i] = slice[i].to_be();
            }
            return Ok(());
        }

        #[cfg(target_endian = "big")]
        fn $be_name(&mut self, slice: &mut [$type]) -> io::Result<()> {
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len() * size_of::<$type>()) };
            return self.read_exact(sl);
        }

        fn $ne_name(&mut self, slice: &mut [$type]) -> io::Result<()> {
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len() * size_of::<$type>()) };
            return self.read_exact(sl);
        }
    }
}

macro_rules! define_endian_read_vec_with_helper_functions {
    ($type:ty, $helper:ty, $le_name:ident, $be_name:ident, $ne_name:ident) => {
        fn $ne_name(&mut self, size: usize) -> io::Result<Vec<$type>> {
            let mut vec : Vec<$type> = vec![0 as $type; size];
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(vec.as_mut_ptr().cast(), vec.len() * size_of::<$type>()) };
            self.read_exact(sl)?;
            return Ok(vec);
        }

        #[cfg(target_endian = "little")]
        fn $le_name(&mut self, size: usize) -> io::Result<Vec<$type>> {
            let mut vec : Vec<$type> = vec![0 as $type; size];
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(vec.as_mut_ptr().cast(), vec.len() * size_of::<$type>()) };
            self.read_exact(sl)?;
            return Ok(vec);
        }

        #[cfg(target_endian = "big")]
        fn $le_name(&mut self, size: usize) -> io::Result<Vec<$type>> {
            let mut vec : Vec<$type> = vec![0 as $type; size];
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(vec.as_mut_ptr().cast(), vec.len() * size_of::<$type>()) };
            let helper :&mut [$helper] = unsafe { std::slice::from_raw_parts_mut(vec.as_mut_ptr().cast(), vec.len()) };
            self.read_exact(sl)?;
            for i in 0 .. helper.len() {
                helper[i] = helper[i].to_le();
            }
            return Ok(vec);
        }


        #[cfg(target_endian = "little")]
        fn $be_name(&mut self, size: usize) -> io::Result<Vec<$type>> {
            let mut vec : Vec<$type> = vec![0 as $type; size];
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(vec.as_mut_ptr().cast(), vec.len() * size_of::<$type>()) };
            let helper :&mut [$helper] = unsafe { std::slice::from_raw_parts_mut(vec.as_mut_ptr().cast(), vec.len()) };
            self.read_exact(sl)?;
            for i in 0 .. helper.len() {
                helper[i] = helper[i].to_be();
            }
            return Ok(vec);
        }

        #[cfg(target_endian = "big")]
        fn $be_name(&mut self, size: usize) -> io::Result<Vec<$type>> {
            let mut vec : Vec<$type> = vec![0 as $type; size];
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(vec.as_mut_ptr().cast(), vec.len() * size_of::<$type>()) };
            self.read_exact(sl)?;
            return Ok(vec);
        }
    }
}

macro_rules! define_endian_read_vec_functions {
    ($type:ty, $le_name:ident, $be_name:ident, $ne_name:ident) => {
        fn $ne_name(&mut self, size: usize) -> io::Result<Vec<$type>> {
            let mut vec : Vec<$type> = vec![0 as $type; size];
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(vec.as_mut_ptr().cast(), vec.len() * size_of::<$type>()) };
            self.read_exact(sl)?;
            return Ok(vec);
        }

        #[cfg(target_endian = "little")]
        fn $le_name(&mut self, size: usize) -> io::Result<Vec<$type>> {
            let mut vec : Vec<$type> = vec![0 as $type; size];
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(vec.as_mut_ptr().cast(), vec.len() * size_of::<$type>()) };
            self.read_exact(sl)?;
            return Ok(vec);
        }

        #[cfg(target_endian = "big")]
        fn $le_name(&mut self, size: usize) -> io::Result<Vec<$type>> {
            let mut vec : Vec<$type> = vec![0 as $type; size];
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(vec.as_mut_ptr().cast(), vec.len() * size_of::<$type>()) };
            self.read_exact(sl)?;
            for i in 0 .. vec.len() {
                vec[i] = vec[i].to_le();
            }
            return Ok(vec);
        }


        #[cfg(target_endian = "little")]
        fn $be_name(&mut self, size: usize) -> io::Result<Vec<$type>> {
            let mut vec : Vec<$type> = vec![0 as $type; size];
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(vec.as_mut_ptr().cast(), vec.len() * size_of::<$type>()) };
            self.read_exact(sl)?;
            for i in 0 .. vec.len() {
                vec[i] = vec[i].to_be();
            }
            return Ok(vec);
        }

        #[cfg(target_endian = "big")]
        fn $be_name(&mut self, size: usize) -> io::Result<Vec<$type>> {
            let mut vec : Vec<$type> = vec![0 as $type; size];
            let sl :&mut [u8] = unsafe { std::slice::from_raw_parts_mut(vec.as_mut_ptr().cast(), vec.len() * size_of::<$type>()) };
            self.read_exact(sl)?;
            return Ok(vec);
        }
    }
}

impl<T> NumRead for T where T: Read {
    fn read_bool(&mut self) -> io::Result<bool> {
        let mut v = [0u8];
        self.read_exact(v.as_mut_slice())?;
        return Ok(v[0] != 0);
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        let mut v = [0u8];
        self.read_exact(v.as_mut_slice())?;
        return Ok(v[0]);
    }

    fn read_i8(&mut self) -> io::Result<i8> {
        let mut v = [0u8];
        self.read_exact(v.as_mut_slice())?;
        return Ok(v[0] as i8);
    }

    define_endian_numeric_read_functions!(u16, read_u16_le, u16::from_le_bytes, read_u16_be, u16::from_be_bytes, read_u16_ne);
    define_endian_numeric_read_functions!(u32, read_u32_le, u32::from_le_bytes, read_u32_be, u32::from_be_bytes, read_u32_ne);
    define_endian_numeric_read_functions!(u64, read_u64_le, u64::from_le_bytes, read_u64_be, u64::from_be_bytes, read_u64_ne);
    define_endian_numeric_read_functions!(u128, read_u128_le, u128::from_le_bytes, read_u128_be, u128::from_be_bytes, read_u128_ne);

    define_endian_numeric_read_functions!(i16, read_i16_le, i16::from_le_bytes, read_i16_be, i16::from_be_bytes, read_i16_ne);
    define_endian_numeric_read_functions!(i32, read_i32_le, i32::from_le_bytes, read_i32_be, i32::from_be_bytes, read_i32_ne);
    define_endian_numeric_read_functions!(i64, read_i64_le, i64::from_le_bytes, read_i64_be, i64::from_be_bytes, read_i64_ne);
    define_endian_numeric_read_functions!(i128, read_i128_le, i128::from_le_bytes, read_i128_be, i128::from_be_bytes, read_i128_ne);

    define_endian_numeric_read_functions!(f32, read_f32_le, f32::from_le_bytes, read_f32_be, f32::from_be_bytes, read_f32_ne);
    define_endian_numeric_read_functions!(f64, read_f64_le, f64::from_le_bytes, read_f64_be, f64::from_be_bytes, read_f64_ne);




    fn read_u8_slice(&mut self, slice: &mut [u8]) -> io::Result<()> {
        return self.read_exact(slice);
    }

    fn read_i8_slice(&mut self, slice: &mut [i8]) -> io::Result<()> {
        let sl: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len()) };
        return self.read_exact(sl);
    }

    define_endian_read_slice_functions!(u16, read_u16_slice_le, read_u16_slice_be, read_u16_slice_ne);
    define_endian_read_slice_functions!(u32, read_u32_slice_le, read_u32_slice_be, read_u32_slice_ne);
    define_endian_read_slice_functions!(u64, read_u64_slice_le, read_u64_slice_be, read_u64_slice_ne);
    define_endian_read_slice_functions!(u128, read_u128_slice_le, read_u128_slice_be, read_u128_slice_ne);

    define_endian_read_slice_functions!(i16, read_i16_slice_le, read_i16_slice_be, read_i16_slice_ne);
    define_endian_read_slice_functions!(i32, read_i32_slice_le, read_i32_slice_be, read_i32_slice_ne);
    define_endian_read_slice_functions!(i64, read_i64_slice_le, read_i64_slice_be, read_i64_slice_ne);
    define_endian_read_slice_functions!(i128, read_i128_slice_le, read_i128_slice_be, read_i128_slice_ne);

    fn read_u8_vec(&mut self, size: usize) -> io::Result<Vec<u8>> {
        let mut x = vec![0u8; size];
        self.read_exact(x.as_mut_slice())?;
        return Ok(x);
    }
    fn read_i8_vec(&mut self, size: usize) -> io::Result<Vec<i8>> {
        let mut x = vec![0i8; size];
        self.read_i8_slice(x.as_mut_slice())?;
        return Ok(x);
    }

    define_endian_read_vec_functions!(u16, read_u16_vec_le, read_u16_vec_be, read_u16_vec_ne);
    define_endian_read_vec_functions!(u32, read_u32_vec_le, read_u32_vec_be, read_u32_vec_ne);
    define_endian_read_vec_functions!(u64, read_u64_vec_le, read_u64_vec_be, read_u64_vec_ne);
    define_endian_read_vec_functions!(u128, read_u128_vec_le, read_u128_vec_be, read_u128_vec_ne);

    define_endian_read_vec_functions!(i16, read_i16_vec_le, read_i16_vec_be, read_i16_vec_ne);
    define_endian_read_vec_functions!(i32, read_i32_vec_le, read_i32_vec_be, read_i32_vec_ne);
    define_endian_read_vec_functions!(i64, read_i64_vec_le, read_i64_vec_be, read_i64_vec_ne);
    define_endian_read_vec_functions!(i128, read_i128_vec_le, read_i128_vec_be, read_i128_vec_ne);

    define_endian_read_slice_with_helper_functions!(f64, u64, read_f64_slice_le, read_f64_slice_be, read_f64_slice_ne);
    define_endian_read_slice_with_helper_functions!(f32, u32, read_f32_slice_le, read_f32_slice_be, read_f32_slice_ne);
    define_endian_read_vec_with_helper_functions!(f64, u64, read_f64_vec_le, read_f64_vec_be, read_f64_vec_ne);
    define_endian_read_vec_with_helper_functions!(f32, u32, read_f32_vec_le, read_f32_vec_be, read_f32_vec_ne);

}

mod private {
    use std::io::Read;

    impl <T> Sealed for T where T: Read {}
    pub trait Sealed {

    }
}
