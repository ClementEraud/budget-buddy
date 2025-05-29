import type { PageLoad } from "./$types";
import { ApiQueryBudget } from "../infratructure/rust-api/queries/budget";
import { ApiQueryAccount } from "../infratructure/rust-api/queries/account";
import { redirect } from "@sveltejs/kit";

export const load: PageLoad = async () => {
  const currentBudgetSummary = await ApiQueryBudget.getCurrentBudgetSummary();

  if (!currentBudgetSummary) {
    redirect(308, "/budget/new");
  }

  const accountSummary = await ApiQueryAccount.getAccountSummary();

  return {
    accountSummary,
    currentBudgetSummary,
  };
};
