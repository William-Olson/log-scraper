import React, { CSSProperties } from 'react';
import { LogMessageData, PaginationModel } from '../../LogTypes';
import { Typography } from '@mui/material';

export interface LogContentListProps {
  logContents: LogMessageData[];
  pagination: PaginationModel;
}

export default function LogContentList(props: LogContentListProps): React.ReactElement {
  const { page, pageSize } = props.pagination;

  const contentStyles: { [k: string]: CSSProperties} = {
    tableArea: {
      border: 'solid 1px rgba(0, 0, 0, 0.1)',
      height: 'calc(100vh - 440px)',
      overflow: 'auto'
    },
    table: {
      borderCollapse: 'collapse',
      width: '100%',
    },
    data: {
      padding: '.5ch 1ch',
      textAlign: 'left'
    }
  };

  const toRenderedRow = (data: LogMessageData) => {
    if (data.project) { // render a new relic log
      const t = new Date(data.timestamp);
      const v = data.logtype.toUpperCase();
      let vColor = 'cyan';
      switch (v) {
        case 'INFO':
          vColor = 'limegreen';
          break;
        case 'WARN':
          vColor = 'yellow';
          break;
        case 'ERROR':
          vColor = 'red';
          break;
        default:
          break;
      }
      const prefix = `${t.toLocaleDateString()} ${t.toLocaleTimeString()} :: ${
        data.requestId
      } :: [`;
      const vHighlight = <span style={{ color: vColor }}>{v}</span>
      const postfix = `] ${data.message}`;
      return <Typography variant='body1'>{prefix}{vHighlight}{postfix}</Typography>;
    }
    return `${data.message}`;
  }

  return <div style={contentStyles.tableArea}>
    <table style={contentStyles.table}>
      <thead>
        <tr>
          <th style={contentStyles.data}>Line Number</th>
          <th style={contentStyles.data}>Message</th>
        </tr>
      </thead>
      <tbody>
       {props.logContents?.length && props.logContents?.map((content, i) => {
        return  <tr key={`content-item-${i}`}>
            <td style={contentStyles.data}>
              {(page) * pageSize + i + 1}
            </td>
            <td style={contentStyles.data}>
              {toRenderedRow(content)}
            </td>
          </tr>
       })}
      </tbody>
    </table>
  </div>;
}
