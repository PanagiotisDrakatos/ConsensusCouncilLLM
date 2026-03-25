import { resolveRedirectTarget } from "../../src/auth/callback";

describe("resolveRedirectTarget", () => {
  it("accepts allowed redirect paths", () => {
    expect(resolveRedirectTarget("/dashboard")).toBe("/dashboard");
  });

  it("rejects empty callback targets", () => {
    expect(resolveRedirectTarget("")).toBeNull();
  });
});
