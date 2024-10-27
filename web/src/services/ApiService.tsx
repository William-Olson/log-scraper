import axios from 'axios';


let API_BASE_URL = ''; // just use host server
if (process.env.NODE_ENV !== 'production') {
  // else use service at local port if dev mode
  API_BASE_URL = 'http://localhost:3333';
  // and allow overriding in dev mode via environment
  if (process.env.REACT_APP_LSUI_API_URL) {
    API_BASE_URL = process.env.REACT_APP_LSUI_API_URL;
  }
}


export interface ApiLogContent {
  page: number;
  page_size: number;
  total: number;
  results: string[];
}

export interface ApiLogList {
  ok: boolean;
  timestamp: Date | string;
  log_files: string[];
}

export class ApiService {
  /**
   * Retrieves the existing log files.
   *
   * @returns API Log Files residing on the server filesystem.
   */
  public async getLogList(): Promise<ApiLogList> {
    return (
      await axios.request({
        url: `${API_BASE_URL}/logs/`,
        method: 'GET',
      })
    ).data;
  }

  /**
   * Retrieves File contents of the given file by line up to pageSize max lines.
   *
   * @returns  API Log file contents
   */
  public async getLogContents(
    filename: string,
    page: number,
    pageSize: number
  ): Promise<ApiLogContent> {
    return (
      await axios.request({
        url: `${API_BASE_URL}/logs/${filename}?page=${page}&page_size=${pageSize}`,
        method: 'GET',
      })
    ).data;
  }

  /**
   * Deletes the log file on disk.
   *
   * @returns object Response containing a success or failure boolean.
   */
  public async deleteLogFile(filename: string) {
    console.log(`removing file: ${filename}`);
    return (
      await axios.request({
        url: `${API_BASE_URL}/logs/${filename}`,
        method: 'DELETE',
      })
    ).data;
  }
}
