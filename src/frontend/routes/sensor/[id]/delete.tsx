import { FreshContext, Handlers } from "$fresh/server.ts";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { deleteSensor, getSensor } from "../../../requests/sensor.ts";
import { Effect } from "effect";

export const handler: Handlers = {
  async GET(req: Request, ctx: FreshContext) {
    const sensorId = ctx.params.id;
    const models = await Effect.runPromise(
      getSensor(req, sensorId),
    );
    const deviceId = models.models[0].device_id;
    await Effect.runPromise(
      deleteSensor(req, sensorId),
    );

    return new Response(null, {
      status: HttpStatusCode.SEE_OTHER,
      headers: { Location: `/device/${deviceId}/sensor` },
    });
  },
};

const Page = () => {
  return <></>;
};

export default Page;
