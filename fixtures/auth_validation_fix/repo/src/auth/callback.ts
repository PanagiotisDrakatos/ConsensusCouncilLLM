const allowedRedirects = new Set(["/dashboard", "/settings"]);

export function normalizeRedirectTarget(target: string): string {
  return target;
}

export function resolveRedirectTarget(input: string): string | null {
  if (!input) {
    return null;
  }

  const normalized = normalizeRedirectTarget(input);
  return allowedRedirects.has(normalized) ? normalized : null;
}
