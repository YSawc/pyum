import { Schema } from "@effect/schema";
import { SensorPurposeSchema } from "./sensor_purpose.ts";

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
);

export type Sensor = Schema.Schema.Type<typeof SensorSchema>;

export const GetSensorsSchema = Schema.Struct({
  models: Schema.Array(SensorAndPurposeSchema),
});

export const GetSensorSchema = Schema.Struct({
  sensor: SensorSchema,
});
export type GetSensor = Schema.Schema.Type<typeof GetSensorSchema>;

export type GetSensors = Schema.Schema.Type<
  typeof GetSensorsSchema
>;
