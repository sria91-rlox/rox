//! Implement [`SourceDebug`] for AST.

use super::*;
use crate::fmt::{debug_sexpr, SourceDebug};
use std::fmt;


impl SourceDebug for &[Item] {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = debug_sexpr(f, "Block");
        for i in self.iter() {
            d.atom(&i.wrap(source));
        }
        d.finish()
    }
}

impl SourceDebug for Item {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Class(inner) => inner.fmt(source, f),
            Item::Fun(inner) => inner.fmt(source, f),
            Item::Let(inner) => inner.fmt(source, f),
            Item::Statement(inner) => inner.fmt(source, f),
        }
    }
}

impl SourceDebug for ClassItem {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_sexpr(f, "Class")
            .atom(&self.name.wrap(source))
            .opt_kw_atom("inherit", self.inherit.as_ref()
                .map(|inherit| inherit.name.wrap(source)))
            // TODO methods
            .finish()
    }
}

impl SourceDebug for FunItem {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.function.fmt(source, f)
    }
}

impl SourceDebug for Function {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_sexpr(f, "Fun")
            .kw_atom("name", &self.name.wrap(source))
            // TODO parameters
            // TODO body
            .finish()
    }
}

impl SourceDebug for LetItem {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_sexpr(f, "Let")
            .atom(&self.name.wrap(source))
            // TODO expression
            .finish()
    }
}

impl SourceDebug for Statement {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Expr(inner) => inner.fmt(source, f),
            // Statement::For(inner) => inner.fmt(source, f),
            // Statement::If(inner) => inner.fmt(source, f),
            // Statement::Assert(inner) => inner.fmt(source, f),
            // Statement::Print(inner) => inner.fmt(source, f),
            // Statement::Return(inner) => inner.fmt(source, f),
            // Statement::While(inner) => inner.fmt(source, f),
            // Statement::Block(inner) => inner.fmt(source, f),
            _ => Ok(())
        }
    }
}

impl SourceDebug for ExprStmt {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.expr.fmt(source, f)
    }
}

impl SourceDebug for Expression {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Binary(inner) => inner.fmt(source, f),
            Expression::Unary(inner) => inner.fmt(source, f),
            // Expression::Field(inner) => inner.fmt(source, f),
            Expression::Group(inner) => inner.fmt(source, f),
            // Expression::Call(inner) => inner.fmt(source, f),
            Expression::Primary(inner) => inner.fmt(source, f),
            _ => Ok(())
        }
    }
}

impl SourceDebug for BinaryExpr {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_sexpr(f, self.operator.span.anchor(source).as_str())
            .atom(&self.lhs.wrap(source))
            .atom(&self.rhs.wrap(source))
            .finish()
    }
}

impl SourceDebug for UnaryExpr {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_sexpr(f, self.operator.span.anchor(source).as_str())
            .atom(&self.expr.wrap(source))
            .finish()
    }
}

impl SourceDebug for GroupExpr {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.expr.fmt(source, f)
    }
}

impl SourceDebug for PrimaryExpr {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token.span.anchor(source).as_str())
    }
}

impl SourceDebug for Identifier {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token.span.anchor(source).as_str())
    }
}
