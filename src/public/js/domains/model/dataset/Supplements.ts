import assert from 'assert';
import yaml from 'js-yaml';

export type Requirement = string;

export default class Supplements {
  static readonly nightSurfaceSubWeaponCount = 1;
  static readonly nightSurfaceChestCount = 3;
  static readonly trueShrineOfTheMotherSealCount = 1;
  static readonly nightSurfacSealCount = 1;
  static readonly wareNoMiseCount = 1;
  public readonly mainWeapons: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  public readonly subWeapons: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  public readonly chests: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  public readonly seals: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  public readonly shops: ReadonlyArray<{
    names: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;

  constructor(
    supplementFiles: {
      weaponsYml: string;
      chestsYml: string;
      sealsYml: string;
      shopsYml: string;
      eventsYml: string;
    },
  ) {
    const { mainWeapons, subWeapons } = yaml.safeLoad(supplementFiles.weaponsYml);
    const chests = yaml.safeLoad(supplementFiles.chestsYml);
    const seals = yaml.safeLoad(supplementFiles.sealsYml);
    const shops = yaml.safeLoad(supplementFiles.shopsYml);
    const events = parseRequirementsOfEvents(yaml.safeLoad(supplementFiles.eventsYml));

    this.mainWeapons = <any>parseEventsInSupplement(parseRequirements(mainWeapons), events);
    this.subWeapons = <any>parseEventsInSupplement(parseRequirements(subWeapons), events);
    this.chests = <any>parseEventsInSupplement(parseRequirements(chests), events);
    this.seals = <any>parseEventsInSupplement(parseRequirements(seals), events);
    this.shops = <any>parseEventsInSupplement(parseRequirements(shops), events);
    assert.deepEqual(
      this.chests.find(x => x.name === 'iceCape')!.requirements,
      [
        ['ankhJewel:templeOfTheSun', 'bronzeMirror', 'shuriken', 'shurikenAmmo'],
        ['holyGrail', 'flareGun', 'grappleClaw'],
        // tslint:disable-next-line:max-line-length
        // ['anchor', 'knife', 'bronzeMirror', 'ankhJewel:gateOfGuidance', 'flareGun', 'grappleClaw'],
        ['bronzeMirror', 'ankhJewel:mausoleumOfTheGiants', 'flareGun', 'grappleClaw'],
        ['holyGrail', 'flareGun', 'feather'],
        // ['anchor', 'knife', 'bronzeMirror', 'ankhJewel:gateOfGuidance', 'flareGun', 'feather'],
        ['bronzeMirror', 'ankhJewel:mausoleumOfTheGiants', 'flareGun', 'feather'],
      ],
    );
  }
}

function parseRequirements(
  items: ReadonlyArray<{ requirements?: ReadonlyArray<string> }>,
) {
  return items.map(x => ({
    ...x,
    requirements: (
      x.requirements == null
        ? null
        : x.requirements.map(y => y.split(',').map(z => z.trim()))
    ),
  }));
}

function parseRequirementsOfEvents(
  items: ReadonlyArray<{ name: string; requirements: ReadonlyArray<string> }>,
) {
  type Events = ReadonlyArray<{ name: string; requirements: ReadonlyArray<ReadonlyArray<string>> }>;
  let current: Events = items.map(x => ({
    ...x,
    requirements: x.requirements.map(y => y.split(',').map(z => z.trim())),
  }));
  for (let i = 0; i < 100; i += 1) {
    const events = current.filter(
      x => x.requirements.every(y => y.every(z => !z.startsWith('event:'))),
    );
    if (events.length === current.length) {
      return current;
    }
    current = current.map(x => ({
      name: x.name,
      requirements: parseEvents(x.requirements, events),
    }));
  }
  console.error(JSON.stringify(current.filter(
    x => x.requirements.some(y => y.some(z => z.startsWith('event:'))),
  )));
  throw new Error();
}

function parseEventsInSupplement(
  list: ReadonlyArray<{ requirements: ReadonlyArray<ReadonlyArray<string>> | null }>,
  events: ReadonlyArray<{
    name: string;
    requirements: ReadonlyArray<ReadonlyArray<string>>;
  }>,
) {
  return list.map(x => (
    x.requirements == null
      ? x
      : {
        ...x,
        requirements: parseEvents(x.requirements, events),
      }
  ));
}

function parseEvents(
  requirements: ReadonlyArray<ReadonlyArray<string>>,
  events: ReadonlyArray<{
    name: string;
    requirements: ReadonlyArray<ReadonlyArray<string>>;
  }>,
) {
  // [['event:a', 'event:b', 'c']]
  // 'event:a': [['d', 'e', 'f']]
  // 'event:b': [['g', 'h'], ['i', 'j']]
  // ↓
  // [['event:b', 'c', 'd', 'e', 'f']]
  // ↓
  // [
  //   ['c', 'd', 'e', 'f', 'g', 'h']
  //   ['c', 'd', 'e', 'f', 'i', 'j']
  // ]
  let current = requirements;
  events.forEach((event) => {
    if (current.every(targetGroup => !targetGroup.includes(event.name))) {
      return;
    }
    current = current
      .map((targetGroup) => {
        if (!targetGroup.includes(event.name)) {
          return [targetGroup];
        }
        return event.requirements.map(eventGroup => (
          eventGroup.concat(targetGroup.filter(x => x !== event.name))
        ));
      })
      .reduce((p, c) => p.concat(c), []);
  });
  return current;
}
