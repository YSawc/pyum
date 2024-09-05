import Title from "../../_title.tsx";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { Effect } from "effect";
import {
  editSensorPurpose,
  getSensorPurpose,
} from "../../../requests/sensor_purpose.ts";
import { SensorPurpose } from "../../../types/request/sensor_purpose.ts";

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

  async POST(req: Request, ctx: FreshContext) {
    const sensorPurposeId = ctx.params.id;
    const formData = await req.formData();
    await Effect.runPromise(
      editSensorPurpose(req, sensorPurposeId, formData),
    );

    return new Response(null, {
      status: HttpStatusCode.SEE_OTHER,
      headers: { Location: `/sensor_purpose/${sensorPurposeId}` },
    });
  },
};

const Page = ({ data }: PageProps<Props>) => {
  const { sensorPurpose } = data;

  return (
    <div class="container">
      <Title title="Edit Device" />
      <div class="w-full max-w-xs">
        <form
          class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4"
          method="post"
          id="edit-sensor_purpose-form"
        >
          <div class="mb-4">
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="description"
            >
              description
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="description"
              name="description"
              type="text"
              placeholder="description"
              value={`${sensorPurpose.description}`}
            />
          </div>
          <div class="flex items-center justify-between">
            <button
              class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              type="submit"
            >
              Edit Sensor Purpose
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Page;
