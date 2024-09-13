import { Schema } from "@effect/schema";

export const SensorPurposeSchema = Schema.Struct({
  id: Schema.Number,
  description: Schema.String,
  color_code: Schema.String,
  image: Schema.String,
});
export type SensorPurpose = Schema.Schema.Type<typeof SensorPurposeSchema>;

export const SensorPurposesSchema = Schema.Struct({
  sensor_purposes: Schema.Array(SensorPurposeSchema),
});
export type SensorPurposes = Schema.Schema.Type<typeof SensorPurposesSchema>;

export const GetSensorPurposeSchema = Schema.Struct({
  sensor_purpose: SensorPurposeSchema,
});
export type GetSensorPurpose = Schema.Schema.Type<
  typeof GetSensorPurposeSchema
>;

export const GetSensorPurposesSchema = SensorPurposesSchema;
export type GetSensorPurposes = Schema.Schema.Type<
  typeof GetSensorPurposesSchema
>;
