export function getCookies() {
  let result: Record<string, string> = {};
  if (!document || !document.cookie) {
    return result;
  }
  result = Object.fromEntries(document.cookie.split(";").map(e => {
    const [key, val] = e.trim().split("=", 2);
    return [key, val];
  }));
  return result;
}

export function setCookie(config: CookieProps) {
  if (!document) {
    return;
  }
  const {name, value, secure, expires, ...cyclable} = config;
  const cookieSet: string[] = [];
  cookieSet.push(`${name}=${value}`);
  if (expires) {
    cookieSet.push(`expires=${expires.toUTCString()}`);
  }
  const associative: {[key: string]: string}= {
    maxAge: "max-age",
    path: "path",
    sameSite: "SameSite",
  };
  let key: keyof typeof cyclable;
  for (key in cyclable) {
    const tmpVal = associative[key];
    if (!cyclable[key] || !tmpVal) {
      return;
    }
    cookieSet.push(`${tmpVal}=${cyclable[key]}`);
  }
  if (secure) {
    cookieSet.push("Secure");
  }
  document.cookie = cookieSet.join("; ");
}

export function resetLogin() {
  setCookie({ name: "auth.t", expires: new Date(0), path: "/", sameSite: "strict"});
  setCookie({ name: "auth.r", expires: new Date(0), path: "/", sameSite: "strict"});
}

export function setLogin(tokens: LoginTokens) {
  if (tokens.ref) {
    setCookie({ name: "auth.r", value: tokens.ref, path: "/", sameSite: "strict" });
  }
  if (tokens.tok) {
    setCookie({ name: "auth.t", value: tokens.tok, path: "/", sameSite: "strict" });
  }
}

interface CookieProps extends ItteratableCookieProps {
  name: string,
  value?: string | number,
  expires?: Date,
  secure?: true,
}

interface ItteratableCookieProps {
  maxAge?: number,
  path?: string,
  sameSite?: "none" | "lax" | "strict",
}

interface LoginTokens {
  tok?: string,
  ref?: string,
}
