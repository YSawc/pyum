import { Effect } from "effect";
import {
  HttpClient,
  HttpClientError,
  HttpClientRequest,
  HttpClientResponse,
} from "@effect/platform";
import { ParseError } from "@effect/schema/ParseResult";
import { getTargetCookieValCombinedAssign } from "../utils/browser/headers/cookie.ts";
import {
  GetSensorPurpose,
  GetSensorPurposes,
  GetSensorPurposeSchema,
  GetSensorPurposesSchema,
} from "../types/request/sensor_purpose.ts";
import { SimpleRes, SimpleResSchema } from "../types/request/util.ts";
import { HttpBodyError } from "@effect/platform/HttpBody";

export const getSensorPurposes = (req: Request): Effect.Effect<
  GetSensorPurposes,
  HttpClientError.HttpClientError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .get(
      `${Deno.env.get("API_URL")}/sensor_purpose`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClient.fetch,
      Effect.andThen(
        HttpClientResponse.schemaBodyJson(GetSensorPurposesSchema),
      ),
      Effect.scoped,
    );
};

export const getSensorPurpose = (
  req: Request,
  sensorPurposeId: string,
): Effect.Effect<
  GetSensorPurpose,
  HttpClientError.HttpClientError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .get(
      `${Deno.env.get("API_URL")}/sensor_purpose/${sensorPurposeId}`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClient.fetch,
      Effect.andThen(HttpClientResponse.schemaBodyJson(GetSensorPurposeSchema)),
      Effect.scoped,
    );
};

export const createSensorPurpose = (
  req: Request,
  formData: FormData,
): Effect.Effect<
  SimpleRes,
  HttpClientError.HttpClientError | HttpBodyError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  console.log("formData");
  console.log(formData);
  console.log(`Number(formData.get("sensor_event_id"))`);
  console.log(Number(formData.get("sensor_event_id")));
  return HttpClientRequest
    .post(
      `${Deno.env.get("API_URL")}/sensor_purpose`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClientRequest.jsonBody({
        description: formData.get("description")?.toString(),
        sensor_event_id: Number(formData.get("sensor_event_id")),
      }),
      Effect.andThen(HttpClient.fetch),
      Effect.andThen(HttpClientResponse.schemaBodyJson(SimpleResSchema)),
      Effect.scoped,
    );
};

export const editSensorPurpose = (
  req: Request,
  sensorPurposeId: string,
  formData: FormData,
): Effect.Effect<
  SimpleRes,
  HttpClientError.HttpClientError | HttpBodyError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  console.log("formData");
  console.log(formData);
  return HttpClientRequest
    .patch(
      `${Deno.env.get("API_URL")}/sensor_purpose/${sensorPurposeId}`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClientRequest.jsonBody({
        description: formData.get("description")?.toString(),
        sensor_event_id: Number(formData.get("sensor_event_id")),
      }),
      Effect.andThen(HttpClient.fetch),
      Effect.andThen(HttpClientResponse.schemaBodyJson(SimpleResSchema)),
      Effect.scoped,
    );
};

export const deleteSensorPurpose = (
  req: Request,
  sensorPurposeId: string,
): Effect.Effect<
  SimpleRes,
  HttpClientError.HttpClientError | HttpBodyError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .del(
      `${Deno.env.get("API_URL")}/sensor_purpose/${sensorPurposeId}`,
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
