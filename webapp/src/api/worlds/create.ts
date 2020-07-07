import { Problem, request } from "../http";

import { DuplicateUrlSlugError } from "./errors";
import { World } from "./model";
import { WorldResponse } from "./response";

/**
 * Create a new World
 * @param world The details of the world to create
 */
export async function createWorld(world: World) {
  try {
    await request<WorldResponse>("/worlds", {
      method: "POST",
      body: {
        name: world.name,
        description: world.description,
        url_slug: world.slug,
      },
    });
  } catch (e) {
    if (e instanceof Problem) {
      switch (e.type) {
        case "tag:multiverse,2020:worlds/problems/duplicate_url_slug":
          throw new DuplicateUrlSlugError();
      }
    }
    throw e;
  }
}
