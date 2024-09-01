import Title from "../../_title.tsx";
import { Effect } from "@effect";
import { Handlers, PageProps } from "$fresh/server.ts";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { createAdminUser } from "../../../requests/admin_user.ts";

interface Props {
  results: string[];
  query: string;
}

export const handler: Handlers<Props> = {
  async POST(req: Request) {
    const resHeaders: ResponseInit = {};
    const headers = new Headers();
    const formData = await req.formData();
    let locationUrl: string;
    try {
      await Effect.runPromise(createAdminUser(formData));
      locationUrl = "/admin_user/login";
    } catch {
      locationUrl = "/admin_user/new";
    }

    headers.set("location", locationUrl);
    resHeaders.status = HttpStatusCode.SEE_OTHER;
    resHeaders.headers = headers;
    return new Response(null, resHeaders);
  },
};

const New = ({}: PageProps<Props>) => {
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
