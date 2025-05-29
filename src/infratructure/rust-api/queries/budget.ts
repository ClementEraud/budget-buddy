import { invoke } from "@tauri-apps/api/core";
import type { ApiSummaryRaw } from "../types";
import type { Nullable } from "../../../shared/types";
import type { Summary } from "../../../domain/types/summary";
import { fromRawSummaryToDomainSummary } from "../utils";

export class ApiQueryBudget {
  static async getCurrentBudgetSummary(): Promise<Nullable<Summary>> {
    return this.getBudgetSummaryForDate({
      year: new Date().getFullYear(),
      month: new Date().getMonth() + 1,
    });
  }

  static async getBudgetSummaryForDate(date: {
    year: number;
    month: number;
  }): Promise<Nullable<Summary>> {
    const budget = await invoke<ApiSummaryRaw>("get_budget_summary_for_date", {
      date,
    });

    if (!budget) return null;

    return fromRawSummaryToDomainSummary(budget);
  }
}
