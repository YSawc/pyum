import { Effect } from "@effect";
import {
  HttpClient,
  HttpClientError,
  HttpClientRequest,
  HttpClientResponse,
} from "@effect/platform";
import { ParseError } from "@effect/schema/ParseResult";
import { getTargetCookieValCombinedAssign } from "../utils/browser/headers/cookie.ts";
import {
  GetDevice,
  GetDevices,
  GetDeviceSchema,
  GetDevicesSchema,
} from "../types/request/device.ts";
import { SimpleRes, SimpleResSchema } from "../types/request/util.ts";
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

export const createDevice = (
  req: Request,
  formData: FormData,
): Effect.Effect<
  SimpleRes,
  HttpClientError.HttpClientError | HttpBodyError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .post(
      `http://localhost:3000/device/new`,
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

export const deleteDevice = (
  req: Request,
  deviceId: string,
): Effect.Effect<
  SimpleRes,
  HttpClientError.HttpClientError | HttpBodyError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .del(
      `http://localhost:3000/device/${deviceId}`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClient.fetch,
      Effect.andThen(HttpClientResponse.schemaBodyJson(SimpleResSchema)),
      Effect.scoped,
    );
};
