import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";
import type { Operations } from "../domain/types/operation";

type ApiOperations = { items: Operations };
type ApiBalance = number;

const transformToOperations = (data: ApiOperations): Operations => {
  return data.items;
};

export const load: PageLoad = async () => {
  return {
    account_balance: invoke<ApiBalance>("get_account_balance"),
    account_incomes: invoke<ApiOperations>("get_account_incomes").then(
      transformToOperations,
    ),
    account_expenses: invoke<ApiOperations>("get_account_expenses").then(
      transformToOperations,
    ),
  };
};
