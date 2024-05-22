// @ts-ignore
import { agent, authClient, backend_webapp, iiUrl } from "./connections";

export async function login() {
  authClient.isAuthenticated().then(async (isAuth) => {
    await new Promise<void>((resolve, reject) => {
      authClient.login({
        identityProvider: iiUrl,
        onSuccess: resolve,
        onError: reject,
      });
    });
  });
}

export async function subscribeGroup() {
  authClient.isAuthenticated().then(async (isAuth) => {
    if (isAuth) {
      await agent.fetchRootKey();
      const myName = (document.getElementById("leaderName") as HTMLInputElement)
        .value;
      const groupName = (
        document.getElementById("groupName") as HTMLInputElement
      ).value;
      const memeberName = (
        document.getElementById("memberName") as HTMLInputElement
      ).value;
      const memberId = (document.getElementById("memberId") as HTMLInputElement)
        .value;

      backend_webapp.subscribe_group(
        [{ name: memeberName, internet_identity: memberId }],
        myName,
        groupName
      );
    }
  });
}

export async function logGroups() {
  await agent.fetchRootKey();
  backend_webapp
    .get_all_groups()
    .then((g) => console.log(g))
    .catch((e) => console.log(e));
}

export async function createEvent() {
  const eventName = (document.getElementById("eventName") as HTMLInputElement)
    .value;
  const eventDescription = (
    document.getElementById("eventD") as HTMLInputElement
  ).value;
  // @ts-ignore
  const file = (document.getElementById("imageForEvent") as HTMLInputElement)
    .files[0];

  fetch(URL.createObjectURL(file))
    .then((response) => response.blob())
    .then((blobData) => {
      const reader = new FileReader();
      reader.onload = async (event) => {
        const arrayBuffer = event.target?.result as ArrayBuffer;
        await agent.fetchRootKey();
        await backend_webapp.create_event(eventName, eventDescription, {
          Blob: new Uint8Array(arrayBuffer),
        });
      };
      reader.readAsArrayBuffer(blobData);
    });
}

export async function logEvents() {
  await agent.fetchRootKey();
  backend_webapp
    .get_all_events()
    .then((e) => console.log(e))
    .catch((e) => console.log(e));
}

export async function assignEventToGroup() {
  authClient.isAuthenticated().then(async (isAuth) => {
    if (isAuth) {
      const eventId = (document.getElementById("eventId") as HTMLInputElement)
        .value;
      const nameMember = (
        document.getElementById("eventMemberName") as HTMLInputElement
      ).value;
      const idMember = (
        document.getElementById("eventMemberId") as HTMLInputElement
      ).value;
      backend_webapp.assign_event_to_group(eventId, [
        { name: nameMember, internet_identity: idMember },
      ]);
    }
  });
}

export async function logAllCollections() {
  await agent.fetchRootKey();
  backend_webapp
    .get_all_icrc7_collections()
    .then((c) => console.log(c))
    .catch((e) => console.log(e));
}

export async function logUserCollections() {
  await agent.fetchRootKey();
  backend_webapp
    .get_user_icrc7_collections()
    .then((c) => console.log(c))
    .catch((e) => console.log(e));
}

export async function logUserTokenCollection() {
  await agent.fetchRootKey();
  backend_webapp
    .get_user_tokens_collection()
    .then((tc) => console.log(tc))
    .catch((e) => console.log(e));
}

export async function whoAmI() {
  await agent.fetchRootKey();
  backend_webapp
    .whoami()
    .then((r) => console.log(r.toString()))
    .catch((e) => console.log(e));
}