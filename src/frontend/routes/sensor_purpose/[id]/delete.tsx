import { FreshContext, Handlers } from "$fresh/server.ts";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { deleteSensorPurpose } from "../../../requests/sensor_purpose.ts";
import { Effect } from "effect";

export const handler: Handlers = {
  async GET(req: Request, ctx: FreshContext) {
    const deviceId = ctx.params.id;
    await Effect.runPromise(
      deleteSensorPurpose(req, deviceId),
    );

    return new Response(null, {
      status: HttpStatusCode.SEE_OTHER,
      headers: { Location: "/sensor_purpose" },
    });
  },
};

const Page = () => {
  return <></>;
};

export default Page;
