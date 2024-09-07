import { Effect } from "effect";
import {
  HttpClient,
  HttpClientError,
  HttpClientRequest,
  HttpClientResponse,
} from "@effect/platform";
import { ParseError } from "@effect/schema/ParseResult";
import { getTargetCookieValCombinedAssign } from "../utils/browser/headers/cookie.ts";
import { GetSensors, GetSensorsSchema } from "../types/request/sensor.ts";

export const getSensorsRelatedDevice = (
  req: Request,
  deviceId: string,
): Effect.Effect<
  GetSensors,
  HttpClientError.HttpClientError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .get(
      `http://localhost:3000/device/${deviceId}/sensor`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClient.fetch,
      Effect.andThen(
        HttpClientResponse.schemaBodyJson(GetSensorsSchema),
      ),
      Effect.scoped,
    );
};
