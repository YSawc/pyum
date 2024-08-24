import { FreshContext } from "$fresh/server.ts";
import { match, P } from "npm:ts-pattern@5.3.1";
import { Effect, Option } from "@effect";
import { type HttpClientError } from "npm:@effect/platform";
import { wrapOption } from "../utils/effect/option.ts";
import HttpStatusCode from "../enums/HttpStatusCode.ts";

const validateSession = (cid: string): boolean => {
  let isValidSession = false;
  const prog: Effect<unknown, HttpClientError> = Effect.tryPromise({
    try: () =>
      fetch("http://localhost:3000/session/check_valid", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          cid: cid,
        }),
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
      console.log(res);
    }),
    Effect.catchAll((err) => {
      console.log(err);
    }),
  );
  Effect.runPromise(prog).then(console.log, console.error);
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
      const maybeRawCookie = wrapOption(req.headers.get("cookie"));
      Option.match(maybeRawCookie, {
        onSome: (rawCookie) => {
          const cidVal = rawCookie.split("=")[1];
          isRedirectNeed = !validateSession(cidVal);
        },
        onNone: () => {
          isRedirectNeed = true;
        },
      });
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
