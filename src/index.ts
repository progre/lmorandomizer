// tslint:disable-next-line:no-implicit-dependencies
try { require('source-map-support').install(); } catch (e) { /* NOP */ }
import App from './App';

async function main() {
  await App.create();
}

main().catch(console.error);
