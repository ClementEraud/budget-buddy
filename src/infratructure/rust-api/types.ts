export type ApiOperationsRaw = {
  items: Array<{ label: string; amount: number }>;
};

export type ApiSummaryRaw = {
  incomes: ApiOperationsRaw;
  expenses: ApiOperationsRaw;
  total_income: number;
  total_expense: number;
  balance: number;
};

export type ApiBudgetRaw = {
  id: string;
  date: {
    year: number;
    month: number;
  };
  operations: ApiOperationsRaw;
};
