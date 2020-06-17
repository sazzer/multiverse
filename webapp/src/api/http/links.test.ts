import * as testSubject from "./links";

describe("Parse links", () => {
  it("Correctly parses an empty header", () => {
    const links = testSubject.parseLinks("");
    expect(links.links).toHaveLength(0);
  });
  it("Correctly parses a single link", () => {
    const links = testSubject.parseLinks('</example>; rel="self"');
    expect(links.links).toHaveLength(1);
    expect(links.links).toContainEqual({
      target: "/example",
      parameters: [
        {
          key: "rel",
          value: "self",
        },
      ],
    });
  });
  it("Correctly parses a link with multiple parameters", () => {
    const links = testSubject.parseLinks(
      '</example>; rel="self"; title="Self Link"'
    );
    expect(links.links).toHaveLength(1);
    expect(links.links).toContainEqual({
      target: "/example",
      parameters: [
        {
          key: "rel",
          value: "self",
        },
        {
          key: "title",
          value: "Self Link",
        },
      ],
    });
  });
  it("Correctly parses a set of link", () => {
    const links = testSubject.parseLinks(
      '</example>; rel="self", </other>; rel="other"'
    );
    expect(links.links).toHaveLength(2);
    expect(links.links).toContainEqual({
      target: "/example",
      parameters: [
        {
          key: "rel",
          value: "self",
        },
      ],
    });
    expect(links.links).toContainEqual({
      target: "/other",
      parameters: [
        {
          key: "rel",
          value: "other",
        },
      ],
    });
  });
});
