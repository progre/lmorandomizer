import {
  Button,
  CircularProgress,
  CssBaseline,
  IconButton,
  Paper,
  Snackbar,
  SnackbarCloseReason,
  TextField,
  Typography,
} from '@mui/material';
import { type } from '@tauri-apps/plugin-os';
import React from 'react';
import Difficulty from './Difficulty';

export default function Index(props: {
  seed: string;
  installDirectory: string;
  difficulty: number;
  snackbar: string;
  isProcessingLaunch: boolean;
  isProcessingApply: boolean;
  isProcessingRestore: boolean;

  onChangeSeed(seed: string): void;
  onChangeInstallDirectory(path: string): void;
  onChangeDifficulty(difficulty: number): void;
  onClickLaunch(): void;
  onClickOpenFolder(): void;
  onClickApply(): void;
  onClickRestore(): void;
  onCloseSnackbar(
    event: React.SyntheticEvent<any> | Event,
    reason?: SnackbarCloseReason | null,
  ): void;
}) {
  const loading =
    props.isProcessingLaunch ||
    props.isProcessingApply ||
    props.isProcessingRestore;
  return (
    <>
      <CssBaseline />
      <div
        style={{
          height: '100%',
          padding: 16,
          display: 'flex',
          flexDirection: 'column',
        }}
      >
        <Configs {...props} />
        <Buttons {...props} loading={loading} />
        <Snackbar
          anchorOrigin={{
            vertical: 'bottom',
            horizontal: 'left',
          }}
          open={props.snackbar.length > 0}
          autoHideDuration={6000}
          onClose={props.onCloseSnackbar}
          message={<span>{props.snackbar}</span>}
          action={[
            <IconButton
              key="close"
              aria-label="Close"
              color="inherit"
              onClick={props.onCloseSnackbar}
            >
              ✕
            </IconButton>,
          ]}
        />
      </div>
    </>
  );
}

function Buttons(props: {
  loading: boolean;
  isProcessingLaunch: boolean;
  isProcessingApply: boolean;
  isProcessingRestore: boolean;
  onClickLaunch(): void;
  onClickOpenFolder(): void;
  onClickApply(): void;
  onClickRestore(): void;
}) {
  return (
    <div
      style={{
        marginTop: 16,
        display: 'flex',
        justifyContent: 'flex-end',
      }}
    >
      {type() === 'windows' ? (
        <>
          <Button
            variant="contained"
            color="inherit"
            disabled={props.loading}
            onClick={props.onClickOpenFolder}
            style={{ position: 'relative' }}
          >
            Open Folder
          </Button>
          <BtnWithProgress
            disabled={props.loading}
            processing={props.isProcessingLaunch}
            onClick={props.onClickLaunch}
          >
            Launch
          </BtnWithProgress>
        </>
      ) : (
        <>
          <BtnWithProgress
            disabled={props.loading}
            processing={props.isProcessingRestore}
            color="inherit"
            onClick={props.onClickRestore}
          >
            Restore
          </BtnWithProgress>
          <BtnWithProgress
            disabled={props.loading}
            processing={props.isProcessingApply}
            onClick={props.onClickApply}
          >
            Apply
          </BtnWithProgress>
        </>
      )}
    </div>
  );
}

function BtnWithProgress(props: {
  disabled: boolean;
  processing: boolean;
  color?: typeof Button.prototype.color;
  onClick: () => void;
  children?: React.ReactNode;
}) {
  return (
    <div style={{ marginLeft: 16, position: 'relative' }}>
      <Button
        variant="contained"
        color={props.color}
        disabled={props.disabled}
        onClick={props.onClick}
      >
        {props.children}
      </Button>
      {!props.processing ? (
        ''
      ) : (
        <CircularProgress
          size={24}
          style={{
            position: 'absolute',
            top: '50%',
            left: '50%',
            marginTop: -12,
            marginLeft: -12,
          }}
        />
      )}
    </div>
  );
}

function Configs(props: {
  seed: string;
  installDirectory: string;
  difficulty: number;

  onChangeSeed(seed: string): void;
  onChangeInstallDirectory(path: string): void;
  onChangeDifficulty(difficulty: number): void;
}) {
  return (
    <Paper elevation={1} style={{ flex: 1, padding: 16 }}>
      <Typography style={{ fontSize: 14 }}>General settings</Typography>
      <TextField
        label="Seed"
        value={props.seed}
        onChange={buildOnChangeInputElement(props.onChangeSeed)}
        margin="dense"
        fullWidth
      />
      <TextField
        label="La-Mulana install directory"
        value={props.installDirectory}
        onChange={buildOnChangeInputElement(props.onChangeInstallDirectory)}
        margin="dense"
        fullWidth
      />
      <Typography sx={{ mt: 2, mb: 1, fontSize: 14 }}>Difficulty</Typography>
      <Difficulty
        difficulty={props.difficulty}
        onChange={props.onChangeDifficulty}
      />
    </Paper>
  );
}

function buildOnChangeInputElement(callback: (value: string) => void) {
  return (ev: React.ChangeEvent<HTMLInputElement>) => {
    callback(ev.target.value);
  };
}
