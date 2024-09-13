import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { getSensorsRelatedDevice } from "../../requests/sensor.ts";
import { GetDevicesWithRelation } from "../../types/request/sensor.ts";
import Title from "../_title.tsx";
import { Effect } from "effect";

interface Props {
  // deviceId: string;
  models: GetDevicesWithRelation;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const models = await Effect.runPromise(
      getSensorsRelatedDevice(req),
    );
    const data: Props = {
      models: models.models,
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
        {devices.map((device) => (
          <div class="max-w-[440px] items-center border border-4 border-[#65e6fa] rounded-xl py-4 px-4">
            <div class="text-center">
              <span class="text-center my-2  text-xl">
                {device[0].name}
              </span>
            </div>
            <div class="flex justify-center">
              <img
                src={`${device[0].image}`}
                width="256"
                height="256"
              />
            </div>
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
                      {sensor[1].description}/
                    </span>
                    <span>
                      {sensor[0].trigger_limit_val}count/
                    </span>
                    <span>
                      {sensor[0].trigger_limit_sequence_count}
                    </span>
                    <span>
                      <img
                        src="data:image/png;base64,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"
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
