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
  start_date: string | null,
  end_date: string | null,
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
  if (start_date !== null && end_date !== null) {
    url = url.concat(`&start_date=${start_date}&end_date=${end_date}`);
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
