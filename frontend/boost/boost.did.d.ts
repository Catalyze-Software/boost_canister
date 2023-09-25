import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Boosted {
  'type_' : string,
  'owner' : Principal,
  'days' : bigint,
  'created_at' : bigint,
  'blockheight' : bigint,
  'identifier' : Principal,
}
export interface _SERVICE {
  'boost' : ActorMethod<[Principal, bigint], [] | [bigint]>,
  'get_boosted_by_identifier' : ActorMethod<[Principal], [] | [Boosted]>,
}
