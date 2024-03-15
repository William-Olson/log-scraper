

export interface LogMessageData {
  id?: number;
  loggerName: string;
  requestId: string;
  logtype: 'info' | 'warn' | 'error' | 'debug';
  message: string;
  messageId: string; // UUID
  project: string;
  timestamp: number; // Unix timestamp
}

export const LogDataKeys: (keyof LogMessageData)[] = [
  'id',
  'loggerName',
  'requestId',
  'logtype',
  'message',
  'messageId',
  'project',
  'timestamp'
];

export interface ParsedApiLogs {
  page: number;
  page_size: number;
  total: number;
  results: LogMessageData[];
}

export const keysOfLogs = (data: Partial<LogMessageData>) => {
  return Object.entries(data).filter(([key]) => {
    return Object.keys(LogDataKeys).includes(key);
  });
};

export interface PaginationModel {
  pageSize: number;
  page: number;
}