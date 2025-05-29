import type { PageLoad } from "./$types";
import { ApiQueryBudget } from "../infratructure/rust-api/queries/budget";
import { ApiQueryAccount } from "../infratructure/rust-api/queries/account";
import { redirect } from "@sveltejs/kit";

export const load: PageLoad = async () => {
  const currentBudget = await ApiQueryBudget.getCurrentBudget();

  if (!currentBudget) {
    redirect(308, "/budget/new");
  }

  return {
    accountSummary: ApiQueryAccount.getAccountSummary(),
    currentBudget,
  };
};
