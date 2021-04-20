use std::ops::{Add, IndexMut};

const SIZE_1K: usize = 1024;
const SIZE_2K: usize = 2048;
const SIZE_4K: usize = 4096;
const SIZE_8K: usize = 8192;
const SIZE_16K: usize = 16384;
const SIZE_32K: usize = 32768;
const SIZE_64K: usize = 65536;
const SIZE_128K: usize = 131072;
const SIZE_256K: usize = 262144;

const PAGE_SIZE: usize = SIZE_1K;

const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }
 
const fn log_2(x: usize) -> usize {
    (num_bits::<usize>() as u32 - x.leading_zeros() - 1) as usize
}

const fn get_bank_index(address: usize, bank_size: usize) -> usize {
    let index_shift = log_2(bank_size);
    (address & !(bank_size - 1)) >> index_shift
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Bank {
    pub offset_mask: usize,
    pub index_mask: usize,
}

impl Bank {
    pub fn new(bank_size: usize, bank_index: usize) -> Bank {
        Bank {
            offset_mask: bank_size - 1,
            index_mask: bank_index << log_2(bank_size),
        }
    }
}

// paging system to map address space to memory
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct AddressMapper<const SIZE_IN_KBS: usize> {
    page_table: [Bank; SIZE_IN_KBS],
}

impl<const SIZE_IN_KBS: usize> AddressMapper<SIZE_IN_KBS> {
    pub fn new() -> AddressMapper<SIZE_IN_KBS> {
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

    pub fn translate_address(&self, address: u16) -> u16 {
        let page_index = get_bank_index( address as usize, PAGE_SIZE);
        let bank = &self.page_table[page_index];
        ((address as usize & bank.offset_mask) | bank.index_mask) as u16
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
        let mut addr_mapper = AddressMapper::<4>::new();
        //println!(" ##### translated address: {} address: {} #####", addr_mapper.translate_address(400), 400);
        assert_eq!(addr_mapper.translate_address(400), 400);
        assert_eq!(addr_mapper.translate_address(1024), 1024);
    }
}
