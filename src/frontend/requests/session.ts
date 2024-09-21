import { Effect } from "effect";
import {
  HttpClient,
  HttpClientError,
  HttpClientRequest,
  HttpClientResponse,
} from "@effect/platform";
import { ParseError } from "@effect/schema/ParseResult";
import { SimpleRes, SimpleResSchema } from "../types/request/util.ts";

export const validateSession = (id: string): Effect.Effect<
  SimpleRes,
  HttpClientError.HttpClientError | ParseError,
  never
> => {
  return HttpClientRequest
    .post(
      `${Deno.env.get("API_URL")}/session/check_valid`,
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
