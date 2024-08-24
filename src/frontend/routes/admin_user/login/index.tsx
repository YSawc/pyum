import { FunctionComponent } from "https://esm.sh/v128/preact@10.19.6/src/index.js";
import Title from "../../_title.tsx";
import { Effect } from "npm:effect@3.6.5";
import { Handlers } from "$fresh/server.ts";
import { type HttpClientError } from "npm:@effect/platform";
import { AdminUserLoginRes } from "../../../types/request/admin_user/login.ts";
import {
  Cookie,
  setCookie,
} from "https://deno.land/std@0.224.0/http/cookie.ts";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";

interface Data {
  results: string[];
  query: string;
}

export const handler: Handlers<Data> = {
  async POST(req) {
    const form = await req.formData();
    const loginProg: Effect<AdminUserLoginRes, HttpClientError> = Effect
      .tryPromise({
        try: () =>
          fetch("http://localhost:3000/admin_user/login", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              name: form.get("name")?.toString(),
              password: form.get("password")?.toString(),
            }),
          }).then((
            res,
          ) => res.json() as Promise<AdminUserLoginRes>),
        catch: (err) =>
          new Error(`In post admin_user/login, something went wrong ${err}`),
      });
    // loginProg().pipe(
    //   Effect.andThen((res) => {
    //     console.log("res.cid");
    //     console.log(res.cid);
    //   }),
    // );

    const headers = new Headers();
    await Effect.runPromise(loginProg).then(
      (res) => {
        const adminUserLoginRes: AdminUserLoginRes = res as AdminUserLoginRes;
        const cookie: Cookie = { name: "cid", value: adminUserLoginRes.cid };
        setCookie(headers, cookie);
      },
    ).catch((err) => console.error(err));

    headers.set("location", "/admin_user/login");
    return new Response(null, {
      status: HttpStatusCode.SEE_OTHER,
      headers,
    });
  },
};

const Login: FunctionComponent = () => {
  return (
    <div class="container">
      <Title title="Login" />
      <div class="w-full max-w-xs">
        <form
          class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4"
          method="post"
          id="login-form"
        >
          <div class="mb-4">
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="name"
            >
              name
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="name"
              name="name"
              type="text"
              placeholder="name"
            />
          </div>
          <div class="mb-4">
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="password"
            >
              password
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="password"
              name="password"
              type="password"
              placeholder="password"
            />
          </div>

          <div class="flex items-center justify-between">
            <button
              class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              type="submit"
            >
              Login
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Login;
