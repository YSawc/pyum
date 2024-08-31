import Title from "../../_title.tsx";
import { Effect } from "@effect";
import { Handlers, PageProps } from "$fresh/server.ts";
import { type HttpClientError } from "npm:@effect/platform";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";

interface Props {
  results: string[];
  query: string;
}

export const handler: Handlers<Props> = {
  async POST(req) {
    const resHeaders: ResponseInit = {};
    const headers = new Headers();
    const form = await req.formData();
    const prog: Effect<unknown, HttpClientError> = Effect.tryPromise({
      try: () =>
        fetch("http://localhost:3000/admin_user/new", {
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
        ) => res.json()),
      catch: (err) =>
        new Error(`In post for admin_user/new, something went wrong ${err}`),
    }).pipe(
      Effect.andThen((res) => {
        resHeaders.status = HttpStatusCode.SEE_OTHER;
        headers.set("location", "/admin_user/login");
        console.log(res);
      }),
      Effect.catchAll((err) => {
        resHeaders.status = HttpStatusCode.SEE_OTHER;
        headers.set("location", "/admin_user/new");
        console.log(err);
      }),
    );
    await Effect.runPromise(prog).then(console.log, console.error);

    resHeaders.headers = headers;
    return new Response(null, resHeaders);
  },
};

const New = ({ }: PageProps<Props>) => {
  return (
    <div class="container">
      <Title title="Create Admin User" />
      <div class="w-full max-w-xs">
        <form
          method="post"
          class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4"
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

export default New;
