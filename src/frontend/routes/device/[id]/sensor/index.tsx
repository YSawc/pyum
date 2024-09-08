import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { getSensorsRelatedDevice } from "../../../../requests/sensor.ts";
import { GetSensors } from "../../../../types/request/sensor.ts";
import Title from "../../../_title.tsx";
import { Effect } from "effect";

interface Props {
  deviceId: string;
  models: GetSensors;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const deviceId = ctx.params.id;
    const models = await Effect.runPromise(
      getSensorsRelatedDevice(req, deviceId),
    );
    const data: Props = {
      deviceId,
      models,
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
      <table class="table-fixed border-separate border-spacing-2">
        <thead>
          <tr>
            <th>Sensor purpose</th>
            <th>Triger limit val</th>
            <th>Triger limit sequence count</th>
          </tr>
        </thead>
        <tbody>
          {data.models.models.map((rels) => (
            <tr
              class="post"
              onClick={"window.location=" + `'/sensor/${rels[0].id}'`}
            >
              <td
                class={`px-2 border-4 border-[#${rels[1].color_code}] rounded`}
              >
                {rels[0].sensor_purpose_id}
              </td>
              <td class="px-2">{rels[0].trigger_limit_val}</td>
              <td class="px-2">{rels[0].trigger_limit_sequence_count}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Page;
