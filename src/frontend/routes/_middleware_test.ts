import { assert } from "https://deno.land/std/assert/assert.ts";
import { check_protected_route } from "./_middleware.ts";
import { assertFalse } from "$std/assert/assert_false.ts";

Deno.test("checking of protected route", async (t) => {
  await t.step("passing protected url", async () => {
    assert(check_protected_route("/device"));
    assert(check_protected_route("/device/new"));
    assert(check_protected_route("/admin_user/new"));
    assertFalse(check_protected_route("/greet"));
  });

  await t.step("passing unprotected url", async () => {
    assertFalse(check_protected_route("/greet"));
  });
});
