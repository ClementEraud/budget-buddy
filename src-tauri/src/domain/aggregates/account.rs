use crate::{domain::entities::operation::Operation, shared::types::array::Array};

#[derive(Debug)]
pub struct Account {
    #[allow(dead_code)]
    id: String,
    operations: Array<Operation>,
}

impl Account {
    pub fn new(id: String, operations: Array<Operation>) -> Self {
        Self { id, operations }
    }

    pub fn get_incomes(&self) -> Array<Operation> {
        self.operations.filter(|op| op.is_income())
    }

    pub fn get_expenses(&self) -> Array<Operation> {
        self.operations.filter(|op| op.is_expense())
    }

    pub fn get_total_from_incomes(&self) -> i32 {
        let mut total = 0;

        for income in self.get_incomes().iter() {
            total += income.get_value()
        }

        return total;
    }

    pub fn get_total_from_expenses(&self) -> i32 {
        let mut total = 0;

        for expense in self.get_expenses().iter() {
            total += expense.get_value()
        }

        return total;
    }

    pub fn get_balance(&self) -> i32 {
        let total_incomes = self.get_total_from_incomes();
        let total_expenses = self.get_total_from_expenses();

        total_incomes - total_expenses
    }
}
