import type { Summary } from "../../domain/types/summary";
import type { ApiSummaryRaw } from "./types";

export const fromRawSummaryToDomainSummary = (
  rawResult: ApiSummaryRaw,
): Summary => {
  return {
    balance: rawResult.balance,
    totalIncome: rawResult.total_income,
    totalExpense: rawResult.total_expense,
    incomes: rawResult.incomes.items,
    expenses: rawResult.expenses.items,
  };
};
