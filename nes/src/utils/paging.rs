
const SIZE_1K: usize = 1024;
const SIZE_2K: usize = 2048;
const SIZE_4K: usize = 4096;
const SIZE_8K: usize = 8192;
const SIZE_16K: usize = 16384;
const SIZE_32K: usize = 32768;
const SIZE_64K: usize = 65536;
const SIZE_128K: usize = 131072;
const SIZE_256K: usize = 262144;

const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }
 
const fn log_2(x: usize) -> usize {
    (num_bits::<usize>() as u32 - x.leading_zeros() - 1) as usize
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Frame {
    pub offset_mask: usize,
    pub window_mask: usize,
}

impl Frame {
    pub fn new(window_size: usize, window: usize) -> Frame {
        Frame {
            offset_mask: log_2(window_size) - 1,
            window_mask: window << log_2(window_size),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct PageBank {
    pub index: usize,
    pub size: usize,           // in 1Kb pages
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct PageTable<const BLOCK_SIZE: usize, const BLOCK_COUNT: usize> {
    ptbl: [Frame; BLOCK_COUNT],  
}

impl<const BLOCK_SIZE: usize, const BLOCK_COUNT: usize> PageTable<BLOCK_SIZE,BLOCK_COUNT> {
    pub fn new() -> PageTable<BLOCK_SIZE,BLOCK_COUNT> {
        PageTable {
            ptbl: [Frame::new(BLOCK_SIZE, 0); BLOCK_COUNT],
        }
    }
}