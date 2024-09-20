// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Instant } from "./Instant";
import type { Visibility } from "./Visibility";

export type SystemData = {
  id: string;
  name: string;
  instants: Array<Instant>;
  /**
   * Frequency in minutes
   */
  frequency: number;
  starts_at: string;
  visibility: Visibility;
};
