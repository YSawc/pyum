import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { deleteDevice, getDevice } from "../../../requests/device.tsx";
import { Device } from "../../../types/request/device/index.ts";
import Title from "../../_title.tsx";
import { Effect } from "@effect";

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
  async DELETE(req: Request, ctx: FreshContext) {
    const deviceId = ctx.params.id;
    if (confirm("realy delete device?")) {
      await deleteDevice(req, ctx);
      return new Response("", {
        status: HttpStatusCode.SEE_OTHER,
        headers: { Location: "/device" },
      });
    } else {
      console.log("false");
      return new Response("", {
        status: HttpStatusCode.SEE_OTHER,
        headers: { Location: `/device/${deviceId}` },
      });
    }
  },
};

const Index = ({ data }: PageProps<Props>) => {
  const { device } = data.device;

  return (
    <div class="container">
      <Title title="Device detail" />
      <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded mb-4">
        delete
      </button>
      <table class="table-fixed">
        <thead>
          <tr>
            <th>Name</th>
            <th>Image</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td
              class="px-2"
              onclick={`window.location='/device/${device.id}/edit'`}
            >
              {device.name}
            </td>
            <td>
              <img
                src={`${device.image}`}
                width="128"
                height="128"
              />
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  );
};

export default Index;
