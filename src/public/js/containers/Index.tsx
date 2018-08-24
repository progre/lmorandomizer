// tslint:disable-next-line:no-implicit-dependencies
import electron from 'electron';
const { ipcRenderer } = electron;
import React from 'react';
import { default as Component } from '../components/Index';

interface Props {
  defaultInstallDirectory: string;
}

const initialState = {
  isProcessingApply: false,
  isProcessingRestore: false,
  installDirectory: '',
  snackbar: '',
};

export default class Index extends React.Component<Props, typeof initialState> {
  constructor(props: Props) {
    super(props);
    this.onChangeInstallDirectory = this.onChangeInstallDirectory.bind(this);
    this.onClickApply = this.onClickApply.bind(this);
    this.onClickRestore = this.onClickRestore.bind(this);
    this.onCloseSnackbar = this.onCloseSnackbar.bind(this);
    this.state = {
      ...initialState,
      installDirectory: props.defaultInstallDirectory,
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

  private onChangeInstallDirectory(ev: React.ChangeEvent<HTMLInputElement>) {
    ipcRenderer.send('setInstallDirectory', ev.target.value);
    this.setState({
      ...this.state,
      installDirectory: ev.target.value,
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
        onChangeInstallDirectory={this.onChangeInstallDirectory}
        onClickApply={this.onClickApply}
        onClickRestore={this.onClickRestore}
        onCloseSnackbar={this.onCloseSnackbar}
      />
    );
  }
}
