import { Actor, HttpAgent, Identity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { isSafari } from "react-device-detect";
import { _SERVICE as _BCKND_SERVICE } from "../../declarations/icrc7_backend/icrc7_backend.did";
// @ts-ignore
import { idlFactory as BackendIdlFactory } from "../candid/backend";

const authClient = await AuthClient.create();

export { authClient };

const backend_id = process.env.CANISTER_ID_ICRC7_BACKEND;

export { backend_id };

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

export { iiUrl };

// At this point we're authenticated, and we can get the identity from the auth client:
const identity = authClient.getIdentity();
// Using the identity obtained from the auth client, we can create an agent to interact with the IC.
const agent = new HttpAgent({ identity: identity as unknown as Identity });

export { agent };

// Using the interface description of our webapp, we create an actor that we use to call the service methods.
const backend_webapp: _BCKND_SERVICE = Actor.createActor(BackendIdlFactory, {
  agent,
  canisterId: backend_id!,
});

export { backend_webapp };
