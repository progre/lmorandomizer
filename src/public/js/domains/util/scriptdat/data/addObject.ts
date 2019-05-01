import LMObject from './LMObject';
import { LMWorld } from './Script';


// Accepts objects and 1 screen, adds to screen.
export default function addObject (
  worlds: ReadonlyArray<LMWorld>,
  fieldn: Number,
  screenx: 0 | 1 | 2 | 3,
  screeny: 0 | 1 | 2 | 3 | 4,
  object: LMObject[],
): ReadonlyArray<LMWorld> {
//    const object = new LMObject(14, 14336, 40960, 200, -1, 185, 0, []);
  return worlds.map(world => ({
    value: world.value,
    fields: world.fields.map(field => (
      field.attrs[0] !== fieldn
        ? field
        : {
          ...field,
          maps: field.maps.map(map => (
            !(map.attrs[0] === screenx && map.attrs[1] === screeny)
              ? map
              : {
                ...map,
            objects: map.objects.concat(object),
          }
        )),
      }
    )),
  }));
}

