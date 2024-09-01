import { Effect } from "effect";
import {
  HttpClient,
  HttpClientError,
  HttpClientRequest,
  HttpClientResponse,
} from "@effect/platform";
import { ParseError } from "@effect/schema/ParseResult";
import { HttpBodyError } from "@effect/platform/HttpBody";
import {
  Cookie,
  deleteCookie,
  setCookie,
} from "https://deno.land/std@0.224.0/http/cookie.ts";
import { SimpleRes, SimpleResSchema } from "../types/request/util.ts";

export const createAdminUser = (
  formData: FormData,
): Effect.Effect<
  SimpleRes,
  HttpClientError.HttpClientError | HttpBodyError | ParseError,
  never
> => {
  return HttpClientRequest
    .post(
      `http://localhost:3000/admin_user`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
      }),
      HttpClientRequest.jsonBody({
        name: formData.get("name")?.toString(),
        password: formData.get("password")?.toString(),
      }),
      Effect.andThen(HttpClient.fetchOk),
      Effect.andThen(HttpClientResponse.schemaBodyJson(SimpleResSchema)),
      Effect.scoped,
    );
};

export const loginAdminUser = (
  formData: FormData,
  headers: Headers,
): Effect.Effect<
  SimpleRes,
  HttpClientError.HttpClientError | HttpBodyError | ParseError,
  never
> => {
  return HttpClientRequest
    .post(
      `http://localhost:3000/admin_user/login`,
    ).pipe(
      HttpClientRequest.setHeaders({
        "Content-Type": "application/json",
      }),
      HttpClientRequest.jsonBody({
        name: formData.get("name")?.toString(),
        password: formData.get("password")?.toString(),
      }),
      Effect.andThen(HttpClient.fetch),
      Effect.andThen((res) => {
        deleteCookie(headers, "id");
        const id =
          res.headers["set-cookie"].split(" ", 1)[0].slice(0, -1).split(
            "=",
          )[1];
        const cookie: Cookie = {
          name: "id",
          value: id,
          path: "/",
        };
        setCookie(headers, cookie);
        return res;
      }),
      Effect.andThen(HttpClientResponse.schemaBodyJson(SimpleResSchema)),
      Effect.scoped,
    );
};
