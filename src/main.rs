
extern crate strum;
extern crate strum_macros;

mod rom;
mod disassembly;

use std::path::Path;
use crate::disassembly::Disassembly;
use std::fs::{File, FileType};
use std::io::Write;


fn main() {
    //save_disassembly(disassemble_ipl3_headerless("/data/storage/roms/n64-nointro/Star Fox 64 (USA).z64"), "/data/storage/roms/n64-nointro/Star Fox 64 (USA).z64.IPL3.disasm");
    //save_disassembly(disassemble_ipl3_headerless("/data/storage/roms/n64-nointro/Lylat Wars (Europe) (En,Fr,De).z64"), "/data/storage/roms/n64-nointro/Lylat Wars (Europe) (En,Fr,De).z64.IPL3.disasm");
    //save_disassembly(disassemble_pifrom("/data/storage/preservation/pifdata.bin"), "/data/storage/preservation/pifdata.bin.disasm");
    save_disassembly(disassemble_ipl3_headerless("/data/storage/roms/n64-nointro/Conker's Bad Fur Day (USA).z64"), "/data/storage/roms/n64-nointro/Conker's Bad Fur Day (USA).z64.IPL3.disasm");
    
    /*let mut count = 0;
    let mut disasms = Vec::new();
    let dirs = std::fs::read_dir("/data/storage/roms/n64-nointro/").unwrap().collect::<Result<Vec<_>, std::io::Error>>().unwrap();
    let dirs_len = dirs.len();
    for entry in dirs {
        let file_type = entry.file_type();
        let file_name_os = entry.file_name();
        let file_name = String::from(file_name_os.to_str().unwrap());
        
        if file_type.is_ok() && file_type.unwrap().is_file() && file_name.ends_with(".z64") {
            disasms.push((disassemble_ipl3_headerless(entry.path().to_str().unwrap()), file_name));
        }
        
        count += 1;
        if count % 16 == 0 {
            println!("Checked: {} of {}  ({:02.0}%)", count, dirs_len, (count as f32) / (dirs_len as f32) * 100f32);
        }
    }
    
    let mut kinds: Vec<(Disassembly, Vec<String>)> = Vec::new();
    for disasm in &disasms {
        let mut exists = false;
        for kind in &mut kinds {
            exists = kind.0.eq(&disasm.0);
            
            if exists {
                kind.1.push(disasm.1.clone());
                break;
            }
        }
        
        if !exists {
            let mut vec = Vec::new();
            vec.push(disasm.1.clone());
            kinds.push((disasm.0.clone(), vec));
        }
    }
    
    for (i, kind) in kinds.iter().enumerate() {
        if kind.1.len() > 1 {
            println!("{}: {}", i, kind.1.len());
            for path in &kind.1 {
                println!("{}", path);
            }
        }
    }*/
}

fn disassemble_ipl3_headerless(path: &str) -> Disassembly {
    let bytes = std::fs::read(Path::new(path)).unwrap();
    
    Disassembly::from_u8(&Vec::from(bytes.split_at(0x1000).0.split_at(0x40).1))
}

fn disassemble_ipl3_withhead(path: &str) -> Disassembly {
    let bytes = std::fs::read(Path::new(path)).unwrap();
    
    Disassembly::from_u8(&Vec::from(bytes.split_at(0x1000).0))
}

fn disassemble_pifrom(path: &str) -> Disassembly {
    let bytes = std::fs::read(Path::new(path)).unwrap();
    
    Disassembly::from_u8(&bytes)
}

fn save_disassembly(disasm: Disassembly, path: &str) {
    let mut out = File::create(path).unwrap();
    for (i, instr) in disasm.instructions.iter().enumerate() {
        out.write_all(format!("[{:#010X}]{}\n", (i * 4) + 0x40, instr).as_bytes()).unwrap();
    }
}