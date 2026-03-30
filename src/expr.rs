use rustc_middle::mir::{Local, Operand, Rvalue};

/// A simplified MIR operand: either a local variable or an integer constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleOperand {
    Local(Local),
    Int(i32),
}

impl SimpleOperand {
    /// Converts a MIR `Operand` into a `SimpleOperand`.
    pub fn from_operand(op: &Operand<'_>) -> Self {
        match op {
            Operand::Copy(place) | Operand::Move(place) => Self::Local(place.local),
            Operand::Constant(c) => Self::Int(c.const_.try_to_scalar_int().unwrap().to_i32()),
        }
    }

    /// Returns `true` if this operand is the given local variable.
    pub fn is_local(&self, local: Local) -> bool {
        matches!(self, Self::Local(l) if *l == local)
    }

    /// Parses `"_N"` as `Local(N)` or `"N"` as `Int(N)`.
    pub fn parse(s: &str) -> Self {
        if let Some(n) = s.strip_prefix('_') {
            Self::Local(Local::from_usize(n.parse().unwrap()))
        } else {
            Self::Int(s.parse::<i32>().unwrap())
        }
    }
}

/// A nontrivial expression: an addition with two operands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Expr {
    pub lhs: SimpleOperand,
    pub rhs: SimpleOperand,
}

impl Expr {
    pub fn new(lhs: SimpleOperand, rhs: SimpleOperand) -> Self {
        Self { lhs, rhs }
    }

    /// Returns `true` if this expression references the given local variable.
    pub fn contains_local(&self, local: Local) -> bool {
        self.lhs.is_local(local) || self.rhs.is_local(local)
    }

    /// Parses `"_1+_2"`, `"_1+42"`, etc.
    pub fn parse(s: &str) -> Self {
        let b = s.as_bytes();
        let op_pos = (1..s.len()).find(|&i| b[i] == b'+').unwrap();
        Self::new(
            SimpleOperand::parse(&s[..op_pos]),
            SimpleOperand::parse(&s[op_pos + 1..]),
        )
    }
}

/// The value assigned by a definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefinitionValue {
    Operand(SimpleOperand),
    Expr(Expr),
}

impl DefinitionValue {
    /// Converts a MIR `Rvalue` into a `DefinitionValue`.
    pub fn from_rvalue(rvalue: &Rvalue<'_>) -> Self {
        match rvalue {
            Rvalue::Use(op) => Self::Operand(SimpleOperand::from_operand(op)),
            Rvalue::BinaryOp(_, box (lhs, rhs)) => Self::Expr(Expr::new(
                SimpleOperand::from_operand(lhs),
                SimpleOperand::from_operand(rhs),
            )),
            _ => unreachable!(),
        }
    }

    /// Parses `"_1"` as `Operand(Local(1))`, `"42"` as `Operand(Const(42))`,
    /// and `"_1+_2"` as `Expr(...)`.
    pub fn parse(s: &str) -> Self {
        if s.as_bytes().contains(&b'+') {
            Self::Expr(Expr::parse(s))
        } else {
            Self::Operand(SimpleOperand::parse(s))
        }
    }
}

/// A reaching definition `x=e`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Definition {
    pub local: Local,
    pub value: DefinitionValue,
}

impl Definition {
    pub fn new(local: Local, value: DefinitionValue) -> Self {
        Self { local, value }
    }

    /// Parses `"_1=_2"` or `"_1=_2+3"` as a definition `x=e`.
    pub fn parse(s: &str) -> Self {
        let (local, value) = s.split_once('=').unwrap();
        Self::new(
            Local::from_usize(local.strip_prefix('_').unwrap().parse().unwrap()),
            DefinitionValue::parse(value),
        )
    }
}
