import { get, ResponseType } from 'positron-components/backend';

export enum SSOType {
  None = 'None',
  Oidc = 'Oidc'
}

export interface SSOConfig {
  sso_type: SSOType;
  instant_redirect: boolean;
}

export const get_sso_config = async () => {
  let res = await get<SSOConfig>('/api/auth/sso_config', ResponseType.Json);
  if (typeof res === 'object') {
    return res;
  }
};

export const get_oidc_url = async () => {
  let res = await get<{ url: string }>('/api/auth/oidc_url', ResponseType.Json);
  if (typeof res === 'object' && 'url' in res) {
    return res.url;
  }
};
