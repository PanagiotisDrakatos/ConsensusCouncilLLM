const allowedRedirects = new Set(["/dashboard", "/settings"]);
const ALLOWED_REDIRECT_BASE = "https://app.example.com";

export function normalizeRedirectTarget(target: string): string | null {
  const candidate = target.trim();
  if (!candidate) {
    return null;
  }

  try {
    const parsed = new URL(candidate, ALLOWED_REDIRECT_BASE);
    if (parsed.origin !== ALLOWED_REDIRECT_BASE) {
      return null;
    }

    const normalized = `${parsed.pathname}${parsed.search}${parsed.hash}`;
    return normalized || "/";
  } catch {
    return null;
  }
}

export function resolveRedirectTarget(input: string): string | null {
  const normalized = normalizeRedirectTarget(input);
  if (!normalized) {
    return null;
  }

  return allowedRedirects.has(normalized) ? normalized : null;
}
