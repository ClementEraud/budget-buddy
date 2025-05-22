import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";
import type { Operations } from "../domain/types/operation";
import type { Summary } from "../domain/types/summary";

type ApiOperations = { items: Operations };
type ApiSummary = {
  incomes: ApiOperations;
  expenses: ApiOperations;
  total_income: number;
  total_expense: number;
  balance: number;
};

const fromApiToDomain = (data: ApiSummary): Summary => {
  return {
    incomes: data.incomes.items,
    expenses: data.expenses.items,
    totalIncome: data.total_income,
    totalExpense: data.total_expense,
    balance: data.balance,
  };
};

export const load: PageLoad = async () => {
  return {
    accountSummary: invoke<ApiSummary>("get_account_summary").then(
      fromApiToDomain,
    ),
  };
};
