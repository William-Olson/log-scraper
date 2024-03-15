import React, { CSSProperties } from 'react';
import { List, ListItem, ListItemButton, ListItemText, Typography } from '@mui/material';
import VisibilityIcon from '@mui/icons-material/Visibility';

export interface LogFileListProps {
  setFilename: (val: string) => void;
  selectedFile?: string;
  logFiles: string[];
}

interface FilenameData {
  id: number;
  filename: string;
  view: boolean;
}

export default function LogFileList(
  props: LogFileListProps
): React.ReactElement {
  // build list data
  const rows: FilenameData[] = (props.logFiles || []).filter(x => x).map((l, i) => ({
    id: i + 1,
    filename: l,
    view: true,
  }));

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

  const buildTable = (rows: FilenameData[]) => {
    return (
      <List disablePadding style={listStyles.table} dense>
        {rows.map((row) => {
          return (
            <ListItem
              onClick={() => onFilenameClick(row)}
              style={listStyles.row}
              key={`file-item-${row.id}`}
            >
              <ListItemButton>
                <ListItemText primary={row.filename} />
                {props.selectedFile && props.selectedFile === row.filename && (
                  <VisibilityIcon style={{ color: 'grey' }} />
                )}
              </ListItemButton>
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
