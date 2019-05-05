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

### What is a game service?

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
- game services that will always cost (and probably up-front):
    - renting game server hosting
    - renting replay file hosting
    - participating in a tournament with a paid entry

Note that this list is not exhaustive and does not represent a roadmap.

### So, not categorically free to play, huh?

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
players, the sum of operational costs and various stretch goals (salary for
myself and an artist, funds to organize offline events, etc). The game would
also display per-user contributions and operational costs.

Since all this data would be available and hopefully automated, it would be
possible to enforce this important detail: when the sum of contributions is
lower than the cost of operations, the "may or may not be free" category of
services will not be available to those who have contributed less than they
cost. Obviously, there would exist a mechanism to prevent this restriction to
be too oppressive, I don't know how such a mechanism would look like yet.


#### TODO (this article is not complete)

- calculating operational costs
- premium services
- explain why salaries
- explain why this solution
- bullshit-free/"for adults"
- prevention to GAAS problems
- explore ability to pay level designers
- player contributions gives access to cosmetics, cannot be promised at the
  moment because no artist on board
