import Title from "../_title.tsx";
import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { SensorPurposesWithRelation } from "../../types/request/capture.ts";
import { GetSensorPurposeWithRelation } from "../../requests/capture.ts";
import { Effect } from "effect";
import HttpStatusCode from "../../enums/HttpStatusCode.ts";

interface Props {
  models: SensorPurposesWithRelation;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const sensorPurposeId = ctx.url.searchParams.get("sensor_purpose_id");
    if (!sensorPurposeId) {
      return new Response(null, {
        status: HttpStatusCode.SEE_OTHER,
        headers: { Location: "/senosr_purpose" },
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
  return (
    <div class="container">
      <Title title="Captures" />
    </div>
  );
};

export default Page;
