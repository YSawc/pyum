import { Effect } from "effect";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import Title from "../../_title.tsx";
import { editSensor, getSensor } from "../../../requests/sensor.ts";
import { GetSensor } from "../../../types/request/sensor.ts";
import { getSensorPurposes } from "../../../requests/sensor_purpose.ts";
import { GetSensorPurposes } from "../../../types/request/sensor_purpose.ts";

interface Props {
  models: GetSensor;
  sensorPurposes: GetSensorPurposes;
}

export const handler: Handlers = {
  async GET(req: Request, ctx: FreshContext) {
    const sensorId = ctx.params.id;
    const models = await Effect.runPromise(
      getSensor(req, sensorId),
    );
    const sensorPurposes = await Effect.runPromise(
      getSensorPurposes(req),
    );
    const pageData: Props = {
      models: models,
      sensorPurposes: sensorPurposes,
    };
    const res: Response = await ctx.render(pageData);
    return res;
  },

  async POST(req: Request, ctx: FreshContext) {
    const formData = await req.formData();
    const sensorId = ctx.params.id;
    await Effect.runPromise(
      editSensor(req, sensorId, formData),
    );

    return new Response(null, {
      status: HttpStatusCode.SEE_OTHER,
      headers: { Location: `/sensor/${sensorId}` },
    });
  },
};

const Page = ({ data }: PageProps<Props>) => {
  const { models } = data.models;

  return (
    <div class="container">
      <Title title="Edit sensor" />
      <div class="w-full max-w-xs">
        <form
          class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4"
          method="post"
          id="create-sensor-form"
        >
          <div class="mb-4">
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="sensor_purpose_id"
            >
              sensor purpose
            </label>
            <select
              class="h-9"
              id="sensor_purpose_id"
              name="sensor_purpose_id"
              value={`${models[0].sensor_purpose_id}`}
              required
            >
              {data.sensorPurposes.sensor_purposes.map((sensorPurpose) => (
                <option value={sensorPurpose[0].id}>
                  {sensorPurpose[0].description}
                </option>
              ))}
            </select>
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="trigger_limit_val"
            >
              trigger limit val
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="trigger_limit_val"
              name="trigger_limit_val"
              type="number"
              value={`${models[0].trigger_limit_val}`}
              required
            />
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="trigger_limit_sequence_count"
            >
              trigger limit sequence count
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="trigger_limit_sequence_count"
              name="trigger_limit_sequence_count"
              type="number"
              placeholder="trigger_limit_sequence_count"
              value={`${models[0].trigger_limit_sequence_count}`}
            />
          </div>
          <div class="flex items-center justify-between">
            <button
              class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              type="submit"
            >
              Edit sensor
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Page;
