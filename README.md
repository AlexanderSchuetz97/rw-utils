# rw-utils
Collection of utilities that enhance the rust Read and Write traits 
by providing various utility method for reading/writing structured data.

### Utilities

* reading/writing of all rust integer types in little and big endian.
* reading/writing of vec/slice of all integer types in little and big endian.
* reading/writing of signed and unsigned leb128 in all/arbitrary sizes
* reading/writing of strings in various encodings
  * Notable mention is a method for reading/writing a string compatible to java's 
    DataInput/DataOutput readUTF/writeUTF methods.

### Example

This example uses File for the implementation of Read or Write. You can do this with any impl of
these traits. `Vec<u8>`/`Cursor`/...
```rust
use std::io;
use rw_utils::num_read::NumRead;
use rw_utils::num_write::NumWrite;

fn main() -> io::Result<()> {
  let mut file = File::create("/tmp/somefile")?;
  //Write u16 in little endian
  file.write_u16_le(0x4418u16)?;
  //Write u16 in big endian
  file.write_u16_be(0x4418u16)?;
  drop(file);
  
  let mut file = File::open("/tmp/somefile")?;
  //Read u16 in little endian
  assert_eq!(file.read_u16_le()?, 0x4418u16);
  //Read u16 in big endian
  assert_eq!(file.read_u16_be()?, 0x4418u16);

  return Ok(());
}
```

### Cargo features

To reduce "size" of the binary when using this library the default
feature is empty. In order for this library to do anything you need to
add at least 1 out of the following list of features:
* "num_read"
* "num_write"
* "string_read"
* "string_write"
* "leb128_read"
* "leb128_write"
* "to_write"
* "from_read"

If you want all features you can add the "all" feature.
#### Cargo.toml:

```toml
[dependencies]
rw-utils = {version = "*", features = ["all"]}
```
