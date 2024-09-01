import { Option } from "effect";

export const wrapOption = <A>(value: A): Option.Option<A> =>
  (typeof value === "undefined" || value === null)
    ? Option.none()
    : Option.some(value);
