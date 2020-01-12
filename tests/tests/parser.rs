use webml::ast::{
    Declaration, DerivedDeclaration, DerivedExprKind, Expr, ExprKind, Pattern, PatternKind, Type,
    AST,
};
use webml::parse;
use webml::prim::*;

#[test]
fn parse_int() {
    let input = r#"val x = 1"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("x"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Literal {
                    value: Literal::Int(1),
                }
            },
        },])
    )
}

#[test]
fn parse_float() {
    let input = r#"val x = 1.0"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("x"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Literal {
                    value: Literal::Real(1.0),
                }
            },
        },])
    )
}

#[test]
fn parse_bool_true() {
    let input = r#"val x = true"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("x"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Constructor {
                    arg: None,
                    name: Symbol::new("true")
                }
            },
        },])
    )
}

#[test]
fn parse_bool_false() {
    let input = r#"val x = false"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("x"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Constructor {
                    arg: None,
                    name: Symbol::new("false")
                }
            },
        },])
    )
}

#[test]
fn parse_unit() {
    let input = r#"val x = ()"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("x"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Tuple { tuple: vec![] }
            }
        }])
    )
}

#[test]
fn parse_binop() {
    let input = r#"infix 6 + val x = 1 + 2"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![
            Declaration::D(DerivedDeclaration::Infix {
                priority: Some(6),
                names: vec![Symbol::new("+")],
            }),
            Declaration::Val {
                rec: false,
                pattern: Pattern {
                    ty: (),
                    inner: PatternKind::Variable {
                        name: Symbol::new("x"),
                    }
                },
                expr: Expr {
                    ty: (),
                    inner: ExprKind::App {
                        fun: Expr {
                            ty: (),
                            inner: ExprKind::Symbol {
                                name: Symbol::new("+")
                            }
                        }
                        .boxed(),
                        arg: Expr {
                            ty: (),
                            inner: ExprKind::Tuple {
                                tuple: vec![
                                    Expr {
                                        ty: (),
                                        inner: ExprKind::Literal {
                                            value: Literal::Int(1),
                                        }
                                    },
                                    Expr {
                                        ty: (),
                                        inner: ExprKind::Literal {
                                            value: Literal::Int(2),
                                        }
                                    }
                                ]
                            }
                        }
                        .boxed()
                    }
                }
            },
        ])
    )
}

#[test]
fn parse_binop_assoc() {
    let input = r#"infix 6 + val x = 1 + 2 + 3"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![
            Declaration::D(DerivedDeclaration::Infix {
                priority: Some(6),
                names: vec![Symbol::new("+")],
            }),
            Declaration::Val {
                rec: false,
                pattern: Pattern {
                    ty: (),
                    inner: PatternKind::Variable {
                        name: Symbol::new("x"),
                    }
                },
                expr: Expr {
                    ty: (),
                    inner: ExprKind::App {
                        fun: Expr {
                            ty: (),
                            inner: ExprKind::Symbol {
                                name: Symbol::new("+"),
                            }
                        }
                        .boxed(),
                        arg: Expr {
                            ty: (),
                            inner: ExprKind::Tuple {
                                tuple: vec![
                                    Expr {
                                        ty: (),
                                        inner: ExprKind::App {
                                            fun: Expr {
                                                ty: (),
                                                inner: ExprKind::Symbol {
                                                    name: Symbol::new("+"),
                                                }
                                            }
                                            .boxed(),
                                            arg: Expr {
                                                ty: (),
                                                inner: ExprKind::Tuple {
                                                    tuple: vec![
                                                        Expr {
                                                            ty: (),
                                                            inner: ExprKind::Literal {
                                                                value: Literal::Int(1),
                                                            }
                                                        },
                                                        Expr {
                                                            ty: (),
                                                            inner: ExprKind::Literal {
                                                                value: Literal::Int(2),
                                                            }
                                                        }
                                                    ]
                                                }
                                            }
                                            .boxed()
                                        }
                                    },
                                    Expr {
                                        ty: (),
                                        inner: ExprKind::Literal {
                                            value: Literal::Int(3),
                                        }
                                    }
                                ]
                            }
                        }
                        .boxed()
                    }
                }
            },
        ])
    )
}

#[test]
fn parse_uiltincall() {
    let input = r#"val ret = _builtincall "add" (x, y)"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("ret"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::BuiltinCall {
                    fun: BIF::Add,
                    args: vec![
                        Expr {
                            ty: (),
                            inner: ExprKind::Symbol {
                                name: Symbol::new("x")
                            }
                        },
                        Expr {
                            ty: (),
                            inner: ExprKind::Symbol {
                                name: Symbol::new("y")
                            }
                        }
                    ]
                }
            }
        }])
    )
}

#[test]
fn parse_binop_pref() {
    let input = r#"infix 6 + infix 7 * val x = 1 + 2 * 3"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![
            Declaration::D(DerivedDeclaration::Infix {
                priority: Some(6),
                names: vec![Symbol::new("+")],
            }),
            Declaration::D(DerivedDeclaration::Infix {
                priority: Some(7),
                names: vec![Symbol::new("*")],
            }),
            Declaration::Val {
                rec: false,
                pattern: Pattern {
                    ty: (),
                    inner: PatternKind::Variable {
                        name: Symbol::new("x"),
                    }
                },
                expr: Expr {
                    ty: (),
                    inner: ExprKind::App {
                        fun: Expr {
                            ty: (),
                            inner: ExprKind::Symbol {
                                name: Symbol::new("+")
                            }
                        }
                        .boxed(),
                        arg: Expr {
                            ty: (),
                            inner: ExprKind::Tuple {
                                tuple: vec![
                                    Expr {
                                        ty: (),
                                        inner: ExprKind::Literal {
                                            value: Literal::Int(1),
                                        }
                                    },
                                    Expr {
                                        ty: (),
                                        inner: ExprKind::App {
                                            fun: Expr {
                                                ty: (),
                                                inner: ExprKind::Symbol {
                                                    name: Symbol::new("*"),
                                                }
                                            }
                                            .boxed(),
                                            arg: Expr {
                                                ty: (),
                                                inner: ExprKind::Tuple {
                                                    tuple: vec![
                                                        Expr {
                                                            ty: (),
                                                            inner: ExprKind::Literal {
                                                                value: Literal::Int(2),
                                                            }
                                                        },
                                                        Expr {
                                                            ty: (),
                                                            inner: ExprKind::Literal {
                                                                value: Literal::Int(3),
                                                            }
                                                        }
                                                    ]
                                                }
                                            }
                                            .boxed()
                                        }
                                    }
                                ],
                            }
                        }
                        .boxed()
                    }
                }
            },
        ])
    )
}

#[test]
fn parse_fn_unary() {
    let input = r#"val f = fn x => x"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("f"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Fn {
                    param: Symbol::new("x"),
                    body: Expr {
                        ty: (),
                        inner: ExprKind::Symbol {
                            name: Symbol::new("x"),
                        }
                    }
                    .boxed(),
                }
            },
        },])
    )
}

#[test]
fn parse_datatype_single() {
    let input = r#"datatype hoge = Hoge"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Datatype {
            name: Symbol::new("hoge"),
            constructors: vec![(Symbol::new("Hoge"), None)]
        },])
    )
}

#[test]
fn parse_datatype_multi() {
    let input = r#"datatype hoge = Hoge | Fuga | Piyo"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Datatype {
            name: Symbol::new("hoge"),
            constructors: vec![
                (Symbol::new("Hoge"), None),
                (Symbol::new("Fuga"), None),
                (Symbol::new("Piyo"), None)
            ]
        },])
    )
}

#[test]
fn parse_datatype_arg1() {
    let input = r#"datatype hoge = Hoge of int | Fuga of real"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Datatype {
            name: Symbol::new("hoge"),
            constructors: vec![
                (Symbol::new("Hoge"), Some(Type::Int)),
                (Symbol::new("Fuga"), Some(Type::Real))
            ]
        },])
    )
}

#[test]
fn parse_datatype_arg2() {
    let input = r#"datatype hoge = Hoge of int | Fuga of real | Piyo of bool -> real -> int"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Datatype {
            name: Symbol::new("hoge"),
            constructors: vec![
                (Symbol::new("Hoge"), Some(Type::Int)),
                (Symbol::new("Fuga"), Some(Type::Real)),
                (
                    Symbol::new("Piyo"),
                    Some(Type::Fun(
                        Box::new(Type::Datatype(Symbol::new("bool"))),
                        Box::new(Type::Fun(Box::new(Type::Real), Box::new(Type::Int)))
                    ))
                )
            ]
        },])
    )
}

#[test]
fn parse_datatype_tuple() {
    let input = r#"datatype hoge = Hoge of int * real"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Datatype {
            name: Symbol::new("hoge"),
            constructors: vec![(
                Symbol::new("Hoge"),
                Some(Type::Tuple(vec![Type::Int, Type::Real]))
            ),]
        },])
    )
}

#[test]
fn parse_datatype_arg3() {
    let input =
        r#"datatype hoge = Hoge of int | Fuga of real | Piyo of bool -> (real -> int) * real"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Datatype {
            name: Symbol::new("hoge"),
            constructors: vec![
                (Symbol::new("Hoge"), Some(Type::Int)),
                (Symbol::new("Fuga"), Some(Type::Real)),
                (
                    Symbol::new("Piyo"),
                    Some(Type::Fun(
                        Box::new(Type::Datatype(Symbol::new("bool"))),
                        Box::new(Type::Tuple(vec![
                            Type::Fun(Box::new(Type::Real), Box::new(Type::Int)),
                            Type::Real
                        ]))
                    ))
                )
            ]
        },])
    )
}

#[test]
fn parse_fun_unary() {
    let input = r#"fun f x = x"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::D(DerivedDeclaration::Fun {
            name: Symbol::new("f"),
            clauses: vec![(
                vec![Pattern {
                    ty: (),
                    inner: PatternKind::Variable {
                        name: Symbol::new("x"),
                    }
                }],
                Expr {
                    ty: (),
                    inner: ExprKind::Symbol {
                        name: Symbol::new("x"),
                    }
                }
            )]
        }),])
    )
}

#[test]
fn parse_fun_binary() {
    let input = r#"fun f x y = x"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::D(DerivedDeclaration::Fun {
            name: Symbol::new("f"),
            clauses: vec![(
                vec![
                    Pattern {
                        ty: (),
                        inner: PatternKind::Variable {
                            name: Symbol::new("x"),
                        }
                    },
                    Pattern {
                        ty: (),
                        inner: PatternKind::Variable {
                            name: Symbol::new("y"),
                        }
                    }
                ],
                Expr {
                    ty: (),
                    inner: ExprKind::Symbol {
                        name: Symbol::new("x"),
                    }
                }
            )]
        }),])
    )
}

#[test]
fn parse_fun_pattern() {
    let input = r#"fun f (x, y) = x"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::D(DerivedDeclaration::Fun {
            name: Symbol::new("f"),
            clauses: vec![(
                vec![Pattern {
                    ty: (),
                    inner: PatternKind::Tuple {
                        tuple: vec![
                            Pattern {
                                ty: (),
                                inner: PatternKind::Variable {
                                    name: Symbol::new("x"),
                                }
                            },
                            Pattern {
                                ty: (),
                                inner: PatternKind::Variable {
                                    name: Symbol::new("y"),
                                }
                            },
                        ]
                    }
                }],
                Expr {
                    ty: (),
                    inner: ExprKind::Symbol {
                        name: Symbol::new("x"),
                    }
                }
            )]
        }),])
    )
}

#[test]
fn parse_fun_op() {
    let input = r#"fun op+(x, y) = x"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::D(DerivedDeclaration::Fun {
            name: Symbol::new("+"),
            clauses: vec![(
                vec![Pattern {
                    ty: (),
                    inner: PatternKind::Tuple {
                        tuple: vec![
                            Pattern {
                                ty: (),
                                inner: PatternKind::Variable {
                                    name: Symbol::new("x"),
                                }
                            },
                            Pattern {
                                ty: (),
                                inner: PatternKind::Variable {
                                    name: Symbol::new("y"),
                                }
                            },
                        ]
                    }
                }],
                Expr {
                    ty: (),
                    inner: ExprKind::Symbol {
                        name: Symbol::new("x"),
                    }
                }
            )]
        }),])
    )
}

#[test]
fn parse_fun_multiclause() {
    let input = r#"fun f Nil _ = Nil | f _ Nil = Nil"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::D(DerivedDeclaration::Fun {
            name: Symbol::new("f"),
            clauses: vec![
                (
                    vec![
                        Pattern {
                            ty: (),
                            inner: PatternKind::Variable {
                                name: Symbol::new("Nil"),
                            }
                        },
                        Pattern {
                            ty: (),
                            inner: PatternKind::Wildcard {}
                        }
                    ],
                    Expr {
                        ty: (),
                        inner: ExprKind::Symbol {
                            name: Symbol::new("Nil"),
                        }
                    }
                ),
                (
                    vec![
                        Pattern {
                            ty: (),
                            inner: PatternKind::Wildcard {}
                        },
                        Pattern {
                            ty: (),
                            inner: PatternKind::Variable {
                                name: Symbol::new("Nil"),
                            }
                        },
                    ],
                    Expr {
                        ty: (),
                        inner: ExprKind::Symbol {
                            name: Symbol::new("Nil"),
                        }
                    }
                )
            ]
        }),])
    )
}

#[test]
fn parse_fun_multiclause_different_fnname() {
    let input = r#"fun f Nil _ = Nil | g _ Nil = Nil"#;
    let ast = parse(input);
    assert!(ast.is_err())
}

#[test]
fn parse_if() {
    let input = r#"val x = if true then false else true"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("x"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::D(DerivedExprKind::If {
                    cond: Expr {
                        ty: (),
                        inner: ExprKind::Constructor {
                            arg: None,
                            name: Symbol::new("true")
                        }
                    }
                    .boxed(),
                    then: Expr {
                        ty: (),
                        inner: ExprKind::Constructor {
                            arg: None,
                            name: Symbol::new("false")
                        }
                    }
                    .boxed(),
                    else_: Expr {
                        ty: (),
                        inner: ExprKind::Constructor {
                            arg: None,
                            name: Symbol::new("true")
                        }
                    }
                    .boxed(),
                })
            },
        },])
    )
}

#[test]
fn parse_case_bool() {
    let input = r#"val x = case true of true => false | false => true"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("x"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Case {
                    cond: Expr {
                        ty: (),
                        inner: ExprKind::Constructor {
                            arg: None,
                            name: Symbol::new("true")
                        }
                    }
                    .boxed(),
                    clauses: vec![
                        (
                            Pattern {
                                ty: (),
                                inner: PatternKind::Constructor {
                                    arg: None,
                                    name: Symbol::new("true")
                                }
                            },
                            Expr {
                                ty: (),
                                inner: ExprKind::Constructor {
                                    arg: None,
                                    name: Symbol::new("false")
                                }
                            },
                        ),
                        (
                            Pattern {
                                ty: (),
                                inner: PatternKind::Constructor {
                                    arg: None,
                                    name: Symbol::new("false"),
                                }
                            },
                            Expr {
                                ty: (),
                                inner: ExprKind::Constructor {
                                    name: Symbol::new("true"),
                                    arg: None,
                                }
                            },
                        ),
                    ],
                }
            },
        },])
    )
}

#[test]
fn parse_case_constructor() {
    let input = r#"val x = case NONE of SOME x => false | NONE => true"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("x"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Case {
                    cond: Expr {
                        ty: (),
                        inner: ExprKind::Symbol {
                            name: Symbol::new("NONE")
                        }
                    }
                    .boxed(),
                    clauses: vec![
                        (
                            Pattern {
                                ty: (),
                                inner: PatternKind::Constructor {
                                    name: Symbol::new("SOME"),
                                    arg: Some(Box::new(Pattern {
                                        ty: (),
                                        inner: PatternKind::Variable {
                                            name: Symbol::new("x"),
                                        }
                                    })),
                                }
                            },
                            Expr {
                                ty: (),
                                inner: ExprKind::Constructor {
                                    arg: None,
                                    name: Symbol::new("false")
                                }
                            },
                        ),
                        (
                            Pattern {
                                ty: (),
                                inner: PatternKind::Variable {
                                    name: Symbol::new("NONE"),
                                }
                            },
                            Expr {
                                ty: (),
                                inner: ExprKind::Constructor {
                                    name: Symbol::new("true"),
                                    arg: None,
                                }
                            },
                        ),
                    ],
                }
            },
        },])
    )
}

#[test]
fn parse_case_var() {
    let input = r#"val x = case true of true => false | x => true"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("x"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Case {
                    cond: Expr {
                        ty: (),
                        inner: ExprKind::Constructor {
                            arg: None,
                            name: Symbol::new("true")
                        }
                    }
                    .boxed(),
                    clauses: vec![
                        (
                            Pattern {
                                ty: (),
                                inner: PatternKind::Constructor {
                                    arg: None,
                                    name: Symbol::new("true")
                                }
                            },
                            Expr {
                                ty: (),
                                inner: ExprKind::Constructor {
                                    arg: None,
                                    name: Symbol::new("false")
                                }
                            },
                        ),
                        (
                            Pattern {
                                ty: (),
                                inner: PatternKind::Variable {
                                    name: Symbol::new("x"),
                                }
                            },
                            Expr {
                                ty: (),
                                inner: ExprKind::Constructor {
                                    arg: None,
                                    name: Symbol::new("true")
                                }
                            },
                        ),
                    ],
                }
            },
        },])
    )
}

#[test]
fn parse_case_wildcard() {
    let input = r#"val x = case true of true => false | _ => true"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("x"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Case {
                    cond: Expr {
                        ty: (),
                        inner: ExprKind::Constructor {
                            arg: None,
                            name: Symbol::new("true")
                        }
                    }
                    .boxed(),
                    clauses: vec![
                        (
                            Pattern {
                                ty: (),
                                inner: PatternKind::Constructor {
                                    arg: None,
                                    name: Symbol::new("true")
                                }
                            },
                            Expr {
                                ty: (),
                                inner: ExprKind::Constructor {
                                    arg: None,
                                    name: Symbol::new("false")
                                }
                            },
                        ),
                        (
                            Pattern {
                                ty: (),
                                inner: PatternKind::Wildcard {}
                            },
                            Expr {
                                ty: (),
                                inner: ExprKind::Constructor {
                                    arg: None,
                                    name: Symbol::new("true")
                                }
                            },
                        ),
                    ],
                }
            },
        },])
    )
}

#[test]
fn parse_case_int() {
    let input = r#"val x = case 3 of 1 => 1 | 2 => 2 | _ => 10"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("x"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Case {
                    cond: Expr {
                        ty: (),
                        inner: ExprKind::Literal {
                            value: Literal::Int(3),
                        }
                    }
                    .boxed(),
                    clauses: vec![
                        (
                            Pattern {
                                ty: (),
                                inner: PatternKind::Constant { value: 1 }
                            },
                            Expr {
                                ty: (),
                                inner: ExprKind::Literal {
                                    value: Literal::Int(1),
                                }
                            },
                        ),
                        (
                            Pattern {
                                ty: (),
                                inner: PatternKind::Constant { value: 2 }
                            },
                            Expr {
                                ty: (),
                                inner: ExprKind::Literal {
                                    value: Literal::Int(2),
                                }
                            },
                        ),
                        (
                            Pattern {
                                ty: (),
                                inner: PatternKind::Wildcard {}
                            },
                            Expr {
                                ty: (),
                                inner: ExprKind::Literal {
                                    value: Literal::Int(10),
                                }
                            },
                        ),
                    ],
                }
            },
        },])
    )
}

#[test]
fn parse_case_tuple() {
    let input = r#"val x = case (1, 2, 3) of (x, y, z) => z"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Variable {
                    name: Symbol::new("x"),
                }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Case {
                    cond: Expr {
                        ty: (),
                        inner: ExprKind::Tuple {
                            tuple: vec![
                                Expr {
                                    ty: (),
                                    inner: ExprKind::Literal {
                                        value: Literal::Int(1),
                                    }
                                },
                                Expr {
                                    ty: (),
                                    inner: ExprKind::Literal {
                                        value: Literal::Int(2),
                                    }
                                },
                                Expr {
                                    ty: (),
                                    inner: ExprKind::Literal {
                                        value: Literal::Int(3),
                                    }
                                },
                            ],
                        }
                    }
                    .boxed(),
                    clauses: vec![(
                        Pattern {
                            ty: (),
                            inner: PatternKind::Tuple {
                                tuple: vec![
                                    Pattern {
                                        ty: (),
                                        inner: PatternKind::Variable {
                                            name: Symbol::new("x"),
                                        }
                                    },
                                    Pattern {
                                        ty: (),
                                        inner: PatternKind::Variable {
                                            name: Symbol::new("y"),
                                        }
                                    },
                                    Pattern {
                                        ty: (),
                                        inner: PatternKind::Variable {
                                            name: Symbol::new("z"),
                                        }
                                    },
                                ],
                            }
                        },
                        Expr {
                            ty: (),
                            inner: ExprKind::Symbol {
                                name: Symbol::new("z"),
                            }
                        },
                    ),],
                }
            },
        },])
    )
}

#[test]
fn parse_pattern_unit() {
    let input = r#"val () = ()"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Tuple { tuple: vec![] }
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Tuple { tuple: vec![] }
            }
        }])
    )
}

#[test]
fn parse_case_val_pattern_wildcard() {
    let input = r#"val _ = 1"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::Val {
            rec: false,
            pattern: Pattern {
                ty: (),
                inner: PatternKind::Wildcard {}
            },
            expr: Expr {
                ty: (),
                inner: ExprKind::Literal {
                    value: Literal::Int(1),
                }
            },
        },])
    )
}

#[test]
fn parse_funarg_pattern() {
    let input = r#"fun xor (SOME _) (SOME _) = NONE | xor NONE (SOME x) = SOME x | xor (SOME x) NONE = SOME x | xor NONE NONE = NONE"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![Declaration::D(DerivedDeclaration::Fun {
            name: Symbol::new("xor"),
            clauses: vec![
                (
                    vec![
                        Pattern {
                            ty: (),
                            inner: PatternKind::Constructor {
                                name: Symbol::new("SOME"),
                                arg: Some(Box::new(Pattern {
                                    ty: (),
                                    inner: PatternKind::Wildcard {}
                                }))
                            }
                        },
                        Pattern {
                            ty: (),
                            inner: PatternKind::Constructor {
                                name: Symbol::new("SOME"),
                                arg: Some(Box::new(Pattern {
                                    ty: (),
                                    inner: PatternKind::Wildcard {}
                                }))
                            }
                        }
                    ],
                    Expr {
                        ty: (),
                        inner: ExprKind::Symbol {
                            name: Symbol::new("NONE"),
                        }
                    }
                ),
                (
                    vec![
                        Pattern {
                            ty: (),
                            inner: PatternKind::Variable {
                                name: Symbol::new("NONE"),
                            }
                        },
                        Pattern {
                            ty: (),
                            inner: PatternKind::Constructor {
                                name: Symbol::new("SOME"),
                                arg: Some(Box::new(Pattern {
                                    ty: (),
                                    inner: PatternKind::Variable {
                                        name: Symbol::new("x")
                                    }
                                }))
                            }
                        }
                    ],
                    Expr {
                        ty: (),
                        inner: ExprKind::App {
                            fun: Expr {
                                ty: (),
                                inner: ExprKind::Symbol {
                                    name: Symbol::new("SOME")
                                }
                            }
                            .boxed(),
                            arg: Expr {
                                ty: (),
                                inner: ExprKind::Symbol {
                                    name: Symbol::new("x")
                                }
                            }
                            .boxed(),
                        }
                    }
                ),
                (
                    vec![
                        Pattern {
                            ty: (),
                            inner: PatternKind::Constructor {
                                name: Symbol::new("SOME"),
                                arg: Some(Box::new(Pattern {
                                    ty: (),
                                    inner: PatternKind::Variable {
                                        name: Symbol::new("x")
                                    }
                                }))
                            }
                        },
                        Pattern {
                            ty: (),
                            inner: PatternKind::Variable {
                                name: Symbol::new("NONE"),
                            }
                        },
                    ],
                    Expr {
                        ty: (),
                        inner: ExprKind::App {
                            fun: Expr {
                                ty: (),
                                inner: ExprKind::Symbol {
                                    name: Symbol::new("SOME")
                                }
                            }
                            .boxed(),
                            arg: Expr {
                                ty: (),
                                inner: ExprKind::Symbol {
                                    name: Symbol::new("x")
                                }
                            }
                            .boxed(),
                        }
                    }
                ),
                (
                    vec![
                        Pattern {
                            ty: (),
                            inner: PatternKind::Variable {
                                name: Symbol::new("NONE"),
                            }
                        },
                        Pattern {
                            ty: (),
                            inner: PatternKind::Variable {
                                name: Symbol::new("NONE"),
                            }
                        },
                    ],
                    Expr {
                        ty: (),
                        inner: ExprKind::Symbol {
                            name: Symbol::new("NONE"),
                        }
                    }
                )
            ]
        })])
    )
}

#[test]
fn parse_multistatement_val_datatype() {
    let input = r#"val version = 1 datatype order = GREATER | EQUAL | LESS"#;
    let ast = parse(input).unwrap();
    assert_eq!(
        ast,
        AST(vec![
            Declaration::Val {
                rec: false,
                pattern: Pattern {
                    ty: (),
                    inner: PatternKind::Variable {
                        name: Symbol::new("version")
                    }
                },
                expr: Expr {
                    ty: (),
                    inner: ExprKind::Literal {
                        value: Literal::Int(1)
                    }
                }
            },
            Declaration::Datatype {
                name: Symbol::new("order"),
                constructors: vec![
                    (Symbol::new("GREATER"), None),
                    (Symbol::new("EQUAL"), None),
                    (Symbol::new("LESS"), None),
                ]
            }
        ])
    )
}
