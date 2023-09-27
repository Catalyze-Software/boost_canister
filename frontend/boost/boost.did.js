export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text });
  const Boosted = IDL.Record({
    'updated_at' : IDL.Nat64,
    'type_' : IDL.Text,
    'owner' : IDL.Principal,
    'seconds' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'blockheight' : IDL.Nat64,
    'identifier' : IDL.Principal,
  });
  return IDL.Service({
    'boost' : IDL.Func([IDL.Principal, IDL.Nat64], [Result], []),
    'get_boosted_events' : IDL.Func([], [IDL.Vec(Boosted)], ['query']),
    'get_boosted_groups' : IDL.Func([], [IDL.Vec(Boosted)], ['query']),
    'get_last_block_height' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_remaining_boost_time_in_seconds' : IDL.Func(
        [IDL.Principal],
        [IDL.Nat64],
        ['query'],
      ),
    'get_timer_ids' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'test_boost' : IDL.Func([IDL.Principal, IDL.Nat64], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
