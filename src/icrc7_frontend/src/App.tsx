import * as Functions from '../utils/functions';

function App() {

  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      <br />
      <form action="#" onSubmit={Functions.subscribeGroup}>
        <label htmlFor="name">Enter your name: &nbsp;</label>
        <input id="leaderName" alt="Name" type="text" placeholder="Enter your name" />
        <input
          id="memberName"
          alt="NameMem"
          type="text"
          placeholder="Name of a member"
        />
        <input
          id="memberId"
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
      <form action="#" onSubmit={Functions.assignEventToGroup}>
        <label htmlFor="eventId">Enter event_id: &nbsp;</label>
        <input id="eventId" alt="Name" type="text" />
        <input id="eventMemberName" type="text" placeholder="Name of a member" />
        <input id="eventMemberId" type="text" placeholder="II of a member" />
        <button type="submit">Assign event to members</button>
      </form>
      <form action="#" onSubmit={Functions.createEvent}>
        <label htmlFor="eventName">Enter event name: &nbsp;</label>
        <input id="eventName" alt="Name" type="text" />
        <label htmlFor="eventD">Enter description: &nbsp;</label>
        <input id="eventD" alt="Name" type="text" />
        <input type="file" id="imageForEvent" />
        <button type="submit">Create event</button>
      </form>
      <section>
        <button id="loginBtn" onClick={Functions.login}>
          Login with Internet Identity
        </button>
      </section>
      <section>
        <button id="showGroups" onClick={Functions.logGroups}>
          Call groups
        </button>
      </section>
      <section>
        <button id="showGroups" onClick={Functions.logEvents}>
          E groups
        </button>
      </section>
      <section>
        <button id="showGroups" onClick={Functions.logAllCollections}>
          C groups
        </button>
      </section>
      <section>
        <button id="showGroups" onClick={Functions.logUserCollections}>
          C2 groups
        </button>
      </section>
      <section>
        <button id="whoAmI" onClick={Functions.whoAmI}>
          WHO AM I
        </button>
      </section>
    </main>
  );
}

export default App;
