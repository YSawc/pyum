import Title from "../_title.tsx";
import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { SensorPurposes } from "../../types/request/sensor_purpose.ts";
import { getSensorPurposes } from "../../requests/sensor_purpose.ts";
import { Effect } from "effect";

interface Props {
  sensorPurposes: SensorPurposes;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const sensorPurposes = await Effect.runPromise(
      getSensorPurposes(req),
    );
    const pageData: Props = {
      sensorPurposes,
    };
    const res: Response = await ctx.render(pageData);
    return res;
  },
};

const Page = ({ data }: PageProps<Props>) => {
  const { sensor_purposes } = data.sensorPurposes;

  return (
    <div class="container">
      <Title title="Sensor Purposes" />
      <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4">
        <a href="/sensor_purpose/new">
          create sensor purpose
        </a>
      </button>
      <table class="table-fixed">
        <thead>
          <tr>
            <th>Descripion</th>
            <th>Color Code</th>
          </tr>
        </thead>
        <tbody>
          {sensor_purposes.map((sensorPurpose) => (
            <tr
              class="post"
              onclick={"window.location=" +
                `'/sensor_purpose/${sensorPurpose.id}'`}
            >
              <td class="px-2">{sensorPurpose.description}</td>
              <td class="px-2">{sensorPurpose.color_code}</td>
            </tr>
          ))}
        </tbody>
        <tfoot>
          <tr>
            <td></td>
            <td>
              <a href="/?page={{ page - 1 }}&sensor_purpose_per_page={{ devices_per_page }}">
                Previous
              </a>
              <a href="/?page={{ page + 1 }}&sensor_purpose_per_page={{ devices_per_page }}">
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
