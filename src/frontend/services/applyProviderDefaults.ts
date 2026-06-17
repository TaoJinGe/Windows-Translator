import { providerOptions } from "../types/models";
import type { AppSettings } from "../types/settings";

export function applyProviderDefaults(form: AppSettings, label: string): AppSettings {
  const provider = providerOptions.find((item) => item.label === label);

  if (!provider) {
    return form;
  }

  return {
    ...form,
    apiBaseUrl: provider.apiBaseUrl || form.apiBaseUrl,
    model: provider.models[0]?.value || form.model
  };
}
