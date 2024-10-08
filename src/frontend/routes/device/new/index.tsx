import { Effect } from "effect";
import Title from "../../_title.tsx";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { Handlers } from "$fresh/server.ts";
import { createDevice } from "../../../requests/device.ts";

export const handler: Handlers = {
  async POST(req: Request) {
    const formData = await req.formData();
    await Effect.runPromise(
      createDevice(req, formData),
    );

    return new Response(null, {
      status: HttpStatusCode.SEE_OTHER,
      headers: { Location: `/device` },
    });
  },
};

const Page = () => {
  return (
    <div class="container">
      <Title title="Create device" />
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
              Create device
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Page;
