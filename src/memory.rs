use super::mips_parser::*;

const PAGE_SIZE: usize = 4000;
const NUM_PAGES: usize = 500;

type MemoryPage = Box<[u8; PAGE_SIZE]>;
type MemoryPageOpt = Option<MemoryPage>;
type MemoryRepList = [MemoryPageOpt; NUM_PAGES];

pub struct MemoryRep {
    memory: MemoryRepList,
}

#[derive(Debug)]
pub enum MemoryError {
    NULLAccess, 
    OverflowAccess,
    PageFault,
    InvalidMem
}

fn check_sane_index(index: usize) -> Result<(), MemoryError> {
    if index == 0 {
        return Err(MemoryError::NULLAccess)
    } else if index >= PAGE_SIZE {
        return Err(MemoryError::OverflowAccess)
    }
    Ok(())
}

impl MemoryRep {
    fn memory_field() -> MemoryRepList {
        unsafe {
            let mut arr: MemoryRepList = std::mem::uninitialized();
            for item in &mut arr[..] {
                std::ptr::write(item, None as MemoryPageOpt);
            }
            assert!(arr[20].is_none());
            arr
        }

    }

    fn addr_exists(&self, addr: usize) -> Result<(), MemoryError> {
        let index: usize = addr / PAGE_SIZE;
        let offset: usize = addr % PAGE_SIZE;
       
        check_sane_index(index)?;
        match &self.memory[index]{
            Some(_) => {
                // TODO: add memory checks
                return Ok(())
            }
            None => return Err(MemoryError::InvalidMem)
        }
    }

    fn init_page<'a>(&self) -> MemoryPage{
        Box::new([0b01100110; PAGE_SIZE])
    }

    fn get_page<'a>(&mut self, addr: usize) -> Result<&mut MemoryPage, MemoryError> {
        let index: usize = addr / PAGE_SIZE;

        check_sane_index(index)?;

        match &mut self.memory[index] {
            Some(memory_index) => {
                return Ok(memory_index);
            }
            None => {
                return Err(MemoryError::PageFault)
            }
        }
    }

    pub fn store_byte(&mut self, addr: usize, byte: u8) -> Result<(), MemoryError>{
        let index: usize = addr / PAGE_SIZE;
        let offset: usize = addr % PAGE_SIZE;
        match self.get_page(addr) {
            Ok(page) => {
                page[offset] = byte;
            }
            Err(MemoryError::PageFault) => {
                let mut page = self.init_page();
                page[offset] = byte;
                self.memory[index] = Some(page);
            }
            Err(_) => {

            }

        }
        Ok(())

    }
    
    pub fn store_word(&mut self, addr: usize, word: u32) -> Result<(), MemoryError>{
        let bytes: [u8; 4] = unsafe { std::mem::transmute(word.to_be()) };
        for i in 0..4 {
            self.store_byte(addr+i, bytes[i]);
        }
        Ok(())

    }
   
    pub fn read_byte(&mut self, addr: usize) -> Result<u8, MemoryError> {
        self.addr_exists(addr)?;

        Ok(self.get_page(addr)?[addr % PAGE_SIZE])

    }

    pub fn read_word(&mut self, addr: usize) -> Result<u32, MemoryError> {
        let mut return_word: u32 = 0;
        for i in 0..4 {
            return_word <<= 8;
            return_word |= self.read_byte(addr + i)? as u32;
        }
        Ok(return_word)
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_empty_memory_rep() -> MemoryRep {
        MemoryRep {
            memory: MemoryRep::memory_field()
        }
    }

    #[test]
    fn simple_read_write(){
        let mut memory = get_empty_memory_rep();
        let address = PAGE_SIZE + 100;
        memory.store_byte(address, 121).expect("Should not fail to store memory");
        match memory.read_byte(address) {
            Ok(byte) => assert_eq!(byte, 121),
            Err(_) => panic!("read_byte returned an error")
        }

    }

    #[test]
    fn simple_word_read_write(){
        let mut memory = get_empty_memory_rep();
        let address = PAGE_SIZE + 100;
        memory.store_word(address, 1234321).expect("Should not fail to store memory");
        match memory.read_word(address) {
            Ok(byte) => assert_eq!(byte, 1234321),
            Err(_) => panic!("read_byte returned an error")
        }
    }
    
    #[test]
    fn word_read_write(){
        let mut memory = get_empty_memory_rep();
        let address = PAGE_SIZE + 100;
        let address_2 = PAGE_SIZE*2 + 100;
        memory.store_word(address, 1234321).expect("Should not fail to store memory");
        memory.store_word(address_2, 1224321).expect("Should not fail to store memory");
        match memory.read_word(address_2) {
            Ok(byte) => assert_eq!(byte, 1224321),
            Err(_) => panic!("read_byte returned an error")
        }
    }
    
    #[test]
    fn word_read_null_fails(){
        let mut memory = get_empty_memory_rep();
        let address = 100;
        memory.store_word(address, 1234321);
        match memory.read_word(address) {
            Ok(byte) => panic!("read_byte should not return"),
            Err(MemoryError::NULLAccess) => (),
            Err(_) => panic!("read_byte returned an unexpected error")
        }
    }
}
