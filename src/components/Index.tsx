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
  isProcessingLaunch: boolean;

  onChangeSeed(seed: string): void;
  onChangeInstallDirectory(path: string): void;
  onChangeDifficulty(difficulty: number): void;
  onClickLaunch(): void;
  onClickOpenFolder(): void;
  onCloseSnackbar(
    event: React.SyntheticEvent<any> | Event,
    reason?: SnackbarCloseReason | null,
  ): void;
}) {
  const loading = props.isProcessingLaunch;
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
          <Button
            variant="contained"
            color="inherit"
            disabled={loading}
            onClick={props.onClickOpenFolder}
            style={{ position: 'relative' }}
          >
            Open Folder
          </Button>
          <div style={{ marginLeft: 16, position: 'relative' }}>
            <Button
              variant="contained"
              disabled={loading}
              onClick={props.onClickLaunch}
            >
              Launch
            </Button>
            {!props.isProcessingLaunch ? (
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
