import { FreshContext, Handlers } from "$fresh/server.ts";
import HttpStatusCode from "../../../enums/HttpStatusCode.ts";
import { deleteDevice } from "../../../requests/device.ts";
import { Effect } from "effect";

export const handler: Handlers = {
  async GET(req: Request, ctx: FreshContext) {
    const deviceId = ctx.params.id;
    await Effect.runPromise(
      deleteDevice(req, deviceId),
    );

    return new Response(null, {
      status: HttpStatusCode.SEE_OTHER,
      headers: { Location: "/device" },
    });
  },
};

const Page = () => {
  return <></>;
};

export default Page;
