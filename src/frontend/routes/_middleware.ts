import { FreshContext } from "$fresh/server.ts";
import { match, P } from "npm:ts-pattern@5.3.1";
import { Effect } from "@effect";
import HttpStatusCode from "../enums/HttpStatusCode.ts";
import { getTargetCookieVal } from "../utils/browser/headers/cookie.ts";
import { validateSession } from "../requests/session.ts";

export const check_protected_route = (url: string): boolean =>
  match(url)
    .with(P.string.regex(/^(?=.*\/device|\/admin_user\/).*$/), () => true)
    .otherwise(() => false);

export async function handler(req: Request, ctx: FreshContext) {
  let isRedirectNeed = false;
  if (ctx.destination === "route") {
    if (check_protected_route(req.url)) {
      isRedirectNeed = true;
      const maybeId = getTargetCookieVal(req.headers, "id");
      if (!maybeId) {
        isRedirectNeed = true;
      } else {
        const id = `id=${maybeId}`;
        try {
          await Effect.runPromise(validateSession(id));
          isRedirectNeed = false;
        } catch {
          isRedirectNeed = true;
        }
      }
    }
  }

  if (isRedirectNeed) {
    return new Response(null, {
      status: HttpStatusCode.SEE_OTHER,
      headers: { Location: "/admin_user/login" },
    });
  } else {
    const resp = await ctx.next();
    return resp;
  }
}
