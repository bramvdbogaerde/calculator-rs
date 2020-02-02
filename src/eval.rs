use crate::ast::Expr;

pub fn eval(e: &Expr) -> usize {
    match e {
        &Expr::NumExp(u) => u,
        &Expr::AddExp(ref e1, ref e2) => eval(e1)+eval(e2),
        &Expr::MulExp(ref e1, ref e2) => eval(e1)*eval(e2),
        &Expr::DivExp(ref e1, ref e2) => eval(e1)/eval(e2),
        &Expr::MinusExp(ref e1, ref e2) => eval(e1)-eval(e2),
        _ => unreachable!()
    }
}
