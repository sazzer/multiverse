import debug from "debug";

/** The logger to use */
const LOGGER = debug("multiverse:api:http:links");

/** Regex to extract the link target */
const LINK_TARGET_REGEX = /<(.+)>(.*)/;

/** The shape of a link parameter */
export interface LinkParameter {
  /** The parameter key */
  key: string;

  /** The parameter value */
  value: string;
}

/** The shape of a link */
export interface Link {
  /** The target of the link */
  target: string;

  /** The */
  parameters: LinkParameter[];
}

/** Representation of a set of links */
export class Links {
  /** The actual links */
  readonly links: Link[];

  /**
   * Construct the links
   * @param links The links
   */
  constructor(links: Link[]) {
    this.links = links;
  }

  /**
   * Get the link that has the given link relation
   * @param rel The link relation to look for
   */
  getLinkByRel(rel: string): Link | undefined {
    return this.links.find((link) =>
      link.parameters.find(
        (parameter) => parameter.key === "rel" && parameter.value === rel
      )
    );
  }
}

/**
 * Parse a single link value from the link header
 * @param value The string to parse
 */
export function parseLink(value: string): Link {
  const match = value.match(LINK_TARGET_REGEX);
  if (match) {
    const target = match[1];
    const parameters = match[2]
      .split(";")
      .map((param) => param.trim())
      .filter((param) => param.length > 0)
      .map((param) => param.split("=", 2))
      .filter((param) => param.length === 2)
      .map(([key, value]) => {
        let trimmedValue = value.trim();
        if (trimmedValue.startsWith('"') && trimmedValue.endsWith('"')) {
          trimmedValue = trimmedValue.substr(1, trimmedValue.length - 2);
        }
        return {
          key: key.trim(),
          value: trimmedValue,
        };
      });
    return { target, parameters };
  } else {
    throw new Error("Malformed link");
  }
}

/**
 * Parse an entire set of linsk from the link header
 * @param value The value to parse
 */
export function parseLinks(value: string): Links {
  const links = value
    .split(",")
    .map((link) => link.trim())
    .filter((link) => link.length > 0)
    .map(parseLink);
  LOGGER("Parsed links: %o", links);
  return new Links(links);
}
