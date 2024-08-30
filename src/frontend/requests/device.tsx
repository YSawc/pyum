import { Effect } from "@effect";
import { FreshContext } from "$fresh/server.ts";
import { getTargetCookieValCombinedAssign } from "../utils/browser/headers/cookie.ts";

export const getDevices = async (req: Request) => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  const prog = Effect
    .tryPromise({
      try: () =>
        fetch(`http://localhost:3000/device`, {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
            "Cookie": id,
          },
        }).then((
          res,
        ) => res.json()),
      catch: (err) => new Error(`In get device/, something went wrong ${err}`),
    }).pipe(
      Effect.andThen((res) => {
        return res;
      }),
      Effect.catchAll((err) => {
        console.log(err);
      }),
    );

  return await Effect.runPromise(prog);
};

export const getDevice = async (req: Request, deviceId: string) => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  const prog = Effect
    .tryPromise({
      try: () =>
        fetch(`http://localhost:3000/device/${deviceId}`, {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
            "Cookie": id,
          },
        }).then((
          res,
        ) => res.json()),
      catch: (err) =>
        new Error(`In get device/${deviceId}, something went wrong ${err}`),
    }).pipe(
      Effect.andThen((res) => {
        return res;
      }),
      Effect.catchAll((err) => {
        console.log(err);
      }),
    );

  return await Effect.runPromise(prog);
};

export const editDevice = async (req: Request, deviceId: string) => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  const form = await req.formData();
  const prog = Effect
    .tryPromise({
      try: () =>
        fetch(`http://localhost:3000/device/${deviceId}`, {
          method: "PATCH",
          headers: {
            "Content-Type": "application/json",
            "Cookie": id,
          },
          body: JSON.stringify({
            name: form.get("name")?.toString(),
            image: form.get("image")?.toString(),
          }),
        }).then((
          res,
        ) => res.json()),
      catch: (err) =>
        new Error(`In patch device/${deviceId}, something went wrong ${err}`),
    }).pipe(
      Effect.andThen((res) => {
        return res;
      }),
      Effect.catchAll((err) => {
        console.log(err);
      }),
    );

  return await Effect.runPromise(prog);
};

export const deleteDevice = async (req: Request, ctx: FreshContext) => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  const deviceId = ctx.params.id;
  const prog = Effect
    .tryPromise({
      try: () =>
        fetch(`http://localhost:3000/device/${deviceId}`, {
          method: "DELETE",
          headers: {
            "Content-Type": "application/json",
            "Cookie": id,
          },
        }).then((
          res,
        ) => res.json()),
      catch: (err) =>
        Error(
          `In delete device/${deviceId}, something went wrong ${err}`,
        ),
    }).pipe(
      Effect.andThen((res) => {
        return res;
      }),
      Effect.catchAll((err) => {
        console.log(err);
      }),
    );

  return await Effect.runPromise(prog);
};
