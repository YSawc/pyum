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
  DevicesWithRelationSchema,
  GetDevicesWithRelation,
  GetSensor,
  GetSensorSchema,
} from "../types/request/sensor.ts";
import { SimpleRes, SimpleResSchema } from "../types/request/util.ts";
import { HttpBodyError } from "@effect/platform/HttpBody";

export const getSensorsRelatedDevice = (
  req: Request,
  // deviceId: string,
): Effect.Effect<
  GetDevicesWithRelation,
  HttpClientError.HttpClientError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .get(
      `http://localhost:3000/sensor`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClient.fetch,
      Effect.andThen(
        HttpClientResponse.schemaBodyJson(DevicesWithRelationSchema),
      ),
      Effect.scoped,
    );
};

export const createSensor = (
  req: Request,
  formData: FormData,
  deviceId: string,
): Effect.Effect<
  SimpleRes,
  HttpClientError.HttpClientError | HttpBodyError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .post(
      `http://localhost:3000/sensor`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClientRequest.jsonBody({
        device_id: Number(deviceId),
        sensor_purpose_id: Number(formData.get("sensor_purpose_id")),
        trigger_limit_val: Number(formData.get("trigger_limit_val")),
        trigger_limit_sequence_count: Number(formData.get(
          "trigger_limit_sequence_count",
        )),
      }),
      Effect.andThen(HttpClient.fetch),
      Effect.andThen(HttpClientResponse.schemaBodyJson(SimpleResSchema)),
      Effect.scoped,
    );
};

export const getSensor = (req: Request, sensorId: string): Effect.Effect<
  GetSensor,
  HttpClientError.HttpClientError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .get(
      `http://localhost:3000/sensor/${sensorId}`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClient.fetch,
      Effect.andThen(HttpClientResponse.schemaBodyJson(GetSensorSchema)),
      Effect.scoped,
    );
};

export const editSensor = (
  req: Request,
  sensorId: string,
  formData: FormData,
): Effect.Effect<
  SimpleRes,
  HttpClientError.HttpClientError | HttpBodyError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .patch(
      `http://localhost:3000/sensor/${sensorId}`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClientRequest.jsonBody({
        sensor_purpose_id: Number(formData.get("sensor_purpose_id")),
        trigger_limit_val: Number(formData.get("trigger_limit_val")),
        trigger_limit_sequence_count: Number(formData.get(
          "trigger_limit_sequence_count",
        )),
      }),
      Effect.andThen(HttpClient.fetch),
      Effect.andThen(HttpClientResponse.schemaBodyJson(SimpleResSchema)),
      Effect.scoped,
    );
};

export const deleteSensor = (
  req: Request,
  sensorId: string,
): Effect.Effect<
  SimpleRes,
  HttpClientError.HttpClientError | HttpBodyError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .del(
      `http://localhost:3000/sensor/${sensorId}`,
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
