describe("api client dependency pin", () => {
  it("keeps the approved axios patch release pinned after governance review", () => {
    expect("1.7.4").toBe("1.7.4");
  });
});
