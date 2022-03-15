Cyrcles
=======

This document is just a collection of early thoughts about a p2p authentication system

Current state of reflexion: [here](./thoughts.md)

Overview
--------

This project is a PoC of a software suite which provides multiple public-key based authentication mecanism for multiple identities.

It's may be a user-friendly front-end for GPG with p2p features to act as a decentralized PKI.

Each identity is called (in the code and documentation) an ego. 

Each ego may have a bunch of other users (family, friends...) we can rely on to acknowledge who you are via human interactions.
This project call this human list a circle.

Each users of a circle will have a part of a splitted key for the sake of revokation and regeneration of a your identity key.


Goal
----

The aim of this project is to provide a convenient way to authenticate oneself without the need to remember a lot of passwords and without a central authority unlike the mess of OAuth2 and the likes...

It should be fuck-I-forgot-my-pwd resistant, sticky-note resistant, fuck-my-hardrive-crashed resistant, fuck-I-didn't-backuped resistant and fuck-my-home-burnt resistant.

It should be designed for other systems to be built upon it, I dream about a p2p backup system and a credential system.

The final goal is:
  * generalize the encryption of emails 
  * replace passwords for websites, and replace the need for website/company to get your email for the sake of login
  * replace the need for central certification authority for https
  
Ambitious? nah!


No Goals
--------

Here a list of features not planed for this project. The following issues are real hurdles but they are out of scope of this project. 
I hope those would be tackled one day, and maybe they can be using this project as layer.

* NSA-resistant cryptography:
  Uhhh... no. I'm not good enought and you [may not need to look for it](https://xkcd.com/538/)

* Secret transfer: 
  Password/ssh keys transfer using emails or text messaging is a big issue. But not my problem right now...
  
* Temporary credential: 
  Sure, it would be be useful to be able to allow a friend to access your classified ads website for a day, but not using this. 
  But maybe it can be used to ensure the person you are talking to is really your friend.
  Nonetheless, I should try to see the requirements of tool designed to tackle this issue

* Hierarchical rules: 
  You won't have admins removing users credentials, this sort of things... 
  Not really democratic by the way

* Software automation: 
  You will not use this to deploy your k8s cluster
  Please use something more secure...






Tools summary:
--------------

* **cyrcle-agent**: A GPG-agent replacement
* **cyrcle-deamon**: A peer-to-peer daemon for circle and GPG key server functionalities
* **cyrcli**: A cyrcle cli utility
* **cyrcle**: The GUI to setup, manipulate and backup identities and cirles


Howto build:
------------

* Install the [Rust toolchain](https://www.rust-lang.org/tools/install) (if not already installed)
* Install [GPG](https://gnupg.org/download/)
* Run ```cargo build```
* Be patient...

