use crate::internal::*;

#[derive(Debug, Clone, new)]
pub struct UnimplementedOp {
    name: String,
    message: String,
    output_arity: usize,
}

impl Op for UnimplementedOp {
    fn name(&self) -> Cow<str> {
        format!("Unimplemented({})", self.name).into()
    }

    not_a_typed_op!();
}

impl StatefullOp for UnimplementedOp {
    fn state(
        &self,
        _session: &mut SessionState,
        node_id: usize,
    ) -> TractResult<Option<Box<dyn OpState>>> {
        bail!("unimplemented operation: #{} {}", node_id, self.name)
    }
}

impl InferenceRulesOp for UnimplementedOp {
    fn rules<'r, 'p: 'r, 's: 'r>(
        &'s self,
        _: &mut Solver<'r>,
        _: &'p [TensorProxy],
        _: &'p [TensorProxy],
    ) -> InferenceResult {
        Ok(())
    }

    fn nboutputs(&self) -> TractResult<usize> {
        Ok(self.output_arity)
    }


    inference_op_as_op!();

    fn to_typed(
        &self,
        _source: &InferenceModel,
        _node: &InferenceNode,
        _target: &mut TypedModel,
        _mapping: &HashMap<OutletId, OutletId>,
    ) -> TractResult<TVec<OutletId>> {
        bail!("Operator can not be made a TypedOp.")
    }
}
