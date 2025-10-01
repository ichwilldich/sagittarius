import type JSEncrypt from 'jsencrypt';
import { browser } from '$app/environment';
import {
  post,
  get,
  ContentType,
  ResponseType,
  RequestError
} from 'positron-components/backend';

let encrypt: false | undefined | JSEncrypt = $state(browser && undefined);

export const getEncrypt = () => {
  return encrypt;
};

export const fetch_key = async () => {
  if (encrypt === false) {
    return RequestError.Other;
  }

  let key = await get<{ key: string }>('/api/auth/key', ResponseType.Json);

  if (typeof key !== 'object') {
    return key;
  }

  const JSEncrypt = (await import('jsencrypt')).JSEncrypt;

  encrypt = new JSEncrypt({ default_key_size: '4096' });
  encrypt.setPublicKey(key.key);
};
fetch_key();

export const password_login = async (name: string, password: string) => {
  if (!encrypt) {
    return RequestError.Other;
  }

  let encrypted_password = encrypt.encrypt(password);
  let res = await post<undefined>(
    '/api/auth/auth',
    ResponseType.None,
    ContentType.Json,
    JSON.stringify({
      name,
      password: encrypted_password
    })
  );

  if (typeof res === 'string') {
    if (res === RequestError.Unauthorized) {
      fetch_key();
    }
    return res;
  }
};
