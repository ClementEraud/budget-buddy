import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  return {
    balance: invoke("get_balance") as Promise<number>,
  };
};
