import Title from "../_title.tsx";
import { Effect } from "@effect";
import { FreshContext, Handlers, PageProps } from "$fresh/server.ts";
import { getTargetCookieValCombinedAssign } from "../../utils/browser/headers/cookie.ts";

interface Data {
  results: string[];
  query: string;
}

export const handler: Handlers = {
  async GET(req: Request, ctx: FreshContext) {
    const devices = await getDevices(req);
    const res: Response = await ctx.render();
    return res;
  },
};

const getDevices = async (req: Request) => {
  const id = getTargetCookieValCombinedAssign(req.headers, "id");
  const prog = Effect
    .tryPromise({
      try: () =>
        fetch(`http://localhost:3000/device/`, {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
            "Cookie": id,
          },
        }).then((
          res,
        ) => res.json()),
      catch: (err) =>
        new Error(`In post admin_user/login, something went wrong ${err}`),
    }).pipe(
      Effect.andThen((res) => {
        return res;
      }),
      Effect.catchAll((err) => {
        console.log(err);
      }),
    );

  return await Effect.runPromise(prog);
};

const Index = (props: PageProps) => {
  return (
    <div class="container">
      <Title title="Devices" />
      <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4">
        <a href="/device/new">
          add device
        </a>
      </button>
      <table class="table-fixed">
        <thead>
          <tr>
            <th>Name</th>
            <th>Image</th>
          </tr>
        </thead>
        <tbody>
          <tr class="post" onclick="window.location='/device/{{ device.id }}';">
            <td class="px-2">device_name</td>
            <td>
              <img src="/assets/images/no_image.jpg" width="128" height="128" />
            </td>
          </tr>
        </tbody>
        <tfoot>
          <tr>
            <td></td>
            <td>
              <a href="/?page={{ page - 1 }}&devices_per_page={{ devices_per_page }}">
                Previous
              </a>
              <a href="/?page={{ page + 1 }}&devices_per_page ={{ devices_per_page }}">
                Next
              </a>
            </td>
          </tr>
        </tfoot>
      </table>
    </div>
  );
};

export default Index;
