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
  limit: string | null,
): Effect.Effect<
  SensorPurposesWithRelation,
  HttpClientError.HttpClientError | ParseError,
  never
> => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  let url = `${Deno.env.get("API_URL")
    }/capture?sensor_purpose_id=${sensorPurposeId}`;
  if (limit !== null) {
    url = url.concat(`&limit=${limit}`);
  }
  return HttpClientRequest
    .get(
      url,
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
