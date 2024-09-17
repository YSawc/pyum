import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { asset } from "$fresh/runtime.ts";
import { ConfirmButton } from "../../../islands/device/[id]/index/ConfirmButton.tsx";
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
      <Title title="Detail sensor purpose" />
      <ConfirmButton
        text="delete"
        confirmText="really delete?"
        url={`/sensor_purpose/${sensorPurpose.id}/delete`}
      />
      <table class="table-fixed border-separate border-spacing-2">
        <thead>
          <tr>
            <th>Description</th>
            <th>Color Code</th>
            <th>Image</th>
            <th>Other</th>
          </tr>
        </thead>
        <tbody>
          <tr
            onClick={`window.location='/sensor_purpose/${sensorPurpose.id}/edit'`}
          >
            <td class="px-2">
              {sensorPurpose.description}
            </td>
            <td
              class={`px-2 border-4 border-[#${sensorPurpose.color_code}] rounded`}
            >
              {sensorPurpose.color_code}
            </td>
            <td>
              <img
                src={sensorPurpose.image
                  ? `${sensorPurpose.image}`
                  : asset(`/icons/no_image.jpg`)}
                width="128"
                height="128"
              />
            </td>
            <td class="px-2 flex flex-col">
              <a
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4"
                href={`/capture?sensor_purpose_id=${sensorPurpose.id}`}
              >
                capture
              </a>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  );
};

export default Page;
