import { Effect } from "effect";
import Title from "../../_title.tsx";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { createSensorPurpose } from "../../../requests/sensor_purpose.ts";
import { SensorEvents } from "../../../types/request/sensor_event.ts";
import { getSensorEvents } from "../../../requests/sensor_event.ts";

interface Props {
  sensorEvents: SensorEvents;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const sensorEvents = await Effect.runPromise(
      getSensorEvents(req),
    );
    const pageData: Props = {
      sensorEvents: sensorEvents,
    };

    const res: Response = await ctx.render(pageData);
    return res;
  },

  async POST(req: Request) {
    const formData = await req.formData();
    await Effect.runPromise(
      createSensorPurpose(req, formData),
    );

    return new Response(null, {
      status: HttpStatusCode.SEE_OTHER,
      headers: { Location: `/sensor_purpose` },
    });
  },
};

const Page = ({ data }: PageProps<Props>) => {
  return (
    <div class="container">
      <Title title="Create sensor purpose" />
      <div class="w-full max-w-xs">
        <form
          class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4"
          method="post"
          id="create-sensor-purpose-form"
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
            />
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="sensor_event_id"
            >
              sensor purpose id
            </label>
            <select
              class="h-9"
              id="sensor_event_id"
              name="sensor_event_id"
            >
              {data.sensorEvents.sensor_events.map((sensorEvent) => (
                <option value={sensorEvent.id}>
                  {sensorEvent.description}
                </option>
              ))}
            </select>
          </div>

          <div class="flex items-center justify-between">
            <button
              class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              type="submit"
            >
              Create sensor purpose
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Page;
