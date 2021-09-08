
extern crate strum;
extern crate strum_macros;

mod rom;
mod disassembly;

use std::path::{Path, PathBuf};
use crate::disassembly::Disassembly;
use std::fs::{File, FileType};
use std::io::Write;
use crate::rom::{Header, Rom};


fn main() {
    //save_disassembly(disassemble_ipl3_headerless("/data/storage/roms/n64-nointro/Star Fox 64 (USA).z64"), "/data/storage/roms/n64-nointro/Star Fox 64 (USA).z64.IPL3.disasm");
    //save_disassembly(disassemble_ipl3_headerless("/data/storage/roms/n64-nointro/Lylat Wars (Europe) (En,Fr,De).z64"), "/data/storage/roms/n64-nointro/Lylat Wars (Europe) (En,Fr,De).z64.IPL3.disasm");
    //save_disassembly(disassemble_pifrom("/data/storage/preservation/pifdata.bin"), "/data/storage/preservation/pifdata.bin.disasm");
    //save_disassembly(disassemble_ipl3_headerless("/data/storage/roms/n64-nointro/Conker's Bad Fur Day (USA).z64"), "/data/storage/roms/n64-nointro/Conker's Bad Fur Day (USA).z64.IPL3.disasm");
    save_disassembly(disassemble_pifrom("/data/storage/roms/n64-nointro/Namco Museum 64 (USA).z64"), "/data/storage/roms/n64-nointro/Namco Museum 64 (USA).z64.disasm");
    
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
    
    //dump_headers("/data/storage/roms/n64-nointro");
}

fn dump_headers(path_str: &str) {
    let path = Path::new(path_str);
    
    let mut valid_counter = 0;
    let mut header_counter = 0;
    let mut save_counter = 0;
    
    let mut out_path = String::new();
    let mut valid_paths: Vec<PathBuf> = Vec::new();
    let mut headers: Vec<Header> = Vec::new();
    if path.is_dir() {
        let dir = path.read_dir().unwrap();
        
        for wrapped_entry in dir {
            let entry = wrapped_entry.unwrap();
            let path = entry.path();
            let extension = path.extension();
            
            if !path.is_dir() && extension.is_some() && extension.unwrap().eq("z64") {
                valid_paths.push(path);
                valid_counter += 1;
            }
        }
        
        out_path.push_str(&[path_str, "/output.csv"].concat());
    } else {
        valid_paths.push(path.to_path_buf());
        
        out_path.push_str(&[path_str, ".output.csv"].concat());
        
        valid_counter += 1;
    }
    
    println!("Valid Paths: {}", valid_counter);
    
    for path in &valid_paths {
        let read_result = std::fs::read(path);
        if read_result.is_ok() {
            let rom = Rom::new(read_result.unwrap());
            
            headers.push(rom.header);
            
            header_counter += 1;
            println!("Headers Extracted: {} of {}", header_counter, valid_counter);
        }
    }
    
    fn u8arr_str(val: &[u8]) -> String { 
        let result = String::from_utf8(val.to_vec());
        
        if result.is_ok() {
            return result.unwrap();
        }
        
        let mut out = String::from("0x");
        for b in val {
            out.push_str(&format!("{:02X}", b));
        }
        
        out
    }
    fn u32_str(val: u32) -> String {
        let result = String::from_utf8(val.to_be_bytes().to_vec());
        
        if result.is_ok() {
            return result.unwrap();
        }
        
        format!("{:#010X}", val)
    }
    fn u16_str(val: u16) -> String {
        let result = String::from_utf8(val.to_be_bytes().to_vec());
        
        if result.is_ok() {
            return result.unwrap();
        }
        
        format!("{:#06X}", val)
    }
    
    let mut csv = String::from("file_name;pi_regs;clockrate;pc;release;crc1;crc2;unknown0;image_name;unknown1;manufacturer_id;cart_id;country\n");
    for header in headers {
        let file_name = valid_paths[save_counter].file_name().unwrap().to_os_string();
        
        csv.push_str(&format!("{};{:#010X};{:#010X};{:#010X};{:#010X};{:#010X};{:#010X};{:#018X};{};{:#010X};{};{};{}\n",
            file_name.to_str().unwrap(),
            header.pi_regs,
            header.clockrate,
            header.pc,
            header.release,
            header.crc1,
            header.crc2,
            header.unknown0,
            u8arr_str(&header.image_name),
            header.unknown1,
            u32_str(header.manu_id),
            u16_str(header.cart_id),
            u16_str(header.country),
        ));
        
        save_counter += 1;
        println!("Headers Parsed: {} of {}", save_counter, valid_counter);
    }
    
    let mut out = File::create(out_path).unwrap();
    out.write_all(csv.as_bytes()).unwrap();
    println!("Complete!");
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
        out.write_all(format!("[{:#010X}]{}\n", (i * 4) + 0x00, instr).as_bytes()).unwrap();
    }
}