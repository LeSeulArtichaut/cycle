use std::fmt;

use crate::{Edge, Expr, Tree};
use crate::{Number, Symbol, SymbolicResult};

use crate::base::alg::AOp;

/// A list of sequential operators.
#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Copy)]
pub enum SqOp {
  Sum,
  Prod,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Sequence {
  pub map: SqOp,
  pub idx: Symbol,
  pub lo: Edge,
  pub up: Edge,
  pub arg: Edge,
}

impl Sequence {
  /// Apply sequential simplifications.
  #[inline]
  pub fn sq_trivial(self) -> SymbolicResult<Tree> {
    let alg = self.algebra();
    let arg = self.arg.trivial()?;

    match (self.lo.trivial()?, self.up.trivial()?) {
      // ```_{i=l->u} f = f[i = l] _ f[i = l + 1] _ ... _ f[i = u - 1] _ f[i = u], l ∈ ℤ, u ∈ ℤ```
      (Tree::Num(Number::Z(l)), Tree::Num(Number::Z(u))) => {
        let sq = (l..=u).map(|i| arg.clone().evaluate(Tree::Sym(self.idx.clone()), Tree::from(i)).edge()).collect();
        Tree::assoc(alg, sq).trivial()
      }

      (lo, up) => Ok(Tree::sequence_order(
        self.map, //.
        self.idx,
        lo.edge(),
        up.edge(),
        arg.edge(),
      )),
    }
  }

  const fn algebra(&self) -> AOp {
    match self.map {
      SqOp::Sum => AOp::Add,
      SqOp::Prod => AOp::Mul,
    }
  }
}

impl fmt::Display for Sequence {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}({}, {}, {}, {})", self.map, self.idx, self.lo, self.up, self.arg)
  }
}

impl fmt::Display for SqOp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      // Sigma sum form
      // ```S(i, l, u, f)```
      SqOp::Sum => {
        write!(f, "S")
      }
      // Pi product form
      // ```P(i, l, u, f)```
      SqOp::Prod => {
        write!(f, "P")
      }
    }
  }
}

impl Tree {
  pub(crate) fn sequence_order(
    //.
    map: SqOp,
    idx: Symbol,
    lo: Edge,
    up: Edge,
    arg: Edge,
  ) -> Tree {
    Tree::Sq(Sequence {
      //.
      map,
      idx,
      lo,
      up,
      arg,
    })
  }
}
