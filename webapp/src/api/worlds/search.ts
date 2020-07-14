import debug from "debug";
import { request } from "../http";
import type { World } from "./model";
import type { WorldResponse } from "./response";

/** The logger to use */
const LOGGER = debug("multiverse:api:worlds:search");

/** The possible ways to sort worlds */
export type WorldsSortField = "name" | "created" | "updated";

/** The direction in which to sort */
export type SortDirection = "ASCENDING" | "DESCENDING";

/** The representation of a field to sort by */
export interface WorldsSort {
  /** The field to sort by */
  field: WorldsSortField;
  /** The direction to sort in */
  direction?: SortDirection;
}

/** The pagination details for the search */
export interface Pagination {
  /** The offset to use */
  offset: number;
  /** The number of records to request */
  count: number;
}

/** The filters to apply for the search */
export interface Filters {
  owner?: string;
}

/** The paged results of the search */
export interface PagedResults<T> {
  /** The actual entries */
  entries: T[];
  /** The pagination details */
  pagination: {
    count: number;
    offset: number;
    total: number;
  };
}

/**
 * Search for worlds that match the requirements
 * @param filters The filters to apply
 * @param pagination The pagination details to use
 * @param sorts The sorting to use
 */
export async function searchWorlds(
  filters: Filters,
  pagination: Pagination,
  sorts: WorldsSort[]
): Promise<PagedResults<World>> {
  LOGGER("Searching for worlds: %o", { filters, pagination, sorts });

  const urlParams: { [key: string]: any } = {};
  if (filters.owner !== undefined) {
    urlParams.owner = filters.owner;
  }
  urlParams.count = pagination.count;
  urlParams.offset = pagination.offset;
  urlParams.sort = sorts.map((f) => f.field).join(",");

  const result = await request<PagedResults<WorldResponse>>(
    "/worlds{?owner,count,offset,sort}",
    {
      urlParams,
      ignoreCache: true,
    }
  );

  const body = result.body!!;

  return {
    pagination: body.pagination,
    entries: body.entries.map((world) => ({
      name: world.name,
      description: world.description,
      slug: world.url_slug,
    })),
  };
}
