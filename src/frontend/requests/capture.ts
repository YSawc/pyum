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
  SensorPurposesWithRelation,
  SensorPurposeWithRelationSchema,
} from "../types/request/capture.ts";

export const GetSensorPurposeWithRelation = (
  req: Request,
  sensorPurposeId: string,
): Effect.Effect<
  SensorPurposesWithRelation,
  HttpClientError.HttpClientError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  return HttpClientRequest
    .get(
      `http://localhost:3000/capture?sensor_purpose_id=${sensorPurposeId}`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
        "Cookie": id,
      }),
      HttpClient.fetch,
      Effect.andThen(
        HttpClientResponse.schemaBodyJson(SensorPurposeWithRelationSchema),
      ),
      Effect.scoped,
    );
};
