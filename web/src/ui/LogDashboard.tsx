import React, { CSSProperties, useState } from 'react';

import LogContentSection from './log-content/LogContentSection';
import LogFilesSection from './log-files/LogFilesSection';

export function LogDashboard(): React.ReactElement {
  const [selectedFilename, setSelectedFilename] = useState<string>();
  const [logFiles, setLogFiles] = useState<string[]>([]);

  const dashboardStyles: { [k: string]: CSSProperties } = {
    logFiles: {
      position: 'absolute',
      bottom: '0px',
      height: '350px',
      background: 'rgba(0, 0, 0, 0.93)',
      color: '#eee',
      border: '1px solid black',
      width: '100%',
    },
    logContent: {
      height: 'calc(100vh - 366px)',
      background: 'rgba(0, 0, 0, 0.9)',
      color: 'cyan'
    },
    defaultBanner: {
      height: 'calc(100vh - 366px - 100px)', 
      background: 'rgba(0, 0, 0, 0.9)',
      padding: '50px 200px',
      color: 'grey'
    },
  };

  return (
    <div>
      {!selectedFilename && (
        <div style={dashboardStyles.defaultBanner}>
          <h4>Select a file below to view its contents.</h4>
        </div>
      )}
      {selectedFilename && (
        <div style={dashboardStyles.logContent}>
          <LogContentSection filename={selectedFilename} />
        </div>
      )}
      <div style={dashboardStyles.logFiles}>
        <LogFilesSection
          logFiles={logFiles}
          setLogFiles={setLogFiles}
          setFilename={setSelectedFilename}
          selectedFile={selectedFilename}
        />
      </div>
    </div>
  );
}
