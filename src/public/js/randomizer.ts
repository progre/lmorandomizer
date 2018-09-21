(<any>global).eval = global.eval = (arg) => {
  // seedrandom
  if (arg === 'this') {
    return global;
  }
  throw new Error(`Sorry, this app does not support window.eval().`);
};

import randomize from './applications/randomize';

onmessage = (e) => {
  const randomized = randomize(e.data.scriptDat, e.data.supplementFiles, e.data.options);
  postMessage(randomized, <any>undefined, [randomized]);
};
