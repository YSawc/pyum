import { Effect } from "@effect";
import {
  HttpClient,
  HttpClientError,
  HttpClientRequest,
  HttpClientResponse,
} from "@effect/platform";
import { ParseError } from "@effect/schema/ParseResult";
import { FreshContext } from "$fresh/server.ts";
import { getTargetCookieValCombinedAssign } from "../utils/browser/headers/cookie.ts";
import {
  GetDevice,
  GetDevices,
  GetDeviceSchema,
  GetDevicesSchema,
} from "../types/request/device/index.ts";

export const getDevices = (req: Request): Effect.Effect<
  GetDevices,
  HttpClientError.HttpClientError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .get(
      `http://localhost:3000/device`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClient.fetch,
      Effect.andThen(HttpClientResponse.schemaBodyJson(GetDevicesSchema)),
      Effect.scoped,
    );
};

export const getDevice = (req: Request, deviceId: string): Effect.Effect<
  GetDevice,
  HttpClientError.HttpClientError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .get(
      `http://localhost:3000/device/${deviceId}`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClient.fetch,
      Effect.andThen(HttpClientResponse.schemaBodyJson(GetDeviceSchema)),
      Effect.scoped,
    );
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
