# WIK-DPS-TP01

Ce projet est une application Rust servant d'API, elle retourne au format JSON les headers des requêtes faites à la route /ping.
Le port d'écoute de l'application est configuration via le fichier .env et la variable d'environnement PING_LISTEN_PORT.
Toutes les autres routes que /ping retourneront une erreur 404.

Pour set-up le projet : 
* Téléchargez et installez [Cargo][https://doc.rust-lang.org/cargo/index.html].
* Téléchargez le repository.

Pour lancer le projet :
* Ouvrez un terminal de commande et déplacez vous dans le repository.
* Tapez ```cargo build```.
* Tapez ```cargo run```.
