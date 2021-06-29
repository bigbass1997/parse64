use crate::rom::Rom;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use colored::*;

const CPU_REG_NAMES: [&'static str; 32] = [
    "zr", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
    "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp", "fp", "ra"
];

const CP0_REG_NAMES: [&'static str; 32] = [
    "Index", "Random", "EntryLo0", "EntryLo1", "Context", "PageMask", "Wired", "Unused7", "BadVAddr", "Count", "EntryHi", "Compare", "SR", "Cause", "EPC", "PRId",
    "Config", "LLAddr", "WatchLo", "WatchHi", "XContext", "Unused21", "Unused22", "Unused23", "Unused24", "Unused25", "PErr", "Unused27", "TagLo", "TagHi", "ErrorEPC", "Unused31"
];

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operand {
    Reg(u8),
    Cp0Reg(u8),
    Lit8(u8),
    Lit16(u16),
    Lit32(u32),
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Operand::*;
        
        match self {
            Reg(reg) => write!(f, "{}", CPU_REG_NAMES[*reg as usize]),
            Cp0Reg(reg) => write!(f, "{}", CP0_REG_NAMES[*reg as usize]),
            Lit8(val) => write!(f, "{:#04X}", val),
            Lit16(val) => write!(f, "{:#06X}", val),
            Lit32(val) => write!(f, "{:#010X}", val),
            _ => write!(f, "{:?}", self)
        }
        
    }
}

#[allow(dead_code)]
#[derive(PartialEq, strum_macros::ToString, Copy, Clone)]
pub enum Operation {
    ADD,
    ADDI,
    ADDIU,
    ADDU,
    AND,
    ANDI,
    
    BCzF,
    BCzFL,
    BCzT,
    BCzTL,
    
    BEQ,
    BEQL,
    
    BGEZ,
    BGEZAL,
    BGEZALL,
    BGEZL,
    
    BGTZ,
    BGTZL,
    
    BLEZ,
    BLEZL,
    
    BLTZ,
    BLTZAL,
    BLTZALL,
    BLTZL,
    
    BNE,
    BNEL,
    
    BREAK,
    CACHE,
    
    CFCz,
    COPz,
    CTCz,
    
    DADD,
    DADDI,
    DADDIU,
    DADDU,
    
    DDIV,
    DDIVU,
    DIV,
    DIVU,
    
    DMFC0,
    DMTC0,
    DMULT,
    DMULTU,
    
    DSLL,
    DSLLV,
    DSLL32,
    
    DSRA,
    DSRAV,
    DSRA32,
    DSRL,
    DSRLV,
    DSRL32,
    
    DSUB,
    DSUBU,
    
    ERET,
    J,
    JAL,
    JALR,
    JR,
    
    LB,
    LBU,
    LD,
    LDCz,
    LDL,
    LDR,
    LH,
    LHU,
    LL,
    LLD,
    LUI,
    LW,
    LWCz,
    LWL,
    LWR,
    LWU,
    
    MFC0,
    MFCz,
    MFHI,
    MFLO,
    MTC0,
    MTCz,
    MTHI,
    MTLO,
    
    MULT,
    MULTU,
    
    NOR,
    OR,
    ORI,
    SB,
    SC,
    SCD,
    SD,
    SDCz,
    SDL,
    SDR,
    SH,
    SLL,
    SLLV,
    SLT,
    SLTI,
    SLTIU,
    SLTU,
    SRA,
    SRAV,
    SRL,
    SRLV,
    
    SUB,
    SUBU,
    SW,
    SWCz,
    SWL,
    SWR,
    
    SYNC,
    SYSCALL,
    
    TEQ,
    TEQI,
    TGE,
    TGEI,
    TGEIU,
    TGEU,
    
    TLBP,
    TLBR,
    TLBWI,
    TLBWR,
    
    TLT,
    TLTI,
    TLTIU,
    TLTU,
    TNE,
    TNEI,
    
    XOR,
    XORI,
    
    NOP,
    Unknown,
}

#[derive(PartialEq)]
pub struct Instruction {
    pub code: u32,
    pub op: Operation,
    pub args: [Option<Operand>; 4],
}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        let mut args = [None, None, None, None];
        for i in 0..args.len() {
            if self.args[i].is_some() {
                args[i] = Some(self.args[i].unwrap());
            }
        }
        
        Instruction {
            code: self.code,
            op: self.op,
            args: args,
        }
    }
}

impl Instruction {
    pub fn new0(code: u32, op: Operation) -> Instruction {
        Instruction { code: code, op: op, args: [None,        None,        None,        None       ] }
    }
    
    pub fn new1(code: u32, op: Operation, oper0: Operand) -> Instruction {
        Instruction { code: code, op: op, args: [Some(oper0), None,        None,        None       ] }
    }
    
    pub fn new2(code: u32, op: Operation, oper0: Operand, oper1: Operand) -> Instruction {
        Instruction { code: code, op: op, args: [Some(oper0), Some(oper1), None,        None       ] }
    }
    
    pub fn new3(code: u32, op: Operation, oper0: Operand, oper1: Operand, oper2: Operand) -> Instruction {
        Instruction { code: code, op: op, args: [Some(oper0), Some(oper1), Some(oper2), None       ] }
    }
    
    pub fn new4(code: u32, op: Operation, oper0: Operand, oper1: Operand, oper2: Operand, oper3: Operand) -> Instruction {
        Instruction { code: code, op: op, args: [Some(oper0), Some(oper1), Some(oper2), Some(oper3)] }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut args = String::new();
        for arg in self.args.iter() {
            if arg.is_some() {
                if !args.is_empty() {
                    args.push_str(", ")
                }
                args.push_str(&arg.as_ref().unwrap().to_string());
            }
        }
        
        //write!(f, "[{:032b}] [{}] [{}]", self.code, self.op.to_string(), args)
        write!(f, "[{:#010X}][{} {}]", self.code, self.op.to_string(), args)
    }
}

#[derive(Clone)]
pub struct Disassembly {
    pub raw: Vec<u32>,
    pub instructions: Vec<Instruction>,
}

impl PartialEq for Disassembly {
    fn eq(&self, rhs: &Self) -> bool {
        self.raw.eq(&rhs.raw)
    }
}

impl Disassembly {
    pub fn from_u8(raw_u8: &Vec<u8>) -> Disassembly {
        let len = raw_u8.len() - (raw_u8.len() % 4);
        let mut raw = Vec::new();
        let mut instructions = Vec::new();
        
        for i in 0..(len / 4) {
            let slice = &raw_u8[(i * 4)..((i * 4) + 4)];
            
            raw.push(to_u32(slice));
            instructions.push(disassemble_u8(slice));
        }
        
        Disassembly {
            raw: raw,
            instructions: instructions,
        }
    }
    
    pub fn from_u32(raw_u32: &Vec<u32>) -> Disassembly {
        let len = raw_u32.len();
        let mut instructions = Vec::new();
        
        for i in 0..len {
            instructions.push(disassemble(raw_u32[i]));
        }
        
        Disassembly {
            raw: raw_u32.clone(),
            instructions: instructions,
        }
    }
    
    /*pub fn find_by_operation(&self, op: Operation, limit: usize, print: bool) -> Vec<(usize, &Instruction)>{
        let mut results = Vec::new();
        
        for (i, instr) in self.all_instructs.iter().enumerate() {
            if instr.op.eq(&op) {
                if print { println!("[{:#010X}] {}", (i * 4) + 0x40, instr); }
                
                results.push((i, instr));
                
                if results.len() == limit {
                    break;
                }
            }
        }
        
        results
    }
    
    pub fn find_by_operation_context(&self, op: Operation, mut range: Range<isize>, limit: usize, print: bool) -> Vec<Vec<(usize, &Instruction)>> {
        let findings = self.find_by_operation(op, limit, false);
        let mut results = Vec::new();
        
        for (i, _) in findings {
            let ii = i as isize;
            let mut context = Vec::new();
            
            if range.start + ii < 0 {
                range.start += -(range.start + ii);
            }
            if range.end + ii > self.all_instructs.len() as isize {
                range.end = -(range.end + ii);
            }
            
            for j in (ii + range.start)..(ii + range.end) {
                let instr = &self.all_instructs[j as usize];
                
                if print {
                    if i == (j as usize) {
                        println!("{}", format!("[{:#010X}] {}", (j * 4) + 0x40, *instr).green());
                    } else {
                        println!("[{:#010X}] {}", (j * 4) + 0x40, *instr);
                    }
                }
                
                context.push((j as usize, instr));
            }
            
            if print && !context.is_empty() {
                println!();
            }
            
            results.push(context);
        }
        
        results
    }*/
}

pub fn disassemble(code: u32) -> Instruction {
    use self::{Operand::*, Operation::*};
    
    let op = code >> 26;
    let special = code & 0x3F;
    let vrt = (code >> 16) & 0x1F;
    let cpz = op & 0x03;
    
    let rs = ((code >> 21) & 0x1F) as u8;
    let rt = ((code >> 16) & 0x1F) as u8;
    let rd = ((code >> 11) & 0x1F) as u8;
    let sa = ((code >> 6) & 0x1F) as u8;
    let base = rs;
    
    let lit16 = (code & 0xFFFF) as u16;
    
    match code {
        0 => Instruction::new0(code, NOP),
        _ => match op {
            0x00 => match special {
                0x00 => Instruction::new3(code, SLL, Reg(rd), Reg(rt), Lit8(sa)),
                0x02 => Instruction::new3(code, SRL, Reg(rd), Reg(rt), Lit8(sa)),
                0x03 => Instruction::new3(code, SRA, Reg(rd), Reg(rt), Lit8(sa)),
                0x04 => Instruction::new3(code, SLLV, Reg(rd), Reg(rt), Reg(rs)),
                0x06 => Instruction::new3(code, SRLV, Reg(rd), Reg(rt), Reg(rs)),
                0x07 => Instruction::new3(code, SRAV, Reg(rd), Reg(rt), Reg(rs)),
                0x08 => Instruction::new1(code, JR, Reg(rs)),
                0x09 => Instruction::new1(code, JALR, Reg(rs)),
                0x0C => Instruction::new0(code, SYSCALL),
                0x0D => Instruction::new0(code, BREAK),
                0x0F => Instruction::new0(code, SYNC),
                
                0x10 => Instruction::new1(code, MFHI, Reg(rd)),
                0x11 => Instruction::new1(code, MTHI, Reg(rs)),
                0x12 => Instruction::new1(code, MFLO, Reg(rd)),
                0x13 => Instruction::new1(code, MTLO, Reg(rs)),
                0x14 => Instruction::new3(code, DSLLV, Reg(rd), Reg(rt), Reg(rs)),
                0x16 => Instruction::new3(code, DSRLV, Reg(rd), Reg(rt), Reg(rs)),
                0x17 => Instruction::new3(code, DSRAV, Reg(rd), Reg(rt), Reg(rs)),
                0x18 => Instruction::new2(code, MULT, Reg(rs), Reg(rt)),
                0x19 => Instruction::new2(code, MULTU, Reg(rs), Reg(rt)),
                0x1A => Instruction::new2(code, DIV, Reg(rs), Reg(rt)),
                0x1B => Instruction::new2(code, DIVU, Reg(rs), Reg(rt)),
                0x1C => Instruction::new2(code, DMULT, Reg(rs), Reg(rt)),
                0x1D => Instruction::new2(code, DMULTU, Reg(rs), Reg(rt)),
                0x1E => Instruction::new2(code, DDIV, Reg(rs), Reg(rt)),
                0x1F => Instruction::new2(code, DDIVU, Reg(rs), Reg(rt)),
                
                0x20 => Instruction::new3(code, ADD, Reg(rd), Reg(rt), Reg(rs)),
                0x21 => Instruction::new3(code, ADDU, Reg(rd), Reg(rt), Reg(rs)),
                0x22 => Instruction::new3(code, SUB, Reg(rd), Reg(rt), Reg(rs)),
                0x23 => Instruction::new3(code, SUBU, Reg(rd), Reg(rt), Reg(rs)),
                0x24 => Instruction::new3(code, AND, Reg(rd), Reg(rt), Reg(rs)),
                0x25 => Instruction::new3(code, OR, Reg(rd), Reg(rt), Reg(rs)),
                0x26 => Instruction::new3(code, XOR, Reg(rd), Reg(rt), Reg(rs)),
                0x27 => Instruction::new3(code, NOR, Reg(rd), Reg(rt), Reg(rs)),
                0x2A => Instruction::new3(code, SLT, Reg(rd), Reg(rt), Reg(rs)),
                0x2B => Instruction::new3(code, SLTU, Reg(rd), Reg(rt), Reg(rs)),
                0x2C => Instruction::new3(code, DADD, Reg(rd), Reg(rt), Reg(rs)),
                0x2D => Instruction::new3(code, DADDU, Reg(rd), Reg(rt), Reg(rs)),
                0x2E => Instruction::new3(code, DSUB, Reg(rd), Reg(rt), Reg(rs)),
                0x2F => Instruction::new3(code, DSUBU, Reg(rd), Reg(rt), Reg(rs)),
                
                0x30 => Instruction::new2(code, TGE, Reg(rs), Reg(rt)), //
                0x31 => Instruction::new2(code, TGEU, Reg(rs), Reg(rt)), //
                0x32 => Instruction::new2(code, TLT, Reg(rs), Reg(rt)), //
                0x33 => Instruction::new2(code, TLTU, Reg(rs), Reg(rt)), //
                0x34 => Instruction::new2(code, TEQ, Reg(rs), Reg(rt)),
                0x36 => Instruction::new2(code, TNE, Reg(rs), Reg(rt)), //
                
                0x38 => Instruction::new3(code, DSLL, Reg(rd), Reg(rt), Reg(rs)),
                0x3A => Instruction::new3(code, DSRL, Reg(rd), Reg(rt), Reg(rs)),
                0x3B => Instruction::new3(code, DSRA, Reg(rd), Reg(rt), Reg(rs)),
                0x3C => Instruction::new3(code, DSLL32, Reg(rd), Reg(rt), Reg(rs)),
                0x3E => Instruction::new3(code, DSRL32, Reg(rd), Reg(rt), Reg(rs)),
                0x3F => Instruction::new3(code, DSRA32, Reg(rd), Reg(rt), Reg(rs)),
                
                _ => Instruction::new0(code, Unknown)
            },
            0x01 => match vrt {
                0x00 => Instruction::new2(code, BLTZ, Reg(rs), Lit16(lit16)),
                0x01 => Instruction::new2(code, BGEZ, Reg(rs), Lit16(lit16)),
                0x02 => Instruction::new2(code, BLTZL, Reg(rs), Lit16(lit16)),
                0x03 => Instruction::new2(code, BGEZL, Reg(rs), Lit16(lit16)),
                
                0x08 => Instruction::new2(code, TGEI, Reg(rs), Lit16(lit16)), //
                0x09 => Instruction::new2(code, TGEIU, Reg(rs), Lit16(lit16)), //
                0x0A => Instruction::new2(code, TLTI, Reg(rs), Lit16(lit16)), //
                0x0B => Instruction::new2(code, TLTIU, Reg(rs), Lit16(lit16)), //
                0x0C => Instruction::new2(code, TEQI, Reg(rs), Lit16(lit16)), //
                0x0E => Instruction::new2(code, TNEI, Reg(rs), Lit16(lit16)), //
                
                0x10 => Instruction::new2(code, BLTZAL, Reg(rs), Lit16(lit16)),
                0x11 => Instruction::new2(code, BGEZAL, Reg(rs), Lit16(lit16)),
                0x12 => Instruction::new2(code, BLTZALL, Reg(rs), Lit16(lit16)),
                0x13 => Instruction::new2(code, BGEZALL, Reg(rs), Lit16(lit16)),
                
                _ => Instruction::new0(code, Unknown)
            },
            0x02 => Instruction::new1(code, J, Lit16(lit16)),
            0x03 => Instruction::new1(code, JAL, Lit16(lit16)),
            0x04 => Instruction::new3(code, BEQ, Reg(rs), Reg(rt), Lit16(lit16)),
            0x05 => Instruction::new3(code, BNE, Reg(rs), Reg(rt), Lit16(lit16)),
            0x06 => Instruction::new2(code, BLEZ, Reg(rs), Lit16(lit16)),
            0x07 => Instruction::new2(code, BGTZ, Reg(rs), Lit16(lit16)),
            0x08 => Instruction::new3(code, ADDI, Reg(rt), Reg(rs), Lit16(lit16)),
            0x09 => Instruction::new3(code, ADDIU, Reg(rt), Reg(rs), Lit16(lit16)),
            0x0A => Instruction::new3(code, SLTI, Reg(rt), Reg(rs), Lit16(lit16)),
            0x0B => Instruction::new3(code, SLTIU, Reg(rt), Reg(rs), Lit16(lit16)),
            0x0C => Instruction::new3(code, ANDI, Reg(rt), Reg(rs), Lit16(lit16)),
            0x0D => Instruction::new3(code, ORI, Reg(rt), Reg(rs), Lit16(lit16)),
            0x0E => Instruction::new3(code, XORI, Reg(rt), Reg(rs), Lit16(lit16)),
            0x0F => Instruction::new2(code, LUI, Reg(rt), Lit16(lit16)),
            
            0x10..=0x13 => match (code >> 21) & 0x1F {
                0x00 if cpz == 0 => Instruction::new2(code, MFC0, Reg(rt), Cp0Reg(rd)),
                0x00 => Instruction::new2(code, MFCz, Reg(rt), Reg(rd)),
                
                0x01 if cpz == 0 => Instruction::new2(code, DMFC0, Reg(rt), Cp0Reg(rd)),
                
                0x02 => Instruction::new2(code, CFCz, Reg(rt), Reg(rd)),
                
                0x04 if cpz == 0 => Instruction::new2(code, MTC0, Reg(rt), Cp0Reg(rd)),
                0x04 => Instruction::new2(code, MTCz, Reg(rt), Reg(rd)),
                
                0x05 if cpz == 0 => Instruction::new2(code, DMTC0, Reg(rt), Cp0Reg(rd)),
                
                0x06 => Instruction::new2(code, CTCz, Reg(rt), Reg(rd)),
                
                0x08 => match (code >> 16) & 0x1F {
                    0x00 => Instruction::new1(code, BCzF, Lit16(lit16)),
                    0x01 => Instruction::new1(code, BCzFL, Lit16(lit16)),
                    0x02 => Instruction::new1(code, BCzT, Lit16(lit16)),
                    0x03 => Instruction::new1(code, BCzTL, Lit16(lit16)),
                    _ => Instruction::new0(code, Unknown)
                },
                
                0x10 if special == 0x01 => Instruction::new0(code, TLBR),
                0x10 if special == 0x02 => Instruction::new0(code, TLBWI),
                0x10 if special == 0x06 => Instruction::new0(code, TLBWR),
                0x10 if special == 0x08 => Instruction::new0(code, TLBP),
                0x10 if special == 0x18 => Instruction::new0(code, ERET),
                0x10..=0x1F => Instruction::new1(code, COPz, Lit32(code & 0x1FFFFFF)),
                
                _ => Instruction::new0(code, Unknown)
            },
            
            0x14 => Instruction::new3(code, BEQL, Reg(rs), Reg(rt), Lit16(lit16)),
            0x15 => Instruction::new3(code, BNEL, Reg(rs), Reg(rt), Lit16(lit16)),
            0x16 => Instruction::new2(code, BLEZL, Reg(rs), Lit16(lit16)),
            0x17 => Instruction::new2(code, BGTZL, Reg(rs), Lit16(lit16)),
            0x18 => Instruction::new3(code, DADDI, Reg(rt), Reg(rs), Lit16(lit16)),
            0x19 => Instruction::new3(code, DADDIU, Reg(rt), Reg(rs), Lit16(lit16)),
            
            0x1A => Instruction::new3(code, LDL, Reg(rt), Reg(base), Lit16(lit16)),
            0x1B => Instruction::new3(code, LDR, Reg(rt), Reg(base), Lit16(lit16)),
            
            0x20 => Instruction::new3(code, LB, Reg(rt), Reg(base), Lit16(lit16)),
            0x21 => Instruction::new3(code, LH, Reg(rt), Reg(base), Lit16(lit16)),
            0x22 => Instruction::new3(code, LWL, Reg(rt), Reg(base), Lit16(lit16)),
            0x23 => Instruction::new3(code, LW, Reg(rt), Reg(base), Lit16(lit16)),
            0x24 => Instruction::new3(code, LBU, Reg(rt), Reg(base), Lit16(lit16)),
            0x25 => Instruction::new3(code, LHU, Reg(rt), Reg(base), Lit16(lit16)),
            0x26 => Instruction::new3(code, LWR, Reg(rt), Reg(base), Lit16(lit16)),
            0x27 => Instruction::new3(code, LWU, Reg(rt), Reg(base), Lit16(lit16)),
            0x28 => Instruction::new3(code, SB, Reg(rt), Reg(base), Lit16(lit16)),
            0x29 => Instruction::new3(code, SH, Reg(rt), Reg(base), Lit16(lit16)),
            0x2A => Instruction::new3(code, SWL, Reg(rt), Reg(base), Lit16(lit16)),
            0x2B => Instruction::new3(code, SW, Reg(rt), Reg(base), Lit16(lit16)),
            0x2C => Instruction::new3(code, SDL, Reg(rt), Reg(base), Lit16(lit16)),
            0x2D => Instruction::new3(code, SDR, Reg(rt), Reg(base), Lit16(lit16)),
            0x2E => Instruction::new3(code, SWR, Reg(rt), Reg(base), Lit16(lit16)),
            0x2F => Instruction::new0(code, CACHE),
            
            0x30 => Instruction::new3(code, LL, Reg(rt), Reg(base), Lit16(lit16)),
            0x31 | 0x32 => Instruction::new3(code, LWCz, Reg(rt), Reg(base), Lit16(lit16)),
            0x34 => Instruction::new3(code, LLD, Reg(rt), Reg(base), Lit16(lit16)),
            0x35 | 0x36 => Instruction::new3(code, LDCz, Reg(rt), Reg(base), Lit16(lit16)),
            0x37 => Instruction::new3(code, LD, Reg(rt), Reg(base), Lit16(lit16)),
            
            0x38 => Instruction::new3(code, SC, Reg(rt), Reg(base), Lit16(lit16)),
            0x39 | 0x3A => Instruction::new3(code, SWCz, Reg(rt), Reg(base), Lit16(lit16)),
            0x3B => Instruction::new3(code, SCD, Reg(rt), Reg(base), Lit16(lit16)),
            0x3C | 0x3D => Instruction::new3(code, SDCz, Reg(rt), Reg(base), Lit16(lit16)),
            0x3F => Instruction::new3(code, SD, Reg(rt), Reg(base), Lit16(lit16)),
            
            _ => Instruction::new0(code, Unknown)
        },
    }
}

pub fn disassemble_u8(code_parts: &[u8]) -> Instruction {
    disassemble(((code_parts[0] as u32) << 24) | ((code_parts[1] as u32) << 16) | ((code_parts[2] as u32) << 8) | (code_parts[3] as u32))
}

fn to_u32(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | (bytes[3] as u32)
}