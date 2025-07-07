import { ApiLogContent } from './ApiService';
import { LogMessageData, ParsedApiLogs } from '../LogTypes';

export class LogUtilService {
  constructor() {
    this.boolBranchMap = this.boolBranchMap.bind(this);
    this.isJsonObjectString = this.isJsonObjectString.bind(this);
    this.mapLogData = this.mapLogData.bind(this);
    this.toDefaultLogMessageData = this.toDefaultLogMessageData.bind(this);
    this.toJson = this.toJson.bind(this);
  }

  /*
    Converts an ApiLogContent response to a ParsedApiLogs object.
  */
  public mapLogData(logDataResp: ApiLogContent): ParsedApiLogs {
    const offset = (logDataResp.page - 1) * logDataResp.page_size;
    const converter = this.boolBranchMap(this.isJsonObjectString, this.toJson, this.toDefaultLogMessageData);
    return {
      filename: logDataResp.filename,
      results: logDataResp.results
        .map(converter)
        .map((j, i) => ({ ...j, id: i + 1 + offset })) as Array<LogMessageData>,
      page: logDataResp.page,
      page_size: logDataResp.page_size,
      total: logDataResp.total,
    } as ParsedApiLogs;
  }

  /*
    Creates a default LogMessageData object with only the message
    field populated from input value parameter.
  */
  private toDefaultLogMessageData(value: any): LogMessageData {
    return {
      loggerName: '',
      requestId: '',
      logtype: 'info',
      message: typeof value === 'string' ? value : JSON.stringify(value),
      messageId: '',
      project: '',
      timestamp: 0,
    } as LogMessageData;
  }

  /*
    Returns a function that accepts a mappable item (I) and invokes
    the trueBranchFn if conditionFn returns true else calls falseBranchFn.
  */
  private boolBranchMap<I, T>(
    conditionFn: (item: I) => boolean,
    trueBranchFn: (item: I) => T,
    falseBranchFn: (item: I) => T
  ): (item: I) => T {
    return function returnedMapFn(item: I): T {
      if (conditionFn(item)) {
        return trueBranchFn(item);
      }
      return falseBranchFn(item);
    };
  }

  /*
    Checks the passed in parameter if it's a string of an object or an array.
  */
  public isJsonObjectString(val?: string | object): boolean {
    if (!val) {
      return false;
    }

    let maybeJson = val;
    if (typeof val === 'object') {
      maybeJson = JSON.stringify(val);
    }

    try {
      if (
        typeof maybeJson === 'string' &&
        (maybeJson.trim().startsWith('{') || maybeJson.trim().startsWith('['))
      ) {
        const result = JSON.parse(maybeJson);
        return !!result;
      } else {
        return false;
      }
    } catch (err) {
      // console.error(err);
      return false;
    }
  }

  /*
    Attempts to parse the passed in parameter as a JSON string and returns the result.
  */
  public toJson(val: string | object): any {
    if (!val) {
      return undefined;
    }

    let jsonString: string =
      typeof val !== 'string' ? JSON.stringify(val) : val;

    try {
      return JSON.parse(jsonString);
    } catch (err) {
      console.error('Error parsing Json Object String: ', err);
      return undefined;
    }
  }
}
