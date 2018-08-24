import {
  Button,
  CircularProgress,
  CssBaseline,
  IconButton,
  Paper,
  Snackbar,
  TextField,
  Typography,
} from '@material-ui/core';
import React from 'react';

export default function Index(props: {
  isProcessingApply: boolean;
  isProcessingRestore: boolean;
  installDirectory: string;
  snackbar: string;

  onChangeInstallDirectory(ev: React.ChangeEvent<HTMLInputElement>): void;
  onClickApply(): void;
  onClickRestore(): void;
  onCloseSnackbar(event: React.SyntheticEvent<any>, reason?: string): void;
}) {
  const loading
    = props.isProcessingApply
    || props.isProcessingRestore;
  return (
    <>
      <CssBaseline />
      <div style={{
        height: '100%',
        padding: 16,
        display: 'flex',
        flexDirection: 'column',
      }}>
        <Paper elevation={1} style={{ flex: 1, padding: 16 }}>
          <Typography style={{ fontSize: 14 }}>
            General settings
        </Typography>
          <TextField
            id="name"
            label="La-Mulana install directory"
            value={props.installDirectory}
            onChange={props.onChangeInstallDirectory}
            margin="dense"
            fullWidth
          />
        </Paper>
        <div style={{
          marginTop: 16,
          display: 'flex',
          justifyContent: 'flex-end',
        }}>
          <div style={{ position: 'relative' }}>
            <Button
              variant="contained"
              disabled={loading}
              onClick={props.onClickRestore}
            >
              Restore
          </Button>
            {!props.isProcessingRestore ? '' : (
              <CircularProgress size={24} style={{
                position: 'absolute',
                top: '50%',
                left: '50%',
                marginTop: -12,
                marginLeft: -12,
              }} />
            )}
          </div>
          <div style={{ marginLeft: 16, position: 'relative' }}>
            <Button
              variant="contained"
              color="primary"
              disabled={loading}
              onClick={props.onClickApply}
            >
              Apply
          </Button>
            {!props.isProcessingApply ? '' : (
              <CircularProgress size={24} style={{
                position: 'absolute',
                top: '50%',
                left: '50%',
                marginTop: -12,
                marginLeft: -12,
              }} />
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
