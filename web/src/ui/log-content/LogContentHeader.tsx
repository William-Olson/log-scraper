import { Typography } from '@mui/material';
import React from 'react';


export default function LogContentHeader(props: { filename: string; }): React.ReactElement {
  return <>
    <Typography variant='h5'  style={{padding: '9px 15px'}}>
      Log Content for {props.filename}
    </Typography>
  </>;
}
