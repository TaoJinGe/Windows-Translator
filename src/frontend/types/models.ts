export interface ModelOption {
  label: string;
  value: string;
}

export interface ProviderOption {
  label: string;
  apiBaseUrl: string;
  models: ModelOption[];
}

export const providerOptions: ProviderOption[] = [
  {
    label: "MiniMax",
    apiBaseUrl: "https://api.minimaxi.com/v1",
    models: [
      { label: "MiniMax-M2.7", value: "MiniMax-M2.7" },
      { label: "MiniMax-M1", value: "MiniMax-M1" },
      { label: "MiniMax-Text-01", value: "MiniMax-Text-01" }
    ]
  },
  {
    label: "OpenAI",
    apiBaseUrl: "https://api.openai.com/v1",
    models: [
      { label: "gpt-4o-mini", value: "gpt-4o-mini" },
      { label: "gpt-4o", value: "gpt-4o" },
      { label: "gpt-4.1-mini", value: "gpt-4.1-mini" }
    ]
  },
  {
    label: "DeepSeek",
    apiBaseUrl: "https://api.deepseek.com/v1",
    models: [
      { label: "deepseek-chat", value: "deepseek-chat" },
      { label: "deepseek-reasoner", value: "deepseek-reasoner" }
    ]
  },
  {
    label: "Qwen",
    apiBaseUrl: "https://dashscope.aliyuncs.com/compatible-mode/v1",
    models: [
      { label: "qwen-plus", value: "qwen-plus" },
      { label: "qwen-turbo", value: "qwen-turbo" },
      { label: "qwen-max", value: "qwen-max" }
    ]
  },
  {
    label: "Moonshot",
    apiBaseUrl: "https://api.moonshot.cn/v1",
    models: [
      { label: "moonshot-v1-8k", value: "moonshot-v1-8k" },
      { label: "moonshot-v1-32k", value: "moonshot-v1-32k" }
    ]
  },
  {
    label: "自定义 OpenAI 兼容",
    apiBaseUrl: "",
    models: [{ label: "自定义模型名", value: "" }]
  }
];
