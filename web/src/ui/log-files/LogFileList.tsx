import React, { CSSProperties } from 'react';
import { Box, List, ListItem, ListItemButton, ListItemText, Typography } from '@mui/material';
import VisibilityIcon from '@mui/icons-material/Visibility';
import { DeleteForever } from '@mui/icons-material';
import ConfirmModal from '../ConfirmModal';
import { ApiService } from '../../services/ApiService';

const api = new ApiService();


export interface LogFileListProps {
  setFilename: (val: string) => void;
  refresh: () => void;
  selectedFile?: string;
  logFiles: string[];
}

interface FilenameData {
  id: number;
  filename: string;
  view: boolean;
  date?: Date;
}

const filenameToFilenameData = (filename: string, index: number): FilenameData => {
  const cleanDateString = (s: string) => s.replaceAll(/.*([1-9]\d\d\d-\d\d-\d\d).*/gim, '$1');
  return {
    id: index + 1,
    filename,
    view: true,
    date: filename && cleanDateString(filename) ? new Date(cleanDateString(filename)) : undefined
  };
}

export default function LogFileList(
  props: LogFileListProps
): React.ReactElement {
  // build list data
  let rows: FilenameData[] = (props.logFiles || [])
    .filter(x => x)
    .map(filenameToFilenameData);


  if (rows.every(r => !!r.date)) {
    rows = rows.sort((a, b) => a.date! < b.date! ? 1 : -1);
  }
  
  const listStyles: { [k: string]: CSSProperties } = {
    table: {
      width: '100%',
      overflowY: 'scroll',
      height: '249px',
      margin: 0,
      padding: 0,
    },
    row: {
      cursor: 'pointer',
    },
  };

  const onFilenameClick = (row: FilenameData) => {
    props.setFilename(row.filename);
  };

  // file deletion modal helpers
  const [isDeletionModalOpen, setIsDeletionModalOpen] = React.useState(false);
  const [fileToDelete, setFileToDelete] = React.useState<FilenameData>();
  const toggleModal = () => {
    setIsDeletionModalOpen(!isDeletionModalOpen);
  };
  const onModalConfirm = async (data?: any) => {
    if (fileToDelete?.filename) {
      console.log('Deleting File:', data.filename);
      await api.deleteLogFile(fileToDelete?.filename);
    }
    setFileToDelete(undefined);
    setIsDeletionModalOpen(false);
    props.setFilename('');
    props.refresh();
  };
  const promptDeleteFile = (row: FilenameData) => {
    setFileToDelete(row);
    setIsDeletionModalOpen(true);
  };

  const buildTable = (rows: FilenameData[]) => {
    return (
      <List disablePadding style={listStyles.table} dense>
        <ConfirmModal 
          cancel={() => toggleModal()}
          isOpen={isDeletionModalOpen}
          setOpen={(v: boolean) => setIsDeletionModalOpen(v)}
          data={fileToDelete}
          text={fileToDelete ? `Delete file named ${fileToDelete?.filename}?` : 'Delete File?'}
          confirm={(d?: any) => onModalConfirm(d)}
        />
        {rows.map((row) => {
          return (
            <ListItem
              onClick={() => onFilenameClick(row)}
              style={listStyles.row}
              key={`file-item-${row.id}`}
            >
              <ListItemText primary={row.filename} />
              <Box display="flex" columnGap={2} alignContent="flex-end">
              <ListItemButton>
                {props.selectedFile && props.selectedFile === row.filename && (
                  <VisibilityIcon style={{ color: 'grey' }} />
                )}
              </ListItemButton>
              <ListItemButton>
                {props.selectedFile && props.selectedFile === row.filename && (
                  <DeleteForever onClick={() => promptDeleteFile(row)} style={{ color: 'grey' }} />
                )}
              </ListItemButton>
              </Box>
            </ListItem>
          );
        })}
      </List>
    );
  };

  if (!rows || !rows.length) {
    return <Typography variant='caption'>No Log Files Found</Typography>
  }

  return buildTable(rows);
}
