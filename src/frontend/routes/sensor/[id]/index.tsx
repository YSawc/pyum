import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { Effect } from "effect";
import { GetSensor } from "../../../types/request/sensor.ts";
import { getSensor } from "../../../requests/sensor.ts";
import Title from "../../_title.tsx";
import { ConfirmButton } from "../../../islands/device/[id]/index/ConfirmButton.tsx";
import { asset } from "$fresh/runtime.ts";

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

      <table class="border-separate border-spacing-2">
        <thead>
          <tr>
            <th class="row-span-2">Sensor id</th>
            <th class="row-span-2">Device id</th>
            <th colSpan={3}>Sensor purpose</th>
            <th class="row-span-2">Sensor trigger limit val</th>
            <th class="row-span-2">Sensor trigger limit sequence count</th>
          </tr>
          <tr>
            <th></th>
            <th></th>
            <th>id</th>
            <th>image</th>
            <th>description</th>
            <th></th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          <tr
            class="post"
            onClick={"window.location=" + `'/sensor/${models[0].id}/edit'`}
          >
            <td class="px-2">{models[0].id}</td>
            <td class="px-2">{models[0].device_id}</td>
            <td
              class={`px-2 border-4 border-[#${models[1].color_code}] rounded`}
            >
              {models[0].sensor_purpose_id}
            </td>
            <td>
              <img
                src={models[1].image
                  ? `${models[1].image}`
                  : asset(`/icons/no_image.jpg`)}
                width="48"
                height="48"
              />
            </td>
            <td
              class={`px-2 border-4 border-[#${models[1].color_code}] rounded`}
            >
              {models[1].description}
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
