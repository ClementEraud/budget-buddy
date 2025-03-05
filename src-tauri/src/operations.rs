use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Array<T: Serialize> {
    items: Vec<T>,
}

#[derive(Clone, Serialize)]
pub struct Income {
    label: String,
    value: i32,
}

#[derive(Clone, Serialize)]
pub struct Expense {
    label: String,
    value: i32,
    is_planned: bool,
}

pub struct Operations {
    incomes: Array<Income>,
    expenses: Array<Expense>,
}

impl Operations {
    pub fn new() -> Self {
        Self {
            incomes: Array {
                items: vec![
                    Income {
                        label: String::from("Salary"),
                        value: 1600,
                    },
                    Income {
                        label: String::from("Social Aids"),
                        value: 400,
                    },
                ],
            },
            expenses: Array {
                items: vec![
                    Expense {
                        label: String::from("Rent"),
                        value: 600,
                        is_planned: true,
                    },
                    Expense {
                        label: String::from("Netflix"),
                        value: 20,
                        is_planned: true,
                    },
                    Expense {
                        label: String::from("Internet"),
                        value: 50,
                        is_planned: true,
                    },
                    Expense {
                        label: String::from("Mobile phone"),
                        value: 50,
                        is_planned: true,
                    },
                    Expense {
                        label: String::from("Electricity"),
                        value: 80,
                        is_planned: true,
                    },
                    Expense {
                        label: String::from("Other bills"),
                        value: 400,
                        is_planned: true,
                    },
                    Expense {
                        label: String::from("Groceries"),
                        value: 300,
                        is_planned: false,
                    },
                ],
            },
        }
    }

    pub fn get_incomes(&self) -> &Array<Income> {
        &self.incomes
    }

    pub fn get_planned_expenses(&self) -> Array<Expense> {
        let mut planned_expenses = Array { items: vec![] };

        for expense in self.expenses.items.iter() {
            if expense.is_planned == true {
                planned_expenses.items.push(expense.clone());
            }
        }

        planned_expenses
    }

    pub fn get_actual_expenses(&self) -> Array<Expense> {
        let mut actual_expenses = Array { items: vec![] };

        for expense in self.expenses.items.iter() {
            if expense.is_planned == false {
                actual_expenses.items.push(expense.clone());
            }
        }

        actual_expenses
    }

    pub fn get_total_from_incomes(&self) -> i32 {
        let mut total = 0;

        for income in self.incomes.items.iter() {
            total += income.value
        }

        return total;
    }

    pub fn get_total_from_planned_expenses(&self) -> i32 {
        let mut total = 0;

        for expense in self.expenses.items.iter() {
            if expense.is_planned == true {
                total += expense.value
            }
        }

        return total;
    }

    pub fn get_total_from_actual_expenses(&self) -> i32 {
        let mut total = 0;

        for expense in self.expenses.items.iter() {
            if expense.is_planned == false {
                total += expense.value
            }
        }

        return total;
    }

    pub fn get_balance(&self) -> i32 {
        let total_incomes = self.get_total_from_incomes();
        let total_planned_expenses = self.get_total_from_planned_expenses();
        let total_actual_expenses = self.get_total_from_actual_expenses();

        total_incomes - total_planned_expenses - total_actual_expenses
    }
}
