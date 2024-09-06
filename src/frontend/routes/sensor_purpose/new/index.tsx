import { Effect } from "effect";
import Title from "../../_title.tsx";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { Handlers } from "$fresh/server.ts";
import { createSensorPurpose } from "../../../requests/sensor_purpose.ts";

export const handler: Handlers = {
  async POST(req: Request) {
    const formData = await req.formData();
    await Effect.runPromise(
      createSensorPurpose(req, formData),
    );

    return new Response(null, {
      status: HttpStatusCode.SEE_OTHER,
      headers: { Location: `/sensor_purpose` },
    });
  },
};

const Page = () => {
  return (
    <div class="container">
      <Title title="Create sensor purpose" />
      <div class="w-full max-w-xs">
        <form
          class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4"
          method="post"
          id="create-sensor-purpose-form"
        >
          <div class="mb-4">
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="description"
            >
              description
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="description"
              name="description"
              type="text"
              placeholder="description"
            />
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="color_code"
            >
              color code
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="color_code"
              name="color_code"
              type="text"
              placeholder="color_code"
            />
          </div>
          <div class="flex items-center justify-between">
            <button
              class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              type="submit"
            >
              Create sensor purpose
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Page;
