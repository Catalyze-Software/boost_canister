import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Boosted {
  'updated_at' : bigint,
  'type_' : string,
  'owner' : Principal,
  'seconds' : bigint,
  'created_at' : bigint,
  'blockheight' : bigint,
  'identifier' : Principal,
}
export type Result = { 'Ok' : bigint } |
  { 'Err' : string };
export interface _SERVICE {
  'boost' : ActorMethod<[Principal, bigint], Result>,
  'get_boosted_events' : ActorMethod<[], Array<Boosted>>,
  'get_boosted_groups' : ActorMethod<[], Array<Boosted>>,
  'get_e8s_per_day' : ActorMethod<[], bigint>,
  'get_remaining_boost_time_in_seconds' : ActorMethod<[Principal], bigint>,
  'read_boosted_entries' : ActorMethod<[], Array<[string, Boosted]>>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
