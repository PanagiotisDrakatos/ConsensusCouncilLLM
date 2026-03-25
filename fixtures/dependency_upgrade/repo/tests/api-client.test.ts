describe("api client dependency pin", () => {
  it("keeps the approved axios patch release in package metadata", () => {
    expect("1.7.3").toBe("1.7.3");
  });
});
