use {
    crate::{
        ast::*,
        env::VarKind,
        ident::Ident,
        lit::{
            Lit,
            TyLit,
        },
        span::Span,
        ty::Ty,
    },
    std::{
        cell::{
            Cell,
            RefCell,
        },
        fmt::Write,
    },
};

pub trait BackendWriter {
    fn write_ty_lit(&self, string: &mut String, ty_lit: TyLit);

    fn write_ident_and_ty(&self, string: &mut String, ident: Ident, ty: &Ty) {
        match *ty {
            Ty::Void => write!(string, "void {}", ident).unwrap(),
            Ty::Bool => {
                self.write_ty_lit(string, TyLit::Bool);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Int => {
                self.write_ty_lit(string, TyLit::Int);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Float => {
                self.write_ty_lit(string, TyLit::Float);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Bvec2 => {
                self.write_ty_lit(string, TyLit::Bvec2);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Bvec3 => {
                self.write_ty_lit(string, TyLit::Bvec3);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Bvec4 => {
                self.write_ty_lit(string, TyLit::Bvec4);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Ivec2 => {
                self.write_ty_lit(string, TyLit::Ivec2);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Ivec3 => {
                self.write_ty_lit(string, TyLit::Ivec3);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Ivec4 => {
                self.write_ty_lit(string, TyLit::Ivec4);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Vec2 => {
                self.write_ty_lit(string, TyLit::Vec2);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Vec3 => {
                self.write_ty_lit(string, TyLit::Vec3);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Vec4 => {
                self.write_ty_lit(string, TyLit::Vec4);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Mat2 => {
                self.write_ty_lit(string, TyLit::Mat2);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Mat3 => {
                self.write_ty_lit(string, TyLit::Mat3);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Mat4 => {
                self.write_ty_lit(string, TyLit::Mat4);
                write!(string, " {}", ident).unwrap();
            },
            Ty::Texture2d => panic!(),
            Ty::Array { ref elem_ty, len } => {
                self.write_ident_and_ty(string, ident, elem_ty);
                write!(string, "[{}]", len).unwrap();
            }
            Ty::Struct {
                ident: struct_ident,
            } => {
                write!(string, "{} {}", struct_ident, ident).unwrap();
            }
        }   
    }
}

pub struct BlockGenerator<'a> {
    pub shader: &'a ShaderAst,
    pub backend_writer: &'a dyn BackendWriter,
    pub use_hidden_parameters: bool,
    pub use_generated_constructors: bool,
    pub indent_level: usize,
    pub string: &'a mut String,
}

impl<'a> BlockGenerator<'a> {
    pub fn generate_block(&mut self, block: &Block) {
        write!(self.string, "{{").unwrap();
        if !block.stmts.is_empty() {
            writeln!(self.string).unwrap();
            self.indent_level += 1;
            for stmt in &block.stmts {
                self.generate_stmt(stmt);
            }
            self.indent_level -= 1;
        }
        write!(self.string, "}}").unwrap()
    }

    fn generate_stmt(&mut self, stmt: &Stmt) {
        self.write_indent();
        match *stmt {
            Stmt::Break {
                span
            } => self.generate_break_stmt(span),
            Stmt::Continue {
                span
            } => self.generate_continue_stmt(span),
            Stmt::For {
                span,
                ident,
                ref from_expr,
                ref to_expr,
                ref step_expr,
                ref block,
            } => self.generate_for_stmt(span, ident, from_expr, to_expr, step_expr, block),
            Stmt::If {
                span,
                ref expr,
                ref block_if_true,
                ref block_if_false,
            } => self.generate_if_stmt(span, expr, block_if_true, block_if_false),
            Stmt::Let {
                span,
                ref ty,
                ident,
                ref ty_expr,
                ref expr,
            } => self.generate_let_stmt(span, ty, ident, ty_expr, expr),
            Stmt::Return {
                span,
                ref expr
            } => self.generate_return_stmt(span, expr),
            Stmt::Block {
                span,
                ref block
            } => self.generate_block_stmt(span, block),
            Stmt::Expr {
                span,
                ref expr
            } => self.generate_expr_stmt(span, expr),
        }
    }

    fn generate_break_stmt(&mut self, _span: Span) {
        writeln!(self.string, "break;").unwrap();
    }

    fn generate_continue_stmt(&mut self, _span: Span) {
        writeln!(self.string, "continue;").unwrap();
    }

    fn generate_for_stmt(
        &mut self,
        _span: Span,
        ident: Ident,
        from_expr: &Expr,
        to_expr: &Expr,
        step_expr: &Option<Expr>,
        block: &Block,
    ) {
        let from = from_expr.val.borrow().as_ref().unwrap().to_int().unwrap();
        let to = to_expr.val.borrow().as_ref().unwrap().to_int().unwrap();
        let step = if let Some(step_expr) = step_expr {
            step_expr.val.borrow().as_ref().unwrap().to_int().unwrap()
        } else if from < to {
            1
        } else {
            -1
        };
        write!(
            self.string,
            "for (int {0} = {1}; {0} {2} {3}; {0} {4} {5}) ",
            ident,
            if from <= to { from } else { from - 1 },
            if from <= to { "<" } else { ">=" },
            to,
            if step > 0 { "+=" } else { "-=" },
            step.abs()
        )
        .unwrap();
        self.generate_block(block);
        writeln!(self.string).unwrap();
    }

    fn generate_if_stmt(
        &mut self,
        _span: Span,
        expr: &Expr,
        block_if_true: &Block,
        block_if_false: &Option<Box<Block>>,
    ) {
        write!(self.string, "if (").unwrap();
        self.generate_expr(expr);
        write!(self.string, " ").unwrap();
        self.generate_block(block_if_true);
        if let Some(block_if_false) = block_if_false {
            self.generate_block(block_if_false);
        }
        writeln!(self.string).unwrap();
    }

    fn generate_let_stmt(
        &mut self,
        _span: Span,
        ty: &RefCell<Option<Ty>>,
        ident: Ident,
        _ty_expr: &Option<TyExpr>,
        expr: &Option<Expr>,
    ) {
        self.write_ident_and_ty(
            ident,
            ty.borrow().as_ref().unwrap()
        );
        if let Some(expr) = expr {
            write!(self.string, " = ").unwrap();
            self.generate_expr(expr);
        }
        writeln!(self.string, ";").unwrap();
    }

    fn generate_return_stmt(&mut self, _span: Span, expr: &Option<Expr>) {
        write!(self.string, "return").unwrap();
        if let Some(expr) = expr {
            write!(self.string, " ").unwrap();
            self.generate_expr(expr);
        }
        writeln!(self.string, ";").unwrap();
    }

    fn generate_block_stmt(&mut self, _span: Span, block: &Block) {
        self.generate_block(block);
        writeln!(self.string).unwrap();
    }

    fn generate_expr_stmt(&mut self, _span: Span, expr: &Expr) {
        self.generate_expr(expr);
        writeln!(self.string, ";").unwrap();
    }

    fn generate_expr(&mut self, expr: &Expr) {
        ExprGenerator {
            shader: self.shader,
            backend_writer: self.backend_writer,
            use_hidden_parameters: self.use_hidden_parameters,
            use_generated_constructors: self.use_generated_constructors,
            string: self.string,
        }
        .generate_expr(expr)
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent_level {
            write!(self.string, "    ").unwrap();
        }
    }

    fn write_ident_and_ty(&mut self, ident: Ident, ty: &Ty) {
        self.backend_writer.write_ident_and_ty(
            &mut self.string,
            ident,
            ty
        );
    }
}

pub struct ExprGenerator<'a> {
    pub shader: &'a ShaderAst,
    pub backend_writer: &'a dyn BackendWriter,
    pub use_hidden_parameters: bool,
    pub use_generated_constructors: bool,
    pub string: &'a mut String,
}

impl<'a> ExprGenerator<'a> {
    pub fn generate_expr(&mut self, expr: &Expr) {
        match expr.kind {
            ExprKind::Cond {
                span,
                ref expr,
                ref expr_if_true,
                ref expr_if_false,
            } => self.generate_cond_expr(span, expr, expr_if_true, expr_if_false),
            ExprKind::Bin {
                span,
                op,
                ref left_expr,
                ref right_expr,
            } => self.generate_bin_expr(span, op, left_expr, right_expr),
            ExprKind::Un {
                span,
                op,
                ref expr
            } => self.generate_un_expr(span, op, expr),
            ExprKind::MethodCall {
                span,
                ident,
                ref arg_exprs
            } => self.generate_method_call_expr(span, ident, arg_exprs),
            ExprKind::Field {
                span,
                ref expr,
                field_ident,
            } => self.generate_field_expr(span, expr, field_ident),
            ExprKind::Index {
                span,
                ref expr,
                ref index_expr,
            } => self.generate_index_expr(span, expr, index_expr),
            ExprKind::Call {
                span,
                ident,
                ref arg_exprs,
            } => self.generate_call_expr(span, ident, arg_exprs),
            ExprKind::MacroCall {
                ref analysis,
                span,
                ident,
                ref arg_exprs,
                ..
            } => self.generate_macro_call_expr(analysis, span, ident, arg_exprs),
            ExprKind::ConsCall {
                span,
                ty_lit,
                ref arg_exprs,
            } => self.generate_cons_call_expr(span, ty_lit, arg_exprs),
            ExprKind::Var {
                span,
                ref is_lvalue,
                ref kind,
                ident,
            } => self.generate_var_expr(span, is_lvalue, kind, ident),
            ExprKind::Lit {
                span,
                lit
            } => self.generate_lit_expr(span, lit),
        }
    }

    fn generate_cond_expr(
        &mut self,
        _span: Span,
        expr: &Expr,
        expr_if_true: &Expr,
        expr_if_false: &Expr
    ) {
        write!(self.string, "(").unwrap();
        self.generate_expr(expr);
        write!(self.string, " ? ").unwrap();
        self.generate_expr(expr_if_true);
        write!(self.string, " : ").unwrap();
        self.generate_expr(expr_if_false);
        write!(self.string, ")").unwrap();
    }

    fn generate_bin_expr(
        &mut self,
        _span: Span,
        op: BinOp,
        left_expr: &Expr,
        right_expr: &Expr
    ) {
        write!(self.string, "(").unwrap();
        self.generate_expr(left_expr);
        write!(self.string, " {} ", op).unwrap();
        self.generate_expr(right_expr);
        write!(self.string, ")").unwrap();
    }

    fn generate_un_expr(
        &mut self,
        _span: Span,
        op: UnOp,
        expr: &Expr
    ) {
        write!(self.string, "{}", op).unwrap();
        self.generate_expr(expr);
    }

    fn generate_method_call_expr(
        &mut self,
        span: Span,
        ident: Ident,
        arg_exprs: &[Expr]
    ) {
        match arg_exprs[0].ty.borrow().as_ref().unwrap() {
            Ty::Struct { ident: struct_ident } => {
                self.generate_call_expr(
                    span,
                    Ident::new(format!("mpsc__{}_{}", struct_ident, ident)),
                    arg_exprs
                );
            },
            _ => panic!()
        }
    }

    fn generate_field_expr(
        &mut self,
        _span: Span,
        expr: &Expr,
        field_ident: Ident
    ) {
        self.generate_expr(expr);
        write!(self.string, ".{}", field_ident).unwrap();
    }

    fn generate_index_expr(
        &mut self,
        _span: Span,
        expr: &Expr,
        index_expr: &Expr
    ) {
        self.generate_expr(expr);
        write!(self.string, "[").unwrap();
        self.generate_expr(index_expr);
        write!(self.string, "]").unwrap();
    }

    fn generate_call_expr(
        &mut self,
        _span: Span,
        ident: Ident,
        arg_exprs: &[Expr],
    ) {
        write!(self.string, "{}(", ident).unwrap();
        let mut sep = "";
        for arg_expr in arg_exprs {
            write!(self.string, "{}", sep).unwrap();
            self.generate_expr(arg_expr);
            sep = ", ";
        }
        if self.use_hidden_parameters {
            if let Some(decl) = self.shader.find_fn_decl(ident) {
                for &ident in decl.uniform_block_deps.borrow().as_ref().unwrap() {
                    write!(self.string, "{}mpsc_{}_uniforms", sep, ident).unwrap();
                    sep = ", ";
                }
                if decl.has_texture_deps.get().unwrap() {
                    write!(self.string, "{}mpsc_textures", sep).unwrap();
                    sep = ", ";
                }
                if decl.is_used_in_vertex_shader.get().unwrap() {
                    if !decl.attribute_deps.borrow().as_ref().unwrap().is_empty() {
                        write!(self.string, "{}mpsc_attributes", sep).unwrap();
                        sep = ", ";
                    }
                    if !decl.instance_deps.borrow().as_ref().unwrap().is_empty() {
                        write!(self.string, "{}mpsc_instances", sep).unwrap();
                        sep = ", ";
                    }
                }
                if decl.is_used_in_fragment_shader.get().unwrap() {
                    if !decl.attribute_deps.borrow().as_ref().unwrap().is_empty()
                        || decl.instance_deps.borrow().as_ref().unwrap().is_empty()
                        || decl.has_in_varying_deps.get().unwrap()
                    {
                        write!(self.string, "{}mpsc_varyings", sep).unwrap();
                    }
                }
            }
        }
        write!(self.string, ")").unwrap();
    }

    fn generate_macro_call_expr(
        &mut self,
        analysis: &Cell<Option<MacroCallAnalysis>>,
        _span: Span,
        _ident: Ident,
        _arg_exprs: &[Expr],
    ) {
        match analysis.get().unwrap() {
            MacroCallAnalysis::Color { r, g, b, a } => {
                write!(self.string, "vec4({}, {}, {}, {})", r, g, b, a).unwrap();
            }
        }
    }

    fn generate_cons_call_expr(
        &mut self,
        _span: Span,
        ty_lit: TyLit,
        arg_exprs: &[Expr]
    ) {
        if self.use_generated_constructors {
            write!(self.string, "mpsc_").unwrap();
            self.write_ty_lit(ty_lit);
            for arg_expr in arg_exprs {
                write!(self.string, "_{}", arg_expr.ty.borrow().as_ref().unwrap()).unwrap();
            }
        } else {
            self.write_ty_lit(ty_lit);
        }
        write!(self.string, "(").unwrap();
        let mut sep = "";
        for arg_expr in arg_exprs {
            write!(self.string, "{}", sep).unwrap();
            self.generate_expr(arg_expr);
            sep = ", ";
        }
        write!(self.string, ")").unwrap();
    }

    fn generate_var_expr(
        &mut self,
        _span: Span,
        _is_lvalue: &Cell<Option<bool>>,
        _kind: &Cell<Option<VarKind>>,
        ident: Ident,
    ) {
        write!(self.string, "{}", ident).unwrap()
    }

    fn generate_lit_expr(
        &mut self,
        _span: Span,
        lit: Lit
    ) {
        write!(self.string, "{}", lit).unwrap();
    }

    fn write_ty_lit(&mut self, ty_lit: TyLit) {
        self.backend_writer.write_ty_lit(&mut self.string, ty_lit);
    }
}

/*
use crate::ast::*;
use crate::env::VarKind;
use crate::ident::Ident;
use crate::lit::{Lit, TyLit};
use crate::span::Span;
use crate::swizzle::Swizzle;
use crate::ty::Ty;
use std::cell::{Cell, RefCell};
use std::fmt::Write;

#[derive(Clone, Copy, Debug)]
pub enum ShaderKind {
    Vertex,
    Pixel,
}

pub fn generate(kind: ShaderKind, shader_ast: &ShaderAst) -> String {
    let mut string = String::new();
    ShaderGenerator {
        string: &mut string,
        kind,
        shader_ast,
    }
    .generate_shader();
    string
}

#[derive(Debug)]
struct ShaderGenerator<'a> {
    string: &'a mut String,
    kind: ShaderKind,
    shader_ast: &'a ShaderAst,
}

impl<'a> ShaderGenerator<'a> {
    fn generate_shader(&mut self) {
        for decl in &self.shader_ast.decls {
            match decl {
                Decl::Struct(decl) => self.generate_struct_decl(decl),
                _ => {}
            }
        }
        for decl in &self.shader_ast.decls {
            match decl {
                Decl::Const(decl) => self.generate_const_decl(decl),
                _ => {}
            }
        }
        for decl in &self.shader_ast.decls {
            match decl {
                Decl::Uniform(decl) => self.generate_uniform_decl(decl),
                _ => {}
            }
        }
        let ident = match self.kind {
            ShaderKind::Vertex => Ident::new("vertex"),
            ShaderKind::Pixel => Ident::new("pixel"),
        };
        for (ty_lit, param_tys) in self
            .shader_ast
            .find_fn_decl(ident)
            .unwrap()
            .cons_deps
            .borrow()
            .as_ref()
            .unwrap()
        {
            self.generate_cons(*ty_lit, param_tys);
        }
        self.generate_fn_defs(match self.kind {
            ShaderKind::Vertex => Ident::new("vertex"),
            ShaderKind::Pixel => Ident::new("pixel"),
        });
        match self.kind {
            ShaderKind::Vertex => self.generate_vertex_shader(),
            ShaderKind::Pixel => self.generate_pixel_shader(),
        }
    }

    fn generate_struct_decl(&mut self, decl: &StructDecl) {
        write!(self.string, "struct {} {{", decl.ident).unwrap();
        if !decl.fields.is_empty() {
            writeln!(self.string).unwrap();
            for field in &decl.fields {
                write!(self.string, "    ").unwrap();
                write_ident_and_ty(
                    &mut self.string,
                    field.ident,
                    field.ty_expr.ty.borrow().as_ref().unwrap(),
                );
                writeln!(self.string, ";").unwrap();
            }
        }
        writeln!(self.string, "}};").unwrap();
    }

    fn generate_const_decl(&mut self, decl: &ConstDecl) {
        write!(self.string, "const ").unwrap();
        write_ident_and_ty(
            &mut self.string,
            decl.ident,
            decl.ty_expr.ty.borrow().as_ref().unwrap(),
        );
        write!(self.string, " = ").unwrap();
        self.generate_expr(&decl.expr);
        writeln!(self.string, ";").unwrap();
    }

    fn generate_uniform_decl(&mut self, decl: &UniformDecl) {
        write!(self.string, "uniform ").unwrap();
        write_ident_and_ty(
            self.string,
            decl.ident,
            decl.ty_expr.ty.borrow().as_ref().unwrap(),
        );
        writeln!(self.string, ";").unwrap();
    }

    fn generate_cons(&mut self, ty_lit: TyLit, param_tys: &[Ty]) {
        write!(self.string, "{0} mpsc_{0}", ty_lit).unwrap();
        for param_ty in param_tys {
            write!(self.string, "_{}", param_ty).unwrap();
        }
        write!(self.string, "(").unwrap();
        let mut sep = "";
        if param_tys.len() == 1 {
            write_ident_and_ty(&mut self.string, Ident::new("x"), &param_tys[0])
        } else {
            for (index, param_ty) in param_tys.iter().enumerate() {
                write!(self.string, "{}", sep).unwrap();
                write_ident_and_ty(
                    &mut self.string,
                    Ident::new(format!("x{}", index)),
                    param_ty,
                );
                sep = ", ";
            }
        }
        writeln!(self.string, ") {{").unwrap();
        write!(self.string, "    {}(", ty_lit).unwrap();
        let ty = ty_lit.to_ty();
        if param_tys.len() == 1 {
            let param_ty = &param_tys[0];
            match param_ty {
                Ty::Bool | Ty::Int | Ty::Float => {
                    let mut sep = "";
                    for _ in 0..ty.size() {
                        write!(self.string, "{}x", sep).unwrap();
                        sep = ", ";
                    }
                }
                Ty::Mat2 | Ty::Mat3 | Ty::Mat4 => {
                    let target_size = match ty {
                        Ty::Mat2 => 2,
                        Ty::Mat3 => 3,
                        Ty::Mat4 => 4,
                        _ => panic!(),
                    };
                    let source_size = match param_ty {
                        Ty::Mat2 => 2,
                        Ty::Mat3 => 3,
                        Ty::Mat4 => 4,
                        _ => panic!(),
                    };
                    let mut sep = "";
                    for column_index in 0..target_size {
                        for row_index in 0..target_size {
                            if row_index < source_size && column_index < source_size {
                                write!(self.string, "{}x[{}][{}]", sep, column_index, row_index)
                                    .unwrap();
                            } else {
                                write!(
                                    self.string,
                                    "{}{}",
                                    sep,
                                    if column_index == row_index { 1.0 } else { 0.0 }
                                )
                                .unwrap();
                            }
                            sep = ", ";
                        }
                    }
                }
                _ => panic!(),
            }
        } else {
            let mut sep = "";
            for (index_0, param_ty) in param_tys.iter().enumerate() {
                if param_ty.size() == 1 {
                    write!(self.string, "{}x{}", sep, index_0).unwrap();
                    sep = ", ";
                } else {
                    for index_1 in 0..param_ty.size() {
                        write!(self.string, "{}x{}[{}]", sep, index_0, index_1).unwrap();
                        sep = ", ";
                    }
                }
            }
        }
        writeln!(self.string, ")").unwrap();
        writeln!(self.string, "}}").unwrap();
    }

    fn generate_fn_defs(&mut self, ident: Ident) {
        let decl = self.shader_ast.find_fn_decl(ident).unwrap();
        for &callee in decl.callees.borrow().as_ref().unwrap().iter() {
            self.generate_fn_defs(callee);
        }
        FnDefGenerator {
            string: self.string,
            indent_level: 0,
            kind: self.kind,
            shader_ast: &self.shader_ast,
            decl,
        }
        .generate_fn_def()
    }

    fn generate_expr(&mut self, expr: &Expr) {
        ExprGenerator {
            string: self.string,
            kind: self.kind,
            shader_ast: self.shader_ast,
        }
        .generate_expr(expr)
    }

    fn generate_vertex_shader(&mut self) {
        let vertex_decl = self.shader_ast.find_fn_decl(Ident::new("vertex")).unwrap();
        self.generate_attributes_struct();
        self.generate_instances_struct();
        self.generate_varyings_struct();
        let total_packed_attribute_size = self.compute_total_packed_attribute_size();
        self.generate_packed_attributes(total_packed_attribute_size);
        let total_packed_varying_size = self.compute_total_packed_varying_size();
        self.generate_packed_varyings(total_packed_varying_size);
        writeln!(self.string, "void main() {{").unwrap();
        writeln!(self.string, "    mpsc_Attributes mpsc_attributes;").unwrap();
        writeln!(self.string, "    mpsc_Instances mpsc_instances;").unwrap();
        self.generate_unpack_attributes(total_packed_attribute_size);
        writeln!(self.string, "    mpsc_Varyings mpsc_varyings;").unwrap();
        write!(self.string, "    gl_Position = vertex(").unwrap();
        let mut sep = "";
        for &ident in vertex_decl.uniform_block_deps.borrow().as_ref().unwrap() {
            write!(self.string, "{}mpsc_{1}_uniforms", sep, ident).unwrap();
            sep = ", ";
        }
        if !vertex_decl
            .attribute_deps
            .borrow()
            .as_ref()
            .unwrap()
            .is_empty()
        {
            write!(self.string, "{}mpsc_attributes", sep).unwrap();
            sep = ", ";
        }
        if vertex_decl.has_out_varying_deps.get().unwrap() {
            write!(self.string, "{}mpsc_varyings", sep).unwrap();
        }
        writeln!(self.string, ");").unwrap();
        self.generate_pack_varyings(total_packed_varying_size);
        writeln!(self.string, "}}").unwrap();
    }

    fn generate_pixel_shader(&mut self) {
        let pixel_decl = self.shader_ast.find_fn_decl(Ident::new("pixel")).unwrap();
        let total_packed_varying_size = self.compute_total_packed_varying_size();
        self.generate_packed_varyings(total_packed_varying_size);
        writeln!(self.string, "void main() {{").unwrap();
        writeln!(self.string, "    mpsc_Varyings mpsc_varyings;").unwrap();
        self.generate_unpack_varyings(total_packed_varying_size);
        write!(self.string, "    gl_FragColor = pixel(").unwrap();
        let mut sep = "";
        for &ident in pixel_decl.uniform_block_deps.borrow().as_ref().unwrap() {
            write!(self.string, "{}mpsc_{1}_uniforms", sep, ident).unwrap();
            sep = ", ";
        }
        if !pixel_decl
            .attribute_deps
            .borrow()
            .as_ref()
            .unwrap()
            .is_empty()
            || pixel_decl.has_out_varying_deps.get().unwrap()
        {
            write!(self.string, "{}mpsc_varyings", sep).unwrap();
        }
        writeln!(self.string, ");").unwrap();
        writeln!(self.string, "}}").unwrap();
    }

    fn generate_attributes_struct(&mut self) {
        writeln!(self.string, "struct mpsc_Attributes {{").unwrap();
        for decl in &self.shader_ast.decls {
            match decl {
                Decl::Attribute(decl) => {
                    write!(self.string, "    ").unwrap();
                    write_ident_and_ty(
                        &mut self.string,
                        decl.ident,
                        decl.ty_expr.ty.borrow().as_ref().unwrap()
                    );
                    writeln!(self.string, ";").unwrap();
                }
                _ => {}
            }
        }
        writeln!(self.string, "}};").unwrap();
    }

    fn generate_instances_struct(&mut self) {
        writeln!(self.string, "struct mpsc_Instances {{").unwrap();
        for decl in &self.shader_ast.decls {
            match decl {
                Decl::Instance(decl) => {
                    write!(self.string, "    ").unwrap();
                    write_ident_and_ty(
                        &mut self.string,
                        decl.ident,
                        decl.ty_expr.ty.borrow().as_ref().unwrap()
                    );
                    writeln!(self.string, ";").unwrap();
                }
                _ => {}
            }
        }
        writeln!(self.string, "}};").unwrap();
    }

    fn generate_varyings_struct(&mut self) {
        let pixel_decl = self.shader_ast.find_fn_decl(Ident::new("pixel")).unwrap();
        writeln!(self.string, "struct mpsc_Varyings {{").unwrap();
        for decl in &self.shader_ast.decls {
            match decl {
                Decl::Attribute(decl)
                    if pixel_decl
                        .attribute_deps
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .contains(&decl.ident) =>
                {
                    write!(self.string, "    ").unwrap();
                    write_ident_and_ty(
                        &mut self.string,
                        decl.ident,
                        decl.ty_expr.ty.borrow().as_ref().unwrap()
                    );
                    writeln!(self.string, ";").unwrap();
                }
                Decl::Instance(decl)
                    if pixel_decl
                        .instance_deps
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .contains(&decl.ident) =>
                {
                    write!(self.string, "    ").unwrap();
                    write_ident_and_ty(
                        &mut self.string,
                        decl.ident,
                        decl.ty_expr.ty.borrow().as_ref().unwrap()
                    );
                    writeln!(self.string, ";").unwrap();
                }
                Decl::Varying(decl) => {
                    write!(self.string, "    ").unwrap();
                    write_ident_and_ty(
                        &mut self.string,
                        decl.ident,
                        decl.ty_expr.ty.borrow().as_ref().unwrap()
                    );
                    writeln!(self.string, ";").unwrap();
                }
                _ => {}
            }
        }
        writeln!(self.string, "}};").unwrap();
    }
}

#[derive(Debug)]
struct FnDefGenerator<'a> {
    string: &'a mut String,
    indent_level: usize,
    kind: ShaderKind,
    shader_ast: &'a ShaderAst,
    decl: &'a FnDecl,
}

impl<'a> FnDefGenerator<'a> {
    fn generate_fn_def(&mut self) {
        write_ident_and_ty(
            &mut self.string,
            self.decl.ident,
            self.decl.return_ty.borrow().as_ref().unwrap(),
        );
        write!(self.string, "(").unwrap();
        let mut sep = "";
        for param in &self.decl.params {
            write!(self.string, "{}", sep).unwrap();
            write_ident_and_ty(
                &mut self.string,
                param.ident,
                param.ty_expr.ty.borrow().as_ref().unwrap(),
            );
            sep = ", ";
        }
        for &ident in self.decl.uniform_block_deps.borrow().as_ref().unwrap() {
            write!(
                self.string,
                "{}mpsc_{1}_Uniforms mpsc_{1}_uniforms",
                sep, ident
            )
            .unwrap();
            sep = ", ";
        }
        match self.kind {
            ShaderKind::Vertex => {
                if !self
                    .decl
                    .attribute_deps
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .is_empty()
                {
                    write!(self.string, "{}mpsc_Attributes mpsc_attributes", sep).unwrap();
                    sep = ", ";
                }
                if self.decl.has_out_varying_deps.get().unwrap() {
                    write!(self.string, "{}out mpsc_Varyings mpsc_varyings", sep).unwrap();
                }
            }
            ShaderKind::Pixel => {
                if !self
                    .decl
                    .attribute_deps
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .is_empty()
                    || self.decl.has_in_varying_deps.get().unwrap()
                {
                    write!(self.string, "{}mpsc_Varyings mpsc_varyings", sep).unwrap();
                }
            }
        }
        write!(self.string, ") ").unwrap();
        self.generate_block(&self.decl.block);
        writeln!(self.string).unwrap();
    }
*/