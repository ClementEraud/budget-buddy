import type { Operation } from "./operation";

export class Budget {
  id: string;
  date: {
    year: number;
    month: number;
  };
  operations: Array<Operation>;

  constructor(
    id: string,
    date: { year: number; month: number },
    operations: Array<Operation>,
  ) {
    this.id = id;
    this.date = date;
    this.operations = operations;
  }
}
