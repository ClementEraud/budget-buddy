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
    balance: invoke<ApiBalance>("get_balance"),
    incomes: invoke<ApiOperations>("get_incomes").then(transformToOperations),
    plannedExpenses: invoke<ApiOperations>("get_planned_expenses").then(
      transformToOperations,
    ),
    actualExpenses: invoke<ApiOperations>("get_actual_expenses").then(
      transformToOperations,
    ),
  };
};
