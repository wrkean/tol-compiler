use std::fmt::{Arguments, Write};

use crate::ast::{
    Param,
    expr::{Expr, ExprKind},
    stmt::{Stmt, StmtKind},
};

/// A pretty printer for AST nodes
pub struct ASTPrettyPrinter<'ast> {
    input_ast: &'ast [Stmt], // Whole ast tree
    indent: usize,
    indent_size: u8,
    buffer: String,
}

impl<'ast> ASTPrettyPrinter<'ast> {
    pub fn new(input_ast: &'ast [Stmt], indent_size: u8) -> Self {
        Self {
            input_ast,
            indent: 0,
            indent_size,
            buffer: String::new(),
        }
    }

    pub fn run(&mut self) {
        for statement in self.input_ast.iter() {
            self.pretty_statement(statement);
        }

        println!("{}", self.buffer);
    }

    pub fn pretty_statement(&mut self, statement: &Stmt) {
        let mut buffer = String::new();
        self.pretty_statement_buf(&mut buffer, statement);
        write!(self.buffer, "{}", buffer);
    }

    fn pretty_statement_buf(&mut self, buf: &mut impl Write, statement: &Stmt) {
        match statement.kind() {
            StmtKind::NameDeclaration {
                is_mutable,
                name,
                ty,
                rhs,
            } => {
                self.writeln_buf(buf, format_args!("Name Declaration {{"));
                self.indent();
                self.writeln_buf(buf, format_args!("is mutable: {}", is_mutable));
                self.writeln_buf(buf, format_args!("name: {}", name.lexeme()));
                self.writeln_buf(buf, format_args!("type: {:?}", ty));
                let rhs = self.pretty_expression(rhs);
                self.writeln_buf(buf, format_args!("rhs: {}", rhs));
                self.dedent();
                self.writeln_buf(buf, format_args!("}}"));
            }
            StmtKind::Expression { expr } => {
                let expr = self.pretty_expression(expr);
                self.writeln_buf(buf, format_args!("Expression Statement ({})", expr));
            }
            StmtKind::FunctionDeclaration {
                name,
                params,
                ret_ty,
                block,
            } => {
                self.writeln_buf(buf, format_args!("Function Declaration {{"));
                self.indent();
                self.writeln_buf(buf, format_args!("name: {}", name.lexeme()));
                self.writeln_buf(buf, format_args!("return type: {:?}", ret_ty));
                let params = self.pretty_params(params.item());
                self.writeln_buf(buf, format_args!("params: {}", params));
                let block = self.pretty_expression(block);
                self.writeln_buf(buf, format_args!("block: {}", block));
                self.dedent();
                self.writeln_buf(buf, format_args!("}}"));
            }
        }
    }

    fn pretty_expression(&mut self, expression: &Expr) -> String {
        let mut expr_buf = String::new();
        match expression.kind() {
            ExprKind::Integer(token) => {
                write!(expr_buf, "Integer ({})", token.lexeme());
            }
            ExprKind::Float(token) => {
                write!(expr_buf, "Float ({})", token.lexeme());
            }
            ExprKind::Identifier(token) => {
                write!(expr_buf, "Identifier ({})", token.lexeme());
            }
            ExprKind::Binary { lhs, rhs, op } => {
                writeln!(&mut expr_buf, "Binary {{");
                self.indent();
                let lhs = self.pretty_expression(lhs);
                self.writeln_buf(&mut expr_buf, format_args!("lhs: {}", lhs));
                let rhs = self.pretty_expression(rhs);
                self.writeln_buf(&mut expr_buf, format_args!("rhs: {}", rhs));
                self.writeln_buf(&mut expr_buf, format_args!("op: {}", op.lexeme()));
                self.dedent();
                write!(&mut expr_buf, "{}}}", " ".repeat(self.indent));
            }
            ExprKind::Block { statements } => {
                writeln!(&mut expr_buf, "Block {{");
                self.indent();
                self.writeln_buf(&mut expr_buf, format_args!("Statements: ["));
                self.indent();
                for (i, statement) in statements.iter().enumerate() {
                    self.pretty_statement_buf(&mut expr_buf, statement);
                }
                self.dedent();
                self.writeln_buf(&mut expr_buf, format_args!("]"));
                self.dedent();
                write!(&mut expr_buf, "{}}}", " ".repeat(self.indent));
            }
        };

        expr_buf
    }

    fn pretty_params(&mut self, params: &[Param]) -> String {
        let mut params_buf = String::new();

        writeln!(params_buf, "[");
        self.indent();
        for (i, param) in params.iter().enumerate() {
            let param = self.pretty_param(param);
            self.writeln_buf(&mut params_buf, format_args!("{}", param));
        }
        self.dedent();
        write!(&mut params_buf, "{}]", " ".repeat(self.indent));

        params_buf
    }

    fn pretty_param(&mut self, param: &Param) -> String {
        let mut param_buf = String::new();

        writeln!(param_buf, "Param {{");
        self.indent();
        self.writeln_buf(
            &mut param_buf,
            format_args!("name: {}", param.name().lexeme()),
        );
        self.writeln_buf(&mut param_buf, format_args!("ty: {:?}", param.ty()));
        self.writeln_buf(&mut param_buf, format_args!("span: {:?}", param.span()));
        self.dedent();
        self.writeln_buf(&mut param_buf, format_args!("}}"));

        param_buf
    }

    pub fn indent(&mut self) {
        self.indent += self.indent_size as usize;
    }

    pub fn dedent(&mut self) {
        self.indent -= self.indent_size as usize;
    }

    pub fn writeln(&mut self, fmt: Arguments) {
        write!(self.buffer, "{}", " ".repeat(self.indent));
        writeln!(self.buffer, "{}", fmt);
    }

    pub fn writeln_buf(&self, buf: &mut impl Write, fmt: Arguments) {
        write!(buf, "{}", " ".repeat(self.indent));
        writeln!(buf, "{}", fmt);
    }
}
