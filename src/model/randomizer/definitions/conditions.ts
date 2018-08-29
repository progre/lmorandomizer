import { EquipmentNumber, MainWeaponNumber, SubWeaponNumber } from './items';

export type PhysicalConditionGroups = ReadonlyArray<PhysicalConditionGroup>;
type PhysicalConditionGroup = ReadonlyArray<PhysicalCondition>;

export type PhysicalCondition = (
  MainWeaponPhysicalCondition
  | SubWeaponPhysicalCondition
  | EquipmentPhysicalCondition
);

interface MainWeaponPhysicalCondition { type: 'mainWeapon'; payload: MainWeaponNumber; }
interface SubWeaponPhysicalCondition { type: 'subWeapon'; payload: SubWeaponNumber; }
interface EquipmentPhysicalCondition { type: 'equipment'; payload: EquipmentNumber; }

export type LogicalConditionGroups = ReadonlyArray<LogicalConditionGroup>;
type LogicalConditionGroup = ReadonlyArray<LogicalCondition>;

type LogicalCondition = PhysicalCondition | EventLogicalCondition;

interface EventLogicalCondition {
  type: 'event';
  payload: string;
}
