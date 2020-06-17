use std::convert::TryFrom;

pub enum Mode {
    ImmediateMode,
    PositionMode,
    RelativeMode,
}
use Mode::*;
impl TryFrom<i64> for Mode {
    type Error = String;
    fn try_from(i: i64) -> Result<Self, Self::Error> {
        match i {
            0 => Ok(PositionMode),
            1 => Ok(ImmediateMode),
            2 => Ok(RelativeMode),
            _ => Err(format!("{:?} is not a valid Mode, must be in [0,1,2]", i)),
        }
    }
}

#[derive(Debug)]
pub enum Addr {
    Immediate(Imm),
    Position(Pos),
    Relative(Rel),
}
impl From<(i64, Mode, i64)> for Addr {
    fn from(tuple: (i64, Mode, i64)) -> Addr {
        let (i, mode, relative_base) = (tuple.0, tuple.1, tuple.2);

        match mode {
            ImmediateMode => Immediate(Imm(i)),
            PositionMode => Position(Pos(i)),
            RelativeMode => Relative(Rel(i, relative_base)),
        }
    }
}
impl DataAddr for Addr {
    fn getData<'a>(&self, data: &'a mut Vec<i64>) -> &'a mut i64 {
        match self {
            Immediate(imm) => imm.getData(data),
            Position(pos) => pos.getData(data),
            Relative(rel) => rel.getData(data),
        }
    }
}
use Addr::*;

pub trait DataAddr {
    fn getData<'a>(&self, data: &'a mut Vec<i64>) -> &'a mut i64;
}
#[derive(Debug)]
pub struct Imm(pub i64);
impl DataAddr for Imm {
    fn getData<'a>(&self, data: &'a mut Vec<i64>) -> &'a mut i64 {
        data.get_mut(self.0 as usize).unwrap()
    }
}
#[derive(Debug)]
pub struct Pos(pub i64);
impl DataAddr for Pos {
    fn getData<'a>(&self, data: &'a mut Vec<i64>) -> &'a mut i64 {
        let val = *(data.get(self.0 as usize).unwrap()) as usize;
        if val >= data.len() {
            data.resize(val + 1, 0);
        }
        data.get_mut(val).unwrap()
    }
}
impl Into<Addr> for Pos {
    fn into(self) -> Addr {
        Position(Pos(self.0))
    }
}
#[derive(Debug)]
pub struct Rel(pub i64, pub i64);
impl DataAddr for Rel {
    fn getData<'a>(&self, data: &'a mut Vec<i64>) -> &'a mut i64 {
        let val = *(data.get(self.0 as usize).unwrap());
        if (val + self.1) as usize >= data.len() {
            data.resize((val + self.1 + 1) as usize, 0);
        }
        data.get_mut((val + self.1) as usize).unwrap()
    }
}
