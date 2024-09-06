import Title from "../../_title.tsx";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { editDevice, getDevice } from "../../../requests/device.ts";
import { Device } from "../../../types/request/device.ts";
import { Effect } from "effect";

interface Props {
  device: Device;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const deviceId = ctx.params.id;
    const device = await Effect.runPromise(
      getDevice(req, deviceId),
    );
    const data: Props = {
      device,
    };
    const res: Response = await ctx.render(data);
    return res;
  },

  async POST(req: Request, ctx: FreshContext) {
    const deviceId = ctx.params.id;
    const formData = await req.formData();
    await Effect.runPromise(
      editDevice(req, deviceId, formData),
    );

    return new Response(null, {
      status: HttpStatusCode.SEE_OTHER,
      headers: { Location: `/device/${deviceId}` },
    });
  },
};

const Page = ({ data }: PageProps<Props>) => {
  const { device } = data.device;
  return (
    <div class="container">
      <Title title="Edit device" />
      <div class="w-full max-w-xs">
        <form
          class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4"
          method="post"
          id="edit-device-form"
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
              value={`${device.name}`}
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
              value={`${device.image}`}
            />
          </div>
          <div class="flex items-center justify-between">
            <button
              class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              type="submit"
            >
              Edit Device
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Page;
