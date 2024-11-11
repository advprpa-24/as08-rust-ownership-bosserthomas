use crate::term::*;
use std::fmt::{self};

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    // TODO: Implement pretty printing for lambda calculus terms.
    match term {
        Term::Var(x) => format!("{x}"),
        Term::Abs(param, body) => format!("λ{}. ({})", param, pretty_print(body)),
        Term::App(t1, t2) => format!("({}) ({})", pretty_print(t1), pretty_print(t2))
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_pretty_print() {
        let var = Term::Var("x".to_string());
        let single_abstraction = Term::Abs("y".to_string(), Box::new(var.clone()));
        let complex_abstraction = Term::Abs("z".to_string(), Box::new(single_abstraction.clone()));
        let application = Term::App(Box::new(complex_abstraction.clone()), Box::new(single_abstraction.clone()));
        assert_eq!(pretty_print(&var), "x".to_string());
        assert_eq!(pretty_print(&single_abstraction), "λy. (x)".to_string());
        assert_eq!(pretty_print(&complex_abstraction), "λz. (λy. (x))".to_string());
        assert_eq!(pretty_print(&application), "(λz. (λy. (x))) (λy. (x))".to_string());
    }
}