mod expr;
mod lexer;
mod parser;

#[cfg(test)]
mod tests {
    use super::*;
    use expr::{ExprContext, Expression, LeafNode, Node, NodeType};    
    
    #[test]
    fn teste1() {
        let mut expr = Expression::new();
        expr.parse_expr(String::from("5 + 4 * 2 / 16 - 1")).unwrap();
        
        match expr.eval(){
            Ok(v) => println!("result = {:?}", v),
            Err(e) => println!("An error ocurred: {:?}", e),
        }
    }

    pub fn sum(values: Vec<f64>, ctx: ExprContext) -> expr::Result<f64>{
        println!("received context: {:#?}", ctx);
        return Ok(values[0] + values[1]);
    }

    #[test]
    fn teste2() {
        let mut ctx = ExprContext::new();
        ctx.set_var(String::from("x"), 5.0);
        ctx.set_func(String::from("sum"), sum);

        let mut expr = Expression::new();
        expr.parse_expr(String::from("sum(x, 3)")).unwrap();
        expr.set_context(ctx);        
        println!("context: {:#?}", expr.context);

        match expr.eval(){
            Ok(v) => println!("result = {:?}", v),
            Err(e) => println!("An error ocurred: {:?}", e),
        }
    }

    #[test]
    fn teste3() {
        let mut expr = Expression::new();
        expr.parse_expr(String::from("(5 + 4) * (3 - 1)")).unwrap();
        
        match expr.eval(){
            Ok(v) => println!("result = {:?}", v),
            Err(e) => println!("An error ocurred: {:?}", e),
        }
    }

    #[test]
    fn teste4() {
        let mut ctx = ExprContext::new();
        ctx.set_var(String::from("x"), 5.0);
        ctx.set_var(String::from("y"), 13.0);
        ctx.set_func(String::from("sum"), sum);

        let mut expr = Expression::new();
        expr.parse_expr(String::from("sum(x, y) / 2 * 7")).unwrap();
        expr.set_context(ctx);        
        println!("context: {:#?}", expr.context);

        match expr.eval(){
            Ok(v) => println!("result = {:?}", v),
            Err(e) => println!("An error ocurred: {:?}", e),
        }
    }
}