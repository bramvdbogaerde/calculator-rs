%start Expr
%left "PLUS" "MIN"
%left "MUL" "DIV"
%%
Expr -> Result<Expr, ()>: 
   "INT" { 
      let v = $1.map_err(|_| ())?;
      Ok(Expr::number($lexer.lexeme_str(&v))) 
   } |
   Addition {
      $1
   } |
   Multiplication {
      $1
   } |
   Division {
      $1
   } |
   Subtraction {
      $1
   } |
   "LPAR" Expr "RPAR" {
      $2
   }
   ;

Addition -> Result<Expr, ()>:
   Expr "PLUS" Expr {
      let e1 = $1?;
      let e2 = $3?;
      Ok(Expr::AddExp(Box::new(e1), Box::new(e2)))
   };

Multiplication -> Result<Expr, ()>:
   Expr "MUL" Expr {
      let e1 = $1?;
      let e2 = $3?;
      Ok(Expr::MulExp(Box::new(e1), Box::new(e2)))
   };

Division -> Result<Expr, ()>:
   Expr "DIV" Expr {
      let e1 = $1?;
      let e2 = $3?;
      Ok(Expr::DivExp(Box::new(e1), Box::new(e2)))
   } ;

Subtraction -> Result<Expr, ()>:
   Expr "MIN" Expr {
      let e1 = $1?;
      let e2 = $3?;
      Ok(Expr::MinusExp(Box::new(e1), Box::new(e2)))
   } ;
%%
use crate::ast::{Expr};
