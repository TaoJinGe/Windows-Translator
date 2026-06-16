export function hasText(value: string): boolean {
  return value.trim().length > 0;
}

export function normalizeError(error: unknown): string {
  const message = errorMessage(error);

  if (isApiKeyError(message)) {
    return "API Key 缺失：请先在设置中填写 API Key。";
  }

  if (isNetworkError(message)) {
    return "网络连接失败：请检查网络连接、API 地址或代理设置。";
  }

  const statusCode = apiStatusCode(message);

  if (statusCode) {
    return `接口请求失败（HTTP ${statusCode}）：${apiStatusHint(statusCode)}`;
  }

  if (isStreamParseError(message)) {
    return "流式解析失败：接口返回的流式数据格式异常，请关闭流式输出后重试。";
  }

  if (isEmptyResultError(message)) {
    return "接口返回为空：请稍后重试，或检查模型和提示词设置。";
  }

  return message || "操作失败";
}

function errorMessage(error: unknown): string {
  if (typeof error === "string") {
    return error;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return "";
}

function isApiKeyError(message: string): boolean {
  return message.includes("请先填写 API Key") || /api key.*(missing|required|empty)/i.test(message);
}

function isNetworkError(message: string): boolean {
  return /network connection failed|failed to fetch|networkerror/i.test(message);
}

function apiStatusCode(message: string): number | undefined {
  const match = message.match(/API request failed:\s*(\d{3})/i);
  return match ? Number(match[1]) : undefined;
}

function apiStatusHint(statusCode: number): string {
  if (statusCode === 401 || statusCode === 403) {
    return "请检查 API Key 是否正确或是否有权限。";
  }

  if (statusCode === 404) {
    return "请检查 API 地址和模型名称是否正确。";
  }

  if (statusCode === 429) {
    return "请求过于频繁或额度不足，请稍后重试。";
  }

  if (statusCode >= 500) {
    return "服务端暂时异常，请稍后重试。";
  }

  return "请检查接口配置后重试。";
}

function isStreamParseError(message: string): boolean {
  return /translation stream format is invalid/i.test(message);
}

function isEmptyResultError(message: string): boolean {
  return /translation result is empty/i.test(message);
}
