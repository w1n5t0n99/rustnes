
pub const SIZE_1K: usize = 1024;
pub const SIZE_2K: usize = 2048;
pub const SIZE_4K: usize = 4096;
pub const SIZE_8K: usize = 8192;
pub const SIZE_16K: usize = 16384;
pub const SIZE_32K: usize = 32768;
pub const SIZE_64K: usize = 65536;
pub const SIZE_128K: usize = 131072;
pub const SIZE_256K: usize = 262144;
const PAGE_SIZE: usize = SIZE_1K;

const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }
 
const fn log_2(x: usize) -> usize {
    (num_bits::<usize>() as u32 - x.leading_zeros() - 1) as usize
}

const fn get_bank_index(address: usize, bank_size: usize) -> usize {
    let index_shift = log_2(bank_size);
    (address & !(bank_size - 1)) >> index_shift
}

const fn get_last_bank_index(bank_size: usize, data_size: usize) -> usize {
    (data_size / bank_size) - 1
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Bank {
    pub offset_mask: usize,
    pub index_mask: usize,
    pub size: usize,
    pub index: usize,
}

impl Bank {
    pub fn new(bank_size: usize, bank_index: usize) -> Bank {
        Bank {
            offset_mask: bank_size - 1,
            index_mask: bank_index << log_2(bank_size),
            size: bank_size,
            index: bank_index,
        }
    }
}

// paging system to map address space to memory
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct AddressMapper<const SIZE_IN_KBS: usize, const START_ADDRESS: usize> {
    page_table: [Bank; SIZE_IN_KBS],
}

impl<const SIZE_IN_KBS: usize, const START_ADDRESS: usize> AddressMapper<SIZE_IN_KBS, START_ADDRESS> {
    pub fn new() -> AddressMapper<SIZE_IN_KBS, START_ADDRESS> {
        let mut am = AddressMapper { page_table: [Bank::new(PAGE_SIZE, 0); SIZE_IN_KBS], };
        for (i, bank) in am.page_table.iter_mut().enumerate() {
            *bank = Bank::new(PAGE_SIZE, i);
        }

        am
    }

    pub fn clear(&mut self) {
        for (i, bank) in self.page_table.iter_mut().enumerate() {
            *bank = Bank::new(PAGE_SIZE, i);
        }
    }

    pub fn set_banking_region(&mut self, addr_bank_index: usize, mem_bank_index: usize, bank_size: usize) {
        let pages_per_bank = bank_size / PAGE_SIZE;
        let start_page = addr_bank_index * pages_per_bank;

        for b in start_page..(start_page+pages_per_bank) {
            self.page_table[b] = Bank::new(bank_size, mem_bank_index);
        }
    }

    pub fn set_banking_region_to_last_bank(&mut self, addr_bank_index: usize, bank_size: usize, data_size: usize) {
        let pages_per_bank = bank_size / PAGE_SIZE;
        let start_page = addr_bank_index * pages_per_bank;
        let mem_bank_index = get_last_bank_index(bank_size, data_size);

        for b in start_page..(start_page+pages_per_bank) {
            self.page_table[b] = Bank::new(bank_size, mem_bank_index);
        }
    }

    pub fn translate_address(&self, address: u16) -> u16 {
        let page_index = get_bank_index( (address as usize) - START_ADDRESS, PAGE_SIZE);
        let bank = &self.page_table[page_index];
        ((address as usize & bank.offset_mask) | bank.index_mask) as u16
    }

    pub fn address_bank_details(self, address: u16) -> (usize, usize) {
        let page_index = get_bank_index( (address as usize) - START_ADDRESS, PAGE_SIZE);
        let bank = &self.page_table[page_index];
        (bank.size, bank.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_index_lookup() {
        assert_eq!(get_bank_index(5030, SIZE_1K), 4);
        assert_eq!(get_bank_index(800, SIZE_1K), 0);
    }

    #[test]
    fn test_address_passthrough() {
        let addr_mapper = AddressMapper::<4, 0>::new();
        //println!(" ##### translated address: {} address: {} #####", addr_mapper.translate_address(400), 400);
        assert_eq!(addr_mapper.translate_address(400), 400);
        assert_eq!(addr_mapper.translate_address(1024), 1024);
    }

    #[test]
    fn test_address_mirroring() {
        let mut addr_mapper = AddressMapper::<32, 0>::new();
        let mut memory: [u8; SIZE_8K] = [0; SIZE_8K];

        addr_mapper.set_banking_region(0, 0, SIZE_8K);
        addr_mapper.set_banking_region(1, 0, SIZE_8K);
        addr_mapper.set_banking_region(2, 0, SIZE_8K);
        addr_mapper.set_banking_region(3, 0, SIZE_8K);

        memory[0] = 99;

        let a0 = addr_mapper.translate_address(0);
        let a1 = addr_mapper.translate_address(8192);

        assert_eq!(memory[a0 as usize], memory[a1 as usize]);
    }

    #[test]
    fn test_multi_sized_banks() {
        let mut addr_mapper = AddressMapper::<32, 0>::new();
        let mut memory: [u8; SIZE_16K] = [0; SIZE_16K];

        // 1 - 8k bank and 2 - 4 kb banks
        addr_mapper.set_banking_region(0, 0, SIZE_8K);
        addr_mapper.set_banking_region(2, 3, SIZE_4K);
        addr_mapper.set_banking_region(3, 3, SIZE_4K);

        memory[0] = 99;
        memory[12288] = 100;

        let a1 = addr_mapper.translate_address(8192);
        let a2 = addr_mapper.translate_address(12288);
        let a3 = addr_mapper.translate_address(4096);
        let a4 = addr_mapper.translate_address(16384);

        assert_eq!(memory[a1 as usize], memory[a2 as usize]);
        assert_ne!(memory[a3 as usize], memory[a2 as usize]);
        // we havent set bankng for this region yet so it should just passthrough address
        assert_eq!(16384, a4);
    }

    #[test]
    fn test_starting_address() {
        // example NES prg rom region
        let mut addr_mapper = AddressMapper::<32, 0x8000>::new();
        let mut memory: [u8; SIZE_16K] = [0; SIZE_16K];

        memory[1] = 99;

        addr_mapper.set_banking_region(0, 0, SIZE_16K);
        addr_mapper.set_banking_region(1, 0, SIZE_16K);

        let a1 = addr_mapper.translate_address(0x8001);
        let a2 = addr_mapper.translate_address(0x8001 + (SIZE_16K as u16));
        assert_eq!(memory[a1 as usize], memory[a2 as usize]);
    }
}
