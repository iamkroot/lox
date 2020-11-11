type Symbol = String;

#[derive(Eq, PartialEq)]
pub enum Term<'t> {
    IntConst(i32),
    Var {
        name: Symbol,
    },
    Func {
        name: Symbol,
        rettype: &'t Term<'t>,
        argtypes: Vec<&'t Term<'t>>,
    },
}

impl<'t> std::fmt::Debug for Term<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::IntConst(val) => write!(f, "IntConst({})", val),
            Term::Var { name } => write!(f, "Var({})", name),
            Term::Func { name, rettype, argtypes} => write!(f, "Func({}{:?} -> {:?})", name, argtypes, rettype),
        }
    }
}
pub struct Unifier<'a> {
    pub subst: std::collections::HashMap<&'a Symbol, &'a Term<'a>>,
}

impl<'a> Unifier<'a> {
    pub fn new() -> Self {
        Self {
            subst: std::collections::HashMap::new(),
        }
    }
    pub fn unify(&mut self, type_x: &'a Term, type_y: &'a Term) -> bool {
        dbg!(&self.subst);
        match (type_x, type_y) {
            (a, b) if *a == *b => {
                return true;
            }
            (Term::Var { .. }, _) => {
                return self.unify_var(type_x, type_y);
            }
            (_, Term::Var { name: _ny }) => {
                return self.unify_var(type_y, type_x);
            }
            (
                Term::Func {
                    name: nx,
                    rettype: rtx,
                    argtypes: atx,
                },
                Term::Func {
                    name: ny,
                    rettype: rty,
                    argtypes: aty,
                },
            ) => {
                if nx != ny || atx.len() != aty.len() {
                    return false;
                }
                if !self.unify(rtx, rty) {
                    return false;
                }
                for (x, y) in atx.iter().zip(aty) {
                    if !self.unify(x, y) {
                        return false;
                    }
                }
                return true;
            }
            _ => (),
        };
        false
    }

    fn unify_var(&mut self, var: &'a Term, other: &'a Term) -> bool {
        match var {
            Term::Var { name } => {
                if let Some(&term) = self.subst.get(name) {
                    return self.unify(term, other);
                } else {
                    if let Term::Var { name: other_name } = other {
                        if let Some(&other_term) = self.subst.get(other_name) {
                            return self.unify(var, other_term);
                        }
                    }
                    if self.occurs_check(var, other) {
                        return false;
                    }
                    self.subst.insert(name, other);
                    return true;
                }
            }
            _ => panic!("Invalid term"),
        };
    }

    fn occurs_check(&self, var: &'a Term, other: &'a Term) -> bool {
        match (var, other) {
            (var, other) if *var == *other => true,
            (_, Term::Var { name: other_name }) if self.subst.get(other_name).is_some() => {
                // waiting on RFC2497 to do this in a single lookup
                self.occurs_check(var, self.subst.get(other_name).unwrap())
            }
            (
                _,
                Term::Func {
                    name: _,
                    rettype,
                    argtypes,
                },
            ) => {
                self.occurs_check(var, rettype)
                    || argtypes.iter().any(|atype| self.occurs_check(var, atype))
            }
            _ => false,
        }
    }
}
