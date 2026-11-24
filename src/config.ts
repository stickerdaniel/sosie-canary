export interface Config {
  retries: number;
  timeoutMs: number;
}

export const defaults: Config = {
  retries: 5,
  timeoutMs: 5000,
};

export function merge(a: Config, b: Partial<Config>): Config {
  return { ...a, ...b };
}
