import type { Operation } from "./operation";

export type Summary = {
  incomes: Array<Operation>;
  expenses: Array<Operation>;
  totalIncome: number;
  totalExpense: number;
  balance: number;
};
