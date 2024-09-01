import { Schema } from "@effect/schema";

export const SimpleResSchema = Schema.Struct({
  message: Schema.String,
});
export type SimpleRes = Schema.Schema.Type<typeof SimpleResSchema>;
