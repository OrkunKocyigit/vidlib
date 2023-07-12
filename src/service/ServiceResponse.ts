export enum ResponseType {
  SUCCESS,
  FAILURE
}

export interface IServiceResponse<T> {
  result: ResponseType;
  response?: T;
  error?: string;
}

export class ServiceResponse<T> implements IServiceResponse<T> {
  error?: string;
  response?: T;
  result: ResponseType;

  constructor(result: ResponseType, response?: T) {
    this.response = response;
    this.result = result;
  }
}
