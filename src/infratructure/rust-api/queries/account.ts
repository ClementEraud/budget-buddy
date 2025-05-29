import { invoke } from "@tauri-apps/api/core";
import type { Summary } from "../../../domain/types/summary";
import type { ApiSummaryRaw } from "../types";
import { fromRawSummaryToDomainSummary } from "../utils";

export class ApiQueryAccount {
  static async getAccountSummary(): Promise<Summary> {
    const result = await invoke<ApiSummaryRaw>("get_account_summary");

    return fromRawSummaryToDomainSummary(result);
  }
}
