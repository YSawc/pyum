import { FunctionComponent } from "https://esm.sh/v128/preact@10.19.6/src/index.js";
import { Effect } from "@effect";
import Title from "../../_title.tsx";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { getTargetCookieValCombinedAssign } from "../../../utils/browser/headers/cookie.ts";

export const handler: Handlers<Data> = {
  async POST(req) {
    const resHeaders: ResponseInit = {};
    const headers = new Headers();
    const form = await req.formData();
    const id = getTargetCookieValCombinedAssign(req.headers, "id");
    const prog: Effect<unknown, HttpClientError> = Effect.tryPromise({
      try: () =>
        fetch("http://localhost:3000/device/new", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            "Cookie": id,
          },
          body: JSON.stringify({
            name: form.get("name")?.toString(),
            image: form.get("image")?.toString(),
          }),
        }).then((
          res,
        ) => res.json()),
      catch: (err) =>
        new Error(`In post for device/new, something went wrong ${err}`),
    }).pipe(
      Effect.andThen((res) => {
        resHeaders.status = HttpStatusCode.SEE_OTHER;
        headers.set("location", "/device");
        console.log(res);
      }),
      Effect.catchAll((err) => {
        resHeaders.status = HttpStatusCode.SEE_OTHER;
        headers.set("location", "/device/new");
        console.log(err);
      }),
    );
    await Effect.runPromise(prog).then(console.log, console.error);

    resHeaders.headers = headers;
    return new Response(null, resHeaders);
  },
};

const createDevices = async (req: Request) => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  const prog = Effect
    .tryPromise({
      try: () =>
        fetch(`http://localhost:3000/device/`, {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
            "Cookie": id,
          },
        }).then((
          res,
        ) => res.json()),
      catch: (err) =>
        new Error(`In post admin_user/login, something went wrong ${err}`),
    }).pipe(
      Effect.andThen((res) => {
        return res;
      }),
      Effect.catchAll((err) => {
        console.log(err);
      }),
    );

  return await Effect.runPromise(prog);
};

const New: FunctionComponent = () => {
  return (
    <div class="container">
      <Title title="Create Device" />
      <div class="w-full max-w-xs">
        <form
          class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4"
          method="post"
          id="create-device-form"
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
              for="image"
            >
              image path
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="image"
              name="image"
              type="text"
              placeholder="image path"
            />
          </div>
          <div class="flex items-center justify-between">
            <button
              class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              type="submit"
            >
              Create Device
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default New;
