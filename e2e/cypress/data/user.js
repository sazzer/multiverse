import { SeedData } from "./seed";
import argon2 from "argon2-wasm-pro";
import { v4 as uuidv4 } from "uuid";

/**
 * Seed Data representing a user
 */
export class User extends SeedData {
  /**
   * Construct the user
   */
  constructor() {
    super();
    this._userId = uuidv4();
    this._version = uuidv4();
    this._created = new Date();
    this._updated = new Date();
    this._username = uuidv4();
    this._displayName = uuidv4();
    this._emailAddress = `${uuidv4()}@example.com`;
    this._password = "TODO";
  }

  /**
   * Set the username of this user
   * @param {string} value The new value for the username
   */
  withUsername(value) {
    this._username = value;
    return this;
  }

  /**
   * Set the display name of this user
   * @param {string} value The new value for the display name
   */
  withDisplayName(value) {
    this._displayName = value;
    return this;
  }

  /**
   * Set the email address of this user
   * @param {string} value The new value for the email address
   */
  withEmailAddress(value) {
    this._emailAddress = value;
    return this;
  }

  /**
   * Set the password of this user
   * @param {string} value The new value for the password
   */
  withPassword(value) {
    this._password = value;
    return this;
  }

  async sql() {
    return "INSERT INTO users(user_id, version, created, updated, username, display_name, email_address, avatar_url, password) VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)";
  }

  async binds() {
    return [
      this._userId,
      this._version,
      this._created,
      this._updated,
      this._username,
      this._displayName,
      this._emailAddress,
      null,
      await hashPassword(this._password),
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
