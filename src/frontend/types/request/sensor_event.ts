import { Schema } from "@effect/schema";

export const SensorEventSchema = Schema.Struct({
  id: Schema.Number,
  description: Schema.String,
  image: Schema.String,
});

export type SensorEvent = Schema.Schema.Type<typeof SensorEventSchema>;

export const SensorEventsSchema = Schema.Struct({
  sensor_events: Schema.Array(SensorEventSchema),
});
export type SensorEvents = Schema.Schema.Type<typeof SensorEventsSchema>;
