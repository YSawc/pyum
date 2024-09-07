import { Schema } from "@effect/schema";

export const SensorSchema = Schema.Struct({
  id: Schema.Number,
  device_id: Schema.Number,
  sensor_purpose_id: Schema.Number,
  trigger_limit_val: Schema.Number,
  trigger_limit_sequence_count: Schema.Number,
  created_at: Schema.String,
  updated_at: Schema.String,
});
export type Sensor = Schema.Schema.Type<typeof SensorSchema>;

export const SensorsSchema = Schema.Struct({
  sensors: Schema.Array(SensorSchema),
});
export type Sensors = Schema.Schema.Type<typeof SensorsSchema>;

export const GetSensorSchema = Schema.Struct({
  sensor: SensorSchema,
});
export type GetSensor = Schema.Schema.Type<typeof GetSensorSchema>;

export const GetSensorsSchema = SensorsSchema;
export type GetSensors = Schema.Schema.Type<typeof GetSensorsSchema>;
