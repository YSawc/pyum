import { assert } from "https://deno.land/std/assert/assert.ts";
import { check_protected_route } from "./_middleware.ts";
import { assertFalse } from "$std/assert/assert_false.ts";

Deno.test("checking of protected route", async (t) => {
  await t.step("passing protected url", () => {
    assert(check_protected_route("/device"));
    assert(check_protected_route("/sensor_purpose"));
    assert(check_protected_route("/sensor"));
  });

  await t.step("passing unprotected url", () => {
    assertFalse(
      check_protected_route("/greet"),
    );
  });
});
