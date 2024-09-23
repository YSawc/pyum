import { Schema } from "@effect/schema";
import { SensorPurposeSchema } from "./sensor_purpose.ts";
import { DeviceSchema } from "./device.ts";
import { SensorEventSchema } from "./sensor_event.ts";

export const SensorSchema = Schema.Struct({
  id: Schema.Number,
  device_id: Schema.Number,
  sensor_purpose_id: Schema.Number,
  trigger_limit_val: Schema.Number,
  trigger_limit_sequence_count: Schema.Number,
  created_at: Schema.String,
  updated_at: Schema.String,
});

export const SensorAndPurposeSchema = Schema.Tuple(
  SensorSchema,
  SensorPurposeSchema,
  SensorEventSchema,
);

export const DeviceWithRelationSchema = Schema.Tuple(
  DeviceSchema,
  Schema.Array(
    SensorAndPurposeSchema,
  ),
);

export const DevicesWithRelationSchema = Schema.Struct({
  models: Schema.Array(DeviceWithRelationSchema),
});

export type Sensor = Schema.Schema.Type<typeof SensorSchema>;

export const GetSensorSchema = Schema.Struct({
  models: SensorAndPurposeSchema,
});

export type GetSensor = Schema.Schema.Type<typeof GetSensorSchema>;

export const GetSensorsSchema = Schema.Struct({
  models: Schema.Array(SensorAndPurposeSchema),
});

export type GetDevicesWithRelation = Schema.Schema.Type<
  typeof DevicesWithRelationSchema
>;
