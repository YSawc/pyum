// DO NOT EDIT. This file is generated by Fresh.
// This file SHOULD be checked into source version control.
// This file is automatically updated during development when running `dev.ts`.

import * as $_404 from "./routes/_404.tsx";
import * as $_app from "./routes/_app.tsx";
import * as $_footer from "./routes/_footer.tsx";
import * as $_head from "./routes/_head.tsx";
import * as $_header from "./routes/_header.tsx";
import * as $_layout from "./routes/_layout.tsx";
import * as $_middleware from "./routes/_middleware.ts";
import * as $_title from "./routes/_title.tsx";
import * as $admin_user_login_index from "./routes/admin_user/login/index.tsx";
import * as $admin_user_new_index from "./routes/admin_user/new/index.tsx";
import * as $device_id_delete from "./routes/device/[id]/delete.tsx";
import * as $device_id_edit from "./routes/device/[id]/edit.tsx";
import * as $device_id_index from "./routes/device/[id]/index.tsx";
import * as $device_index from "./routes/device/index.tsx";
import * as $device_new_index from "./routes/device/new/index.tsx";
import * as $greet_name_ from "./routes/greet/[name].tsx";
import * as $sensor_purpose_index from "./routes/sensor_purpose/index.tsx";
import * as $Counter from "./islands/Counter.tsx";
import * as $routes_device_id_index_ConfirmButton from "./islands/routes/device/[id]/index/ConfirmButton.tsx";
import { type Manifest } from "$fresh/server.ts";

const manifest = {
  routes: {
    "./routes/_404.tsx": $_404,
    "./routes/_app.tsx": $_app,
    "./routes/_footer.tsx": $_footer,
    "./routes/_head.tsx": $_head,
    "./routes/_header.tsx": $_header,
    "./routes/_layout.tsx": $_layout,
    "./routes/_middleware.ts": $_middleware,
    "./routes/_title.tsx": $_title,
    "./routes/admin_user/login/index.tsx": $admin_user_login_index,
    "./routes/admin_user/new/index.tsx": $admin_user_new_index,
    "./routes/device/[id]/delete.tsx": $device_id_delete,
    "./routes/device/[id]/edit.tsx": $device_id_edit,
    "./routes/device/[id]/index.tsx": $device_id_index,
    "./routes/device/index.tsx": $device_index,
    "./routes/device/new/index.tsx": $device_new_index,
    "./routes/greet/[name].tsx": $greet_name_,
    "./routes/sensor_purpose/index.tsx": $sensor_purpose_index,
  },
  islands: {
    "./islands/Counter.tsx": $Counter,
    "./islands/routes/device/[id]/index/ConfirmButton.tsx":
      $routes_device_id_index_ConfirmButton,
  },
  baseUrl: import.meta.url,
} satisfies Manifest;

export default manifest;
