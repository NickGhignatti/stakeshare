import { useState } from 'react';
import { AuthClient } from '@dfinity/auth-client';
import { Actor, HttpAgent, Identity } from '@dfinity/agent';
import { _SERVICE as _FCTRY_SERVICE, Arg } from '../../declarations/factory/factory.did';
import { _SERVICE as _BCKND_SERVICE } from '../../declarations/icrc7_backend/icrc7_backend.did';
import { idlFactory as FactoryIdlFactory } from '../candid/factory';
import { idlFactory as BackendIdlFactory } from '../candid/backend';

const authClient = await AuthClient.create();

function App() {
  const [greeting, setGreeting] = useState('');
  const webapp_id = process.env.CANISTER_ID_FACTORY;
  const backend_webapp_id = process.env.CANISTER_ID_ICRC7_BACKEND;
  const nft_webapp_id = process.env.CANISTER_ID_NFT_BACKEND;
  const local_iiUrl1 = `http://127.0.0.1:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`;
  const local_iiUrl2 = `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`;

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
  })

  async function login() {
    authClient.isAuthenticated().then(async isAuth => {
        await new Promise<void>((resolve, reject) => {
          authClient.login({
            identityProvider: local_iiUrl2,
            onSuccess: resolve,
            onError: reject,
          });
        });
    }) 
    console.log(identity.getPrincipal().toString());
  }

  async function handleSubmit() {
    console.log(identity.getPrincipal().toString());
    await agent.fetchRootKey();
    authClient.isAuthenticated().then(isAuth => {
      if (isAuth) {
        // webapp.show_collections().then(s => console.log(s));
        webapp.mint_collection_canister({
          icrc7_supply_cap : [],
          icrc7_description : ["ProvaF"],
          tx_window : [],
          icrc7_max_query_batch_size : [],
          permitted_drift : [],
          icrc7_max_take_value : [],
          icrc7_max_memo_size : [],
          icrc7_symbol : "MySymbol",
          icrc7_max_update_batch_size : [],
          icrc7_atomic_batch_transfers : [],
          icrc7_default_take_value : [],
          icrc7_logo : [],
          icrc7_name : "ProvaF",
        }).then(s => console.log(s)).catch(e => console.log(e));
      }
    })
  }

  async function callWhoAmI() {
    await agent.fetchRootKey();
    const canisterId = await webapp.show_collections();
    backend_webapp.call_canister_whoami(canisterId[0][0]).then(s => console.log(s));
  }

  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      <br />
      <form action="#" onSubmit={handleSubmit}>
        <label htmlFor="name">Enter your name: &nbsp;</label>
        <input id="name" alt="Name" type="text" />
        <button type="submit">Click Me!</button>
      </form>
      <section>
        <button id="loginBtn" onClick={login}>Login with Internet Identity</button>
      </section>
      <section>
        <button id="testBtn" onClick={callWhoAmI}>Call canisterId whoami</button>
      </section>
      <section id="greeting">{greeting}</section>
    </main>
  );
}

export default App;
