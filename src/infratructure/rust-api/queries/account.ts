import { invoke } from "@tauri-apps/api/core";
import type { Summary } from "../../../domain/types/summary";
import type { ApiSummaryRaw } from "../types";
import { fromRawSummaryToDomainSummary } from "../utils";
import type { Nullable } from "../../../shared/types";

export class ApiQueryAccount {
  static async getCurrentAccountSummary(): Promise<Nullable<Summary>> {
    return this.getAccountSummaryForDate({
      year: new Date().getFullYear(),
      month: new Date().getMonth() + 1,
    });
  }

  static async getAccountSummaryForDate(date: {
    year: number;
    month: number;
  }): Promise<Nullable<Summary>> {
    const budget = await invoke<ApiSummaryRaw>("get_account_summary_for_date", {
      date,
    });

    if (!budget) return null;

    return fromRawSummaryToDomainSummary(budget);
  }
}
