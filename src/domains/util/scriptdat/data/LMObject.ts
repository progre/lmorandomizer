export default interface LMObject {
  readonly number: number;
  readonly x: number;
  readonly y: number;
  readonly op1: number;
  readonly op2: number;
  readonly op3: number;
  readonly op4: number;
  readonly starts: ReadonlyArray<Readonly<LMStart>>;
}

export interface LMStart {
  number: number;
  value: boolean;
}
