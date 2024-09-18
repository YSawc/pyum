import Title from "../_title.tsx";
import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { SensorPurposesWithRelation } from "../../types/request/capture.ts";
import { GetSensorPurposeWithRelation } from "../../requests/capture.ts";
import { Effect } from "effect";
import HttpStatusCode from "../../enums/HttpStatusCode.ts";
import { Chart } from "$fresh_charts/mod.ts";
import { ChartColors, transparentize } from "$fresh_charts/utils.ts";

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
    const models = await Effect.runPromise(
      GetSensorPurposeWithRelation(req, sensorPurposeId),
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
      <div class="p-4 mx-auto max-w-screen-md">
        <Chart
          type="line"
          options={{
            devicePixelRatio: 1,
            scales: { y: { beginAtZero: true } },
          }}
          data={{
            labels: [
              "2024/09/18:13:21",
              "2024/09/18:13:22",
              "2024/09/18:13:23",
              "2024/09/18:13:24",
              "2024/09/18:13:25",
              "2024/09/18:13:26",
              "2024/09/18:13:27",
              "2024/09/18:13:28",
              "2024/09/18:13:29",
            ],
            datasets: [
              {
                label: "Captures",
                data: [1.3, 0.3, 2.8, 2.6, 1.1, 0.3, 0.1, 0.0, 0.0],
                borderColor: ChartColors.Red,
                backgroundColor: transparentize(ChartColors.Red, 0.5),
                borderWidth: 1,
              },
              {
                label: "Base",
                data: [2, 2, 2, 2, 2, 2, 2, 2, 2],
                borderColor: ChartColors.Grey,
                backgroundColor: transparentize(ChartColors.Grey, 0.5),
                borderWidth: 4,
              },
            ],
          }}
        />
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
