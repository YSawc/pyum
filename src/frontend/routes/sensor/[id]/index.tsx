import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { Effect } from "effect";
import { GetSensor } from "../../../types/request/sensor.ts";
import { getSensor } from "../../../requests/sensor.ts";
import Title from "../../_title.tsx";
import { ConfirmButton } from "../../../islands/routes/device/[id]/index/ConfirmButton.tsx";

interface Props {
  models: GetSensor;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const sensorId = ctx.params.id;
    const models = await Effect.runPromise(
      getSensor(req, sensorId),
    );
    const pageData: Props = {
      models: models,
    };
    const res: Response = await ctx.render(pageData);
    return res;
  },
};

const Page = ({ data }: PageProps<Props>) => {
  const { models } = data.models;

  return (
    <div class="container">
      <Title title="Sensor detail" />
      <ConfirmButton
        text="delete"
        confirmText="really delete?"
        url={`/sensor/${models[0].id}/delete`}
      />

      <table class="table-fixed border-separate border-spacing-2">
        <thead>
          <tr>
            <th>Device id</th>
            <th>Sensor purpose id</th>
            <th>Sensor trigger limit val</th>
            <th>Sensor trigger limit sequence count</th>
          </tr>
        </thead>
        <tbody>
          <tr
            class="post"
            onClick={"window.location=" + `'/sensor/${models[0].id}/edit'`}
          >
            <td class="px-2">{models[0].device_id}</td>
            <td
              class={`px-2 border-4 border-[#${models[1].color_code}] rounded`}
            >
              {models[0].sensor_purpose_id}
            </td>
            <td class="px-2">{models[0].trigger_limit_val}</td>
            <td class="px-2">{models[0].trigger_limit_sequence_count}</td>
          </tr>
        </tbody>
      </table>
    </div>
  );
};

export default Page;
