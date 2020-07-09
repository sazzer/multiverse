import { SeedData } from "./seed";
import { v4 as uuidv4 } from "uuid";

/**
 * Seed Data representing a world
 */
export class World extends SeedData {
  /**
   * Construct the world
   */
  constructor() {
    super();
    this._worldId = uuidv4();
    this._version = uuidv4();
    this._created = new Date();
    this._updated = new Date();
    this._name = uuidv4();
    this._description = "";
    this._urlSlug = uuidv4();
    this._ownerId = uuidv4();
  }

  /**
   * Set the name of this world
   * @param {string} value The new value for the name
   */
  withName(value) {
    this._name = value;
    return this;
  }

  /**
   * Set the description of this world
   * @param {string} value The new value for the description
   */
  withDescription(value) {
    this._description = value;
    return this;
  }

  /**
   * Set the URL Slug of this world
   * @param {string} value The new value for the URL Slug
   */
  withUrlSlug(value) {
    this._urlSlug = value;
    return this;
  }

  /**
   * Set the Owner ID of this world
   * @param {string} value The new value for the Owner ID
   */
  withOwnerId(value) {
    this._ownerId = value;
    return this;
  }

  async sql() {
    return "INSERT INTO worlds(world_id, version, created, updated, name, description, url_slug, owner_id) VALUES($1, $2, $3, $4, $5, $6, $7, $8)";
  }

  async binds() {
    return [
      this._worldId,
      this._version,
      this._created,
      this._updated,
      this._name,
      this._description,
      this._urlSlug,
      this._ownerId,
    ];
  }
}

async function hashPassword(password) {
  const { encoded } = await argon2.hash({
    pass: password,
    salt: "saltsaltsaltsaltsaltsaltsaltsalt",
  });
  return encoded;
}
