import { FunctionComponent } from "https://esm.sh/v128/preact@10.19.6/src/index.js";
import Title from "../../_title.tsx";
import { Effect } from "effect";
import { Handlers } from "$fresh/server.ts";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { loginAdminUser } from "../../../requests/admin_user.ts";

interface Props {
  results: string[];
  query: string;
}

export const handler: Handlers<Props> = {
  async POST(req: Request) {
    const formData = await req.formData();
    const headers = new Headers();
    await Effect.runPromise(
      loginAdminUser(formData, headers),
    );

    headers.set("location", "/device");
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
