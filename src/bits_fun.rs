use num_traits::*;

trait FromByteArr {
    fn set_from(&mut self, buf : &[u8]) -> Result<(), &str>;
}

union Shifter {
    u : u16,
    arr : [u8; 2]
}

struct Field<T> {
    value: T,
    start_bit_pos: u16,
    bit_size : u16,
}

struct RawValue<T>(T);

impl <T> FromByteArr for Field<RawValue<T>> {

    fn set_from(&mut self, buf : &[u8]) -> Result<(), &str> {
        let mut byte_needed = ((self.bit_size + self.start_bit_pos) / 8) as usize;
        if (self.bit_size + self.start_bit_pos) % 8 > 0 {
            byte_needed += 1;
        };

        if buf.len() < byte_needed {
            Err("Unexpected len")
        } else {
            let mut ptr = &mut self.value.0 as *mut T as *mut u8;

            let buf_offset = (self.start_bit_pos / 8) as usize;
            let bit_offset = (self.start_bit_pos % 8) as usize;
            let upper_bound = byte_needed - buf_offset;
            unsafe {
            for ofs in 0 .. upper_bound {
                let mut out = Shifter { u : 0 };
                if ofs != upper_bound - 1 { 
                    out.arr[1] = buf[buf_offset + ofs + 1];
                    out.u >>= bit_offset;
                }
                out.arr[0] |= buf[buf_offset + ofs] >> bit_offset;
                *ptr = out.arr[0];
                ptr = ptr.offset(1);
            }
            *ptr.offset(-1) &= !(std::u8::MAX << (self.bit_size % 8));
            }
            Ok(())
        }
    }    
}

trait __PrimitiveLimit {}

macro_rules! impl_primitive_limit {
    ($($T:ident),*) => {
        $(impl __PrimitiveLimit for $T {}
        )*
    };
}

impl_primitive_limit!(u8, u16, u32, u64, 
                      i8, i16, i32, i64, 
                      f32, f64);

struct PrimValue<T>(T);

// Max value size = 64bit
impl <T : FromPrimitive + __PrimitiveLimit> FromByteArr for Field<PrimValue<T>> {

    fn set_from(&mut self, buf : &[u8]) -> Result<(), &str> {
        let mut byte_needed = (self.bit_size + self.start_bit_pos) / 8;

        if (self.bit_size + self.start_bit_pos) % 8 > 0 {
            byte_needed += 1;
        };

        if buf.len() < byte_needed as usize {
            Err("Unexpected len")
        } else {
            let mut out = 0u64;
            let ptr  = &mut out as *mut u64 as *mut u8;

            unsafe {
                let buff_ptr = buf.as_ptr().offset((self.start_bit_pos / 8) as isize);
                for ofs in 0..std::mem::size_of::<T>() {
                    *ptr.offset(ofs as isize) = *buff_ptr.offset(ofs as isize);
                }
            }
            
            out >>= self.start_bit_pos % 8;
            out &= !(std::u64::MAX << self.bit_size);
            
            match T::from_u64(out) {
                None => return Err("Wrong type"),
                Some(v) => self.value.0 = v
            }
            Ok(())
        }
    }    
}


trait BitsGet {
    fn get_bit(&self, bit_pos: usize) -> Option<bool>;
}

impl BitsGet for [u8] {
    fn get_bit(&self, bit_pos: usize) -> Option<bool> {
        let byte_pos = bit_pos / 8;
    
        if byte_pos + 1 > self.len() {
            None
        } else {
            let mut out = self[byte_pos] >> bit_pos % 8;
            out &= 0b01;
            Some(out != 0)
        }
    }
}


#[test] 
fn check_get_bit_func() {
    let mut arr = [0x00u8; 8];
    arr[0] = 0x10;
    arr[1] = 0x20;
    arr[2] = 0x10;
    arr[7] = 0x80;

    assert_eq!( arr.get_bit(4), Some(true) );
    assert_eq!( arr.get_bit(8), Some(false) );
    assert_eq!( arr.get_bit(5), Some(false) );
    assert_eq!( arr.get_bit(13), Some(true) );
    assert_eq!( arr.get_bit(20), Some(true) );
    assert_eq!( arr.get_bit(63), Some(true) );
    assert_eq!( arr.get_bit(64), None);
    arr[7] = 0;
    assert_eq!( arr.get_bit(63), Some(false) );
    assert_eq!( arr.get_bit(64), None);
}

#[test]
fn check_bytes_fields() {
    let mut buf = [0u8; 5];
    buf[3] = 0x80;
    buf[4] = 0x40;
 
    let mut val = Field{  value : PrimValue(0u128), start_bit_pos: 31, bit_size : 8 };
    //TODO
}
