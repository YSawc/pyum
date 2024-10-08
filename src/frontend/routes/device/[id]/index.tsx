import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { asset } from "$fresh/runtime.ts";
import { ConfirmButton } from "../../../islands/device/[id]/index/ConfirmButton.tsx";
import { getDevice } from "../../../requests/device.ts";
import { Device } from "../../../types/request/device.ts";
import Title from "../../_title.tsx";
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
};

const Page = ({ data }: PageProps<Props>) => {
  const { device } = data.device;

  return (
    <div class="container">
      <Title title="Device detail" />
      <ConfirmButton
        text="delete"
        confirmText="really delete?"
        url={`/device/${device.id}/delete`}
      />
      <table class="table-fixed">
        <thead>
          <tr>
            <th>Name</th>
            <th>Image</th>
            <th>Other</th>
          </tr>
        </thead>
        <tbody>
          <tr
            onClick={`window.location='/device/${device.id}/edit'`}
          >
            <td class="px-2">
              {device.name}
            </td>
            <td>
              <img
                src={device.image ? `${device.image}` : asset(`/icons/no_image.jpg`)}
                width="128"
                height="128"
              />
            </td>
            <td class="px-2 flex flex-col">
              <a
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4"
                href={`/sensor/new?device_id=${device.id}`}
              >
                Create sensor
              </a>

              <a
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4"
                href={`/sensor?device_id=${device.id}`}
              >
                Sensors
              </a>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  );
};

export default Page;
