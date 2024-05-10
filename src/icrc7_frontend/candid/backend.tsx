// @ts-ignore
export const idlFactory = ({ IDL }) => {
  const Member = IDL.Record({
    'name' : IDL.Text,
    'internet_identity' : IDL.Principal,
  });
  const Group = IDL.Record({
    'group_members' : IDL.Vec(Member),
    'group_name' : IDL.Text,
  });
  const Event = IDL.Record({
    'id' : IDL.Text,
    'title' : IDL.Text,
    'description' : IDL.Text,
  });
  return IDL.Service({
    'assign_event_to_group' : IDL.Func([IDL.Text, IDL.Text], [], []),
    'call_canister_whoami' : IDL.Func([IDL.Principal], [IDL.Text], []),
    'create_event' : IDL.Func([IDL.Text, IDL.Text], [], []),
    'get_group_members' : IDL.Func([IDL.Text], [IDL.Vec(Member)], ['query']),
    'get_user_collections' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Principal))],
        [],
      ),
    'print_groups' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, Group))],
        ['query'],
      ),
    'remove_all_groups' : IDL.Func([], [], []),
    'remove_event' : IDL.Func([IDL.Text], [], []),
    'remove_group' : IDL.Func([IDL.Text], [], []),
    'show_events' : IDL.Func([], [IDL.Vec(Event)], ['query']),
    'subscribe_group' : IDL.Func([Group], [], []),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
  });
};
// @ts-ignore
export const init = ({ IDL }) => { return []; };
