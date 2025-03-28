//! # PB (Multi-Criteria) Uncapacitated Facility Location Problem Encoding

use rustsat::{
    encodings::atomics,
    instances::fio::opb,
    lit,
    types::{constraints::CardConstraint, Lit},
};

use crate::encodings::facilitylocation::FacilityLocation;

#[derive(Default)]
enum Line {
    /// Hint line
    #[default]
    Hint,
    /// Description
    Description,
    /// An objective with given index
    Objective(usize),
    /// Constraints stating that exactly one facility is selected per customer
    OneFacility(usize),
    /// Constraints stating that if a facility (first argument) is selected by a customer (second
    /// argument), it must be opened
    MustOpen(usize, usize),
}

pub struct Encoding {
    data: FacilityLocation,
    next_line: Option<Line>,
}

impl Encoding {
    pub fn new(data: FacilityLocation) -> Self {
        Self {
            data,
            next_line: Some(Line::default()),
        }
    }
}

impl Iterator for Encoding {
    type Item = opb::FileLine<<Vec<(Lit, usize)> as IntoIterator>::IntoIter>;

    fn next(&mut self) -> Option<Self::Item> {
        let selected = |customer: usize, facility: usize| {
            lit![(customer * self.data.n_facilities() + facility) as u32]
        };
        let opening = |facility: usize| {
            lit![(self.data.n_facilities() * self.data.n_customers() + facility) as u32]
        };
        match self.next_line.take() {
            Some(line) => Some(match line {
                Line::Hint => {
                    self.next_line = Some(Line::Description);
                    opb::FileLine::Comment(format!(
                        "#variable= {} #constraint= {}",
                        (self.data.n_customers() + 1) * self.data.n_facilities(),
                        self.data.n_customers() * (self.data.n_facilities() + 1)
                    ))
                }
                Line::Description => {
                    self.next_line = Some(Line::Objective(0));
                    opb::FileLine::Comment(
                        "MO uncapacitated facility location instance generated by RustSAT"
                            .to_string(),
                    )
                }
                Line::Objective(objidx) => {
                    let data = &self.data;
                    let opening_cost = (0..self.data.n_facilities())
                        .map(|fac| (opening(fac), data.opening_cost(objidx, fac)));
                    let obj: Vec<_> = (0..self.data.n_customers())
                        .flat_map(|customer| {
                            (0..self.data.n_facilities()).map(move |fac| {
                                (
                                    selected(customer, fac),
                                    data.supply_cost(objidx, customer, fac),
                                )
                            })
                        })
                        .chain(opening_cost)
                        .collect();
                    self.next_line = Some(if objidx + 1 < self.data.n_objs() {
                        Line::Objective(objidx + 1)
                    } else {
                        Line::OneFacility(0)
                    });
                    opb::FileLine::Objective(obj.into_iter())
                }
                Line::OneFacility(customer) => {
                    let constr = CardConstraint::new_eq(
                        (0..self.data.n_facilities()).map(|fac| selected(customer, fac)),
                        1,
                    );
                    self.next_line = Some(if customer + 1 < self.data.n_customers() {
                        Line::OneFacility(customer + 1)
                    } else {
                        Line::MustOpen(0, 0)
                    });
                    opb::FileLine::Card(constr)
                }
                Line::MustOpen(fac, customer) => {
                    let constr = atomics::lit_impl_lit(selected(customer, fac), opening(fac));
                    self.next_line = if customer + 1 < self.data.n_customers() {
                        Some(Line::MustOpen(fac, customer + 1))
                    } else if fac + 1 < self.data.n_facilities() {
                        Some(Line::MustOpen(fac + 1, 0))
                    } else {
                        None
                    };
                    opb::FileLine::Clause(constr)
                }
            }),
            None => None,
        }
    }
}
