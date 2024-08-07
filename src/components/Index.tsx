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
import React from 'react';
import Difficulty from './Difficulty';

export default function Index(props: {
  seed: string;
  installDirectory: string;
  difficulty: number;
  snackbar: string;
  isProcessingApply: boolean;
  isProcessingRestore: boolean;

  onChangeSeed(seed: string): void;
  onChangeInstallDirectory(path: string): void;
  onChangeDifficulty(difficulty: number): void;
  onClickApply(): void;
  onClickRestore(): void;
  onCloseSnackbar(
    event: React.SyntheticEvent<any> | Event,
    reason?: SnackbarCloseReason | null
  ): void;
}) {
  const loading = props.isProcessingApply || props.isProcessingRestore;
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
        <div
          style={{
            marginTop: 16,
            display: 'flex',
            justifyContent: 'flex-end',
          }}
        >
          <div style={{ position: 'relative' }}>
            <Button
              variant="contained"
              color="inherit"
              disabled={loading}
              onClick={props.onClickRestore}
            >
              Restore
            </Button>
            {!props.isProcessingRestore ? (
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
          <div style={{ marginLeft: 16, position: 'relative' }}>
            <Button
              variant="contained"
              disabled={loading}
              onClick={props.onClickApply}
            >
              Apply
            </Button>
            {!props.isProcessingApply ? (
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
        </div>
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
