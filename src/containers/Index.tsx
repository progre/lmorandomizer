import { SnackbarCloseReason } from '@mui/material';
import { invoke } from '@tauri-apps/api/core';
import { error } from '@tauri-apps/plugin-log';
import React from 'react';
import { default as Component } from '../components/Index';

function toDifficulty(state: typeof initialState): number {
  if (state.absolutelyShuffle) {
    return 4;
  } else if (state.needGlitches) {
    return 3;
  } else if (state.shuffleSecretRoms) {
    return 2;
  } else if (state.shuffleSecretItems) {
    return 1;
  } else {
    return 0;
  }
}

interface Props {
  defaultSeed: string;
  defaultInstallDirectory: string;
  defaultShuffleSecretItems: boolean;
  defaultShuffleSecretRoms: boolean;
  defaultNeedGlitches: boolean;
  defaultAbsolutelyShuffle: boolean;
}

const initialState = {
  seed: '',
  installDirectory: '',
  shuffleSecretItems: false,
  shuffleSecretRoms: false,
  needGlitches: false,
  absolutelyShuffle: false,
  snackbar: '',
  isProcessingApply: false,
  isProcessingRestore: false,
};

export default class Index extends React.Component<Props, typeof initialState> {
  constructor(props: Props) {
    super(props);
    this.onChangeSeed = this.onChangeSeed.bind(this);
    this.onChangeInstallDirectory = this.onChangeInstallDirectory.bind(this);
    this.onChangeDifficulty = this.onChangeDifficulty.bind(this);
    this.onClickApply = this.onClickApply.bind(this);
    this.onClickRestore = this.onClickRestore.bind(this);
    this.onCloseSnackbar = this.onCloseSnackbar.bind(this);
    this.state = {
      ...initialState,
      seed: props.defaultSeed,
      installDirectory: props.defaultInstallDirectory,
      shuffleSecretItems: props.defaultShuffleSecretItems,
      shuffleSecretRoms: props.defaultShuffleSecretRoms,
      needGlitches: props.defaultNeedGlitches,
      absolutelyShuffle: props.defaultAbsolutelyShuffle,
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

  private onChangeDifficulty(difficulty: number) {
    const shuffleSecretItems = difficulty >= 1;
    const shuffleSecretRoms = difficulty >= 2;
    const needGlitches = difficulty >= 3;
    const absolutelyShuffle = difficulty >= 4;

    invoke('set_shuffle_secret_items', { value: shuffleSecretItems }).catch(
      error
    );
    invoke('set_shuffle_secret_roms', { value: shuffleSecretRoms }).catch(
      error
    );
    invoke('set_need_glitches', { value: needGlitches }).catch(error);
    invoke('set_absolutely_shuffle', { value: absolutelyShuffle }).catch(error);
    this.setState({
      ...this.state,
      shuffleSecretItems: shuffleSecretItems,
      shuffleSecretRoms: shuffleSecretRoms,
      needGlitches,
      absolutelyShuffle,
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
      result = await invoke('apply', {
        installDirectory: this.state.installDirectory,
        options: {
          seed: this.state.seed,
          shuffleSecretItems: this.state.shuffleSecretItems,
          shuffleSecretRoms: this.state.shuffleSecretRoms,
          needGlitches: this.state.needGlitches,
          absolutelyShuffle: this.state.absolutelyShuffle,
        },
      });
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
      result = await invoke('restore', {
        installDirectory: this.state.installDirectory,
      });
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
        difficulty={toDifficulty(this.state)}
        onChangeSeed={this.onChangeSeed}
        onChangeInstallDirectory={this.onChangeInstallDirectory}
        onChangeDifficulty={this.onChangeDifficulty}
        onClickApply={this.onClickApply}
        onClickRestore={this.onClickRestore}
        onCloseSnackbar={this.onCloseSnackbar}
      />
    );
  }
}
