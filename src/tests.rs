use std::collections::HashSet;

use rustc_middle::{
    mir::{BasicBlock, Local, Location},
    ty::TyCtxt,
};

use crate::{
    analysis,
    expr::{Definition, Expr},
    utils,
};

fn print_mir(tcx: TyCtxt<'_>) {
    for def_id in tcx.hir_body_owners() {
        if tcx.item_name(def_id.to_def_id()).as_str() == "f" {
            let body = tcx.optimized_mir(def_id);
            for (bb, bbd) in body.basic_blocks.iter_enumerated() {
                println!("{bb:?}");
                for (i, stmt) in bbd.statements.iter().enumerate() {
                    println!("  {i}: {stmt:?}");
                }
                println!("  {}: {:?}", bbd.statements.len(), bbd.terminator().kind);
            }
        }
    }
}

fn embed_mir(params: &str, body: &str) -> String {
    format!(
        r#"
        #![feature(core_intrinsics, custom_mir)]
        #![allow(internal_features)]
        use core::intrinsics::mir::*;
        #[custom_mir(dialect = "runtime", phase = "optimized")]
        fn f({params}) -> i32 {{
            mir! {{ {body} }}
        }}
        "#,
    )
}

fn test_live_variables(params: &str, body: &str, expected: &[(usize, usize, &[usize])]) {
    let code = embed_mir(params, body);
    let res = utils::run_compiler_on_str(&code, |tcx| {
        print_mir(tcx);
        analysis::analyze_live_variables(tcx)
    })
    .unwrap();
    println!("{res:#?}");
    for (bb, stmt_idx, locals) in expected {
        let loc = Location {
            block: BasicBlock::from_usize(*bb),
            statement_index: *stmt_idx,
        };
        let state = res.get(&loc).unwrap();
        assert_eq!(
            state,
            &locals
                .iter()
                .map(|i| Local::from_usize(*i))
                .collect::<HashSet<_>>(),
            "bb{bb}:{stmt_idx}"
        );
    }
}

fn test_available_expressions(params: &str, body: &str, expected: &[(usize, usize, &[&str])]) {
    let code = embed_mir(params, body);
    let res = utils::run_compiler_on_str(&code, |tcx| {
        print_mir(tcx);
        analysis::analyze_available_expressions(tcx)
    })
    .unwrap();
    println!("{res:#?}");
    for (bb, stmt_idx, exprs) in expected {
        let loc = Location {
            block: BasicBlock::from_usize(*bb),
            statement_index: *stmt_idx,
        };
        let state = res.get(&loc).unwrap();
        assert_eq!(
            state,
            &exprs.iter().map(|s| Expr::parse(s)).collect::<HashSet<_>>(),
            "bb{bb}:{stmt_idx}"
        );
    }
}

fn test_very_busy_expressions(params: &str, body: &str, expected: &[(usize, usize, &[&str])]) {
    let code = embed_mir(params, body);
    let res = utils::run_compiler_on_str(&code, |tcx| {
        print_mir(tcx);
        analysis::analyze_very_busy_expressions(tcx)
    })
    .unwrap();
    println!("{res:#?}");
    for (bb, stmt_idx, exprs) in expected {
        let loc = Location {
            block: BasicBlock::from_usize(*bb),
            statement_index: *stmt_idx,
        };
        let state = res.get(&loc).unwrap();
        assert_eq!(
            state,
            &exprs.iter().map(|s| Expr::parse(s)).collect::<HashSet<_>>(),
            "bb{bb}:{stmt_idx}"
        );
    }
}

fn test_reaching_definitions(params: &str, body: &str, expected: &[(usize, usize, &[&str])]) {
    let code = embed_mir(params, body);
    let res = utils::run_compiler_on_str(&code, |tcx| {
        print_mir(tcx);
        analysis::analyze_reaching_definitions(tcx)
    })
    .unwrap();
    println!("{res:#?}");
    for (bb, stmt_idx, defs) in expected {
        let loc = Location {
            block: BasicBlock::from_usize(*bb),
            statement_index: *stmt_idx,
        };
        let state = res.get(&loc).unwrap();
        assert_eq!(
            state,
            &defs
                .iter()
                .map(|s| Definition::parse(s))
                .collect::<HashSet<_>>(),
            "bb{bb}:{stmt_idx}"
        );
    }
}

#[test]
fn test_live_variables_simple() {
    let params = "";
    let body = "
        let x;
        let y;
        {
            x = 1;
            y = x;
            RET = y;
            Return()
        }
    ";
    let expected: &[(usize, usize, &[usize])] =
        &[(0, 0, &[1]), (0, 1, &[2]), (0, 2, &[0]), (0, 3, &[])];
    test_live_variables(params, body, expected);
}

#[test]
fn test_available_expressions_simple() {
    let params = "x: i32, y: i32";
    let body = "
        let a;
        {
            a = x + y;
            x = 1;
            RET = a + y;
            Return()
        }
    ";
    let expected: &[(usize, usize, &[&str])] = &[
        (0, 0, &[]),
        (0, 1, &["_1+_2"]),
        (0, 2, &[]),
        (0, 3, &["_3+_2"]),
    ];
    test_available_expressions(params, body, expected);
}

#[test]
fn test_very_busy_expressions_loop() {
    let params = "x: i32, y: i32";
    let body = "
        let a;
        let b;
        {
            a = x + y;
            b = a + 1;
            Goto(bb1)
        }
        bb1 = {
            match x {
                0 => bb3,
                _ => bb2,
            }
        }
        bb2 = {
            a = b + y;
            b = a + 1;
            x = b + a;
            Goto(bb1)
        }
        bb3 = {
            RET = b + y;
            Return()
        }
    ";
    let expected: &[(usize, usize, &[&str])] = &[
        (0, 0, &["_3+1"]),
        (0, 1, &["_4+_2"]),
        (0, 2, &["_4+_2"]),
        (1, 0, &["_4+_2"]),
        (2, 0, &["_3+1"]),
        (2, 1, &["_4+_2", "_4+_3"]),
        (2, 2, &["_4+_2"]),
        (2, 3, &["_4+_2"]),
        (3, 0, &[]),
        (3, 1, &[]),
    ];
    test_very_busy_expressions(params, body, expected);
}

#[test]
fn test_reaching_definitions_loop() {
    let params = "x: i32, y: i32";
    let body = "
        let a;
        let b;
        {
            a = x;
            b = y;
            Goto(bb1)
        }
        bb1 = {
            match x {
                0 => bb3,
                _ => bb2,
            }
        }
        bb2 = {
            a = b + y;
            b = a + 1;
            x = b;
            Goto(bb1)
        }
        bb3 = {
            RET = a + b;
            Return()
        }
    ";
    let expected: &[(usize, usize, &[&str])] = &[
        (0, 1, &["_3=_1"]),
        (0, 2, &["_3=_1", "_4=_2"]),
        (1, 0, &["_1=_4", "_3=_1", "_3=_4+_2", "_4=_2", "_4=_3+1"]),
        (2, 0, &["_1=_4", "_3=_1", "_3=_4+_2", "_4=_2", "_4=_3+1"]),
        (2, 1, &["_1=_4", "_3=_4+_2", "_4=_2", "_4=_3+1"]),
        (2, 2, &["_1=_4", "_3=_4+_2", "_4=_3+1"]),
        (2, 3, &["_1=_4", "_3=_4+_2", "_4=_3+1"]),
        (3, 0, &["_1=_4", "_3=_1", "_3=_4+_2", "_4=_2", "_4=_3+1"]),
        (
            3,
            1,
            &["_0=_3+_4", "_1=_4", "_3=_1", "_3=_4+_2", "_4=_2", "_4=_3+1"],
        ),
    ];
    test_reaching_definitions(params, body, expected);
}
