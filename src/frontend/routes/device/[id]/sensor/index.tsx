import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { getSensorsRelatedDevice } from "../../../../requests/sensor.ts";
import { Sensors } from "../../../../types/request/sensor.ts";
import Title from "../../../_title.tsx";
import { Effect } from "effect";

interface Props {
  sensors: Sensors;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const deviceId = ctx.params.id;
    const sensors = await Effect.runPromise(
      getSensorsRelatedDevice(req, deviceId),
    );
    const data: Props = {
      sensors,
    };
    const res: Response = await ctx.render(data);
    return res;
  },
};

const Page = ({ data }: PageProps<Props>) => {
  const { sensors } = data.sensors;

  return (
    <div class="container">
      <Title title="Sensors related device" />
      <table class="table-fixed">
        <thead>
          <tr>
            <th>Triger limit val</th>
            <th>Triger limit sequence count</th>
          </tr>
        </thead>
        <tbody>
          {sensors.map((sensor) => (
            <tr
              class="post"
              onclick={"window.location=" + `'/sensor/${sensor.id}'`}
            >
              <td class="px-2">{sensor.triger_limit_val}</td>
              <td class="px-2">{sensor.triger_limit_sequence_count}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Page;
