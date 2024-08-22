import Title from "../_title.tsx";
import { Effect } from "npm:effect@3.6.5";
import { Handlers, PageProps } from "$fresh/server.ts";
import { type HttpClientError } from "npm:@effect/platform";

interface Data {
  results: string[];
  query: string;
}

export const handler: Handlers<Data> = {
  async POST(req) {
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
      Effect.andThen((res) => console.log(res)),
      Effect.catchAll((err) => console.log(err)),
    );
    Effect.runPromise(prog).then(console.log, console.error);

    const headers = new Headers();
    headers.set("location", "/admin_user/login");
    return new Response(null, {
      status: 303, // See Other
      headers,
    });
  },
};

const New = ({ }: PageProps<Data>) => {
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
