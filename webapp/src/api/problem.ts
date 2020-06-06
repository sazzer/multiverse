/**
 * Representation of an RFC-7807 Problem response
 */
export class Problem {
  /** The problem type */
  readonly type: string;
  /** The human readable title of the problem */
  readonly title: string;
  /** The status code */
  readonly status: number;
  /** Any extra details of the problem */
  readonly details: { [key: string]: any };

  /**
   * Construct the problem
   * @param type The problem type
   * @param title The human readable title of the problem
   * @param status The status code
   * @param details Any extra details of the problem
   */
  constructor(
    type: string,
    title: string,
    status: number,
    details: { [key: string]: any }
  ) {
    this.type = type;
    this.title = title;
    this.status = status;
    this.details = details;
  }
}
