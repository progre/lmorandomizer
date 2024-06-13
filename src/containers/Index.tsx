import { SnackbarCloseReason } from '@mui/material';
import { invoke } from '@tauri-apps/api/core';
import { error } from '@tauri-apps/plugin-log';
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
  }

  private onChangeSeed(seed: string) {
    invoke('set_seed', { value: seed }).catch(error);
    this.setState({
      ...this.state,
      seed,
    });
  }

  private onChangeInstallDirectory(path: string) {
    invoke('set_install_directory', { value: path }).catch(error);
    this.setState({
      ...this.state,
      installDirectory: path,
    });
  }

  private onChangeEasyMode(easyMode: boolean) {
    invoke('set_easy_mode', { value: easyMode }).catch(error);
    this.setState({
      ...this.state,
      easyMode,
    });
  }

  private async onClickApply() {
    this.setState({
      ...this.state,
      isProcessingApply: true,
      snackbar: '',
    });
    let result: string;
    try {
      result = await invoke(
        'apply',
        {
          installDirectory: this.state.installDirectory,
          options: {
            seed: this.state.seed || '',
            easyMode: this.state.easyMode || false,
          }
        }
      );
    } catch (err) {
      console.error(err);
      result = `${err}`;
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
    let result: string;
    try {
      result = await invoke(
        'restore',
        { installDirectory: this.state.installDirectory },
      );
    } catch (err) {
      console.error(err);
      result = `${err}`;
    }
    this.setState({
      ...this.state,
      isProcessingApply: false,
      isProcessingRestore: false,
      snackbar: result,
    });
  }

  private onCloseSnackbar(
    _event: React.SyntheticEvent<any>,
    reason?: SnackbarCloseReason | null
  ) {
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
