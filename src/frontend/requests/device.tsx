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
import { SimpleResSchema } from "../types/request/util.tsx";
import { SimpleRes } from "../types/request/util.tsx";
import { HttpBodyError } from "@effect/platform/HttpBody";

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

export const editDevice = (
  req: Request,
  deviceId: string,
  formData: FormData,
): Effect.Effect<
  SimpleRes,
  HttpClientError.HttpClientError | HttpBodyError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .patch(
      `http://localhost:3000/device/${deviceId}`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClientRequest.jsonBody({
        name: formData.get("name")?.toString(),
        image: formData.get("image")?.toString(),
      }),
      Effect.andThen(HttpClient.fetch),
      Effect.andThen(HttpClientResponse.schemaBodyJson(SimpleResSchema)),
      Effect.scoped,
    );
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
