use crate::{
    domain::{entities::operation::Operation, value_objects::summary::Summary},
    shared::types::array::Array,
};

pub trait FinancialAggregate {
    fn get_operations(&self) -> Array<Operation>;

    fn get_incomes(&self) -> Array<Operation> {
        self.get_operations().filter(|op| op.is_income())
    }

    fn get_expenses(&self) -> Array<Operation> {
        self.get_operations().filter(|op| op.is_expense())
    }

    fn get_total_from_incomes(&self) -> f64 {
        let mut total = 0.0;

        for income in self.get_incomes().iter() {
            total += income.get_amount()
        }

        return total;
    }

    fn get_total_from_expenses(&self) -> f64 {
        let mut total = 0.0;

        for expense in self.get_expenses().iter() {
            total += expense.get_amount()
        }

        return total;
    }

    fn get_balance(&self) -> f64 {
        let total_incomes = self.get_total_from_incomes();
        let total_expenses = self.get_total_from_expenses();

        total_incomes - total_expenses
    }

    fn get_summary(&self) -> Summary {
        let incomes = self.get_incomes();
        let expenses = self.get_expenses();
        let total_income = self.get_total_from_incomes();
        let total_expense = self.get_total_from_expenses();
        let balance = self.get_balance();

        Summary {
            incomes,
            expenses,
            total_income,
            total_expense,
            balance,
        }
    }
}
