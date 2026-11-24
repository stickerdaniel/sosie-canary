export interface Config {
  retries: number;
  timeoutMs: number;
  verbose: boolean;
}

export const defaults: Config = {
  retries: 3,
  timeoutMs: 5000,
  verbose: false,
};

export function merge(a: Config, b: Partial<Config>): Config {
  return { ...a, ...b };
}
