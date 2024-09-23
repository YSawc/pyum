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
    const models = await Effect.runPromise(
      getSensorsRelatedDevice(req),
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
  return (
    <div class="container">
      <Title title="Sensors related device" />
      <div class="flex flex-col gap gap-8">
        {devices.models.map((device) => (
          <div class="max-w-[440px] items-center border-4 border-[#65e6fa] rounded-xl py-4 px-4">
            <a href={`/device/${device[0].id}`}>
              <div class="text-center">
                <span class="text-center my-2  text-xl">
                  {device[0].name}
                </span>
              </div>
              <div class="flex justify-center">
                <img
                  src={device[0].image
                    ? `${device[0].image}`
                    : asset(`/icons/no_image.jpg`)}
                  width="256"
                  height="256"
                />
              </div>
            </a>
            <div class="text-center text-lg mb-2">
              <span>sensors</span>
            </div>
            <div class="flex flex-col gap gap-y-8 text-lg">
              {device[1].map((sensor) => (
                <a href={`/sensor/${sensor[0].id}`}>
                  <div
                    class={`flex mx-4 min-h-[56px] border-2 border-[#${sensor[1].color_code
                      }] rounded align-middle my-auto items-center px-2
              `}
                  >
                    <span>
                      <img
                        src={sensor[2].image
                          ? `${sensor[2].image}`
                          : asset(`/icons/no_image.jpg`)}
                        width="48"
                        height="48"
                      />
                    </span>
                    <span class="ml-2">
                      {sensor[1].description}
                    </span>
                    <span class="ml-2">
                      {sensor[0].trigger_limit_val}
                    </span>
                    <span>
                      <img
                        src="icons/chart.svg"
                        width="24"
                        height="24"
                      />
                    </span>
                    <span>
                      {sensor[0].trigger_limit_sequence_count}
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
