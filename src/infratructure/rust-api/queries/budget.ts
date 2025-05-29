import { invoke } from "@tauri-apps/api/core";
import { Budget } from "../../../domain/types/budget";
import type { ApiBudgetRaw } from "../types";
import type { Nullable } from "../../../shared/types";

export class ApiQueryBudget {
  static async getCurrentBudget(): Promise<Nullable<Budget>> {
    return this.getBudgetForDate({
      year: new Date().getFullYear(),
      month: new Date().getMonth() + 1,
    });
  }

  private static async getBudgetForDate(date: {
    year: number;
    month: number;
  }): Promise<Nullable<Budget>> {
    const budget = await invoke<ApiBudgetRaw>("get_budget_for_date", { date });
    if (!budget) return null;
    return ApiQueryBudget.fromRawResultToDomain(budget);
  }

  private static fromRawResultToDomain(rawResult: ApiBudgetRaw): Budget {
    return {
      id: rawResult.id,
      date: rawResult.date,
      operations: rawResult.operations.items,
    };
  }
}
