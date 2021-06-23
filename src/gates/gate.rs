use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::Index;
use std::sync::Arc;

use crate::circuit_builder::CircuitBuilder;
use crate::field::extension_field::target::ExtensionTarget;
use crate::field::extension_field::{Extendable, FieldExtension};
use crate::field::field::Field;
use crate::gates::gate_tree::Tree;
use crate::generator::WitnessGenerator;
use crate::vars::{EvaluationTargets, EvaluationVars, EvaluationVarsBase};

/// A custom gate.
pub trait Gate<F: Extendable<D>, const D: usize>: 'static + Send + Sync {
    fn id(&self) -> String;

    fn eval_unfiltered(&self, vars: EvaluationVars<F, D>) -> Vec<F::Extension>;

    /// Like `eval_unfiltered`, but specialized for points in the base field.
    ///
    /// By default, this just calls `eval_unfiltered`, which treats the point as an extension field
    /// element. This isn't very efficient.
    fn eval_unfiltered_base(&self, vars_base: EvaluationVarsBase<F>) -> Vec<F> {
        let local_constants = &vars_base
            .local_constants
            .iter()
            .map(|c| F::Extension::from_basefield(*c))
            .collect::<Vec<_>>();
        let local_wires = &vars_base
            .local_wires
            .iter()
            .map(|w| F::Extension::from_basefield(*w))
            .collect::<Vec<_>>();
        let vars = EvaluationVars {
            local_constants,
            local_wires,
        };
        let values = self.eval_unfiltered(vars);

        // Each value should be in the base field, i.e. only the degree-zero part should be nonzero.
        values
            .into_iter()
            .map(|value| {
                // TODO: Change to debug-only once our gate code is mostly finished/stable.
                assert!(F::Extension::is_in_basefield(&value));
                value.to_basefield_array()[0]
            })
            .collect()
    }

    fn eval_unfiltered_recursively(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        vars: EvaluationTargets<D>,
    ) -> Vec<ExtensionTarget<D>>;

    fn eval_filtered(&self, vars: EvaluationVars<F, D>, prefix: &[bool]) -> Vec<F::Extension> {
        let filter = compute_filter(prefix, vars.local_constants);
        self.eval_unfiltered(vars)
            .into_iter()
            .map(|c| filter * c)
            .collect()
    }

    /// Like `eval_filtered`, but specialized for points in the base field.
    fn eval_filtered_base(&self, vars: EvaluationVarsBase<F>, prefix: &[bool]) -> Vec<F> {
        let filter = compute_filter(prefix, vars.local_constants);
        self.eval_unfiltered_base(vars)
            .into_iter()
            .map(|c| c * filter)
            .collect()
    }

    fn eval_filtered_recursively(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        vars: EvaluationTargets<D>,
    ) -> Vec<ExtensionTarget<D>> {
        // TODO: Filter
        self.eval_unfiltered_recursively(builder, vars)
    }

    fn generators(
        &self,
        gate_index: usize,
        local_constants: &[F],
    ) -> Vec<Box<dyn WitnessGenerator<F>>>;

    /// The number of wires used by this gate.
    fn num_wires(&self) -> usize;

    /// The number of constants used by this gate.
    fn num_constants(&self) -> usize;

    /// The maximum degree among this gate's constraint polynomials.
    fn degree(&self) -> usize;

    fn num_constraints(&self) -> usize;
}

/// A wrapper around an `Rc<Gate>` which implements `PartialEq`, `Eq` and `Hash` based on gate IDs.
#[derive(Clone)]
pub struct GateRef<F: Extendable<D>, const D: usize>(pub(crate) Arc<dyn Gate<F, D>>);

impl<F: Extendable<D>, const D: usize> GateRef<F, D> {
    pub fn new<G: Gate<F, D>>(gate: G) -> GateRef<F, D> {
        GateRef(Arc::new(gate))
    }
}

impl<F: Extendable<D>, const D: usize> PartialEq for GateRef<F, D> {
    fn eq(&self, other: &Self) -> bool {
        self.0.id() == other.0.id()
    }
}

impl<F: Extendable<D>, const D: usize> Hash for GateRef<F, D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.id().hash(state)
    }
}

impl<F: Extendable<D>, const D: usize> Eq for GateRef<F, D> {}

impl<F: Extendable<D>, const D: usize> Debug for GateRef<F, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0.id())
    }
}

/// A gate along with any constants used to configure it.
pub struct GateInstance<F: Extendable<D>, const D: usize> {
    pub gate_type: GateRef<F, D>,
    pub constants: Vec<F>,
}

/// Map each gate to a boolean prefix used to construct the gate's selector polynomial.
#[derive(Debug, Clone)]
pub struct GatePrefixes<F: Extendable<D>, const D: usize> {
    pub prefixes: HashMap<GateRef<F, D>, Vec<bool>>,
}

impl<F: Extendable<D>, const D: usize> From<Tree<GateRef<F, D>>> for GatePrefixes<F, D> {
    fn from(tree: Tree<GateRef<F, D>>) -> Self {
        GatePrefixes {
            prefixes: HashMap::from_iter(tree.traversal()),
        }
    }
}

impl<F: Extendable<D>, T: Borrow<GateRef<F, D>>, const D: usize> Index<T> for GatePrefixes<F, D> {
    type Output = Vec<bool>;

    fn index(&self, index: T) -> &Self::Output {
        &self.prefixes[index.borrow()]
    }
}

fn compute_filter<K: Field>(prefix: &[bool], constants: &[K]) -> K {
    prefix
        .iter()
        .enumerate()
        .map(|(i, &b)| {
            if b {
                constants[i]
            } else {
                K::ONE - constants[i]
            }
        })
        .product()
}
