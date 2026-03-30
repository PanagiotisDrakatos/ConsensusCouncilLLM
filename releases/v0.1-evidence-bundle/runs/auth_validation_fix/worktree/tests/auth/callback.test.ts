import { resolveRedirectTarget } from "../../src/auth/callback";

describe("resolveRedirectTarget", () => {
  it("accepts allowed redirect paths", () => {
    expect(resolveRedirectTarget("/dashboard")).toBe("/dashboard");
  });

  it("accepts same-origin absolute callback URLs", () => {
    expect(resolveRedirectTarget("https://app.example.com/dashboard")).toBe(
      "/dashboard"
    );
  });

  it("rejects malformed callback targets", () => {
    expect(resolveRedirectTarget("%%%")).toBeNull();
  });

  it("rejects foreign-origin callback targets", () => {
    expect(resolveRedirectTarget("https://evil.example.com/dashboard")).toBeNull();
  });
});
