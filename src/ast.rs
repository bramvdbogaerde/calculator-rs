#[derive(Debug)]
pub enum Expr {
    NumExp(usize),
    AddExp(Box<Expr>, Box<Expr>),
    MulExp(Box<Expr>, Box<Expr>),
    DivExp(Box<Expr>, Box<Expr>),
    MinusExp(Box<Expr>, Box<Expr>)
}

impl Expr {
    pub fn number<S: Into<String>>(num: S) -> Expr {
        let parsed = num.into().parse::<usize>().unwrap();
        Expr::NumExp(parsed)
    }
}
