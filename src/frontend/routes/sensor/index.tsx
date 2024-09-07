import Title from "../_title.tsx";
import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { getDevices } from "../../requests/device.ts";
import { Effect } from "effect";
import { Sensors } from "../../types/request/sensor.ts";

interface Props {
  sensors: Sensors;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const sensors = await Effect.runPromise(
      getDevices(req),
    );
    const pageData: Props = {
      sensors: sensors,
    };
    const res: Response = await ctx.render(pageData);
    return res;
  },
};

const Page = ({ data }: PageProps<Props>) => {
  const { sensors } = data.sensors;

  return (
    <div class="container">
      <Title title="Devices" />
      <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4">
        <a href="/device/new">
          create device
        </a>
      </button>
      <table class="table-fixed">
        <thead>
          <tr>
            <th>Device id</th>
          </tr>
        </thead>
        <tbody>
          {sensors.map((sensor) => (
            <tr
              class="post"
              onclick={"window.location=" + `'/sensor/${sensor.id}'`}
            >
              <td class="px-2">{sensor.device_id}</td>
            </tr>
          ))}
        </tbody>
        <tfoot>
          <tr>
            <td></td>
            <td>
              <a href="/?page={{ page - 1 }}&models_per_page={{ models_per_page }}">
                Previous
              </a>
              <a href="/?page={{ page + 1 }}&models_per_page ={{ models_per_page }}">
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
