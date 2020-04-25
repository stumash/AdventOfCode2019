use crate::Addr::*;
use crate::OpCode::*;
use std::io::{self, BufRead};

trait DataAddr {
    fn getData<'a>(&self, data: &'a mut Vec<i32>) -> &'a mut i32;
}
#[derive(Debug)]
pub enum Addr {
    Immediate(Imm),
    Position(Pos),
}
#[derive(Debug)]
pub struct Imm(i32);
impl Into<Addr> for Imm {
    fn into(self) -> Addr {
        Immediate(Imm(self.0))
    }
}
impl DataAddr for Imm {
    fn getData<'a>(&self, data: &'a mut Vec<i32>) -> &'a mut i32 {
        data.get_mut(self.0 as usize).unwrap()
    }
}
#[derive(Debug)]
pub struct Pos(i32);
impl DataAddr for Pos {
    fn getData<'a>(&self, data: &'a mut Vec<i32>) -> &'a mut i32 {
        let val = *(data.get(self.0 as usize).unwrap()) as usize;
        data.get_mut(val).unwrap()
    }
}
impl Into<Addr> for &Pos {
    fn into(self) -> Addr {
        Position(Pos(self.0))
    }
}
impl From<(i32, bool)> for Addr {
    fn from(tuple: (i32, bool)) -> Addr {
        let (i, isImmediate) = (tuple.0, tuple.1);

        match isImmediate {
            true => Immediate(Imm(i)),
            false => Position(Pos(i)),
        }
    }
}
impl DataAddr for Addr {
    fn getData<'a>(&self, data: &'a mut Vec<i32>) -> &'a mut i32 {
        match self {
            Immediate(Imm(i)) => data.get_mut((*i) as usize).unwrap(),
            Position(Pos(i)) => {
                let val = *(data.get((*i) as usize).unwrap()) as usize;
                data.get_mut(val).unwrap()
            }
        }
    }
}

#[derive(Debug)]
pub enum OpCode {
    Add {
        opAddr1: Addr,
        opAddr2: Addr,
        destAddr: Pos,
    },
    Mult {
        opAddr1: Addr,
        opAddr2: Addr,
        destAddr: Pos,
    },
    Read {
        destAddr: Pos,
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
        destAddr: Pos,
    }, // set to 1 or 0
    SetIfEq {
        opAddr1: Addr,
        opAddr2: Addr,
        destAddr: Pos,
    },
    Halt,
}
impl From<(&Vec<i32>, i32)> for OpCode {
    fn from(tuple: (&Vec<i32>, i32)) -> OpCode {
        let data: &Vec<i32> = tuple.0;
        let i: i32 = tuple.1;

        let opcode = data.get(i as usize).unwrap();
        let mode2 = ((opcode / 1000) % 10) == 1;
        let mode1 = ((opcode / 100) % 10) == 1;
        let op = opcode % 100;

        match op {
            1 => Add {
                opAddr1: Addr::from((i + 1, mode1)),
                opAddr2: Addr::from((i + 2, mode2)),
                destAddr: Pos(i + 3),
            },
            2 => Mult {
                opAddr1: Addr::from((i + 1, mode1)),
                opAddr2: Addr::from((i + 2, mode2)),
                destAddr: Pos(i + 3),
            },
            3 => Read {
                destAddr: Pos(i + 1),
            },
            4 => Write {
                opAddr1: Addr::from((i + 1, mode1)),
            },
            5 => JumpIf {
                boolAddr: Addr::from((i + 1, mode1)),
                jumpAddr: Addr::from((i + 2, mode2)),
            },
            6 => JumpIfNot {
                boolAddr: Addr::from((i + 1, mode1)),
                jumpAddr: Addr::from((i + 2, mode2)),
            },
            7 => SetIfLt {
                opAddr1: Addr::from((i + 1, mode1)),
                opAddr2: Addr::from((i + 2, mode2)),
                destAddr: Pos(i + 3),
            },
            8 => SetIfEq {
                opAddr1: Addr::from((i + 1, mode1)),
                opAddr2: Addr::from((i + 2, mode2)),
                destAddr: Pos(i + 3),
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
            Halt => 0,
        }
    }
    pub fn executeIntruction(
        &self,
        data: &mut Vec<i32>,
        i: &mut i32,
        input: &mut dyn Iterator<Item=i32>,
    ) -> Option<i32> {
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
            Write { opAddr1 } => {
                Some(*opAddr1.getData(data))
            }
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
            Halt => {
                *i = -1;
                jumped = true;
                None
            }
        };
        if !jumped {
            *i += self.numFields() + 1;
        }
        retval
    }
}
