// tslint:disable-next-line:no-implicit-dependencies
import electron from 'electron';
const { ipcRenderer } = electron;
import React from 'react';
import { default as Component } from '../components/Index';

interface Props {
  defaultSeed: string;
  defaultInstallDirectory: string;
  defaultEasyMode: boolean;
}

const initialState = {
  seed: '',
  installDirectory: '',
  easyMode: false,
  snackbar: '',
  isProcessingApply: false,
  isProcessingRestore: false,
};

export default class Index extends React.Component<Props, typeof initialState> {
  constructor(props: Props) {
    super(props);
    this.onChangeSeed = this.onChangeSeed.bind(this);
    this.onChangeInstallDirectory = this.onChangeInstallDirectory.bind(this);
    this.onChangeEasyMode = this.onChangeEasyMode.bind(this);
    this.onClickApply = this.onClickApply.bind(this);
    this.onClickRestore = this.onClickRestore.bind(this);
    this.onCloseSnackbar = this.onCloseSnackbar.bind(this);
    this.state = {
      ...initialState,
      seed: props.defaultSeed,
      installDirectory: props.defaultInstallDirectory,
      easyMode: props.defaultEasyMode,
    };

    ipcRenderer.on('result', (ev: any, message: string) => {
      this.setState({
        ...this.state,
        isProcessingApply: false,
        isProcessingRestore: false,
        snackbar: message,
      });
    });
  }

  private onChangeSeed(seed: string) {
    ipcRenderer.send('setSeed', seed);
    this.setState({
      ...this.state,
      seed,
    });
  }

  private onChangeInstallDirectory(path: string) {
    ipcRenderer.send('setInstallDirectory', path);
    this.setState({
      ...this.state,
      installDirectory: path,
    });
  }

  private onChangeEasyMode(easyMode: boolean) {
    ipcRenderer.send('setEasyMode', easyMode);
    this.setState({
      ...this.state,
      easyMode,
    });
  }

  private onClickApply() {
    this.setState({
      ...this.state,
      isProcessingApply: true,
      snackbar: '',
    });
    ipcRenderer.send('apply');
  }

  private onClickRestore() {
    this.setState({
      ...this.state,
      isProcessingRestore: true,
      snackbar: '',
    });
    ipcRenderer.send('restore');
  }

  private onCloseSnackbar(event: React.SyntheticEvent<any>, reason?: string) {
    if (reason === 'clickaway') {
      return;
    }
    this.setState({
      ...this.state,
      snackbar: '',
    });
  }

  render() {
    return (
      <Component
        {...this.state}
        onChangeSeed={this.onChangeSeed}
        onChangeInstallDirectory={this.onChangeInstallDirectory}
        onChangeEasyMode={this.onChangeEasyMode}
        onClickApply={this.onClickApply}
        onClickRestore={this.onClickRestore}
        onCloseSnackbar={this.onCloseSnackbar}
      />
    );
  }
}
