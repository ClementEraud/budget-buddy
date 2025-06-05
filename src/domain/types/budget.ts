import type { Operation } from "./operation";

export class Budget {
  id: string;
  date: {
    year: number;
    month: number;
  };
  incomes: Array<Operation>;
  expenses: Array<Operation>;

  constructor(
    id: string,
    date: { year: number; month: number },
    incomes: Array<Operation>,
    expenses: Array<Operation>,
  ) {
    this.id = id;
    this.date = date;
    this.incomes = incomes;
    this.expenses = expenses;
  }
}
