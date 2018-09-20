import assert from 'assert';
import parse5 from 'parse5';
import { subWeaponNumbers } from '../../../model/randomizer/items';
import LMObject from '../data/LMObject';
import { LMChild, LMWorld } from '../data/Script';
import ShopItemsData from './ShopItemsData';

export function parseScriptTxt(txt: string) {
  const root: ReadonlyArray<any> = (
    (<any>parse5.parse(txt)).childNodes[0].childNodes[1].childNodes
  );
  const talks = (
    root
      .filter(x => x.tagName === 'talk')
      .map((x) => {
        assert.equal(x.childNodes.length, 1);
        assert.equal(x.childNodes[0].nodeName, '#text');
        assert.equal(x.childNodes[0].value[0], '\n');
        return (<string>x.childNodes[0].value).slice(1);
      })
  );
  assert.equal(ShopItemsData.parse(talks[252])[0].number, subWeaponNumbers.handScanner);
  assert.equal(ShopItemsData.parse(talks[252])[0].price, 20);
  assert.equal(ShopItemsData.parse(talks[252])[0].flag, 65279);
  assert.equal(ShopItemsData.parse(talks[252])[1].number, subWeaponNumbers.ammunition);
  assert.equal(ShopItemsData.parse(talks[252])[1].price, 500);
  assert.equal(ShopItemsData.parse(talks[252])[1].flag, 65279);
  assert.equal(ShopItemsData.parse(talks[252])[2].number, subWeaponNumbers.buckler);
  assert.equal(ShopItemsData.parse(talks[252])[2].price, 80);
  assert.equal(ShopItemsData.parse(talks[252])[2].flag, 697);
  const worlds = (
    root
      .filter(world => world.tagName === 'world')
      .map(world => ({
        value: parseAttrs(world.attrs)[0],
        fields: (
          (<ReadonlyArray<any>>world.childNodes)
            .filter(field => field.tagName === 'field')
            .map((field) => {
              const children = flatChildren(field);
              return {
                attrs: parseAttrs(field.attrs),
                children: (
                  children
                    .filter(child => (
                      child.tagName !== 'object'
                      && child.tagName !== 'map'
                      && child.nodeName !== '#text'
                    ))
                    .map(parseChild)
                ),
                objects: (
                  children
                    .filter(child => child.tagName === 'object')
                    .map(parseObject)
                ),
                maps: (
                  children
                    .filter(child => child.tagName === 'map')
                    .map((child) => {
                      const mapChildren = flatChildren(child);
                      return {
                        attrs: parseAttrs(child.attrs),
                        children: (
                          mapChildren
                            .filter(x => (
                              x.tagName !== 'object'
                              && x.nodeName !== '#text'
                            ))
                            .map(parseChild)
                        ),
                        objects: (
                          mapChildren
                            .filter(object => object.tagName === 'object')
                            .map(parseObject)
                        ),
                      };
                    })
                ),
              };
            })
        ),
      }))
  );

  assert.equal(talks.length, 905);
  assert.equal(worlds[0].fields[0].objects[0].starts[0].number, 99999);
  assert.equal(worlds[0].fields[0].maps[0].objects[5].starts[0].number, 58);
  return { talks, worlds };
}

function parseAttrs(attrs: ReadonlyArray<{ name: string; value: string }>) {
  assert.equal(attrs.length, 1);
  assert.equal(attrs[0].value, '');
  return attrs[0].name.split(',').map(Number);
}

function flatChildren(
  root: { childNodes: ReadonlyArray<{ tagName: string; childNodes?: ReadonlyArray<any> }> },
): ReadonlyArray<any> {
  return root.childNodes.map((x) => {
    if (x.childNodes == null) {
      return [x];
    }
    if (x.tagName === 'object' || x.tagName === 'map') {
      return [x];
    }
    return [x].concat(flatChildren(<{ childNodes: ReadonlyArray<any> }>x));
  }).reduce((p, c) => p.concat(c), []);
}

function parseChild(child: any): LMChild {
  return {
    name: child.tagName,
    attrs: parseAttrs(child.attrs),
  };
}

function parseObject(object: any) {
  const attrs = parseAttrs(object.attrs);
  return new LMObject(
    attrs[0],
    attrs[1],
    attrs[2],
    attrs[3],
    attrs[4],
    attrs[5],
    attrs[6],
    flatChildren(object)
      .filter(x => x.nodeName !== '#text')
      .map((x) => {
        const startAttrs = parseAttrs(x.attrs);
        return {
          number: startAttrs[0],
          value: Boolean(startAttrs[1]),
        };
      }),
  );
}

export function stringifyScriptTxt(
  talks: ReadonlyArray<string>,
  worlds: ReadonlyArray<LMWorld>,
) {
  return [
    talks.map(x => (
      `<TALK>\n${x}</TALK>\n`
    )).join(''),
    worlds.map(world => [
      `<WORLD ${world.value}>\n`,
      world.fields.map(field => [
        `<FIELD ${field.attrs.join(',')}>\n`,
        field.children.map(stringifyChild).join(''),
        field.objects.map(stringifyObject).join(''),
        field.maps.map(map => [
          `<MAP ${map.attrs.join(',')}>\n`,
          map.children.map(stringifyChild).join(''),
          map.objects.map(stringifyObject).join(''),
          `</MAP>\n`,
        ].join('')).join(''),
        `</FIELD>\n`,
      ].join('')).join(''),
      `</WORLD>\n`,
    ].join('')).join(''),
  ].join('');
}

function stringifyChild(child: LMChild) {
  return `<${child.name.toUpperCase()} ${child.attrs.join(',')}>\n`;
}

function stringifyObject(obj: LMObject) {
  return [
    `<OBJECT ${obj.number},${obj.x},${obj.y},${obj.op1},${obj.op2},${obj.op3},${obj.op4}>\n`,
    obj.starts.map(start => (
      `<START ${start.number},${Number(start.value)}>\n`
    )).join(''),
    `</OBJECT>\n`,
  ].join('');
}
