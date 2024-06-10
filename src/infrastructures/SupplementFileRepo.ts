export default class SupplementFileRepo {
  async read() {
    const [weaponsYml, chestsYml, sealsYml, shopsYml, eventsYml] = await Promise.all([
      fetch(`res/weapons.yml`).then(async x => x.text()),
      fetch(`res/chests.yml`).then(async x => x.text()),
      fetch(`res/seals.yml`).then(async x => x.text()),
      fetch(`res/shops.yml`).then(async x => x.text()),
      fetch(`res/events.yml`).then(async x => x.text()),
    ]);
    return { weaponsYml, chestsYml, sealsYml, shopsYml, eventsYml };
  }
}
