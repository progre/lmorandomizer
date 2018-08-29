import { PhysicalConditionGroups } from './conditions';

export type Place = ChestPlace;

export interface ChestPlace { type: 'chest'; payload: Chest; }

export interface Chest {
  source: string;
  conditionGroups: PhysicalConditionGroups;
}
