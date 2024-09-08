import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { Effect } from "effect";
import { GetSensor } from "../../../types/request/sensor.ts";
import { getSensor } from "../../../requests/sensor.ts";
import Title from "../../_title.tsx";

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
      <table class="table-fixed">
        <thead>
          <tr>
            <th>Device id</th>
            <th>Sensor purpose id</th>
          </tr>
        </thead>
        <tbody>
          <tr
            class="post"
            onClick={"window.location=" + `'/sensor/${models[0].id}'`}
          >
            <td class="px-2">{models[0].device_id}</td>
            <td
              class={`px-2 border-4 border-[#${models[1].color_code}] rounded`}
            >
              {models[0].sensor_purpose_id}
            </td>
          </tr>
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
