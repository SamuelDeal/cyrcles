Roadmap
=======

v0.1.0
------

* Basic GPG-agent User Interface (Assuan Protocol, see sequoia-ipc)
* Basic GPA-like User Interface
* Basic GPPME-like CLI


v0.2.0
------

* Base circle implementation for key regeneration, and private symetric key
* Unsecure but functional p2p 
* GPG keyserver-like features
* Installer generation


v0.3.0
------

* Cryptographic agility and versioning
* First PoP implementation

v0.4.0
------

* Accessibility options (Please, slint, implement this quickly)
* Rework PoP for accessibility and translations

v0.5.0
------

* agent quota and ban list, (TODO: read about binary signatures)
* First draft documentation
* First draft protocol formalisation


v0.6.0
------

* Get some code/security reviews
* First attemps to make public deployement, to get a sample large enough for the next step


v0.8.0
------

* First version to start to think about adversarial nodes
* Several attemps of reputation or other byzantine and sibil attack defense mechanisms (A/B testing ?)


v0.9.0
------

* Choose defense mechanism after previous step and make it default
* Maybe publish some paper based on previous step results
* Rework UI, noob oriented, with incentive for backups, etc...


v0.10.0
-------

* Finalize protocols
* Security audit


v0.11.0
-------

* Implement multi-device synchronisation mechanism
* Implement some satellite tools like:
  * An http bridge for gpg key server
  * a C library for web auth
  * Thunderbird extension
  * Firefox extension


v0.12.0
-------

* GPG smartcards, YUBI keys and alike
* 2FA
* Windows CNG
* TPM


v1.0.0
-------

* Security audit
* Translations
* Documentation
