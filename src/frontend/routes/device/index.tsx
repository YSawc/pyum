import Title from "../_title.tsx";
import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { Devices } from "../../types/request/device.ts";
import { getDevices } from "../../requests/device.ts";
import { Effect } from "@effect";

interface Props {
  devices: Devices;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const devices = await Effect.runPromise(
      getDevices(req),
    );
    const pageData: Props = {
      devices: devices,
    };
    const res: Response = await ctx.render(pageData);
    return res;
  },
};

const Page = ({ data }: PageProps<Props>) => {
  const { devices } = data.devices;

  return (
    <div class="container">
      <Title title="Devices" />
      <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4">
        <a href="/device/new">
          add device
        </a>
      </button>
      <table class="table-fixed">
        <thead>
          <tr>
            <th>Name</th>
            <th>Image</th>
          </tr>
        </thead>
        <tbody>
          {devices.map((device) => (
            <tr
              class="post"
              onclick={"window.location=" + `'/device/${device.id}'`}
            >
              <td class="px-2">{device.name}</td>
              <td>
                <img
                  src={`${device.image}`}
                  width="128"
                  height="128"
                />
              </td>
            </tr>
          ))}
        </tbody>
        <tfoot>
          <tr>
            <td></td>
            <td>
              <a href="/?page={{ page - 1 }}&devices_per_page={{ devices_per_page }}">
                Previous
              </a>
              <a href="/?page={{ page + 1 }}&devices_per_page ={{ devices_per_page }}">
                Next
              </a>
            </td>
          </tr>
        </tfoot>
      </table>
    </div>
  );
};

export default Page;
