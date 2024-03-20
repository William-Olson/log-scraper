import * as React from 'react';
import Button from '@mui/material/Button';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogContentText from '@mui/material/DialogContentText';
import DialogTitle from '@mui/material/DialogTitle';

export interface ConfirmModalProps {
    isOpen: boolean;
    text?: string;
    title?: string;
    data?: any;
    setOpen: (v: boolean) => void;
    confirm: (v?: any) => void;
    cancel: () => void;
}

export default function ConfirmModal(props: ConfirmModalProps) {
  return (
    <React.Fragment>
      <Dialog
        open={props.isOpen}
        onClose={() => { console.log('closing modal') }}
        aria-labelledby="alert-dialog-title"
        aria-describedby="alert-dialog-description"
      >
        <DialogTitle id="alert-dialog-title">
          {props.title || "Confirm Action"}
        </DialogTitle>
        <DialogContent>
          <DialogContentText id="alert-dialog-description">
           {props.text || "Are you sure?"}
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => props.cancel()}>Cancel</Button>
          <Button onClick={() => props.confirm(props.data)} autoFocus>
            Confirm
          </Button>
        </DialogActions>
      </Dialog>
    </React.Fragment>
  );
}