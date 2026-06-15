export function hasText(value: string): boolean {
  return value.trim().length > 0;
}

export function normalizeError(error: unknown): string {
  if (typeof error === "string") {
    return error;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return "操作失败";
}
