use num_traits::*;

union Shifter {
    u : u16,
    arr : [u8; 2]
}

struct Field<T> {
    value: T,
    start_bit_pos: u16,
    bit_size : u16,
}


impl <T> Field <T> {
    fn new(value : T, start_bit_pos: u16, bit_size : u16 ) -> Option<Field<T>> {
        if std::mem::size_of::<u64>() >= std::mem::size_of::<T>() as usize {
            Some(
                Field { value,
                        start_bit_pos,
                        bit_size
                }
            )
        } else {
            None
        }
    }

    fn set_from(&mut self, buf : &[u8]) -> Result<(), &str> {
        let mut byte_needed = ((self.bit_size + self.start_bit_pos) / 8) as usize;

        if (self.bit_size + self.start_bit_pos) % 8 > 0 {
            byte_needed += 1;
        };

        if buf.len() < byte_needed {
            Err("Unexpected len")
        } else {
            let mut ptr = &mut self.value as *mut T as *mut u8;

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
/*
impl <T : FromPrimitive> Field <T> {
    fn new(value : T, start_bit_pos: u16, bit_size : u16 ) -> Option<Field<T>> {
        if std::mem::size_of::<u64>() >= std::mem::size_of::<T>() as usize {
            Some(
                Field { value,
                        start_bit_pos,
                        bit_size
                }
            )
        } else {
            None
        }
    }

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
                Some(v) => self.value = v
            }
            Ok(())
        }
    }    
}
*/

fn main() {
   let mut buf = [0u8; 5];
   buf[3] = 0x80;
   buf[4] = 0x40;

   let mut val = Field::new(0u16, 31, 8).unwrap();
   if let Err(v) = val.set_from(&buf[..]) {
       eprintln!("{}", v);
       std::process::exit(1);
   }
   
   println!("Value = {}", val.value);
}

