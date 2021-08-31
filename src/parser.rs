use crate::lexer::{Lexer, Token, TokenKind};
use std::num::ParseFloatError;


pub mod ast;

use ast::*;


// Parsing implementation
//
// This parsing algorithm is **not** resilient, it will bail on the first error encountered.
// Precedence on infix expressions is handled using the Pratt binding power algorithm.

#[derive(Debug)]
pub enum Error {
    UnexpectedToken {
        found: Token,
        expected: TokenKind,
    },
    ExpectedExpressionStart {
        found: Token,
    },
    ExpectedInfixOrPostfixOperator {
        found: Token,
    },
    InvalidNumberLiteral {
        token: Token,
        cause: ParseFloatError,
    },
    InvalidAssignmentTarget,
}

struct Parser<'src> {
    lexer: Lexer<'src>,
}

type Result<T> = std::result::Result<T, Error>;

pub fn parse(src: &str) -> Result<Program> {
    Parser {
        lexer: Lexer::new(src),
    }.program()
}

// Utility functions for parsing
impl<'src> Parser<'src> {
    fn expect_next(&mut self, kind: TokenKind) -> Result<Token> {
        let token = self.lexer.next2();
        if token.kind == kind {
            Ok(token)
        } else {
            Err(Error::UnexpectedToken {
                found: token,
                expected: kind,
            })
        }
    }

    fn peek_kind(&self) -> TokenKind {
        self.lexer.peek().kind
    }

    fn match_peek(&self, kind: TokenKind) -> Option<Token> {
        let token = self.lexer.peek();
        if token.kind == kind {
            Some(token)
        } else {
            None
        }
    }
}

impl<'src> Parser<'src> {
    fn program(&mut self) -> Result<Program> {
        let mut program = Vec::new();
        while self.peek_kind() != TokenKind::Eof {
            let declaration = self.declaration()?;
            program.push(declaration);
        }
        Ok(program)
    }

    fn declaration(&mut self) -> Result<Declaration> {
        Ok(match self.peek_kind() {
            TokenKind::Class => Declaration::Class(self.class_decl()?),
            TokenKind::Fun => Declaration::Fun(self.fun_decl()?),
            TokenKind::Var => Declaration::Var(self.var_decl()?),
            _ => Declaration::Statement(self.statement()?),
        })
    }

    fn class_decl(&mut self) -> Result<ClassDecl> {
        todo!()
    }

    fn fun_decl(&mut self) -> Result<FunDecl> {
        todo!()
    }

    fn var_decl(&mut self) -> Result<VarDecl> {
        let var_tok = self.expect_next(TokenKind::Var)?;
        let ident = self.ident()?;
        let init = if let Some(equal_tok) = self.match_peek(TokenKind::Equal) {
            self.lexer.next2();
            let expr = self.expression()?;
            Some(VarInit { equal_tok, expr })
        } else {
            None
        };
        let semicolon_tok = self.expect_next(TokenKind::Semicolon)?;
        Ok(VarDecl { var_tok, ident, init, semicolon_tok })
    }

    fn statement(&mut self) -> Result<Statement> {
        Ok(match self.peek_kind() {
            TokenKind::For => Statement::For(self.for_stmt()?),
            TokenKind::If => Statement::If(self.if_stmt()?),
            TokenKind::Assert => Statement::Assert(self.assert_stmt()?),
            TokenKind::Print => Statement::Print(self.print_stmt()?),
            TokenKind::Return => Statement::Return(self.return_stmt()?),
            TokenKind::While => Statement::While(self.while_stmt()?),
            TokenKind::LeftBrace => Statement::Block(self.block()?),
            _ => Statement::Expr(self.expr_stmt()?),
        })
    }

    fn for_stmt(&mut self) -> Result<ForStmt> {
        todo!()
    }

    fn if_stmt(&mut self) -> Result<IfStmt> {
        let if_tok = self.expect_next(TokenKind::If)?;
        let pred = self.expression()?;
        let body = self.block()?;
        let else_branch = if let Some(else_tok) = self.match_peek(TokenKind::Else) {
            self.lexer.next2();
            let body = self.block()?;
            Some(ElseBranch { else_tok, body })
        } else {
            None
        };
        Ok(IfStmt { if_tok, pred, body, else_branch })
    }

    fn assert_stmt(&mut self) -> Result<AssertStmt> {
        let assert_tok = self.expect_next(TokenKind::Assert)?;
        let expr = self.expression()?;
        let semicolon_tok = self.expect_next(TokenKind::Semicolon)?;
        Ok(AssertStmt { assert_tok, expr, semicolon_tok })
    }

    fn print_stmt(&mut self) -> Result<PrintStmt> {
        let print_tok = self.expect_next(TokenKind::Print)?;
        let expr = self.expression()?;
        let semicolon_tok = self.expect_next(TokenKind::Semicolon)?;
        Ok(PrintStmt { print_tok, expr, semicolon_tok })
    }

    fn return_stmt(&mut self) -> Result<ReturnStmt> {
        let return_tok = self.expect_next(TokenKind::Return)?;
        let expr = self.expression()?;
        let semicolon_tok = self.expect_next(TokenKind::Semicolon)?;
        Ok(ReturnStmt { return_tok, expr, semicolon_tok })
    }

    fn while_stmt(&mut self) -> Result<WhileStmt> {
        let while_tok = self.expect_next(TokenKind::While)?;
        let pred = self.expression()?;
        let body = self.block()?;
        Ok(WhileStmt { while_tok, pred, body })
    }

    fn block(&mut self) -> Result<Block> {
        let left_brace_tok = self.expect_next(TokenKind::LeftBrace)?;
        let mut body = Vec::new();
        while !matches!(self.peek_kind(), TokenKind::RightBrace | TokenKind::Eof) {
            let declaration = self.declaration()?;
            body.push(declaration);
        }
        let right_brace_tok = self.expect_next(TokenKind::RightBrace)?;
        Ok(Block { left_brace_tok, body, right_brace_tok })
    }

    fn expr_stmt(&mut self) -> Result<ExprStmt> {
        let expr = self.expression()?;
        let semicolon_tok = self.expect_next(TokenKind::Semicolon)?;
        Ok(ExprStmt { expr, semicolon_tok })
    }

    fn expression(&mut self) -> Result<Expression> {
        self.expr_bp(0) // start with binding power none = 0
    }

    fn expr_bp(&mut self, min_bp: u8) -> Result<Expression> {
        let mut lhs = {
            let token = self.lexer.next2();
            match token.kind {
                TokenKind::LeftParen => {
                    let left_paren_tok = token;
                    let expr = self.expr_bp(0)?;
                    let right_paren_tok = self.expect_next(TokenKind::RightParen)?;
                    Expression::Group(GroupExpr {
                        left_paren_tok,
                        expr: Box::new(expr),
                        right_paren_tok,
                    })
                }
                TokenKind::Minus |
                TokenKind::Bang => {
                    let ((), r_bp) = prefix_binding_power(token.kind);
                    let expr = self.expr_bp(r_bp)?;
                    Expression::Unary(UnaryExpr {
                        operator: token,
                        expr: Box::new(expr),
                    })
                }
                TokenKind::Nil |
                TokenKind::True |
                TokenKind::False |
                TokenKind::This |
                TokenKind::Super |
                TokenKind::Number |
                TokenKind::String |
                TokenKind::Identifier => {
                    Expression::Primary(PrimaryExpr { token })
                }
                _ => {
                    return Err(Error::ExpectedExpressionStart {
                        found: token,
                    });
                },
            }
        };

        loop {
            let operator = self.lexer.peek();
            match operator.kind {
                // infix operators
                TokenKind::Equal |
                TokenKind::Or |
                TokenKind::And |
                TokenKind::BangEqual |
                TokenKind::EqualEqual |
                TokenKind::Greater |
                TokenKind::GreaterEqual |
                TokenKind::Less |
                TokenKind::LessEqual |
                TokenKind::Minus |
                TokenKind::Plus |
                TokenKind::Slash |
                TokenKind::Star => {}
                // postfix operators
                TokenKind::LeftParen => {}
                // expression end
                TokenKind::Semicolon |
                TokenKind::RightParen |
                TokenKind::LeftBrace |
                TokenKind::Eof => {
                    break;
                }
                _ => {
                    return Err(Error::ExpectedInfixOrPostfixOperator {
                        found: operator,
                    });
                }
            }

            if let Some((l_bp, ())) = postfix_binding_power(operator.kind) {
                if l_bp < min_bp {
                    break;
                }

                todo!("parse function call");
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(operator.kind) {
                if l_bp < min_bp {
                    break;
                }

                self.lexer.next2(); // throw away the peeked operator token

                let rhs = self.expr_bp(r_bp)?;
                lhs = Expression::Binary(BinaryExpr {
                    lhs: Box::new(lhs),
                    operator,
                    rhs: Box::new(rhs),
                });

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn ident(&mut self) -> Result<Identifier> {
        let token = self.expect_next(TokenKind::Identifier)?;
        Ok(Identifier { token })
    }
}

fn prefix_binding_power(kind: TokenKind) -> ((), u8) {
    match kind {
        // unary (higher than factor for infix operators)
        TokenKind::Bang |
        TokenKind::Minus => ((), 15),

        _ => unreachable!(),
    }
}

fn postfix_binding_power(_kind: TokenKind) -> Option<(u8, ())> {
    // TODO
    None
}

fn infix_binding_power(kind: TokenKind) -> Option<(u8, u8)> {
    // left associative:  l_bp < r_bp
    // right associative: l_bp > r_bp
    Some(match kind {
        // assignment
        TokenKind::Equal        => (2, 1),
        // or
        TokenKind::Or           => (3, 4),
        // and
        TokenKind::And          => (5, 6),
        // equality
        TokenKind::EqualEqual |
        TokenKind::BangEqual    => (7, 8),
        // comparison
        TokenKind::Less |
        TokenKind::LessEqual |
        TokenKind::Greater |
        TokenKind::GreaterEqual => (9, 10),
        // term
        TokenKind::Minus |
        TokenKind::Plus         => (11, 12),
        // factor
        TokenKind::Slash |
        TokenKind::Star         => (13, 14),

        _ => return None,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fmt::SourceDebug;

    #[test]
    fn binary_expr_precedence() {
        let src = "!(5 - 4 > 3 * 2 == !nil);";
        let res = parse(src);
        match res {
            Ok(ast) => {
                let sexpr = format!("{:?}", ast[0].wrap(src));
                assert_eq!(
                    sexpr,
                    "(! (== (> (- 5 4) (* 3 2)) (! nil)))",
                );
            }
            Err(e) => {
                dbg!(e);
                panic!();
            }
        }
    }
}
