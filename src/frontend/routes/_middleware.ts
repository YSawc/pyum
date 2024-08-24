import { FreshContext } from "$fresh/server.ts";
import { match, P } from "npm:ts-pattern@5.3.1";

export const check_protected_route = (url: string): boolean =>
  match(url)
    .with(P.string.regex(/^(?=.*\/device|\/admin_user\/).*$/), () => true)
    .otherwise(() => false);

export async function handler(req: Request, ctx: FreshContext) {
  if (ctx.destination === "route") {
    console.log("ctx.destination: %o", ctx.destination);
    console.log("req.url: %o", req.url);
  }
  const resp = await ctx.next();
  return resp;
}
