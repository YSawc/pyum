import Title from "../_title.tsx";
import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { SensorPurposesWithRelation } from "../../types/request/capture.ts";
import { GetSensorPurposeWithRelation } from "../../requests/capture.ts";
import { Effect } from "effect";
import HttpStatusCode from "../../enums/HttpStatusCode.ts";
import { Chart } from "$fresh_charts/mod.ts";
import { ChartColors, transparentize } from "$fresh_charts/utils.ts";
import { LimitationButton } from "../../islands/capture/limitationButton.tsx";

interface Props {
  models: SensorPurposesWithRelation;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const sensorPurposeId = ctx.url.searchParams.get("sensor_purpose_id");
    if (!sensorPurposeId) {
      return new Response(null, {
        status: HttpStatusCode.SEE_OTHER,
        headers: { Location: "/sensor_purpose" },
      });
    }
    const limit: string | null = ctx.url.searchParams.get("limit");
    const models = await Effect.runPromise(
      GetSensorPurposeWithRelation(req, sensorPurposeId, limit),
    );
    const data: Props = {
      models: models,
    };
    const res: Response = await ctx.render(data);
    return res;
  },
};

const Page = ({ data }: PageProps<Props>) => {
  const sensorPurpose = data.models.models[0];
  const models = data.models.models[1];

  return (
    <div>
      <div class="container">
        <Title title={`Captures related ${sensorPurpose.description}`} />
      </div>
      <a
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4"
        href={`/sensor_purpose/${sensorPurpose.id}`}
      >
        Back to sensor purpose
      </a>
      <div class="flex my-4 py-2 align-middle justify-center ">
        <p class="mr-4 my-auto">limit number</p>
        {[100, 200, 300, 400, 500].map((elm) => {
          return (
            <LimitationButton
              limit_num={elm}
            />
          );
        })}
      </div>
      {models.map((model) => (
        <div class="p-4 mx-auto max-w-screen-md">
          <p>
            device_id:{model[0].device_id},
          </p>
          <p>
            trigger_limit_val:{model[0].trigger_limit_val},
          </p>
          <p>
            trigger_limit_sequence_count:{" "}
            {model[0].trigger_limit_sequence_count}
          </p>
          <Chart
            type="line"
            options={{
              devicePixelRatio: 1,
              scales: { y: { beginAtZero: true } },
            }}
            data={{
              labels: model[1].map((elm) => elm.created_at),
              datasets: [
                {
                  label: "Captures",
                  data: model[1].map((elm) =>
                    elm.capture_val / (10 ** elm.shift_digit)
                  ),
                  borderColor: ChartColors.Red,
                  backgroundColor: transparentize(ChartColors.Red, 0.5),
                  borderWidth: 1,
                },
                {
                  label: "Trigger limit value",
                  data: [...Array(model[1].length)].map(() =>
                    model[0].trigger_limit_val
                  ),
                  borderColor: ChartColors.Grey,
                  backgroundColor: transparentize(ChartColors.Grey, 0.5),
                  borderWidth: 4,
                },
              ],
            }}
          />
        </div>
      ))}
    </div>
  );
};

export default Page;
