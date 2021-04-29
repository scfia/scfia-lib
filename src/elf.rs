use std::collections::HashMap;
use std::fs;

use xmas_elf::program::ProgramHeader::Ph64;
use xmas_elf::{program, ElfFile};

use crate::memory::stable_memory_region::StableMemoryRegion;
use crate::memory::ScfiaMemory;
use crate::values::value8::Value8;

pub fn load_elf(path: &str, memory: &mut ScfiaMemory) {
    let binary_blob = fs::read(path).expect("Can't read binary");
    let elf = ElfFile::new(&binary_blob).unwrap();
    for program_header in elf.program_iter() {
        if let Ph64(ph64) = program_header {
            match program_header.get_type().unwrap() {
                program::Type::Load => {
                    let mut i = 0;
                    let mut data = HashMap::new();
                    for b in ph64.raw_data(&elf) {
                        data.insert(ph64.virtual_addr + i, Value8::Concrete(*b));
                        i += 1;
                    }
                    println!(
                        "MemoryRegionType::Stable {:016x}-{:016x}",
                        ph64.virtual_addr,
                        ph64.virtual_addr + i
                    );
                    memory.stable_regions.push(StableMemoryRegion {
                        data: data,
                        range: ph64.virtual_addr..ph64.virtual_addr + i,
                    });
                }
                x => println!("[WARNING] ignoring section type {:?}", x),
            }
        }
    }
}
