import { Option } from "@effect";
import { wrapOption } from "../effect/option.ts";
import HttpStatusCode from "../../enums/HttpStatusCode.ts";

export const wrapSomeOrResponse = <A>(
  maybeValue: A,
  res: Response,
): A | Response => {
  const wrapVal = wrapOption(maybeValue);
  return Option.match(wrapVal, {
    onSome: (val: A) => {
      return val;
    },
    onNone: () => {
      return res;
    },
  });
};
