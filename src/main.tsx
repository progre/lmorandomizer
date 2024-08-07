import { invoke } from '@tauri-apps/api/core';
import React from 'react';
import ReactDOM from 'react-dom/client';
import Index from './containers/Index';
import { InitialData } from './types';

async function main() {
  const json = (await invoke('initial_data')) as InitialData;

  ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
      <Index
        defaultSeed={json.seed}
        defaultInstallDirectory={json.installDirectory}
        defaultShuffleSecretRoms={json.shuffleSecretRoms}
        defaultNeedGlitches={json.needGlitches}
        defaultAbsolutelyShuffle={json.absolutelyShuffle}
      />
    </React.StrictMode>
  );
  await invoke('ready');
}

main().catch(console.error);
