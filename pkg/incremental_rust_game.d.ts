/* tslint:disable */
/* eslint-disable */
/**
* @returns {number}
*/
export function collect_wood_global(): number;
/**
* @returns {number}
*/
export function collect_stone_global(): number;
/**
* @returns {boolean}
*/
export function craft_axe_global(): boolean;
/**
* @returns {boolean}
*/
export function craft_pickaxe_global(): boolean;
/**
* @returns {boolean}
*/
export function has_axe_global(): boolean;
/**
* @returns {boolean}
*/
export function has_pickaxe_global(): boolean;
/**
*/
export function passive_wood_collection(): void;
/**
*/
export function passive_stone_collection(): void;
/**
*/
export function run(): void;
/**
*/
export class Game {
  free(): void;
/**
*/
  constructor();
/**
*/
  collect_wood(): void;
/**
*/
  collect_stone(): void;
/**
* @returns {boolean}
*/
  craft_axe(): boolean;
/**
* @returns {boolean}
*/
  craft_pickaxe(): boolean;
/**
* @returns {number}
*/
  get_wood(): number;
/**
* @returns {number}
*/
  get_stone(): number;
/**
* @returns {boolean}
*/
  has_axe(): boolean;
/**
* @returns {boolean}
*/
  has_pickaxe(): boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_game_free: (a: number) => void;
  readonly game_new: () => number;
  readonly game_collect_wood: (a: number) => void;
  readonly game_collect_stone: (a: number) => void;
  readonly game_craft_axe: (a: number) => number;
  readonly game_craft_pickaxe: (a: number) => number;
  readonly game_get_wood: (a: number) => number;
  readonly game_get_stone: (a: number) => number;
  readonly game_has_axe: (a: number) => number;
  readonly game_has_pickaxe: (a: number) => number;
  readonly collect_wood_global: () => number;
  readonly collect_stone_global: () => number;
  readonly craft_axe_global: () => number;
  readonly craft_pickaxe_global: () => number;
  readonly has_axe_global: () => number;
  readonly has_pickaxe_global: () => number;
  readonly passive_wood_collection: () => void;
  readonly passive_stone_collection: () => void;
  readonly run: () => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb852b00e79f0952e: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
