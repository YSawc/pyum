import { FreshContext } from "$fresh/server.ts";
import { match, P } from "npm:ts-pattern@5.3.1";
import { Effect } from "@effect";
import HttpStatusCode from "../enums/HttpStatusCode.ts";
import { getTargetCookieVal } from "../utils/browser/headers/cookie.ts";

const validateSession = async (id: string): Promise<boolean> => {
  let isValidSession = false;
  const prog = Effect
    .tryPromise({
      try: () =>
        fetch("http://localhost:3000/session/check_valid", {
          method: "POST",
          credentials: "include",
          headers: {
            "Content-Type": "application/json",
            "Cookie": id,
          },
          body: JSON.stringify({}),
        }).then((
          res,
        ) => res.json()),
      catch: (err) =>
        new Error(
          `While request for session/check_valid, something went wrong ${err}`,
        ),
    }).pipe(
      Effect.andThen((res) => {
        isValidSession = true;
      }),
      Effect.catchAll((err) => {
        console.log(err);
      }),
    );
  await Effect.runPromise(prog).then(console.log, console.error);
  return isValidSession;
};

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
        const isValid = await validateSession(id);
        isRedirectNeed = !isValid;
      }
    }
  }

  if (isRedirectNeed) {
    return new Response("", {
      status: HttpStatusCode.SEE_OTHER,
      headers: { Location: "/admin_user/login" },
    });
  } else {
    const resp = await ctx.next();
    return resp;
  }
}
