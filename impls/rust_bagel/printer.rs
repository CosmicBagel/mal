use crate::types::AST;

pub fn pr_str(ast: AST) {
    /*
       take a mal data structure and return a string representation of it. But pr_str is much
       simpler and is basically just a switch statement on the type of the input object:
         - symbol: return the string name of the symbol
         - number: return the number as a string
         - list: iterate through each element of the list calling pr_str on it, then join the results with a space separator, and surround the final result with parens
    */
}
