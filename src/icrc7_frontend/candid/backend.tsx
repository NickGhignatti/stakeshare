// @ts-ignore
export const idlFactory = ({ IDL }) => {
  const OperationCode = IDL.Variant({
    'DuplicateEntry' : IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text }),
    'MintingError' : IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text }),
    'RemoveOk' : IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text }),
    'MintOk' : IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text }),
    'InsertError' : IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text }),
    'RetrieveError' : IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text }),
  });
  const MetadataValue = IDL.Variant({
    'Int' : IDL.Int,
    'Nat' : IDL.Nat,
    'Blob' : IDL.Vec(IDL.Nat8),
    'Text' : IDL.Text,
  });
  const Event = IDL.Record({
    'id' : IDL.Text,
    'title' : IDL.Text,
    'metadata' : MetadataValue,
    'description' : IDL.Text,
  });
  const Member = IDL.Record({
    'name' : IDL.Text,
    'internet_identity' : IDL.Text,
  });
  const Group = IDL.Record({
    'group_members' : IDL.Vec(Member),
    'group_name' : IDL.Text,
  });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const TransferArg = IDL.Record({
    'to' : Account,
    'token_id' : IDL.Nat,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'from_subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
  });
  const TransferError = IDL.Variant({
    'GenericError' : IDL.Record({
      'message' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'Duplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat }),
    'NonExistingTokenId' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'CreatedInFuture' : IDL.Record({ 'ledger_time' : IDL.Nat64 }),
    'InvalidRecipient' : IDL.Null,
    'GenericBatchError' : IDL.Record({
      'message' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'TooOld' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : TransferError });
  return IDL.Service({
    'assign_event_to_group' : IDL.Func(
        [IDL.Text, IDL.Text],
        [OperationCode],
        [],
      ),
    'create_event' : IDL.Func([IDL.Text, IDL.Text, MetadataValue], [], []),
    'get_all_events' : IDL.Func([], [IDL.Vec(Event)], ['query']),
    'get_all_groups' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, Group))],
        ['query'],
      ),
    'get_all_nft_collections' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Principal))],
        [],
      ),
    'get_group_members' : IDL.Func([IDL.Text], [IDL.Vec(Member)], ['query']),
    'get_icrc7_description' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Text)],
        ['composite_query'],
      ),
    'get_icrc7_logo' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Text)],
        ['composite_query'],
      ),
    'get_icrc7_max_memo_size' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Nat)],
        ['composite_query'],
      ),
    'get_icrc7_max_query_batch_size' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Nat)],
        ['composite_query'],
      ),
    'get_icrc7_max_take_value' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Nat)],
        ['composite_query'],
      ),
    'get_icrc7_max_update_batch_size' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Nat)],
        ['composite_query'],
      ),
    'get_icrc7_name' : IDL.Func(
        [IDL.Principal],
        [IDL.Text],
        ['composite_query'],
      ),
    'get_icrc7_supply_cap' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Nat)],
        ['composite_query'],
      ),
    'get_icrc7_symbol' : IDL.Func(
        [IDL.Principal],
        [IDL.Text],
        ['composite_query'],
      ),
    'get_icrc7_token_metadata' : IDL.Func(
        [IDL.Vec(IDL.Nat), IDL.Principal],
        [IDL.Vec(IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, MetadataValue))))],
        ['composite_query'],
      ),
    'get_icrc7_total_supply' : IDL.Func(
        [IDL.Principal],
        [IDL.Nat],
        ['composite_query'],
      ),
    'get_user_collection' : IDL.Func([], [IDL.Vec(IDL.Nat)], []),
    'get_user_collections' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Principal))],
        [],
      ),
    'icrc7_atomic_batch_transfers' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Bool)],
        ['composite_query'],
      ),
    'icrc7_balance_of' : IDL.Func(
        [IDL.Vec(Account), IDL.Principal],
        [IDL.Vec(IDL.Nat)],
        ['composite_query'],
      ),
    'icrc7_owner_of' : IDL.Func(
        [IDL.Vec(IDL.Nat), IDL.Principal],
        [IDL.Vec(IDL.Opt(Account))],
        ['composite_query'],
      ),
    'icrc7_tokens' : IDL.Func(
        [IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat), IDL.Principal],
        [IDL.Vec(IDL.Nat)],
        ['composite_query'],
      ),
    'icrc7_tokens_of' : IDL.Func(
        [Account, IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat), IDL.Principal],
        [IDL.Vec(IDL.Nat)],
        ['composite_query'],
      ),
    'icrc7_transfer' : IDL.Func(
        [IDL.Principal, IDL.Vec(TransferArg), IDL.Principal],
        [IDL.Vec(IDL.Opt(Result))],
        [],
      ),
    'remove_all_groups' : IDL.Func([], [], []),
    'remove_event' : IDL.Func([IDL.Text], [], []),
    'remove_group' : IDL.Func([IDL.Text], [OperationCode], []),
    'subscribe_group' : IDL.Func(
        [IDL.Vec(Member), IDL.Text, IDL.Text],
        [OperationCode],
        [],
      ),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
  });
};
// @ts-ignore
export const init = ({ IDL }) => { return []; };
