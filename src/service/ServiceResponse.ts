export enum ResponseType {
  SUCCESS,
  FAILURE
}

export interface ServiceResponse<T> {
  result: ResponseType;
  response?: T;
  error?: string;
}
