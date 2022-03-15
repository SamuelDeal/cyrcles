Thoughts
========

Threats evaluations
-------------------

Need review

* The major issue is identity thief. This project needs a lot of reviews and auditing. For now, the PKI part should use a GPG key server as backup

* This project is a really open door for social engineering attacks, it will need to educate users... a lost cause ?

* SInce we provide p2p messaging, this project can be a channel for spam. We should use captcha.

* If to many members of a circle are corrupted, the ego is lost.

* The software distribution may be a vector of attack. Multiple implementations may be a good thing.

* Someone may acquire information, like emails, or social informations. We need to reduce as much as possible the amount of information someone can acquire scrapping the network

* global DoS, of course

* MITM may while adding a new friend to a circle ? In this case, if their is several bad actor in your circle, the ego can be stolen

* DoS on the DHT may saturate a lot of the peers memory

* DoS on captcha request may saturate the memory of a peer

* Sybil attacks on the network can poison the pk retrivale and make someone 
  * unable to authenticate
  * unable to be recognized as the legit message sender
  * unable to read messages sent to him/her

* Someone may find several ego are related to the same peer. We should provide a protection against this (but I don't know how yet)

* local software (or website page) may spam requests for auth. We could provide a quota, and banning rules to limit that


Main hurdles
------------

* Prior work:
  This idea seems not revolutionary, so I need to search for prior work on this. Is there something already implemented, if yes why is ths not in every computer already? If not, is there an inherent design flaw that make the project doomed form the start?

* Security:
  I'm a security noob, so there will be a lot of flaws in the design of this beast. It will need some security experts to review the design and maybe also to implement this properly. And it will need security audits, wich require money. If this project finally work, it will be a really sensitive  

* Popularity:
  For a p2p service to work properly. Also for the key sharing to be efficient, it will need suffisant friends/family members to be online to work. I don't know what the critical mass is but surely a lot more than I can achieve

* Availability:
  This projects need to run even when people don't actively needs it unlike bittorent in order to accept key regeneration. So users needs to be sufficiently motivated and numerous and online. 

* Man power:
  It is obviously a lot more that I can handle on my spare time. So it will need to find other people to work on this, and this is not really a sexy project. It will certainly be abandoned soon... but let's give it a try nonetheless.

* Money:
  This project can't and shouldn't be monetized. Maybe sponsorship is ok, but hard to get. It will be hard to pay for the secutiry audits it requires and full-time developers.




Misc design ideas
-----------------

* Different circle should have a different level of security. For your gaming circle, it may be ok that half of your circle validate who you are. This allow redundancy. Meanwhile for your administrative ego, you may be more paranoid and require all your family member to acknowledge your identity. It also allow your family to access your administrative ego in case of death.

* Keys have to be invalidated regularly, it should be automatic and without the need for human interaction for the current user or the circle members. 

* When your hard-drive crashed and you want to revoke all our egos and regenerate your keys, it's a rare case so it's ok if this process take few hours/days.

* When your hard-drive crashed, you will need to send requests to all the members of all your cicles. It's ok you have to remeber all your egos but it's not ok you have to remember all the users of each circle. So circle members have to be public (or is there a better way to handle this?).
  
* We have to avoid circle members to be flooded by attackers requests so we need:
  * a preventive captcha to solve
  * a preventive question, common to the entire circle
  * a quota mechanism

* the DHT should use as much as possible hash as key to limit scraping





Some implementation ideas
-------------------------

All of this need more thoughts, just temporary ideas

* A circle has a unique owner. If Bob and Alice are siblings, Bob can be part of Alice "family" circle, and Alice in the Bob's "family" circle, but those 2 circles are not the same and may have different users inside.

* A circle is anonym, and should be identified by an UUID. The user frendly name is only known by the circle owner.

* A user can have several client software. We must pay attention on how to sync them, this may be crucial to inhibit forks.

* An ego have only one circle.

* We can have two kind of circles, public and privates ones. Private circles must be the default.

* In private circles, only members of the circle have the data of the circle (members, owner, etc...). In that case, if Alice loose her hard-drive, she can call (by phone ?) one of her circle member to ask for revokation and regeneration of a key.

* Public circles must only be use when the user can't talk to any other circle member using a non Cyrcles way (eg. a guild in a game using Cyrcles authentication).

* Certificate chain 

* We need a fixed, shared master symetric key to enable backup-like features ?

* A request for the agent should show a "reason" for authentication. The agent may have a quota, and manual ban mechanism

* Is there a way for the agent to ensure the binary which requested auth ? May be usefull to ban it 

* We need user backup incentives in case a full circle is corrupted (virus, too small circle, etc...)

* Use WASM for captcha generation for quick iterations and deployement

* port 314 is not used anymore (but require a system-level daemon)












Keys life-cycle
---------------






#### Generation of an ego:
  ```mermaid
  sequenceDiagram
    autonumber
    actor Alice
    participant PKI

    Alice->>Alice: create a root key pair
    Alice->>Alice: create an ego key pair
    Alice->>Alice: sign the ego pk with the circle sk
    Alice->>PKI: publish the root pk
    Alice->>PKI: publish the ego pk
  ```


#### Alice add her first friend in a circle:

 ```mermaid
  sequenceDiagram
    autonumber
    actor Bob
    actor Alice
    participant PKI

    Bob->>Alice: give ego pk
    Alice->>+PKI: ask Bob ego pk data
    PKI->>-Alice: Bob's ego pk data
    Alice->>Alice: check the Bob's ego pk validity
    Alice->>Alice: save the Bob's ego pk and root pk
  ```


#### Alice add another friend in a circle:

 ```mermaid
  sequenceDiagram
    autonumber
    actor Bob
    actor Alice
    actor Old Friend(s)
    participant PKI

    Bob->>Alice: give ego pk
    Alice->>+PKI: ask Bob ego pk data
    PKI->>-Alice: Bob's ego pk data
    Alice->>Alice: check the Bob's ego pk validity
    Alice->>Alice: save the Bob's ego pk and root certificate
    Alice->>Alice: generate a new circle key pair
    Alice->>Alice: sign the ego pk with the new circle sk
    alt first secret sharing
      Alice->>Alice: sign the new circle pk with the root sk
    else circle secret already shared
      Alice->>+Old Friend(s): ask sign the new circle pk
      Old Friend(s)->>Old Friend(s): sign the new circle pk using MPC
      Old Friend(s)->>-Alice: the signed new circle pk
    end
    Alice->>Alice: split the new circle sk using SSS
    Alice->>Bob: a fragment of the new circle pk
    Bob->>Bob: Save fragment
    Alice->>Old Friend(s): a fragment of the new circle pk
    Old Friend(s)->>Old Friend(s): Save fragment
    Old Friend(s)->>Old Friend(s): remove the fragments of the old circle sk
    Alice->>PKI: publish the new circle pk
    Alice->>PKI: publish the newly signed ego pk 
    Alice->>Alice: remove the new circle sk
    alt first secret sharing
      Alice->>Alice: remove root sk
    end
  ```


#### Ego key rotation:

```mermaid
  sequenceDiagram
    autonumber
    actor Alice
    actor Friends
    participant PKI


    Alice->>Alice: generate a new ego key pair
    alt not enought friends
      Alice->>Alice: sign the new ego key with the circle sk
    else enought friends
      Alice->>+Friends: ask sign the new ego pk
      Friends->>Friends: sign the new ego pk using MPC
      Friends->>-Alice: the signed new ego pk
    end
    Alice->>PKI: publish the new ego pk
  ```

#### Circle key rotation:

```mermaid
  sequenceDiagram
    autonumber
    actor Friends
    actor Alice
    participant PKI


    Alice->>Alice: generate a new ego circle key pair
    Alice->>Alice: sign the ego pk with the new circle sk
    Note right of Alice: TODO: Check MPC for key generation
    alt not enought friends
      Alice->>Alice: sign the new ego key with the circle sk
    else enought friends
      Alice->>+Friends: ask sign the new circle pk
      Friends->>Friends: sign the new circle pk using MPC
      Friends->>-Alice: the signed new circle pk
    end
    Alice->>Alice: split the new circle sk using SSS
    Alice->>Friends: a fragment of the new circle pk
    Friends->>Friends: Save fragment
    Friends->>Friends: remove the fragments of the old circle sk
    Alice->>PKI: publish the new ego pk
    Alice->>PKI: publish the newly signed circle pk
  ```

Naive workflow
--------------

TL;DR: Captcha everywhere....


#### First communication with a peer

  ```mermaid
  sequenceDiagram
    autonumber
    actor Alice
    actor Bob
    participant PKI

    par Ahead of contact
      Alice->>+PKI: What's Bob pub key ?
      PKI->-Alice: pkB
    and Untrusted TLS communication
      Note right of Bob: No human interaction
      Alice->>+Bob: I want to talk to Bob
      Bob->>+PKI: Ask pkA
      PKI->>-Bob: pkA
      Bob->>Bob: Generate captcha
      Bob->>-Alice: Captcha to solve
      Alice->>Alice: Solve Captcha
      Alice->>+Bob: Captcha response
      Note right of Bob: Wait some random amount of time
      Bob->>-Alice: Ok
    and Still untrusted
      Note right of Bob: Now received UI interactions
      Alice->>Bob: Hello
      Note over Alice,Bob: Some untrusted conversation
      Bob->>Alice: Start authn process
    and Alice authn process
      Bob->>Alice: Question about a prior shared human secret (PSHS)<br/>eg. Who brings us together? 
      Note right of Bob: Start timer
      Alice->>Alice: Generate captcha
      Alice->>Alice: Derive a key (DK) from<br/> the PSHS + catpcha
      Alice->>Bob: DK(pkA, challenge) and captcha
      Note right of Bob: Stop timer
      Bob->>Bob: Check secret if time response is good enought
      Bob->>Bob: Solve captcha
      Bob->>Bob: Check response (Bob can try several spellings)
      Bob->>Bob: Increase pkA authenticity trust
      Bob->>PKI: Confirm pkA authenticity
    and Bob authn process
      Note over Alice,Bob: Repeat the operation symmetrically<br/>with another personal question
      Alice->>PKI: Confirm pkB authenticity
    and Trusted TLS communication
      Note over Alice,Bob: trusted conversation
    end
  ```



#### Login in a third party app
  ```mermaid
  sequenceDiagram
    autonumber
    actor Alice
    participant Bob's website
    participant DHT

    Alice->>+Bob's website: Hello I'm @alice, here's my pub key
    Bob's website->>+DHT: Ask for current pub key validity for alice
    Note right of Bob's website: The pub key can be cached
    DHT->>-Bob's website: It's still the good one
    Bob's website->>+Alice: Challenge
    Alice->>Alice: Encrypt and sign challenge 
    Alice->>-Bob's website: Challenge response
    Bob's website->>-Alice: Ok
  ```



#### Bootstrap:
  Use a hard-coded first user ?



Technical issues
----------------

* Prevent MITM without a CA (can we use lets-encrypt ?)
* Can we avoid using a PoW and PoS ?
* How can we avoid flooding of requests and spam ?
* Uncentralised P2P kickstart ?
* Keys TTL ?
* Bypass firewalls and NAT ?
* Decentralized secured software update ? (I will not focuse on this for now)
* Which robust library to use ?
* Which approved protocols to implement ?
* Prevent DDoS ? Poisoning ?

Misc
----

Securing user agent connections: 
* DBUS -> (assuming you are using dbus-daemon to route the > messages) -> GetConnectionUnixProcessID && GetConnectionUnixUser 
