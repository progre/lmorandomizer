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
    const [weaponsYml, chestsYml, sealsYml, shopsYml] = await Promise.all([
      readFile(`${this.directoryPath}/weapons.yml`, 'utf-8'),
      readFile(`${this.directoryPath}/chests.yml`, 'utf-8'),
      readFile(`${this.directoryPath}/seals.yml`, 'utf-8'),
      readFile(`${this.directoryPath}/shops.yml`, 'utf-8'),
    ]);
    const { mainWeapons, subWeapons } = yaml.safeLoad(weaponsYml);
    const chests = yaml.safeLoad(chestsYml);
    const seals = yaml.safeLoad(sealsYml);
    const shops = yaml.safeLoad(shopsYml);
    return new Supplements(
      <any>parseRequirements(mainWeapons),
      <any>parseRequirements(subWeapons),
      <any>parseRequirements(chests),
      <any>parseRequirements(seals),
      <any>parseRequirements(shops),
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
