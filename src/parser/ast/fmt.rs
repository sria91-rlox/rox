//! Implement [`SourceDebug`] for AST.

use super::*;
use crate::fmt::SourceDebug;
use std::fmt;


impl<T: SourceDebug> SourceDebug for &[T] {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.iter().map(|e| e.wrap(source)))
            .finish()
    }
}

impl SourceDebug for Item {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Class(inner) => inner.fmt(source, f),
            Item::Fn(inner) => inner.fmt(source, f),
            Item::Let(inner) => inner.fmt(source, f),
            Item::Statement(inner) => inner.fmt(source, f),
        }
    }
}

impl SourceDebug for ClassItem {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut w = f.debug_struct("Class");
        w.field("name", &self.name.wrap(source));
        if let Some(inherit) = &self.inherit {
            w.field("inherit", &inherit.name.wrap(source));
        }
        w.field("methods", &self.methods.as_slice().wrap(source));
        w.finish()
    }
}

impl SourceDebug for FnItem {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.function.fmt(source, f)
    }
}

impl SourceDebug for Function {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fun")
            .field("name", &self.name.wrap(source))
            .field("params", &self.parameters.items.as_slice().wrap(source))
            .field("body", &self.body.wrap(source))
            .finish()
    }
}

impl SourceDebug for FnParam {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mut_tok.is_some() {
            write!(f, "Mut({:?})", self.name.wrap(source))
        } else {
            self.name.fmt(source, f)
        }
    }
}

impl SourceDebug for LetItem {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut w = if self.mut_tok.is_some() {
            f.debug_tuple("LetMut")
        } else {
            f.debug_tuple("Let")
        };
        w.field(&self.name.wrap(source));
        if let Some(init) = &self.init {
            w.field(&init.expr.wrap(source));
        }
        w.finish()
    }
}

impl SourceDebug for Statement {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Expr(inner) => inner.fmt(source, f),
            Statement::For(inner) => inner.fmt(source, f),
            Statement::If(inner) => inner.fmt(source, f),
            Statement::Assert(inner) => inner.fmt(source, f),
            Statement::Print(inner) => inner.fmt(source, f),
            Statement::Return(inner) => inner.fmt(source, f),
            Statement::While(inner) => inner.fmt(source, f),
            Statement::Block(inner) => inner.fmt(source, f),
        }
    }
}

impl SourceDebug for ExprStmt {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.expr.fmt(source, f)
    }
}

impl SourceDebug for ForStmt {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("For")
            .field("elem", &self.elem.wrap(source))
            .field("iter", &self.iter.wrap(source))
            .field("body", &self.body.wrap(source))
            .finish()
    }
}

impl SourceDebug for IfStmt {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut w = f.debug_struct("If");
        w.field("pred", &self.pred.wrap(source));
        w.field("then", &self.body.wrap(source));
        if let Some(else_branch) = &self.else_branch {
            w.field("else", &else_branch.body.wrap(source));
        }
        w.finish()
    }
}

impl SourceDebug for AssertStmt {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Assert")
            .field(&self.expr.wrap(source))
            .finish()
    }
}

impl SourceDebug for PrintStmt {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Print")
            .field(&self.expr.wrap(source))
            .finish()
    }
}

impl SourceDebug for ReturnStmt {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Return")
            .field(&self.expr.wrap(source))
            .finish()
    }
}

impl SourceDebug for WhileStmt {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("While")
            .field("pred", &self.pred.wrap(source))
            .field("body", &self.body.wrap(source))
            .finish()
    }
}

impl SourceDebug for Block {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.body.as_slice().fmt(source, f)
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
        f.debug_tuple(self.operator.span.anchor(source).as_str())
            .field(&self.lhs.wrap(source))
            .field(&self.rhs.wrap(source))
            .finish()
    }
}

impl SourceDebug for UnaryExpr {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(self.operator.span.anchor(source).as_str())
            .field(&self.expr.wrap(source))
            .finish()
    }
}

impl SourceDebug for GroupExpr {
    fn fmt(&self, source: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(expr) = self.expr.as_ref() {
            expr.fmt(source, f)
        } else {
            f.write_str("Unit")
        }
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
