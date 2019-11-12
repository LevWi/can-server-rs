use num_traits::*;

struct Field<T> {
    value: T,
    start_bit_pos: u16,
    bit_size : u16 
}


impl <T : FromPrimitive> Field <T> {
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


fn main() {
   let mut buf = [0u8; 7];
   buf[6] = 0x80;

   let mut val = Field::<u16> { value: 0, start_bit_pos: 52, bit_size : 4 };
   if let Err(v) = val.set_from(&buf[..]) {
       eprintln!("{}", v);
       std::process::exit(1);
   }
   
   println!("Value = {}", val.value);
}

