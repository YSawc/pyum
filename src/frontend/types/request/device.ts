import { Schema } from "@effect/schema";

export const DeviceSchema = Schema.Struct({
  id: Schema.Number,
  name: Schema.String,
  image: Schema.String,
});
export type Device = Schema.Schema.Type<typeof DeviceSchema>;

export const DevicesSchema = Schema.Struct({
  devices: Schema.Array(DeviceSchema),
});
export type Devices = Schema.Schema.Type<typeof DevicesSchema>;

export const GetDeviceSchema = Schema.Struct({
  device: DeviceSchema,
});
export type GetDevice = Schema.Schema.Type<typeof GetDeviceSchema>;

export const GetDevicesSchema = DevicesSchema;
export type GetDevices = Schema.Schema.Type<typeof GetDevicesSchema>;
