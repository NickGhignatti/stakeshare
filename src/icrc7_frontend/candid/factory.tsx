// @ts-ignore
export const idlFactory = ({ IDL }) => {
  const Arg = IDL.Record({
    'icrc7_supply_cap' : IDL.Opt(IDL.Nat),
    'icrc7_description' : IDL.Opt(IDL.Text),
    'tx_window' : IDL.Opt(IDL.Nat64),
    'icrc7_max_query_batch_size' : IDL.Opt(IDL.Nat),
    'permitted_drift' : IDL.Opt(IDL.Nat64),
    'icrc7_max_take_value' : IDL.Opt(IDL.Nat),
    'icrc7_max_memo_size' : IDL.Opt(IDL.Nat),
    'icrc7_symbol' : IDL.Text,
    'icrc7_max_update_batch_size' : IDL.Opt(IDL.Nat),
    'icrc7_atomic_batch_transfers' : IDL.Opt(IDL.Bool),
    'icrc7_default_take_value' : IDL.Opt(IDL.Nat),
    'icrc7_logo' : IDL.Opt(IDL.Text),
    'icrc7_name' : IDL.Text,
  });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Principal, 'Err' : IDL.Text });
  return IDL.Service({
    'check_collection_ownership' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [IDL.Bool],
        ['query'],
      ),
    'get_user_collections' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(IDL.Principal)],
        ['query'],
      ),
    'mint_collection_canister' : IDL.Func([Arg, Account], [Result], []),
    'show_collections' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Principal))],
        ['query'],
      ),
    'update_minting_aythority' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [IDL.Bool],
        [],
      ),
    'whoami' : IDL.Func([IDL.Principal], [IDL.Text], ['query']),
  });
};
// @ts-ignore
export const init = ({ IDL }) => { return []; };
