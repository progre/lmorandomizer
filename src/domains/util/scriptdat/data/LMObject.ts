import assert from '../../../../assert';

export default class LMObject {
  constructor(
    public readonly number: number,
    public readonly x: number,
    public readonly y: number,
    public readonly op1: number,
    public readonly op2: number,
    public readonly op3: number,
    public readonly op4: number,
    public readonly starts: ReadonlyArray<Readonly<LMStart>>,
  ) {
  }

  asMainWeapon() {
    return { mainWeaponNumber: this.op1, flag: this.op2 };
  }

  asSubWeapon() {
    return { subWeaponNumber: this.op1, count: this.op2, flag: this.op3 };
  }

  asChestItem() {
    assert.equal(this.number, 1);
    return { chestItemNumber: this.op2, openFlag: this.op1, flag: this.op3 };
  }

  asSeal() {
    return { sealNumber: this.op1, flag: this.op2 };
  }

  getItemFlag() {
    switch (this.number) {
      case 77: return this.asMainWeapon().flag;
      case 13: return this.asSubWeapon().flag;
      case 1: return this.asChestItem().flag;
      case 71: return this.asSeal().flag;
      default: throw new Error();
    }
  }
}

export interface LMStart {
  number: number;
  value: boolean;
}
