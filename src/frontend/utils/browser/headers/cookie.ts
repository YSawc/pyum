import { getCookies } from "https://deno.land/std@0.224.0/http/cookie.ts";

export const getTargetCookieVal = (
  headers: Headers,
  key: string,
): string | null => {
  const cookies = getCookies(headers);
  return cookies[key];
};

export const getTargetCookieValCombinedAssign = (
  headers: Headers,
  key: string,
): string => {
  return `${key}=${getTargetCookieVal(headers, key)!}`;
};
