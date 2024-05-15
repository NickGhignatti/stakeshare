import { useState } from "react";
import { Principal } from "@dfinity/principal";
import { AuthClient } from "@dfinity/auth-client";
import { Actor, HttpAgent, Identity } from "@dfinity/agent";
import { _SERVICE as _FCTRY_SERVICE } from "../../declarations/factory/factory.did";
import { _SERVICE as _BCKND_SERVICE } from "../../declarations/icrc7_backend/icrc7_backend.did";
import { idlFactory as FactoryIdlFactory } from "../candid/factory";
import { idlFactory as BackendIdlFactory } from "../candid/backend";
import { isSafari } from "react-device-detect";

const authClient = await AuthClient.create();

function App() {
  const [greeting, setGreeting] = useState("");
  const webapp_id = process.env.CANISTER_ID_FACTORY;
  const backend_webapp_id = process.env.CANISTER_ID_ICRC7_BACKEND;
  const local_iiUrl = isSafari
    ? `http://127.0.0.1:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`
    : `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`;

  let iiUrl = local_iiUrl;
  if (process.env.DFX_NETWORK === "local") {
    iiUrl = local_iiUrl;
  } else if (process.env.DFX_NETWORK === "ic") {
    iiUrl = `https://${process.env.CANISTER_ID_INTERNET_IDENTITY}.ic0.app`;
  } else {
    iiUrl = local_iiUrl;
  }

  // At this point we're authenticated, and we can get the identity from the auth client:
  const identity = authClient.getIdentity();
  // Using the identity obtained from the auth client, we can create an agent to interact with the IC.
  const agent = new HttpAgent({ identity: identity as unknown as Identity });
  // Using the interface description of our webapp, we create an actor that we use to call the service methods.
  const webapp: _FCTRY_SERVICE = Actor.createActor(FactoryIdlFactory, {
    agent,
    canisterId: webapp_id!,
  });
  const backend_webapp: _BCKND_SERVICE = Actor.createActor(BackendIdlFactory, {
    agent,
    canisterId: backend_webapp_id!,
  });

  async function login() {
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

  async function createGroup() {
    authClient.isAuthenticated().then(async (isAuth) => {
      await agent.fetchRootKey();
      if (isAuth) {
        const groupId = (document.getElementById("groupId") as HTMLInputElement)
          .value;
        const eventId = (document.getElementById("eventId") as HTMLInputElement)
          .value;
        backend_webapp.assign_event_to_group(eventId, groupId);
      }
    });
  }

  async function printGroups() {
    await agent.fetchRootKey();
    backend_webapp
      .get_all_groups()
      .then((groups) => console.log(groups))
      .catch((e) => console.log(e));
  }

  async function removeGroup() {
    const groupId = (document.getElementById("groupName") as HTMLInputElement)
      .value;
    console.log(groupId);
    await agent.fetchRootKey();
    backend_webapp.remove_group(groupId).then(() => console.log("DONE"));
  }

  async function handleSubmit() {
    authClient.isAuthenticated().then(async (isAuth) => {
      if (isAuth) {
        await agent.fetchRootKey();
        const myName = (document.getElementById("name") as HTMLInputElement)
          .value;
        const groupName = (document.getElementById("groupName") as HTMLInputElement)
          .value;
        const memberName = (
          document.getElementById("nameMem") as HTMLInputElement
        ).value;
        const memberId = (document.getElementById("idMem") as HTMLInputElement)
          .value;
        backend_webapp.subscribe_group(
          [],
          myName,
          groupName
        );
      }
    });
  }

  async function createEvent() {
    await agent.fetchRootKey();
    const eventName = (document.getElementById("eventName") as HTMLInputElement)
      .value;
    const eventDescription = (document.getElementById("eventD") as HTMLInputElement)
      .value;
    // @ts-ignore
    const file = (document.getElementById("imageForEvent") as HTMLInputElement)
      .files[0];

    fetch(URL.createObjectURL(file)).then(response => response.blob()).then(blobData => {
      const reader = new FileReader();
      reader.onload = async (event) => {
        const arrayBuffer = event.target?.result as ArrayBuffer;
        await backend_webapp.create_event(eventName, eventDescription, { 'Blob' : new Uint8Array(arrayBuffer) });
      }
      reader.readAsArrayBuffer(blobData);
    })

  }

  function showEvents() {
    backend_webapp.get_all_events().then((events) => console.log(events));
  }

  async function callWhoAmI() {
    await agent.fetchRootKey();
    const canisterId = await webapp.show_collections();
  }

  async function assignEvent() {
    const groupId = (document.getElementById("groupId") as HTMLInputElement)
      .value;
    const eventId = (document.getElementById("eventId") as HTMLInputElement)
      .value;
    await agent.fetchRootKey();
    backend_webapp
      .assign_event_to_group(eventId, groupId)
      .then((s) => console.log(s));
  }

  async function getMyCollection() {
    await agent.fetchRootKey();
    backend_webapp.get_user_collection().then(coll => console.log(coll));
  }

  async function whoAmI() {
    await agent.fetchRootKey();
    backend_webapp.whoami().then(s => console.log(s.toString()));
  }

  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      <br />
      <form action="#" onSubmit={handleSubmit}>
        <label htmlFor="name">Enter your name: &nbsp;</label>
        <input id="name" alt="Name" type="text" placeholder="Enter your name" />
        <input
          id="nameMem"
          alt="NameMem"
          type="text"
          placeholder="Name of a member"
        />
        <input
          id="idMem"
          alt="IdMem"
          type="text"
          placeholder="Internet Identity of a member"
        />
        <input
          id="groupName"
          alt="GroupName"
          type="text"
          placeholder="Group name"
        />
        <button type="submit">Subscribe Group!</button>
      </form>
      <form action="#" onSubmit={createGroup}>
        <label htmlFor="eventId">Enter event_id: &nbsp;</label>
        <input id="eventId" alt="Name" type="text" />
        <label htmlFor="groupId">Enter group_id: &nbsp;</label>
        <input id="groupId" alt="Name" type="text" />
        <input type="file" id="imageN" />
        <button type="submit">Boh</button>
      </form>
      <form action="#" onSubmit={createEvent}>
        <label htmlFor="eventName">Enter event name: &nbsp;</label>
        <input id="eventName" alt="Name" type="text" />
        <label htmlFor="eventD">Enter description: &nbsp;</label>
        <input id="eventD" alt="Name" type="text" />
        <input type="file" id="imageForEvent" />
        <button type="submit">Create event</button>
      </form>
      <section>
        <button id="loginBtn" onClick={login}>
          Login with Internet Identity
        </button>
      </section>
      <section>
        <button id="testBtn" onClick={callWhoAmI}>
          Call canisterId whoami
        </button>
      </section>
      <section>
        <button id="showGroups" onClick={printGroups}>
          Call groups
        </button>
      </section>
      <section>
        <button id="showGroups" onClick={removeGroup}>
          R groups
        </button>
      </section>
      <section>
        <button id="showGroups" onClick={showEvents}>
          E groups
        </button>
      </section>
      <section>
        <button id="showGroups" onClick={getMyCollection}>
          C groups
        </button>
      </section>
      <section>
        <button id="whoAmI" onClick={whoAmI}>
          WHO AM I
        </button>
      </section>
      <section id="greeting">{greeting}</section>
    </main>
  );
}

export default App;
