use std::collections::{HashMap, HashSet};

use rustc_middle::{
    mir::{Body, Local, Location, Rvalue, Statement, StatementKind, Terminator, TerminatorKind},
    ty::TyCtxt,
};

use crate::expr::{Definition, Expr};

/// Runs live variable analysis on the function `f` and returns the set of live
/// variables after each MIR location.
pub fn analyze_live_variables(tcx: TyCtxt<'_>) -> HashMap<Location, HashSet<Local>> {
    analyze::<Backward, LiveVariableAnalysis>(tcx)
        .into_iter()
        .map(|(loc, s)| (loc, s.0))
        .collect()
}

/// Runs available expression analysis on the function `f` and returns the set of
/// available expressions before each MIR location.
pub fn analyze_available_expressions(tcx: TyCtxt<'_>) -> HashMap<Location, HashSet<Expr>> {
    analyze::<Forward, AvailableExpressionAnalysis>(tcx)
        .into_iter()
        .map(|(loc, s)| match s {
            MustSet::Set(s) => (loc, s),
            MustSet::Bot => panic!("unsupported"),
        })
        .collect()
}

/// Runs very busy expression analysis on the function `f` and returns the set of
/// very busy expressions after each MIR location.
pub fn analyze_very_busy_expressions(tcx: TyCtxt<'_>) -> HashMap<Location, HashSet<Expr>> {
    analyze::<Backward, VeryBusyExpressionAnalysis>(tcx)
        .into_iter()
        .map(|(loc, s)| match s {
            MustSet::Set(s) => (loc, s),
            MustSet::Bot => panic!("unsupported"),
        })
        .collect()
}

/// Runs reaching definition analysis on the function `f` and returns the set of
/// reaching definitions before each MIR location.
pub fn analyze_reaching_definitions(tcx: TyCtxt<'_>) -> HashMap<Location, HashSet<Definition>> {
    analyze::<Forward, ReachingDefinitionAnalysis>(tcx)
        .into_iter()
        .map(|(loc, s)| (loc, s.0))
        .collect()
}

/// Finds the function named `f` in the crate and runs the dataflow analysis on it.
fn analyze<D: Direction, T: TransferFns>(tcx: TyCtxt<'_>) -> HashMap<Location, T::S> {
    for def_id in tcx.hir_body_owners() {
        if tcx.item_name(def_id.to_def_id()).as_str() == "f" {
            let body = tcx.optimized_mir(def_id);
            return find_fixed_point::<D, T>(body);
        }
    }
    unreachable!()
}

/// Implements `PropagationWorkListAlgorithm`.
fn find_fixed_point<D: Direction, T: TransferFns>(_body: &Body<'_>) -> HashMap<Location, T::S> {
    let mut states: HashMap<Location, T::S> = HashMap::new();
    let mut worklist: Vec<Location> = vec![];
    // TODO
    while let Some(_loc) = worklist.pop() {
        // TODO
    }
    states
}

/// An abstract state.
trait AbsState {
    /// Returns the bottom element of the lattice.
    fn bot() -> Self;
    /// Returns `true` if `self` and `other` are different.
    fn ne(&self, other: &Self) -> bool;
    /// Computes the least upper bound of `self` and `other`.
    fn join(&self, other: &Self) -> Self;
}

/// Power set lattice `(P(T), ⊆)`.
#[derive(Debug, Clone)]
struct MaySet<T>(HashSet<T>);

impl<T: Clone + Eq + std::hash::Hash> AbsState for MaySet<T> {
    fn bot() -> Self {
        todo!()
    }

    fn ne(&self, _other: &Self) -> bool {
        todo!()
    }

    fn join(&self, _other: &Self) -> Self {
        todo!()
    }
}

/// Reverse power set lattice `(P(T), ⊇)`.
#[derive(Debug, Clone)]
enum MustSet<T> {
    Bot,
    Set(HashSet<T>),
}

impl<T: Clone + Eq + std::hash::Hash> AbsState for MustSet<T> {
    fn bot() -> Self {
        todo!()
    }

    fn ne(&self, _other: &Self) -> bool {
        todo!()
    }

    fn join(&self, _other: &Self) -> Self {
        todo!()
    }
}

/// The direction of the analysis.
trait Direction {
    /// Returns the location where the analysis starts.
    fn start_location(body: &Body<'_>) -> Location;
    /// Returns the locations that should receive the transferred state from `loc`.
    fn dep(body: &Body<'_>, loc: Location) -> Vec<Location>;
}

/// Forward direction.
struct Forward;

impl Direction for Forward {
    fn start_location(_body: &Body<'_>) -> Location {
        todo!()
    }

    fn dep(_body: &Body<'_>, _loc: Location) -> Vec<Location> {
        todo!()
    }
}

/// Backward direction.
struct Backward;

impl Direction for Backward {
    fn start_location(_body: &Body<'_>) -> Location {
        todo!()
    }

    fn dep(_body: &Body<'_>, _loc: Location) -> Vec<Location> {
        todo!()
    }
}

/// Transfer functions for a specific dataflow analysis.
trait TransferFns {
    type S: AbsState;
    /// The initial abstract state at the start location.
    fn start_state() -> Self::S;
    /// Applies the transfer function for a statement.
    fn transfer_stmt(stmt: &Statement<'_>, state: &Self::S) -> Self::S;
    /// Applies the transfer function for a terminator.
    fn transfer_term(term: &Terminator<'_>, state: &Self::S) -> Self::S;
}

struct LiveVariableAnalysis;

impl TransferFns for LiveVariableAnalysis {
    type S = MaySet<Local>;

    fn start_state() -> Self::S {
        todo!()
    }

    fn transfer_stmt(stmt: &Statement<'_>, _state: &Self::S) -> Self::S {
        if let StatementKind::Assign(box (_l, r)) = &stmt.kind {
            match r {
                Rvalue::Use(..) => todo!(),
                Rvalue::BinaryOp(..) => todo!(),
                _ => panic!("unsupported"),
            }
        } else {
            panic!("unsupported")
        }
    }

    fn transfer_term(term: &Terminator<'_>, _state: &Self::S) -> Self::S {
        match &term.kind {
            TerminatorKind::Goto { .. } => todo!(),
            TerminatorKind::SwitchInt { .. } => todo!(),
            TerminatorKind::Return => todo!(),
            _ => panic!("unsupported"),
        }
    }
}

struct AvailableExpressionAnalysis;

impl TransferFns for AvailableExpressionAnalysis {
    type S = MustSet<Expr>;

    fn start_state() -> Self::S {
        todo!()
    }

    fn transfer_stmt(stmt: &Statement<'_>, _state: &Self::S) -> Self::S {
        if let StatementKind::Assign(box (_l, r)) = &stmt.kind {
            match r {
                Rvalue::Use(..) => todo!(),
                Rvalue::BinaryOp(..) => todo!(),
                _ => panic!("unsupported"),
            }
        } else {
            panic!("unsupported")
        }
    }

    fn transfer_term(term: &Terminator<'_>, _state: &Self::S) -> Self::S {
        match &term.kind {
            TerminatorKind::Goto { .. } => todo!(),
            TerminatorKind::SwitchInt { .. } => todo!(),
            TerminatorKind::Return => todo!(),
            _ => panic!("unsupported"),
        }
    }
}

struct VeryBusyExpressionAnalysis;

impl TransferFns for VeryBusyExpressionAnalysis {
    type S = MustSet<Expr>;

    fn start_state() -> Self::S {
        todo!()
    }

    fn transfer_stmt(stmt: &Statement<'_>, _state: &Self::S) -> Self::S {
        if let StatementKind::Assign(box (_l, r)) = &stmt.kind {
            match r {
                Rvalue::Use(..) => todo!(),
                Rvalue::BinaryOp(..) => todo!(),
                _ => panic!("unsupported"),
            }
        } else {
            panic!("unsupported")
        }
    }

    fn transfer_term(term: &Terminator<'_>, _state: &Self::S) -> Self::S {
        match &term.kind {
            TerminatorKind::Goto { .. } => todo!(),
            TerminatorKind::SwitchInt { .. } => todo!(),
            TerminatorKind::Return => todo!(),
            _ => panic!("unsupported"),
        }
    }
}

struct ReachingDefinitionAnalysis;

impl TransferFns for ReachingDefinitionAnalysis {
    type S = MaySet<Definition>;

    fn start_state() -> Self::S {
        todo!()
    }

    fn transfer_stmt(stmt: &Statement<'_>, _state: &Self::S) -> Self::S {
        if let StatementKind::Assign(box (_l, r)) = &stmt.kind {
            match r {
                Rvalue::Use(..) => todo!(),
                Rvalue::BinaryOp(..) => todo!(),
                _ => panic!("unsupported"),
            }
        } else {
            panic!("unsupported")
        }
    }

    fn transfer_term(term: &Terminator<'_>, _state: &Self::S) -> Self::S {
        match &term.kind {
            TerminatorKind::Goto { .. } => todo!(),
            TerminatorKind::SwitchInt { .. } => todo!(),
            TerminatorKind::Return => todo!(),
            _ => panic!("unsupported"),
        }
    }
}
