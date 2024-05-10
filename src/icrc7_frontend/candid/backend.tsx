// @ts-ignore
export const idlFactory = ({ IDL }) => {
  const Event = IDL.Record({
    'id' : IDL.Text,
    'title' : IDL.Text,
    'description' : IDL.Text,
  });
  const Member = IDL.Record({
    'name' : IDL.Text,
    'internet_identity' : IDL.Principal,
  });
  const Group = IDL.Record({
    'group_members' : IDL.Vec(Member),
    'group_name' : IDL.Text,
  });
  const OperationCode = IDL.Variant({
    'DuplicateEntry' : IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text }),
    'MintingError' : IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text }),
    'RemoveOk' : IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text }),
    'MintOk' : IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text }),
    'InsertError' : IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text }),
    'RetrieveError' : IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text }),
  });
  return IDL.Service({
    'assign_event_to_group' : IDL.Func([IDL.Text, IDL.Text], [], []),
    'create_event' : IDL.Func([IDL.Text, IDL.Text], [], []),
    'get_all_events' : IDL.Func([], [IDL.Vec(Event)], ['query']),
    'get_all_groups' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, Group))],
        ['query'],
      ),
    'get_group_members' : IDL.Func([IDL.Text], [IDL.Vec(Member)], ['query']),
    'get_user_collections' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Principal))],
        [],
      ),
    'remove_all_groups' : IDL.Func([], [], []),
    'remove_event' : IDL.Func([IDL.Text], [], []),
    'remove_group' : IDL.Func([IDL.Text], [OperationCode], []),
    'subscribe_group' : IDL.Func([Group], [OperationCode], []),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
  });
};
// @ts-ignore
export const init = ({ IDL }) => { return []; };
