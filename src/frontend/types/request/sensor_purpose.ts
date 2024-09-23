import { Schema } from "@effect/schema";
import { SensorEventSchema } from "./sensor_event.ts";

export const SensorPurposeSchema = Schema.Struct({
  id: Schema.Number,
  sensor_event_id: Schema.Number,
  description: Schema.String,
});
export type SensorPurpose = Schema.Schema.Type<typeof SensorPurposeSchema>;

export const SensorPurposesSchema = Schema.Struct({
  sensor_purposes: Schema.Array(
    Schema.Tuple(
      SensorPurposeSchema,
      SensorEventSchema,
    ),
  ),
});
export type SensorPurposes = Schema.Schema.Type<typeof SensorPurposesSchema>;

export const GetSensorPurposeSchema = Schema.Struct({
  sensor_purpose: SensorPurposeSchema,
  sensor_event: SensorEventSchema,
});
export type GetSensorPurpose = Schema.Schema.Type<
  typeof GetSensorPurposeSchema
>;

export const GetSensorPurposesSchema = SensorPurposesSchema;
export type GetSensorPurposes = Schema.Schema.Type<
  typeof GetSensorPurposesSchema
>;
