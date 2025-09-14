import { ContentType, RequestError, ResponseType } from './types.svelte';

export const post = async <T>(
  path: string,
  res_type: ResponseType,
  content_type: ContentType,
  body: any,
  signal?: AbortSignal
): Promise<T | RequestError> => {
  return await request(path, 'POST', res_type, content_type, body, signal);
};

export const get = async <T>(
  path: string,
  res_type: ResponseType,
  signal?: AbortSignal
): Promise<T | RequestError> => {
  return await request(path, 'GET', res_type, undefined, undefined, signal);
};

const request = async <T>(
  path: string,
  method: string,
  res_type: ResponseType,
  content_type?: ContentType,
  body?: any,
  signal?: AbortSignal
): Promise<T | RequestError> => {
  let headers;
  if (content_type && body) {
    headers = {
      'Content-Type': content_type
    };
  }

  try {
    let res = await fetch!(`${path}`, {
      method,
      headers,
      body,
      signal
    });

    switch (res.status) {
      case 200:
        break;
      case 401:
        return RequestError.Unauthorized;
      case 409:
        return RequestError.Conflict;
      case 410:
        return RequestError.Gone;
      default:
        return RequestError.Other;
    }

    switch (res_type) {
      case ResponseType.Json:
        let json = await res.json();
        return json as T;
      case ResponseType.Text:
        let text = await res.text();
        return text as T;
      case ResponseType.None:
        return undefined as T;
    }
  } catch (_) {
    return RequestError.Other;
  }
};
