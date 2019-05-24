// tslint:disable-next-line:no-implicit-dependencies
import electron from 'electron';
const { ipcRenderer } = electron;
import React from 'react';
import App from '../applications/app';
import { default as Component } from '../components/Index';

interface Props {
  defaultSeed: string;
  defaultInstallDirectory: string;
  defaultEasyMode: boolean;
  defaultTabletSave: boolean;
  defaultGrailStart: boolean;
  defaultScannerStart: boolean;
  defaultGameMasterStart: boolean;
  defaultReaderStart: boolean;
}

const initialState = {
  seed: '',
  installDirectory: '',
  easyMode: false,
  tabletSave: false,
  grailStart: true,
  scannerStart: false,
  gameMasterStart: true,
  readerStart: false,
  snackbar: '',
  isProcessingApply: false,
  isProcessingRestore: false,
};

export default class Index extends React.Component<Props, typeof initialState> {
  private app = new App();

  constructor(props: Props) {
    super(props);
    this.onChangeSeed = this.onChangeSeed.bind(this);
    this.onChangeInstallDirectory = this.onChangeInstallDirectory.bind(this);
    this.onChangeEasyMode = this.onChangeEasyMode.bind(this);
    this.onChangeTabletSave = this.onChangeTabletSave.bind(this);
    this.onChangeGrailStart = this.onChangeGrailStart.bind(this);
    this.onChangeScannerStart = this.onChangeScannerStart.bind(this);
    this.onChangeGameMasterStart = this.onChangeGameMasterStart.bind(this);
    this.onChangeReaderStart = this.onChangeReaderStart.bind(this);
    this.onClickApply = this.onClickApply.bind(this);
    this.onClickRestore = this.onClickRestore.bind(this);
    this.onCloseSnackbar = this.onCloseSnackbar.bind(this);
    this.state = {
      ...initialState,
      seed: props.defaultSeed,
      installDirectory: props.defaultInstallDirectory,
      easyMode: props.defaultEasyMode,
      tabletSave: props.defaultTabletSave,
      grailStart: props.defaultGrailStart,
      scannerStart: props.defaultScannerStart,
      gameMasterStart: props.defaultGameMasterStart,
      readerStart: props.defaultReaderStart,
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
  
  private onChangeTabletSave(tabletSave: boolean) {
    ipcRenderer.send('setTabletSave', tabletSave);
    this.setState({
      ...this.state,
      tabletSave,
    });
  }

  private onChangeGrailStart(grailStart: boolean) {
    ipcRenderer.send('setGrailStart', grailStart);
    this.setState({
      ...this.state,
      grailStart,
    });
  }

  private onChangeScannerStart(scannerStart: boolean) {
    ipcRenderer.send('setScannerStart', scannerStart);
    this.setState({
      ...this.state,
      scannerStart,
    });
  }

  private onChangeGameMasterStart(gameMasterStart: boolean) {
    ipcRenderer.send('setGameMasterStart', gameMasterStart);
    this.setState({
      ...this.state,
      gameMasterStart,
    });
  }

  private onChangeReaderStart(readerStart: boolean) {
    ipcRenderer.send('setReaderStart', readerStart);
    this.setState({
      ...this.state,
      readerStart,
    });
  }

  private async onClickApply() {
    this.setState({
      ...this.state,
      isProcessingApply: true,
      snackbar: '',
    });
    let result;
    try {
      result = await this.app.apply(
        this.state.installDirectory,
        {
          seed: this.state.seed || '',
          easyMode: this.state.easyMode || false,
          tabletSave: this.state.tabletSave || false,
          grailStart: this.state.grailStart || false,
          scannerStart: this.state.scannerStart || false,
          gameMasterStart: this.state.gameMasterStart || false,
          readerStart: this.state.readerStart || false,
        },
      );
    } catch (err) {
      console.error(err);
      result = err.toString();
    }
    this.setState({
      ...this.state,
      isProcessingApply: false,
      isProcessingRestore: false,
      snackbar: result,
    });
  }

  private async onClickRestore() {
    this.setState({
      ...this.state,
      isProcessingRestore: true,
      snackbar: '',
    });
    let result;
    try {
      result = await this.app.restore(this.state.installDirectory);
    } catch (err) {
      console.error(err);
      result = err.toString();
    }
    this.setState({
      ...this.state,
      isProcessingApply: false,
      isProcessingRestore: false,
      snackbar: result,
    });
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
		onChangeTabletSave={this.onChangeTabletSave}
        onChangeGrailStart={this.onChangeGrailStart}
        onChangeScannerStart={this.onChangeScannerStart}
        onChangeGameMasterStart={this.onChangeGameMasterStart}
        onChangeReaderStart={this.onChangeReaderStart}
        onClickApply={this.onClickApply}
        onClickRestore={this.onClickRestore}
        onCloseSnackbar={this.onCloseSnackbar}
      />
    );
  }
}
