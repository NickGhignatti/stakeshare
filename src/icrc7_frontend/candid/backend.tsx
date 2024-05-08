//@ts-ignore
export const idlFactory = ({ IDL }) => {
  const Member = IDL.Record({
    'name' : IDL.Text,
    'internet_identity' : IDL.Principal,
  });
  const Group = IDL.Record({
    'group_members' : IDL.Vec(Member),
    'group_name' : IDL.Text,
  });
  return IDL.Service({
    'call_canister_whoami' : IDL.Func([IDL.Principal], [IDL.Text], []),
    'get_group_members' : IDL.Func([IDL.Text], [IDL.Vec(Member)], ['query']),
    'print_groups' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, Group))],
        ['query'],
      ),
    'remove_group' : IDL.Func([IDL.Text], [], []),
    'subscribe_group' : IDL.Func([Group], [], []),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
  });
};
// @ts-ignore
export const init = ({ IDL }) => { return []; };
  