import { Schema } from "@effect/schema";

export const SensorPurposeSchema = Schema.Struct({
  id: Schema.Number,
  descripion: Schema.String,
});
export type SensorPurpose = Schema.Schema.Type<typeof SensorPurposeSchema>;

export const SensorPurposesSchema = Schema.Struct({
  sensor_purposes: Schema.Array(SensorPurposeSchema),
});
export type SensorPurposes = Schema.Schema.Type<typeof SensorPurposesSchema>;

export const GetSensorPurposeSchema = Schema.Struct({
  device: SensorPurposeSchema,
});
export type GetSensorPurpose = Schema.Schema.Type<
  typeof GetSensorPurposeSchema
>;

export const GetSensorPurposesSchema = SensorPurposesSchema;
export type GetSensorPurposes = Schema.Schema.Type<
  typeof GetSensorPurposesSchema
>;
