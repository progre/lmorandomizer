import fs from 'fs';
import yaml from 'js-yaml';
import util from 'util';
import Supplements from '../model/dataset/Supplements';

const readFile = util.promisify(fs.readFile);

export default class SupplementsRepo {
  constructor(
    private directoryPath: string,
  ) {
  }

  async read(): Promise<Supplements> {
    const [weaponsYml, chestsYml, sealsYml, shopsYml, eventsYml] = await Promise.all([
      readFile(`${this.directoryPath}/weapons.yml`, 'utf-8'),
      readFile(`${this.directoryPath}/chests.yml`, 'utf-8'),
      readFile(`${this.directoryPath}/seals.yml`, 'utf-8'),
      readFile(`${this.directoryPath}/shops.yml`, 'utf-8'),
      readFile(`${this.directoryPath}/events.yml`, 'utf-8'),
    ]);
    const { mainWeapons, subWeapons } = yaml.safeLoad(weaponsYml);
    const chests = yaml.safeLoad(chestsYml);
    const seals = yaml.safeLoad(sealsYml);
    const shops = yaml.safeLoad(shopsYml);
    const events = parseRequirementsOfEvents(yaml.safeLoad(eventsYml));
    return new Supplements(
      <any>parseEventsInSupplement(parseRequirements(mainWeapons), events),
      <any>parseEventsInSupplement(parseRequirements(subWeapons), events),
      <any>parseEventsInSupplement(parseRequirements(chests), events),
      <any>parseEventsInSupplement(parseRequirements(seals), events),
      <any>parseEventsInSupplement(parseRequirements(shops), events),
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
