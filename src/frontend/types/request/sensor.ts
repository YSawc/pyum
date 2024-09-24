import { Schema } from "@effect/schema";
import { SensorPurposeSchema } from "./sensor_purpose.ts";
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

export const DeviceWithRelationSchema = Schema.Struct({
  device_id: Schema.Number,
  device_name: Schema.String,
  device_image: Schema.String,
  sensor_ids: Schema.String,
  sensor_purpose_ids: Schema.String,
  trigger_limit_vals: Schema.String,
  trigger_limit_sequence_counts: Schema.String,
  sensor_event_ids: Schema.String,
  sensor_event_descriptions: Schema.String,
  sensor_event_images: Schema.String,
});

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
