import React from 'react';
import ReactDOM from 'react-dom';
import { InitialParameters } from '../../types';
import Index from './containers/Index';

const json: InitialParameters
  = JSON.parse(decodeURIComponent(location.search.slice(1)));

ReactDOM.render(
  (
    <Index
      defaultSeed={json.seed}
      defaultInstallDirectory={json.installDirectory}
      defaultEasyMode={json.easyMode}
    />
  ),
  document.getElementById('root'),
);
