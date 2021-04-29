use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, LineWriter},
    ops::Range,
    path::PathBuf,
};

use log::{debug, info, trace};

use crate::{
    symbols::scfia_symbol_manager::ScfiaSymbolManager,
    system_states::aarch64::ScfiaAarch64SystemState,
    transformation_functions::transformation_function_expression::ScfiaTransformationFunctionExpression,
    values::value8::Value8,
};

use self::{
    stable_memory_region::StableMemoryRegion,
    symbolic_volatile_memory_region::SymbolicVolatileMemoryRegion,
    volatile_memory_region::VolatileMemoryRegion,
};
use std::io::Write;

pub mod stable_memory_region;
pub mod symbolic_volatile_memory_region;
pub mod volatile_memory_region;

#[derive(Clone, Debug)]
pub struct ScfiaMemory {
    pub stable_regions: Vec<StableMemoryRegion>,
    pub volatile_regions: Vec<VolatileMemoryRegion>,
    pub symbolic_volatile_regions: Vec<SymbolicVolatileMemoryRegion>,
}

impl ScfiaMemory {
    pub fn new() -> Self {
        ScfiaMemory {
            stable_regions: vec![],
            volatile_regions: vec![],
            symbolic_volatile_regions: vec![],
        }
    }

    pub fn add_volatile_region(&mut self, range: Range<u64>) {
        self.volatile_regions
            .push(VolatileMemoryRegion { range: range })
    }

    pub fn add_stable_region(&mut self, data: HashMap<u64, Value8>, range: Range<u64>) {
        self.stable_regions.push(StableMemoryRegion {
            data: data,
            range: range,
        })
    }

    pub fn write(&mut self, address: u64, value: Value8, decremented_symbols: &mut Vec<u64>) {
        for region in &mut self.stable_regions {
            if region.range.contains(&address) {
                match region.data.insert(address, value).unwrap() {
                    Value8::Concrete(_) => {}
                    Value8::Symbolic(s) => {
                        decremented_symbols.push(s);
                    }
                }
                return;
            }
        }

        for region in &mut self.volatile_regions {
            if region.range.contains(&address) {
                return;
            }
        }

        panic!()
    }

    /// Replaces a concrete pointer to a volatile memory region with a pointer to a symbolic volatile memory region.
    pub fn generalize_volatile_pointer_to_symbolic_volatile_pointer(
        &mut self,
        volatile_pointer_address: u64,
        volatile_region_len: u32,
        symbol_manager: &mut ScfiaSymbolManager,
    ) {
        let volatile_pointer = self.read_u64_le_concrete(volatile_pointer_address);
        trace!(
            "generalize_volatile_pointer_to_symbolic_volatile_pointer overwriting {} at {}",
            volatile_pointer, volatile_pointer_address
        );
        assert!(self.is_in_volatile_region(volatile_pointer, volatile_region_len));

        let symbolic_volatile_region = if let Some(region) = self.get_symbolic_volatile_memory_region(volatile_region_len) {
            region.clone()
        } else {
            let symbolic_volatile_region =
                SymbolicVolatileMemoryRegion::new(volatile_region_len, symbol_manager);
            self.symbolic_volatile_regions
                .push(symbolic_volatile_region);
            self.symbolic_volatile_regions
                .get(self.symbolic_volatile_regions.len() - 1)
                .unwrap()
                .clone()
        };

        let mut decremented_symbols = vec![];
        for i in 0..8 {
            let symbol = symbolic_volatile_region.little_endian_base_symbols[i as usize].clone();
            self.write(
                volatile_pointer_address + i,
                Value8::Symbolic(symbol),
                &mut decremented_symbols,
            );
        }
        //TODO decrement symbols
    }

    fn get_symbolic_volatile_memory_region(&self, length: u32) -> Option<&SymbolicVolatileMemoryRegion> {
        for region in &self.symbolic_volatile_regions {
            if region.length >= length {
                return Some(region)
            }
        }
        None
    }

    fn is_in_volatile_region(&self, address: u64, len: u32) -> bool {
        for volatile_region in &self.volatile_regions {
            if volatile_region.range.start <= address
                && address + len as u64 <= volatile_region.range.end
            {
                //TODO check overflow
                return true;
            }
        }
        false
    }

    pub fn read_u64_le_concrete(&self, address: u64) -> u64 {
        let mut val: u64 = 0;
        for i in 0..8 {
            for stable_region in &self.stable_regions {
                if stable_region.range.contains(&(address + i)) {
                    let v = stable_region
                        .data
                        .get(&(address + i))
                        .unwrap()
                        .as_concrete();
                    val |= (v as u64) << (i * 8);
                    break;
                }
            }
        }
        val
    }

    pub fn read_stable(&self, address: u64) -> Option<Value8> {
        for region in &self.stable_regions {
            if region.range.contains(&address) {
                return Some(region.data.get(&address).unwrap().clone());
            }
        }
        panic!()
    }

    pub fn serialize(&self, folder: &PathBuf) {
        let mut i = 0;
        for region in &self.stable_regions {
            let file = File::create(folder.join(format!(
                "stable_memory_region_{}_{:x}_{:x}",
                i, region.range.start, region.range.end
            )))
            .unwrap();
            let mut file = LineWriter::new(file);
            for i in region.range.start..region.range.end {
                match region.data.get(&i).unwrap() {
                    Value8::Concrete(v) => {
                        file.write(format!("{:x}={:x}\n", i, v).as_bytes()).unwrap();
                    }
                    Value8::Symbolic(_symbol) => {}
                }
            }
            i += 1;
        }
        for region in &self.volatile_regions {
            File::create(folder.join(format!(
                "volatile_memory_region_{}_{:x}_{:x}",
                i, region.range.start, region.range.end
            )))
            .unwrap();
            i += 1;
        }
    }

    pub fn deserialize(&mut self, folder: &PathBuf, symbol_manager: &mut ScfiaSymbolManager) {
        // TODO! volatile regions
        let candidates = fs::read_dir(folder).unwrap();
        for candidate in candidates {
            let candidate = candidate.unwrap();
            let filename = candidate.file_name().to_str().unwrap().to_owned();
            if filename.starts_with("stable_memory_region_") {
                let parts = filename.split('_').collect::<Vec<&str>>();
                let begin = u64::from_str_radix(parts[4], 16).unwrap();
                let end = u64::from_str_radix(parts[5], 16).unwrap();
                let mut data = HashMap::new();

                let mut i = begin;
                info!("loading {}", filename);
                let lines = BufReader::new(File::open(candidate.path()).unwrap()).lines();
                for line in lines {
                    let line = line.unwrap();
                    let parts = line.split('=').collect::<Vec<&str>>();
                    let address = u64::from_str_radix(parts[0], 16).unwrap();
                    let value = u8::from_str_radix(parts[1], 16).unwrap();
                    while i < address {
                        data.insert(i, Value8::Symbolic(symbol_manager.create_symbol_bv(8)));
                        i += 1;
                    }
                    data.insert(i, Value8::Concrete(value));
                    i += 1;
                }
                while i < end {
                    data.insert(i, Value8::Symbolic(symbol_manager.create_symbol_bv(8)));
                    i += 1;
                }
                self.add_stable_region(data, begin..end);
            } else if filename.starts_with("volatile_memory_region_") {
                let parts = filename.split('_').collect::<Vec<&str>>();
                let begin = u64::from_str_radix(parts[4], 16).unwrap();
                let end = u64::from_str_radix(parts[5], 16).unwrap();
                self.add_volatile_region(begin..end);
            }
        }
    }
}
