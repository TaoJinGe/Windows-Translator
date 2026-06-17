import { providerOptions } from "../types/models";

export function findSelectedProvider(apiBaseUrl: string) {
  return (
    providerOptions.find((item) => item.apiBaseUrl === apiBaseUrl) ??
    providerOptions[providerOptions.length - 1]
  );
}
