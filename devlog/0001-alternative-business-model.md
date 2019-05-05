# Alternative Business Model

Welcome to the `eia3de` devlog, sorry to instantly jump into a subject without
prior context, I'll write something along the lines of `0000-intro.md` at some
point.

---

Forewarning, I will admit to having polarizing opinions on the matter and I
don't expect any other project to be ran similarly, you can most definitely
attribute this to my young age and lack of experience in the games industry if
you wish to. I do consider those ideas to have enough potential that the cost
of experimentation is worth it.

This game will be as free as it can possibly be, and what I mean by this is
that you will be able to download and play forever without ever being charged.
Here is the caveat: you are not entitled free access to all of the game
services, I will go into detail as to what this means.

> What is a game service?

A game service is whatever `eia3de` (or other party) provides to your game.
This might be a little vague, let me give you examples in order to let you
figure out the intuition:

- not game services:
    - launching the game
    - playing locally
    - hosting a game server
    - initiating a connection to a game server
    - keeping or playing a replay file
    - building a level
    - inspecting the game's source code
- game services that should always be free:
    - owning a player identity
    - accessing the master servre list
    - listing a game server on the master server list
- game services may or may not be free:
    - playing or spectating on someone else's game server
    - queuing up to play a ranked game
    - organizing or participating in a tournament
    - viewing a match's player/team stats
    - publishing or downloading a level
- game services that will always cost up front:
    - renting game server hosting
    - renting replay file archival
    - participating in a tournament with a paid entry

Note that this list is not exhaustive and does not represent a roadmap.

> So, not categorically free to play, huh?

That's pretty much right, I can't really argue against the case where hosting
a game server is impossible and no other game server accepts your connection,
in which case, you are not entitled to the core multiplayer experience.
Thankfully, I'm quite certain this scenario will never be an issue.

Obviously, you're neither entitled to any service if you're broken the Terms of
Service.

**Yes, you will be able to contribute monetarily**, and doing so will ensure
you can keep using the "may or may not be free" category of game services.

Here is how I think I would implement this: "everything" would be public and
incredibly transparent, the game would display the sum of contributions from
players, the sum of operational costs and various stretch goals (funding
development and assets, tournaments and offline events, etc). The game would
also display per-user contributions and operational costs.

For the sake of communication, let's call "negative balance" the case where
costs are bigger than the contributions and "positive balance" the opposite.

Since all this data would be available and hopefully automated, it would be
possible to enforce this important detail: when the global balance is
negative, the "may or may not be free" category of game services will not be
available to players with a negative balance. Obviously, there would exist a
mechanism to prevent this restriction from being too oppressive, I don't know
how such a mechanism would look like yet.

Calculating operational costs isn't exactly a hard problem but I dont think I
can fledge out all the details until a lot more is implemented. Basically, as
I see it now, there are costs that are obvious and easily linked to players,
and some that are more vague or that wouldn't really make sense to link to
players. Easily linked would be ranked game servers, bandwidth for asset
downloads. More vague would be storage for player-made levels or storage and
bandwidth for player identity and the master server list, they could be linked
but it's not very sensical. What I think I would do is equally spread the costs
for the latter on the active player base on a per-week/per-month basis.

In the list, I mentioned more premium game services that would always cost up
front; these would only be accessible to players with a positive balance. Their
costs would be shared: In the example of replay file archival, both the
tournament organizers, team sponsors, players and fans could decide to keep
some replays archived, and each would pay equal share. In the example of game
server hosting, a group of players could decide to have a private hangout spot
with custom rules and a password, or you could decide to support a public
server that you enjoy frequenting.

I've come to design this business model recently, I find this solution
reasonable and honest, if not too blunt. It puts a shared responsibility on the
players, hopefully it gets them thinking like adults as well.

If reached, stretch goals would first fund development work, particularly art
and cosmetic assets. Trading cosmetics quickly comes to mind after mentioning
this. If realistic, I would want to offer the promise that contributions could
be used to acquire untradeable but possibly limited cosmetics. I would also
definitely offer a way to disable rendering cosmetics at a premium.

There are more interesting ideas that could fit in this model, I believe the
contribution system should not be limited to monetary donations, various work
will be required, such as level design, analysing reports of bad behavior and
cheating suspicions, administrating tournaments, etc. I would probably go as
far as considering content like frag movies as contributions as well. This
does seem like a hard problem to tackle however, as price would have to be put
on the contributions, which end up requiring more work to be done.

The last idea I'd like to write about is the possibility of selling some
*things* in limited supply and have people resell or bid over them, such as
special spots in the master server list, VIP tickets for tournament broadcasts,
billboards in level decors, etc.

That's basically how I view this problem and a possible at this point in time,
I'm quite keen on it, which is what pushed the start of the devlog in the first
place, I felt strong enough about it that I needed to write it down.
