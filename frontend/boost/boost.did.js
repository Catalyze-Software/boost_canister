export const idlFactory = ({ IDL }) => {
  const Boosted = IDL.Record({
    'type_' : IDL.Text,
    'owner' : IDL.Principal,
    'days' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'blockheight' : IDL.Nat64,
    'identifier' : IDL.Principal,
  });
  return IDL.Service({
    'boost' : IDL.Func([IDL.Principal, IDL.Nat64], [IDL.Opt(IDL.Nat64)], []),
    'get_boosted_by_identifier' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(Boosted)],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
