import { Schema } from "@effect/schema";
import { SensorSchema } from "./sensor.ts";
import { SensorPurposeSchema } from "./sensor_purpose.ts";

export const CaptureSchema = Schema.Struct({
  id: Schema.Number,
  sensor_id: Schema.Number,
  capture_val: Schema.Number,
  shift_digit: Schema.Number,
  created_at: Schema.String,
});

export const SensorAndCapturesSchema = Schema.Tuple(
  SensorSchema,
  Schema.Array(
    CaptureSchema,
  ),
);

export const SensorPurposeWithRelationSchema = Schema.Struct({
  models: Schema.Tuple(
    SensorPurposeSchema,
    Schema.Array(
      SensorAndCapturesSchema,
    ),
  ),
});

export type SensorPurposesWithRelation = Schema.Schema.Type<
  typeof SensorPurposeWithRelationSchema
>;
