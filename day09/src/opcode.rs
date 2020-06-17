mod addr;
use addr::*;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum OpCode {
    Add {
        opAddr1: Addr,
        opAddr2: Addr,
        destAddr: Addr,
    },
    Mult {
        opAddr1: Addr,
        opAddr2: Addr,
        destAddr: Addr,
    },
    Read {
        destAddr: Addr,
    },
    Write {
        opAddr1: Addr,
    },
    JumpIf {
        boolAddr: Addr,
        jumpAddr: Addr,
    },
    JumpIfNot {
        boolAddr: Addr,
        jumpAddr: Addr,
    },
    SetIfLt {
        opAddr1: Addr,
        opAddr2: Addr,
        destAddr: Addr,
    }, // set to 1 or 0
    SetIfEq {
        opAddr1: Addr,
        opAddr2: Addr,
        destAddr: Addr,
    },
    SetRelBase {
        relAddrChange: Addr,
    },
    Halt,
}
use OpCode::*;

pub struct CurrentState<'data> {
    pub data: &'data Vec<i64>,
    pub program_counter: i64,
    pub relative_base: i64,
}
impl From<CurrentState<'_>> for OpCode {
    fn from(cs: CurrentState) -> OpCode {
        let data: &Vec<i64> = cs.data;
        let program_counter: i64 = cs.program_counter;
        let relative_base: i64 = cs.relative_base;

        let opcode = data.get(program_counter as usize).unwrap();
        let mode3 = Mode::try_from((opcode / 10000) % 10).unwrap();
        let mode2 = Mode::try_from((opcode / 1000) % 10).unwrap();
        let mode1 = Mode::try_from((opcode / 100) % 10).unwrap();
        let op = opcode % 100;

        match op {
            1 => Add {
                opAddr1: Addr::from((program_counter + 1, mode1, relative_base)),
                opAddr2: Addr::from((program_counter + 2, mode2, relative_base)),
                destAddr: Addr::from((program_counter + 3, mode3, relative_base)),
            },
            2 => Mult {
                opAddr1: Addr::from((program_counter + 1, mode1, relative_base)),
                opAddr2: Addr::from((program_counter + 2, mode2, relative_base)),
                destAddr: Addr::from((program_counter + 3, mode3, relative_base)),
            },
            3 => Read {
                destAddr: Addr::from((program_counter + 1, mode1, relative_base)),
            },
            4 => Write {
                opAddr1: Addr::from((program_counter + 1, mode1, relative_base)),
            },
            5 => JumpIf {
                boolAddr: Addr::from((program_counter + 1, mode1, relative_base)),
                jumpAddr: Addr::from((program_counter + 2, mode2, relative_base)),
            },
            6 => JumpIfNot {
                boolAddr: Addr::from((program_counter + 1, mode1, relative_base)),
                jumpAddr: Addr::from((program_counter + 2, mode2, relative_base)),
            },
            7 => SetIfLt {
                opAddr1: Addr::from((program_counter + 1, mode1, relative_base)),
                opAddr2: Addr::from((program_counter + 2, mode2, relative_base)),
                destAddr: Addr::from((program_counter + 3, mode3, relative_base)),
            },
            8 => SetIfEq {
                opAddr1: Addr::from((program_counter + 1, mode1, relative_base)),
                opAddr2: Addr::from((program_counter + 2, mode2, relative_base)),
                destAddr: Addr::from((program_counter + 3, mode3, relative_base)),
            },
            9 => SetRelBase {
                relAddrChange: Addr::from((program_counter + 1, mode1, relative_base)),
            },
            99 => Halt,
            _ => panic!(format!("{} is an invalid opcode. must be in [0,8]", opcode)),
        }
    }
}
impl OpCode {
    fn numFields(&self) -> i32 {
        match self {
            Add {
                opAddr1: _,
                opAddr2: _,
                destAddr: _,
            } => 3,
            Mult {
                opAddr1: _,
                opAddr2: _,
                destAddr: _,
            } => 3,
            Read { destAddr: _ } => 1,
            Write { opAddr1: _ } => 1,
            JumpIf {
                boolAddr: _,
                jumpAddr: _,
            } => 2,
            JumpIfNot {
                boolAddr: _,
                jumpAddr: _,
            } => 2,
            SetIfLt {
                opAddr1: _,
                opAddr2: _,
                destAddr: _,
            } => 3,
            SetIfEq {
                opAddr1: _,
                opAddr2: _,
                destAddr: _,
            } => 3,
            SetRelBase { relAddrChange: _ } => 1,
            Halt => 0,
        }
    }
    pub fn executeIntruction(
        &self,
        rel_base: &mut i64,
        data: &mut Vec<i64>,
        i: &mut i64,
        input: &mut dyn Iterator<Item = i64>,
    ) -> Option<i64> {
        let mut jumped = false;
        let retval = match self {
            Add {
                opAddr1,
                opAddr2,
                destAddr,
            } => {
                *destAddr.getData(data) = *opAddr1.getData(data) + *opAddr2.getData(data);
                None
            }
            Mult {
                opAddr1,
                opAddr2,
                destAddr,
            } => {
                *destAddr.getData(data) = *opAddr1.getData(data) * *opAddr2.getData(data);
                None
            }
            Read { destAddr } => {
                *destAddr.getData(data) = input.next().unwrap();
                None
            }
            Write { opAddr1 } => Some(*opAddr1.getData(data)),
            JumpIf { boolAddr, jumpAddr } => {
                if *boolAddr.getData(data) != 0 {
                    *i = *jumpAddr.getData(data);
                    jumped = true;
                }
                None
            }
            JumpIfNot { boolAddr, jumpAddr } => {
                if *boolAddr.getData(data) == 0 {
                    *i = *jumpAddr.getData(data);
                    jumped = true;
                }
                None
            }
            SetIfLt {
                opAddr1,
                opAddr2,
                destAddr,
            } => {
                *destAddr.getData(data) = if *opAddr1.getData(data) < *opAddr2.getData(data) {
                    1
                } else {
                    0
                };
                None
            }
            SetIfEq {
                opAddr1,
                opAddr2,
                destAddr,
            } => {
                *destAddr.getData(data) = if *opAddr1.getData(data) == *opAddr2.getData(data) {
                    1
                } else {
                    0
                };
                None
            }
            SetRelBase { relAddrChange } => {
                *rel_base += *relAddrChange.getData(data);
                None
            }
            Halt => {
                *i = -1;
                jumped = true;
                None
            }
        };
        if !jumped {
            *i += self.numFields() as i64 + 1;
        }
        retval
    }
}
