import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { ConfirmButton } from "../../../islands/routes/device/[id]/index/ConfirmButton.tsx";
import { getDevice } from "../../../requests/device.tsx";
import { Device } from "../../../types/request/device/index.ts";
import Title from "../../_title.tsx";
import { Effect } from "@effect";

interface Props {
  device: Device;
}

export const handler: Handlers<Props> = {
  async GET(req: Request, ctx: FreshContext) {
    const deviceId = ctx.params.id;
    const device = await Effect.runPromise(
      getDevice(req, deviceId),
    );
    const data: Props = {
      device,
    };
    const res: Response = await ctx.render(data);
    return res;
  },
};

const Page = ({ data }: PageProps<Props>) => {
  const { device } = data.device;

  return (
    <div class="container">
      <Title title="Device detail" />
      <ConfirmButton
        text="delete"
        confirmText="really delete?"
        url={`/device/${device.id}/delete`}
      />
      <table class="table-fixed">
        <thead>
          <tr>
            <th>Name</th>
            <th>Image</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td
              class="px-2"
              onclick={`window.location='/device/${device.id}/edit'`}
            >
              {device.name}
            </td>
            <td>
              <img
                src={`${device.image}`}
                width="128"
                height="128"
              />
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  );
};

export default Page;
