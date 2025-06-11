/* tslint:disable */
/* eslint-disable */
/**
 * JS facing game API
 */
export class Game {
  free(): void;
  /**
   * Create a new game instance
   */
  constructor();
  /**
   * Advance the game state based on the current timestamp (seconds)
   */
  static tick(now: number): void;
  /**
   * Attempt to build a building by name
   */
  static build(name: string): boolean;
  /**
   * Get a resource amount by name
   */
  static get_resource(name: string): number;
  /**
   * Get the cost of constructing the next level of a building as a JSON string
   */
  static building_cost(name: string): string;
  /**
   * Save game to a base64 string
   */
  static save(): string;
  /**
   * Load game from a base64 string
   */
  static load(data: string): void;
  /**
   * Change tick rate in seconds
   */
  static set_tick_rate(rate: number): void;
  /**
   * Retrieve the next log message from the game if available
   */
  static pop_log(): string | undefined;
  /**
   * Get the list of unlocked achievements as a JSON string
   */
  static achievements(): string;
  /**
   * Attempt to research a technology using science
   */
  static research(name: string): boolean;
  /**
   * Perform a prestige reset
   */
  static prestige(): void;
  /**
   * Current prestige points
   */
  static prestige_points(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_game_free: (a: number, b: number) => void;
  readonly game_new: () => number;
  readonly game_tick: (a: number) => void;
  readonly game_build: (a: number, b: number) => number;
  readonly game_get_resource: (a: number, b: number) => number;
  readonly game_building_cost: (a: number, b: number) => [number, number];
  readonly game_save: () => [number, number];
  readonly game_load: (a: number, b: number) => void;
  readonly game_set_tick_rate: (a: number) => void;
  readonly game_pop_log: () => [number, number];
  readonly game_achievements: () => [number, number];
  readonly game_research: (a: number, b: number) => number;
  readonly game_prestige: () => void;
  readonly game_prestige_points: () => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
