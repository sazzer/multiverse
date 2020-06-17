import * as testSubject from "./links";

describe("Parse links", () => {
  test("Correctly parses an empty header", () => {
    const links = testSubject.parseLinks("");
    expect(links.links).toHaveLength(0);
  });

  test("Correctly parses a single link", () => {
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

  test("Correctly parses a link with multiple parameters", () => {
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

  test("Correctly parses a set of link", () => {
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

describe("Find Links", () => {
  describe("Finding by link relation", () => {
    test("When no links match", () => {
      const links = testSubject.parseLinks('</example>; rel="self"');
      const link = links.getLinkByRel("other");
      expect(link).toBeUndefined;
    });

    test("When one link matches", () => {
      const links = testSubject.parseLinks('</example>; rel="self"');
      const link = links.getLinkByRel("self");
      expect(link).toEqual({
        target: "/example",
        parameters: [
          {
            key: "rel",
            value: "self",
          },
        ],
      });
    });

    test("When two links match", () => {
      const links = testSubject.parseLinks(
        '</first>; rel="self",</second>; rel="self"'
      );
      const link = links.getLinkByRel("self");
      expect(link).toEqual({
        target: "/first",
        parameters: [
          {
            key: "rel",
            value: "self",
          },
        ],
      });
    });
  });
});
