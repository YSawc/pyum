import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { asset } from "$fresh/runtime.ts";
import { getSensorsRelatedDevice } from "../../requests/sensor.ts";
import { GetDevicesWithRelation } from "../../types/request/sensor.ts";
import Title from "../_title.tsx";
import { Effect } from "effect";

interface Props {
  models: GetDevicesWithRelation;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const device_id: string | null = ctx.url.searchParams.get("device_id");
    const models = await Effect.runPromise(
      getSensorsRelatedDevice(req, device_id),
    );
    const data: Props = {
      models: models,
    };
    const res: Response = await ctx.render(data);
    return res;
  },
};

const Page = ({ data }: PageProps<Props>) => {
  const devices: GetDevicesWithRelation = data.models;
  const custom_devices = devices.models.map((device) => {
    const sensor_ids = device.sensor_ids.split(",");
    const sensor_purpose_ids = device.sensor_purpose_ids.split(",");
    const trigger_limit_vals = device.trigger_limit_vals.split(",");
    const trigger_limit_sequence_counts = device.trigger_limit_sequence_counts
      .split(",");
    const sensor_event_ids = device.sensor_event_ids.split(",");
    const sensor_event_descriptions = device.sensor_event_descriptions.split(
      ",",
    );
    const sensor_event_images = device.sensor_event_images.split(
      ",",
    );
    return {
      device_id: device.device_id,
      device_name: device.device_name,
      device_image: device.device_image,
      sensor_ids,
      sensor_purpose_ids,
      trigger_limit_vals,
      trigger_limit_sequence_counts,
      sensor_event_ids,
      sensor_event_descriptions,
      sensor_event_images,
    };
  });

  return (
    <div class="container">
      <Title title="Sensors related device" />
      <div class="flex flex-col gap gap-8">
        {custom_devices.map((device) => (
          <div class="max-w-[440px] items-center border-4 border-[#65e6fa] rounded-xl py-4 px-4">
            <a href={`/device/${device.device_id}`}>
              <div class="text-center">
                <span class="text-center my-2  text-xl">
                  {device.device_name}
                </span>
              </div>
              <div class="flex justify-center">
                <img
                  src={device.device_image
                    ? `${device.device_image}`
                    : asset(`/icons/no_image.jpg`)}
                  width="256"
                  height="256"
                />
              </div>
            </a>
            <div class="text-center text-lg mb-2">
              <span>sensors</span>
            </div>
            <div class="flex flex-col text-lg">
              {device.sensor_ids.map((_, index) => (
                <a href={`/sensor/${device.sensor_ids[index]}`}>
                  <div
                    class={`flex mx-4 min-h-[56px] rounded align-middle my-auto items-center px-2
              `}
                  >
                    <span>
                      <img
                        src={device.sensor_event_images[index]
                          ? `${device.sensor_event_images[index]}`
                          : asset(`/icons/no_image.jpg`)}
                        width="48"
                        height="48"
                      />
                    </span>
                    <span class="ml-2">
                      {device.sensor_event_descriptions[index]}
                    </span>
                    <span class="ml-2">
                      {device.trigger_limit_vals[index]}
                    </span>
                    <span>
                      <img
                        src="icons/chart.svg"
                        width="24"
                        height="24"
                      />
                    </span>
                    <span>
                      {device.trigger_limit_sequence_counts[index]}
                    </span>
                    <span>
                      <img
                        src="icons/cycle.svg"
                        width="24"
                        height="24"
                      />
                    </span>
                  </div>
                </a>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Page;
