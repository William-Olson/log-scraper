import React, { CSSProperties, useCallback, useEffect, useState } from 'react';
import { Tab, Tabs, Typography } from '@mui/material';
import DescriptionIcon from '@mui/icons-material/Description';
import InfoIcon from '@mui/icons-material/Info';
import LogFileList from './LogFileList';
import { ApiService } from '../../services/ApiService';
import { TabPanel } from './TabPanel';
import pJson from '../../../package.json';

const LOG_FILE_REGEXP = /.*_\d\d\d\d-\d\d-\d\d[.].*/gim;
const LOG_FILE_NAME_DELIMITER = '_';

const api = new ApiService();

export interface LogFilesSectionProps {
  setLogFiles: (val: string[]) => void;
  logFiles: string[];
  setFilename: (val: string) => void;
  selectedFile?: string;
}

function LogFilesSection(props: LogFilesSectionProps): React.ReactElement {
  // tabs
  const [selectedTab, selectTab] = useState<number>(0);
  const styles: {[k: string]: CSSProperties} = {
    defaultBanner: {
      padding: '15px'
    },
    tabPanel: {
      width: '100%',
      padding: '0px',
      margin: '5px 0px'
    }
  };

  const { logFiles, setLogFiles } = props;
  const fetchLogFiles = useCallback(() => api
    .getLogList()
    // sort the log files by dates in the names
    .then(({ log_files }) => log_files.sort((entryA, entryB) => {
      const isLogFile = (s: string = '') => LOG_FILE_REGEXP.test(s);
      if (!isLogFile(entryA) || !isLogFile(entryB)) {
        return entryA < entryB ? 1 : (entryA === entryB ? 0 : -1);
      }
      const fileANoExt = entryA.split('.')[0] || '';
      const fileBNoExt = entryB.split('.')[0] || '';
      const fileADate = new Date((fileANoExt || '').split(LOG_FILE_NAME_DELIMITER).pop() as string);
      const fileBDate = new Date((fileBNoExt || '').split(LOG_FILE_NAME_DELIMITER).pop() as string);

      return fileADate > fileBDate ? 1 : -1;
    }))
    .then(setLogFiles), [setLogFiles]);

  // init log files
  useEffect(() => {
    if (logFiles?.length > 0) {
      return;
    }
    fetchLogFiles();
  }, [logFiles?.length, setLogFiles, fetchLogFiles]);

  // handler for updating selected log file
  const updateSelectedFile = (value: string) => {
    props.setFilename(value);
  };

  return (
    <div>
      <Tabs
        value={selectedTab}
        onChange={(_: any, val: any) => selectTab(val)}
        style={{
          // position: 'absolute',
          textAlign: 'left',
          top: '0',
        }}
      >
        <Tab value={0} label={<span><DescriptionIcon /></span>} />
        <Tab value={1} label={<span><InfoIcon /></span>} />
      </Tabs>
      <TabPanel value={0} index={selectedTab} style={styles.tabPanel}>
        <div>
          {logFiles && logFiles.length > 0 && (
            <LogFileList
              refresh={() => fetchLogFiles()}
              selectedFile={props.selectedFile}
              setFilename={updateSelectedFile}
              logFiles={logFiles}
            />
          )}
          {(!logFiles || !logFiles.length) && (
            <Typography variant='caption'>No Log Files Found</Typography>
          )}
        </div>
      </TabPanel>
      <TabPanel value={1} index={selectedTab} style={styles.tabPanel}>
        <div style={styles.defaultBanner}>
          <Typography>
            App Version: {pJson.version}
          </Typography>
          <Typography>
            React Version: {React.version}
          </Typography>
        </div>
      </TabPanel>
    </div>
  );
}

export default LogFilesSection;
