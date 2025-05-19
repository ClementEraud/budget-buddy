use crate::{Array, Expense, Income};

#[derive(Debug)]
pub struct Operations {
    incomes: Array<Income>,
    planned_expenses: Array<Expense>,
    actual_expenses: Array<Expense>,
}

impl Operations {
    pub fn new(
        incomes: Array<Income>,
        planned_expenses: Array<Expense>,
        actual_expenses: Array<Expense>,
    ) -> Self {
        Self {
            incomes,
            planned_expenses,
            actual_expenses,
        }
    }

    pub fn get_incomes(&self) -> &Array<Income> {
        &self.incomes
    }

    pub fn get_planned_expenses(&self) -> &Array<Expense> {
        &self.planned_expenses
    }

    pub fn get_actual_expenses(&self) -> &Array<Expense> {
        &self.actual_expenses
    }

    pub fn get_total_from_incomes(&self) -> i32 {
        let mut total = 0;

        for income in self.incomes.iter() {
            total += income.get_value()
        }

        return total;
    }

    pub fn get_total_from_planned_expenses(&self) -> i32 {
        let mut total = 0;

        for expense in self.planned_expenses.iter() {
            total += expense.get_value()
        }

        return total;
    }

    pub fn get_total_from_actual_expenses(&self) -> i32 {
        let mut total = 0;

        for expense in self.actual_expenses.iter() {
            total += expense.get_value()
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
