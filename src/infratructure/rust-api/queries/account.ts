import { invoke } from "@tauri-apps/api/core";
import type { Summary } from "../../../domain/types/summary";
import type { ApiSummaryRaw } from "../types";

export class ApiQueryAccount {
  static async getAccountSummary(): Promise<Summary> {
    const result = await invoke<ApiSummaryRaw>("get_account_summary");

    return ApiQueryAccount.fromRawResultToDomain(result);
  }

  private static fromRawResultToDomain(rawResult: ApiSummaryRaw): Summary {
    return {
      balance: rawResult.balance,
      totalIncome: rawResult.total_income,
      totalExpense: rawResult.total_expense,
      incomes: rawResult.incomes.items,
      expenses: rawResult.expenses.items,
    };
  }
}
