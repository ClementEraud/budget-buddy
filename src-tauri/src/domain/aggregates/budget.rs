use crate::{domain::entities::operation::Operation, shared::types::array::Array};

#[derive(Debug)]
pub struct Budget {
    #[allow(dead_code)]
    id: String,
    operations: Array<Operation>,
}

impl Budget {
    #[allow(dead_code)]
    pub fn new(id: String, operations: Array<Operation>) -> Self {
        Self { id, operations }
    }

    #[allow(dead_code)]
    pub fn get_incomes(&self) -> Array<Operation> {
        self.operations.filter(|op| op.is_income())
    }

    #[allow(dead_code)]
    pub fn get_expenses(&self) -> Array<Operation> {
        self.operations.filter(|op| op.is_expense())
    }

    #[allow(dead_code)]
    pub fn get_total_from_incomes(&self) -> i32 {
        let mut total = 0;

        for income in self.get_incomes().iter() {
            total += income.get_amount()
        }

        return total;
    }

    #[allow(dead_code)]
    pub fn get_total_from_expenses(&self) -> i32 {
        let mut total = 0;

        for expense in self.get_expenses().iter() {
            total += expense.get_amount()
        }

        return total;
    }

    #[allow(dead_code)]
    pub fn get_balance(&self) -> i32 {
        let total_incomes = self.get_total_from_incomes();
        let total_expenses = self.get_total_from_expenses();

        total_incomes - total_expenses
    }
}
