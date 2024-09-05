import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { ConfirmButton } from "../../../islands/routes/device/[id]/index/ConfirmButton.tsx";
import { getSensorPurpose } from "../../../requests/sensor_purpose.ts";
import { SensorPurpose } from "../../../types/request/sensor_purpose.ts";
import Title from "../../_title.tsx";
import { Effect } from "effect";

interface Props {
  sensorPurpose: SensorPurpose;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const sensorPurposeId = ctx.params.id;
    const res = await Effect.runPromise(
      getSensorPurpose(req, sensorPurposeId),
    );
    const data: Props = {
      sensorPurpose: res.sensor_purpose,
    };
    const resp: Response = await ctx.render(data);
    return resp;
  },
};

const Page = ({ data }: PageProps<Props>) => {
  const { sensorPurpose } = data;

  return (
    <div class="container">
      <Title title="Device detail" />
      <ConfirmButton
        text="delete"
        confirmText="really delete?"
        url={`/sensor_purpose/${sensorPurpose.id}/delete`}
      />
      <table class="table-fixed">
        <thead>
          <tr>
            <th>Description</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td
              class="px-2"
              onclick={`window.location='/sensor_purpose/${sensorPurpose.id}/edit'`}
            >
              {sensorPurpose.description}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  );
};

export default Page;
