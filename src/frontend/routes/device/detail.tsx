import { FunctionComponent } from "https://esm.sh/v128/preact@10.19.6/src/index.js";
import Title from "../_title.tsx";

const Detail: FunctionComponent = () => {
  return (
    <div class="container">
      <Title title="Detail" />
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

export default Detail;
