use crate::cil_op::CILOp;
use rustc_middle::{
    mir::{Body, NonDivergingIntrinsic, Statement, StatementKind},
    ty::{Instance, TyCtxt},
};
pub fn handle_statement<'tcx>(
    statement: &Statement<'tcx>,
    _body: &'tcx Body<'tcx>,
    tyctx: TyCtxt<'tcx>,
    method: &rustc_middle::mir::Body<'tcx>,
    method_instance: Instance<'tcx>,
) -> Vec<CILOp> {
    let kind = &statement.kind;
    let res = match kind {
        StatementKind::StorageLive(_local) => {
            vec![]
        }
        StatementKind::StorageDead(_local) => {
            vec![]
        }
        StatementKind::Assign(palce_rvalue) => {
            let place = palce_rvalue.as_ref().0;
            let rvalue = &palce_rvalue.as_ref().1;
            let rvalue_ops =
                crate::rvalue::handle_rvalue(rvalue, tyctx, &place, method, method_instance);
            crate::place::place_set(&place, tyctx, rvalue_ops, method, method_instance)
        }
        StatementKind::Intrinsic(non_diverging_intirinsic) => {
            match non_diverging_intirinsic.as_ref() {
                NonDivergingIntrinsic::Assume(_) => vec![],
                _ => {
                    todo!("Can't handle non-diverging intrinsics {non_diverging_intirinsic:?} yet!")
                }
            }
        }
        _ => todo!("Unsuported statement kind {kind:?}"),
    };
    res
}
