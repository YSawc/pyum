import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { getSensorsRelatedDevice } from "../../../../requests/sensor.ts";
import { Sensors } from "../../../../types/request/sensor.ts";
import Title from "../../../_title.tsx";
import { Effect } from "effect";

interface Props {
  deviceId: string;
  sensors: Sensors;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const deviceId = ctx.params.id;
    const sensors = await Effect.runPromise(
      getSensorsRelatedDevice(req, deviceId),
    );
    const data: Props = {
      deviceId,
      sensors,
    };
    const res: Response = await ctx.render(data);
    return res;
  },
};

const Page = ({ data }: PageProps<Props>) => {
  return (
    <div class="container">
      <Title title="Sensors related device" />
      <a
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4"
        href={`/device/${data.deviceId}`}
      >
        Back to device detail
      </a>
      <table class="table-fixed">
        <thead>
          <tr>
            <th>Sensor purpose</th>
            <th>Triger limit val</th>
            <th>Triger limit sequence count</th>
          </tr>
        </thead>
        <tbody>
          {data.sensors.sensors.map((sensor) => (
            <tr
              class="post"
              onclick={"window.location=" + `'/sensor/${sensor.id}'`}
            >
              <td class="px-2">{sensor.sensor_purpose_id}</td>
              <td class="px-2">{sensor.trigger_limit_val}</td>
              <td class="px-2">{sensor.trigger_limit_sequence_count}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Page;
