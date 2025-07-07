import { useEffect, useState } from 'react';
import { PaginationSection } from '../PaginationSection';
import LogContentHeader from './LogContentHeader';
import LogContentList from './LogContentList';
import React from 'react';
import { ApiService } from '../../services/ApiService';
import { PaginationModel, ParsedApiLogs } from '../../LogTypes';
import { LogUtilService } from '../../services/UtilService';

const api = new ApiService();
const util = new LogUtilService();

export interface LogContentSectionProps {
  filename: string;
}
const fetchLogData = async (
  filename: string,
  page?: number,
  page_size?: number
): Promise<ParsedApiLogs> => {
  const logDataResp = await api.getLogContents(
    filename,
    page || 0,
    page_size || 0
  );
  return util.mapLogData(logDataResp);
};

const defaultPaging: PaginationModel = {
  pageSize: 25,
  page: 0,
};

export default function LogContentSection(
  props: LogContentSectionProps
): React.ReactElement {
  const [paginationModel, setPaginationModel] = useState(defaultPaging);
  let [logData, setLogData] = useState<ParsedApiLogs>();

  const setLogs = () => {
    fetchLogData(
      props.filename,
      paginationModel.page + 1,
      paginationModel.pageSize
    ).then((parsedLogs) => {
      setLogData(parsedLogs);
      setPaginationModel(paginationModel);
    });
  };

  const resetPagination = () => {
      if (logData && props.filename !== logData.filename) {
        setPaginationModel(defaultPaging);
      }
  };

  useEffect(setLogs, [paginationModel, props.filename]);
  useEffect(resetPagination, [props.filename, logData]);
  return (
    <div>
      <LogContentHeader filename={props.filename} />
      {logData?.results && logData?.results.length && (
        <LogContentList
          pagination={paginationModel}
          logContents={logData?.results}
        />
      )}
      <PaginationSection
        results={logData}
        pagination={paginationModel}
        update={setPaginationModel}
      />
    </div>
  );
}
