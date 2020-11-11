use lox_typeinf::{Term, Unifier};
#[allow(non_snake_case)]
#[allow(unused_variables)]

fn main() {
    let mut unif = Unifier::new();
    let X = Term::Var{name: String::from("X")};
    let Y = Term::Var{name: String::from("Y")};
    let Z = Term::Var{name: String::from("Z")};
    let f = Term::Func{
        name: String::from("foo"),
        rettype: &X,
        argtypes: vec![&Term::IntConst(5), &Z]
    };
    let b = Term::Func{
        name: String::from("foo"),
        rettype: &Y,
        argtypes: vec![&X, &X]
    };
    let M = Term::Var{name: String::from("M")};
    let res =unif.unify(&f,&b);
    println!("{}", res);
    println!("{:?}", unif.subst);
}
