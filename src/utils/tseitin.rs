use super::*;

// Optimized Tseitin encoding as proposed on page 13
pub fn tseitin(pred: &Predicate) -> Vec<Predicate> {
    let mut auxes = vec![];
    let mut cnf = vec![];
    fn recursive(pred: &Predicate, auxes: &mut Vec<String>, cnf: &mut Vec<Predicate>) -> () {
        match pred {
            Predicate::ATOM(_) => cnf.push(Predicate::OR(vec![pred.clone()])),

            Predicate::NOT(p) => match &**p {
                Predicate::ATOM(x) => {
                    if auxes.len() == 0 {
                        cnf.push(Predicate::OR(vec![Predicate::NOT(Box::new(
                            Predicate::ATOM(x.clone()),
                        ))]));
                    } else {
                        let aux_atom_0 = Predicate::ATOM(auxes.last().unwrap().clone());
                        cnf.push(Predicate::OR(vec![
                            Predicate::NOT(Box::new(aux_atom_0.clone())),
                            Predicate::NOT(Box::new(Predicate::ATOM(x.clone()))),
                        ]));
                        cnf.push(Predicate::OR(vec![aux_atom_0, Predicate::ATOM(x.clone())]));
                    }
                }
                _ => {
                    if auxes.len() == 0 {
                        let aux_atom_0 = Predicate::ATOM(format!("$aux{}", auxes.len()));
                        auxes.push(format!("$aux{}", auxes.len()));
                        let aux_atom_1 = Predicate::ATOM(format!("$aux{}", auxes.len()));
                        auxes.push(format!("$aux{}", auxes.len()));
                        cnf.push(Predicate::OR(vec![aux_atom_0.clone()]));
                        cnf.push(Predicate::OR(vec![
                            Predicate::NOT(Box::new(aux_atom_0.clone())),
                            Predicate::NOT(Box::new(aux_atom_1.clone())),
                        ]));
                        cnf.push(Predicate::OR(vec![aux_atom_0, aux_atom_1]));
                    } else {
                        let aux_atom_0 = Predicate::ATOM(auxes.last().unwrap().clone());
                        let aux_atom_1 = Predicate::ATOM(format!("$aux{}", auxes.len()));
                        auxes.push(format!("$aux{}", auxes.len()));
                        cnf.push(Predicate::OR(vec![
                            Predicate::NOT(Box::new(aux_atom_0.clone())),
                            Predicate::NOT(Box::new(aux_atom_1.clone())),
                        ]));
                        cnf.push(Predicate::OR(vec![aux_atom_0, aux_atom_1]));
                    }
                    recursive(&p, auxes, cnf)
                }
            },

            Predicate::AND(p) => match p.len() {
                0 => (),
                1 => recursive(&p[0], auxes, cnf),
                _ => {
                    if auxes.len() == 0 {
                        let aux_atom_0 = Predicate::ATOM(format!("$aux{}", auxes.len()));
                        auxes.push(format!("$aux{}", auxes.len()));
                        cnf.push(Predicate::OR(vec![aux_atom_0.clone()]));
                        let mut disj = vec![];
                        for z in p {
                            match &z {
                                Predicate::ATOM(x) => {
                                    cnf.push(Predicate::OR(vec![
                                        Predicate::NOT(Box::new(aux_atom_0.clone())),
                                        Predicate::ATOM(x.clone()),
                                    ]));
                                    disj.push(Predicate::NOT(Box::new(Predicate::ATOM(x.clone()))));
                                }
                                _ => {
                                    let aux_atom_1 =
                                        Predicate::ATOM(format!("$aux{}", auxes.len()));
                                    auxes.push(format!("$aux{}", auxes.len()));
                                    cnf.push(Predicate::OR(vec![
                                        Predicate::NOT(Box::new(aux_atom_0.clone())),
                                        aux_atom_1.clone(),
                                    ]));
                                    disj.push(Predicate::NOT(Box::new(aux_atom_1.clone())));
                                    recursive(&z, auxes, cnf)
                                }
                            }
                        }
                        let mut total_disj_vec = vec![];
                        total_disj_vec.push(aux_atom_0);
                        for d in disj {
                            total_disj_vec.push(d)
                        }
                        cnf.push(Predicate::OR(total_disj_vec));
                    } else {
                        let aux_atom_0 = Predicate::ATOM(auxes.last().unwrap().clone());
                        let mut disj = vec![];
                        for z in p {
                            match &z {
                                Predicate::ATOM(x) => {
                                    cnf.push(Predicate::OR(vec![
                                        Predicate::NOT(Box::new(aux_atom_0.clone())),
                                        Predicate::ATOM(x.clone()),
                                    ]));
                                    disj.push(Predicate::NOT(Box::new(Predicate::ATOM(x.clone()))));
                                }
                                _ => {
                                    let aux_atom_1 =
                                        Predicate::ATOM(format!("$aux{}", auxes.len()));
                                    auxes.push(format!("$aux{}", auxes.len()));
                                    cnf.push(Predicate::OR(vec![
                                        Predicate::NOT(Box::new(aux_atom_0.clone())),
                                        aux_atom_1.clone(),
                                    ]));
                                    disj.push(Predicate::NOT(Box::new(aux_atom_1.clone())));
                                    recursive(&z, auxes, cnf)
                                }
                            }
                        }
                        let mut total_disj_vec = vec![];
                        total_disj_vec.push(aux_atom_0);
                        for d in disj {
                            total_disj_vec.push(d)
                        }
                        cnf.push(Predicate::OR(total_disj_vec));
                    }
                }
            },

            Predicate::OR(p) => match p.len() {
                0 => (),
                1 => recursive(&p[0], auxes, cnf),
                _ => {
                    if auxes.len() == 0 {
                        let aux_atom_0 = Predicate::ATOM(format!("$aux{}", auxes.len()));
                        auxes.push(format!("$aux{}", auxes.len()));
                        cnf.push(Predicate::OR(vec![aux_atom_0.clone()]));
                        let mut disj = vec![];
                        for z in p {
                            match &z {
                                Predicate::ATOM(x) => {
                                    cnf.push(Predicate::OR(vec![
                                        aux_atom_0.clone(),
                                        Predicate::NOT(Box::new(Predicate::ATOM(x.clone()))),
                                    ]));
                                    disj.push(Predicate::ATOM(x.clone()));
                                }
                                _ => {
                                    let aux_atom_1 =
                                        Predicate::ATOM(format!("$aux{}", auxes.len()));
                                    auxes.push(format!("$aux{}", auxes.len()));
                                    cnf.push(Predicate::OR(vec![
                                        aux_atom_0.clone(),
                                        Predicate::NOT(Box::new(aux_atom_1.clone())),
                                    ]));
                                    disj.push(aux_atom_1.clone());
                                    recursive(&z, auxes, cnf)
                                }
                            }
                        }
                        let mut total_disj_vec = vec![];
                        total_disj_vec.push(Predicate::NOT(Box::new(aux_atom_0)));
                        for d in disj {
                            total_disj_vec.push(d)
                        }
                        cnf.push(Predicate::OR(total_disj_vec));
                    } else {
                        let aux_atom_0 = Predicate::ATOM(auxes.last().unwrap().clone());
                        let mut disj = vec![];
                        for z in p {
                            match &z {
                                Predicate::ATOM(x) => {
                                    cnf.push(Predicate::OR(vec![
                                        aux_atom_0.clone(),
                                        Predicate::NOT(Box::new(Predicate::ATOM(x.clone()))),
                                    ]));
                                    disj.push(Predicate::ATOM(x.clone()));
                                }
                                _ => {
                                    let aux_atom_1 =
                                        Predicate::ATOM(format!("$aux{}", auxes.len()));
                                    auxes.push(format!("$aux{}", auxes.len()));
                                    cnf.push(Predicate::OR(vec![
                                        aux_atom_0.clone(),
                                        Predicate::NOT(Box::new(aux_atom_1.clone())),
                                    ]));
                                    disj.push(aux_atom_1.clone());
                                    recursive(&z, auxes, cnf)
                                }
                            }
                        }
                        let mut total_disj_vec = vec![];
                        total_disj_vec.push(Predicate::NOT(Box::new(aux_atom_0)));
                        for d in disj {
                            total_disj_vec.push(d)
                        }
                        cnf.push(Predicate::OR(total_disj_vec));
                    }
                }
            },
        }
    }
    recursive(&pred, &mut auxes, &mut cnf);
    cnf
}

pub fn predicate_cnf_to_dpll_cnf(formula: &Vec<Predicate>) -> Vec<Vec<(String, bool)>> {
    formula
        .iter()
        .map(|x| match x {
            Predicate::OR(y) => y
                .iter()
                .map(|z| match z {
                    Predicate::ATOM(k) => (k.to_owned(), true),
                    Predicate::NOT(k) => match &**k {
                        Predicate::ATOM(l) => (l.to_owned(), false),
                        _ => panic!("BAD TSEITIN TRANSFORMATION 1"),
                    },
                    _ => panic!("BAD TSEITIN TRANSFORMATION 2"),
                })
                .collect(),
            _ => panic!("BAD TSEITIN TRANSFORMATION 3"),
        })
        .collect()
}
