lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Statut"),
        ("Your Desktop", "Communauté de Communes du Pays des Herbiers"),
        ("desk_tip", "Merci de communiquer le numéro ci-dessous à notre équipe d'assistance"),
        ("Password", "Mot de passe"),
        ("Ready", "Prêt"),
        ("Established", "Établi"),
        ("connecting_status", "Connexion au réseau d'assistance..."),
        ("Enable service", "Autoriser le service"),
        ("Start service", "Démarrer le service"),
        ("Service is running", "Le service est en cours d'exécution"),
        ("Service is not running", "Le service ne fonctionne pas"),
        ("not_ready_status", "Pas prêt, veuillez vérifier la connexion réseau"),
        ("Control Remote Desktop", "Contrôler le bureau à distance"),
        ("Transfer file", "Transfert de fichiers"),
        ("Connect", "Se connecter"),
        ("Recent sessions", "Sessions récentes"),
        ("Address book", "Carnet d'adresses"),
        ("Confirmation", "Confirmation"),
        ("TCP tunneling", "Tunnel TCP"),
        ("Remove", "Supprimer"),
        ("Refresh random password", "Actualiser le mot de passe aléatoire"),
        ("Set your own password", "Définir votre propre mot de passe"),
        ("Enable keyboard/mouse", "Activer le contrôle clavier/souris"),
        ("Enable clipboard", "Activer la synchronisation du presse-papier"),
        ("Enable file transfer", "Activer le transfert de fichiers"),
        ("Enable TCP tunneling", "Activer le tunnel TCP"),
        ("IP Whitelisting", "Liste blanche IP"),
        ("ID/Relay Server", "ID/Serveur Relais"),
        ("Import server config", "Importer la configuration du serveur"),
        ("Export Server Config", "Exporter la configuration du serveur"),
        ("Import server configuration successfully", "Configuration du serveur importée avec succès"),
        ("Export server configuration successfully", "Configuration du serveur exportée avec succès"),
        ("Invalid server configuration", "Configuration du serveur non valide"),
        ("Clipboard is empty", "Presse-papier vide"),
        ("Stop service", "Arrêter le service"),
        ("Change ID", "Changer d'ID"),
        ("Your new ID", "Votre nouvel ID"),
        ("length %min% to %max%", "longueur de %min% à %max%"),
        ("starts with a letter", "commence par une lettre"),
        ("allowed characters", "caractères autorisés"),
        ("id_change_tip", "Seules les lettres a-z, A-Z, 0-9, _ (trait de soulignement) peuvent être utilisées. La première lettre doit être a-z, A-Z. La longueur doit être comprise entre 6 et 16."),
        ("Website", "Site Web"),
        ("About", "À propos de"),
        ("Slogan_tip", "Direction des Systèmes d'Information du Pays des Herbiers"),
        ("Privacy Statement", "Déclaration de confidentialité"),
        ("Mute", "Muet"),
        ("Build Date", "Date de compilation"),
        ("Version", "Version"),
        ("Home", "Accueil"),
        ("Audio Input", "Entrée audio"),
        ("Enhancements", "Améliorations"),
        ("Hardware Codec", "Transcodage matériel"),
        ("Adaptive bitrate", "Débit adaptatif"),
        ("ID Server", "Serveur ID"),
        ("Relay Server", "Serveur relais"),
        ("API Server", "Serveur API"),
        ("invalid_http", "Doit commencer par http:// ou https://"),
        ("Invalid IP", "IP invalide"),
        ("Invalid format", "Format invalide"),
        ("server_not_support", "Pas encore supporté par le serveur"),
        ("Not available", "Indisponible"),
        ("Too frequent", "Modifié trop fréquemment, veuillez réessayer plus tard"),
        ("Cancel", "Annuler"),
        ("Skip", "Ignorer"),
        ("Close", "Fermer"),
        ("Retry", "Réessayer"),
        ("OK", "Valider"),
        ("Password Required", "Mot de passe requis"),
        ("Please enter your password", "Veuillez saisir votre mot de passe"),
        ("Remember password", "Mémoriser le mot de passe"),
        ("Wrong Password", "Mauvais mot de passe"),
        ("Do you want to enter again?", "Voulez-vous participer à nouveau ?"),
        ("Connection Error", "Erreur de connexion"),
        ("Error", "Erreur"),
        ("Reset by the peer", "La connexion a été fermée par l'appareil distant"),
        ("Connecting...", "Connexion..."),
        ("Connection in progress. Please wait.", "Connexion en cours. Veuillez patienter."),
        ("Please try 1 minute later", "Réessayez dans une minute"),
        ("Login Error", "Erreur de connexion"),
        ("Successful", "Succès"),
        ("Connected, waiting for image...", "Connecté, en attente de transmission d'image..."),
        ("Name", "Nom"),
        ("Type", "Type"),
        ("Modified", "Modifié le"),
        ("Size", "Taille"),
        ("Show Hidden Files", "Afficher les fichiers cachés"),
        ("Receive", "Recevoir"),
        ("Send", "Envoyer"),
        ("Refresh File", "Rafraîchir le contenu"),
        ("Local", "Local"),
        ("Remote", "Distant"),
        ("Remote Computer", "Appareil distant"),
        ("Local Computer", "Appareil local"),
        ("Confirm Delete", "Confirmer la suppression"),
        ("Delete", "Supprimer"),
        ("Properties", "Propriétés"),
        ("Multi Select", "Sélection multiple"),
        ("Select All", "Tout sélectionner"),
        ("Unselect All", "Tout déselectionner"),
        ("Empty Directory", "Répertoire vide"),
        ("Not an empty directory", "Pas un répertoire vide"),
        ("Are you sure you want to delete this file?", "Voulez-vous vraiment supprimer ce fichier ?"),
        ("Are you sure you want to delete this empty directory?", "Voulez-vous vraiment supprimer ce répertoire vide ?"),
        ("Are you sure you want to delete the file of this directory?", "Voulez-vous vraiment supprimer le fichier de ce répertoire ?"),
        ("Do this for all conflicts", "Appliquer à d'autres conflits"),
        ("This is irreversible!", "C'est irréversible !"),
        ("Deleting", "Suppression"),
        ("files", "fichier"),
        ("Waiting", "En attente..."),
        ("Finished", "Terminé"),
        ("Speed", "Vitesse"),
        ("Custom Image Quality", "Définir la qualité d'image"),
        ("Privacy mode", "Mode privé"),
        ("Block user input", "Bloquer la saisie de l'utilisateur"),
        ("Unblock user input", "Débloquer l'entrée de l'utilisateur"),
        ("Adjust Window", "Ajuster la fenêtre"),
        ("Original", "Ratio d'origine"),
        ("Shrink", "Rétrécir"),
        ("Stretch", "Étirer"),
        ("Scrollbar", "Barre de défilement"),
        ("ScrollAuto", "Défilement automatique"),
        ("Good image quality", "Bonne qualité d'image"),
        ("Balanced", "Qualité d'image normale"),
        ("Optimize reaction time", "Optimiser le temps de réaction"),
        ("Custom", "Personnalisé"),
        ("Show remote cursor", "Afficher le curseur distant"),
        ("Show quality monitor", "Afficher le moniteur de qualité"),
        ("Disable clipboard", "Désactiver le presse-papier"),
        ("Lock after session end", "Verrouiller l'appareil distant après la déconnexion"),
        ("Insert", "Envoyer"),
        ("Insert Lock", "Verrouiller l'appareil distant"),
        ("Refresh", "Rafraîchir l'écran"),
        ("ID does not exist", "L'ID n'existe pas"),
        ("Failed to connect to rendezvous server", "Échec de la connexion au serveur rendezvous"),
        ("Please try later", "Veuillez essayer plus tard"),
        ("Remote desktop is offline", "Le bureau à distance est hors ligne"),
        ("Key mismatch", "Discordance de clés"),
        ("Timeout", "Connexion expirée"),
        ("Failed to connect to relay server", "Échec de la connexion au serveur relais"),
        ("Failed to connect via rendezvous server", "Échec de l'établissement d'une connexion via le serveur rendezvous"),
        ("Failed to connect via relay server", "Impossible d'établir une connexion via le serveur relais"),
        ("Failed to make direct connection to remote desktop", "Impossible d'établir une connexion directe"),
        ("Set Password", "Définir le mot de passe"),
        ("OS Password", "Mot de passe du système d'exploitation"),
        ("install_tip", "Vous utilisez une version non installée. En raison des restrictions UAC, en tant que terminal contrôlé, dans certains cas, il ne sera pas en mesure de contrôler la souris et le clavier ou d'enregistrer l'écran. Veuillez cliquer sur le bouton ci-dessous pour installer RustDesk au système pour éviter la question ci-dessus."),
        ("Click to upgrade", "Cliquer pour mettre à niveau"),
        ("Click to download", "Cliquer pour télécharger"),
        ("Click to update", "Cliquer pour mettre à jour"),
        ("Configure", "Configurer"),
        ("config_acc", "Afin de pouvoir contrôler votre bureau à distance, veuillez donner l'autorisation \"accessibilité\" à RustDesk."),
        ("config_screen", "Afin de pouvoir accéder à votre bureau à distance, veuillez donner à RustDesk l'autorisation \"enregistrement d'écran\"."),
        ("Installing ...", "Installation..."),
        ("Install", "Installer"),
        ("Installation", "Installation"),
        ("Installation Path", "Chemin d'installation"),
        ("Create start menu shortcuts", "Créer des raccourcis dans le menu démarrer"),
        ("Create desktop icon", "Créer une icône sur le bureau"),
        ("agreement_tip", "Démarrer l'installation signifie accepter le contrat de licence."),
        ("Accept and Install", "Accepter et installer"),
        ("End-user license agreement", "Contrat d'utilisateur"),
        ("Generating ...", "Génération..."),
        ("Your installation is lower version.", "La version que vous avez installée est inférieure à la version en cours d'exécution."),
        ("not_close_tcp_tip", "Veuillez ne pas fermer cette fenêtre lors de l'utilisation du tunnel"),
        ("Listening ...", "En attente de connexion tunnel..."),
        ("Remote Host", "Hôte distant"),
        ("Remote Port", "Port distant"),
        ("Action", "Action"),
        ("Add", "Ajouter"),
        ("Local Port", "Port local"),
        ("Local Address", "Adresse locale"),
        ("Change Local Port", "Changer le port local"),
        ("setup_server_tip", "Si vous avez besoin d'une vitesse de connexion plus rapide, vous pouvez choisir de créer votre propre serveur"),
        ("Too short, at least 6 characters.", "Trop court, au moins 6 caractères."),
        ("The confirmation is not identical.", "Les deux entrées ne correspondent pas"),
        ("Permissions", "Autorisations"),
        ("Accept", "Accepter"),
        ("Dismiss", "Rejeter"),
        ("Disconnect", "Déconnecter"),
        ("Enable file copy and paste", "Autoriser le copier-coller de fichiers"),
        ("Connected", "Connecté"),
        ("Direct and encrypted connection", "Connexion directe chiffrée"),
        ("Relayed and encrypted connection", "Connexion relais chiffrée"),
        ("Direct and unencrypted connection", "Connexion directe non chiffrée"),
        ("Relayed and unencrypted connection", "Connexion relais non chiffrée"),
        ("Enter Remote ID", "Entrer l'ID de l'appareil distant"),
        ("Enter your password", "Entrer votre mot de passe"),
        ("Logging in...", "En cours de connexion ..."),
        ("Enable RDP session sharing", "Activer le partage de session RDP"),
        ("Auto Login", "Connexion automatique (le verrouillage ne sera effectif qu'après la désactivation du premier paramètre)"),
        ("Enable direct IP access", "Autoriser l'accès direct par IP"),
        ("Rename", "Renommer"),
        ("Space", "Espace"),
        ("Create desktop shortcut", "Créer un raccourci sur le bureau"),
        ("Change Path", "Changer de chemin"),
        ("Create Folder", "Créer un dossier"),
        ("Please enter the folder name", "Veuillez saisir le nom du dossier"),
        ("Fix it", "Réparer"),
        ("Warning", "Avertissement"),
        ("Login screen using Wayland is not supported", "L'écran de connexion utilisant Wayland n'est pas pris en charge"),
        ("Reboot required", "Redémarrage requis"),
        ("Unsupported display server", "Le serveur d'affichage actuel n'est pas pris en charge"),
        ("x11 expected", "x11 requis"),
        ("Port", "Port"),
        ("Settings", "Paramètres"),
        ("Username", " Nom d'utilisateur"),
        ("Invalid port", "Port invalide"),
        ("Closed manually by the peer", "Fermé manuellement par l'appareil distant"),
        ("Enable remote configuration modification", "Autoriser la modification de la configuration à distance"),
        ("Run without install", "Exécuter sans installer"),
        ("Connect via relay", "Connexion via relais"),
        ("Always connect via relay", "Forcer la connexion relais"),
        ("whitelist_tip", "Seule une IP de la liste blanche peut accéder à mon appareil"),
        ("Login", "Connexion"),
        ("Verify", "Vérifier"),
        ("Remember me", "Se souvenir de moi"),
        ("Trust this device", "Faire confiance à cet appareil"),
        ("Verification code", "Code de vérification"),
        ("verification_tip", "Un nouvel appareil a été détecté et un code de vérification a été envoyé à l'adresse e-mail enregistrée, entrez le code de vérification pour continuer la connexion."),
        ("Logout", "Déconnexion"),
        ("Tags", "Étiquettes"),
        ("Search ID", "Rechercher un ID"),
        ("whitelist_sep", "Vous pouvez utiliser une virgule, un point-virgule, un espace ou une nouvelle ligne comme séparateur"),
        ("Add ID", "Ajouter un ID"),
        ("Add Tag", "Ajout étiquette(s)"),
        ("Unselect all tags", "Désélectionner toutes les étiquettes"),
        ("Network error", "Erreur réseau"),
        ("Username missed", "Nom d'utilisateur manquant"),
        ("Password missed", "Mot de passe manquant"),
        ("Wrong credentials", "Identifiant ou mot de passe erroné"),
        ("The verification code is incorrect or has expired", "Le code de vérification est incorrect ou a expiré"),
        ("Edit Tag", "Gestion étiquettes"),
        ("Forget Password", "Oublier le Mot de passe"),
        ("Favorites", "Favoris"),
        ("Add to Favorites", "Ajouter aux Favoris"),
        ("Remove from Favorites", "Retirer des favoris"),
        ("Empty", "Vide"),
        ("Invalid folder name", "Nom de dossier invalide"),
        ("Socks5 Proxy", "Socks5 Agents"),
        ("Discovered", "Découvert"),
        ("install_daemon_tip", "Pour une exécution au démarrage du système, vous devez installer le service système."),
        ("Remote ID", "ID de l'appareil distant"),
        ("Paste", "Coller"),
        ("Paste here?", "Coller ici ?"),
        ("Are you sure to close the connection?", "Êtes-vous sûr de fermer la connexion ?"),
        ("Download new version", "Télécharger la nouvelle version"),
        ("Touch mode", "Mode tactile"),
        ("Mouse mode", "Mode souris"),
        ("One-Finger Tap", "Tapez d'un doigt"),
        ("Left Mouse", "Bouton gauche de la souris"),
        ("One-Long Tap", "Un touché long"),
        ("Two-Finger Tap", "Tapez à deux doigts"),
        ("Right Mouse", "Bouton droit de la souris"),
        ("One-Finger Move", "Mouvement à un doigt"),
        ("Double Tap & Move", "Appuyez deux fois et déplacez"),
        ("Mouse Drag", "Glissement de la souris"),
        ("Three-Finger vertically", "Trois doigts verticalement"),
        ("Mouse Wheel", "Roulette de la souris"),
        ("Two-Finger Move", "Mouvement à deux doigts"),
        ("Canvas Move", "Déplacer la vue"),
        ("Pinch to Zoom", "Pincer pour zoomer"),
        ("Canvas Zoom", "Zoom sur la vue"),
        ("Reset canvas", "Réinitialiser la vue"),
        ("No permission of file transfer", "Aucune autorisation de transfert de fichiers"),
        ("Note", "Noter"),
        ("Connection", "Connexion"),
        ("Share Screen", "Partager l'écran"),
        ("Chat", "Discuter"),
        ("Total", "Total"),
        ("items", "éléments"),
        ("Selected", "Sélectionné(s)"),
        ("Screen Capture", "Capture d'écran"),
        ("Input Control", "Contrôle de saisie"),
        ("Audio Capture", "Capture audio"),
        ("File Connection", "Connexion de fichier"),
        ("Screen Connection", "Connexion de l'écran"),
        ("Do you accept?", "Acceptez-vous ?"),
        ("Open System Setting", "Ouvrir les paramètres système"),
        ("How to get Android input permission?", "Comment obtenir l'autorisation d'entrée Android ?"),
        ("android_input_permission_tip1", "Pour qu'un appareil distant puisse contrôler votre appareil Android via la souris ou le toucher, vous devez autoriser RustDesk à utiliser le service \"Accessibilité\"."),
        ("android_input_permission_tip2", "Veuillez accéder à la page suivante des paramètres système, recherchez et entrez [Services installés], activez le service [RustDesk Input]."),
        ("android_new_connection_tip", "Une nouvelle demande de contrôle a été reçue, elle souhaite contrôler votre appareil actuel."),
        ("android_service_will_start_tip", "L'activation de la capture d'écran démarrera automatiquement le service, permettant à d'autres appareils de demander une connexion à partir de cet appareil."),
        ("android_stop_service_tip", "La fermeture du service fermera automatiquement toutes les connexions établies."),
        ("android_version_audio_tip", "La version actuelle d'Android ne prend pas en charge la capture audio, veuillez passer à Android 10 ou supérieur."),
        ("android_start_service_tip", "Appuyez sur [Démarrer le service] ou activez l'autorisation [Capture d'écran] pour démarrer le service de partage d'écran."),
        ("android_permission_may_not_change_tip", "Les autorisations pour les connexions établies peuvent ne pas être prisent en compte instantanément ou avant la reconnection."),
        ("Account", "Compte"),
        ("Overwrite", "Écraser"),
        ("This file exists, skip or overwrite this file?", "Ce fichier existe, ignorer ou écraser ce fichier ?"),
        ("Quit", "Quitter"),
        ("Help", "Aider"),
        ("Failed", "échouer"),
        ("Succeeded", "Succès"),
        ("Someone turns on privacy mode, exit", "Quelqu'un active le mode de confidentialité, quittez"),
        ("Unsupported", "Non pris en charge"),
        ("Peer denied", "Appareil distant refusé"),
        ("Please install plugins", "Veuillez installer les plugins"),
        ("Peer exit", "Appareil distant déconnecté"),
        ("Failed to turn off", "Échec de la désactivation"),
        ("Turned off", "Désactivé"),
        ("Language", "Langue"),
        ("Keep RustDesk background service", "Gardez le service RustDesk en arrière plan"),
        ("Ignore Battery Optimizations", "Ignorer les optimisations batterie"),
        ("android_open_battery_optimizations_tip", "Conseil android d'optimisation de batterie"),
        ("Start on boot", "Lancer au démarrage"),
        ("Start the screen sharing service on boot, requires special permissions", "Lancer le service de partage d'écran au démarrage, nécessite des autorisations spéciales"),
        ("Connection not allowed", "Connexion non autorisée"),
        ("Legacy mode", "Mode hérité"),
        ("Map mode", "Mode correspondance"),
        ("Translate mode", "Mode traduction"),
        ("Use permanent password", "Utiliser un mot de passe permanent"),
        ("Use both passwords", "Utiliser les mots de passe unique et permanent"),
        ("Set permanent password", "Définir le mot de passe permanent"),
        ("Enable remote restart", "Activer le redémarrage à distance"),
        ("Restart remote device", "Redémarrer l'appareil à distance"),
        ("Are you sure you want to restart", "Êtes-vous sûr de vouloir redémarrer l'appareil ?"),
        ("Restarting remote device", "Redémarrage de l'appareil distant"),
        ("remote_restarting_tip", "L'appareil distant redémarre, veuillez fermer cette boîte de message et vous reconnecter avec un mot de passe permanent après un certain temps"),
        ("Copied", "Copié"),
        ("Exit Fullscreen", "Quitter le mode plein écran"),
        ("Fullscreen", "Plein écran"),
        ("Mobile Actions", "Actions mobiles"),
        ("Select Monitor", "Sélection du Moniteur"),
        ("Control Actions", "Actions de contrôle"),
        ("Display Settings", "Paramètres d'affichage"),
        ("Ratio", "Rapport"),
        ("Image Quality", "Qualité d'image"),
        ("Scroll Style", "Style de défilement"),
        ("Show Toolbar", "Afficher la barre d'outils"),
        ("Hide Toolbar", "Masquer la barre d'outils"),
        ("Direct Connection", "Connexion directe"),
        ("Relay Connection", "Connexion relais"),
        ("Secure Connection", "Connexion sécurisée"),
        ("Insecure Connection", "Connexion non sécurisée"),
        ("Scale original", "Échelle 100%"),
        ("Scale adaptive", "Mise à l'échelle Auto"),
        ("General", "Général"),
        ("Security", "Sécurité"),
        ("Theme", "Thème"),
        ("Dark Theme", "Thème sombre"),
        ("Light Theme", "Thème clair"),
        ("Dark", "Sombre"),
        ("Light", "Clair"),
        ("Follow System", "Suivi système"),
        ("Enable hardware codec", "Activer le transcodage matériel"),
        ("Unlock Security Settings", "Déverrouiller les configurations de sécurité"),
        ("Enable audio", "Activer l'audio"),
        ("Unlock Network Settings", "Déverrouiller les configurations réseau"),
        ("Server", "Serveur"),
        ("Direct IP Access", "Accès IP direct"),
        ("Proxy", "Proxy"),
        ("Apply", "Appliquer"),
        ("Disconnect all devices?", "Déconnecter tous les appareils ?"),
        ("Clear", "Effacer"),
        ("Audio Input Device", "Périphérique source audio"),
        ("Use IP Whitelisting", "Utiliser une liste blanche d'IP"),
        ("Network", "Réseau"),
        ("Pin Toolbar", "Épingler la barre d'outil"),
        ("Unpin Toolbar", "Détacher la barre d'outil"),
        ("Recording", "Enregistrement"),
        ("Directory", "Répertoire"),
        ("Automatically record incoming sessions", "Enregistrement automatique des sessions entrantes"),
        ("Change", "Modifier"),
        ("Start session recording", "Commencer l'enregistrement"),
        ("Stop session recording", "Stopper l'enregistrement"),
        ("Enable recording session", "Activer l'enregistrement de session"),
        ("Enable LAN discovery", "Activer la découverte sur réseau local"),
        ("Deny LAN discovery", "Interdir la découverte sur réseau local"),
        ("Write a message", "Ecrire un message"),
        ("Prompt", "Annonce"),
        ("Please wait for confirmation of UAC...", "Veuillez attendre la confirmation de l'UAC..."),
        ("elevated_foreground_window_tip", "La fenêtre actuelle de l'appareil distant nécessite des privilèges plus élevés pour fonctionner, elle ne peut donc pas être atteinte par la souris et le clavier. Vous pouvez demander à l'utilisateur distant de réduire la fenêtre actuelle ou de cliquer sur le bouton d'élévation dans la fenêtre de gestion des connexions. Pour éviter ce problème, il est recommandé d'installer le logiciel sur l'appareil distant."),
        ("Disconnected", "Déconnecté"),
        ("Other", "Divers"),
        ("Confirm before closing multiple tabs", "Confirmer avant de fermer plusieurs onglets"),
        ("Keyboard Settings", "Configuration clavier"),
        ("Full Access", "Accès total"),
        ("Screen Share", "Partage d'écran"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland nécessite Ubuntu 21.04 ou une version supérieure."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland nécessite une version supérieure de la distribution Linux. Veuillez essayer le bureau X11 ou changer votre système d'exploitation."),
        ("JumpLink", "Afficher"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Veuillez sélectionner l'écran à partager (côté appareil distant)."),
        ("Show RustDesk", "Afficher l'assistance"),
        ("This PC", "Ce PC"),
        ("or", "ou"),
        ("Continue with", "Continuer avec"),
        ("Elevate", "Autoriser l'accès"),
        ("Zoom cursor", "Augmenter la taille du curseur"),
        ("Accept sessions via password", "Accepter les sessions via mot de passe"),
        ("Accept sessions via click", "Accepter les sessions via clique de confirmation"),
        ("Accept sessions via both", "Accepter les sessions via mot de passe ou clique de confirmation"),
        ("Please wait for the remote side to accept your session request...", "Veuillez attendre que votre demande de session distante soit accepter ..."),
        ("One-time Password", "Mot de passe unique"),
        ("Use one-time password", "Utiliser un mot de passe unique"),
        ("One-time password length", "Longueur du mot de passe unique"),
        ("Request access to your device", "Demande d'accès à votre appareil"),
        ("Hide connection management window", "Masquer la fenêtre de gestion des connexions"),
        ("hide_cm_tip", "Autoriser le masquage uniquement si vous acceptez des sessions via un mot de passe et utilisez un mot de passe permanent"),
        ("wayland_experiment_tip", "Le support Wayland est en phase expérimentale, veuillez utiliser X11 si vous avez besoin d'un accès sans surveillance."),
        ("Right click to select tabs", "Clique droit pour selectionner les onglets"),
        ("Skipped", "Ignoré"),
        ("Add to address book", "Ajouter au carnet d'adresses"),
        ("Group", "Groupe"),
        ("Search", "Rechercher"),
        ("Closed manually by web console", "Fermé manuellement par la console Web"),
        ("Local keyboard type", "Disposition du clavier local"),
        ("Select local keyboard type", "Selectionner la disposition du clavier local"),
        ("software_render_tip", "Si vous avez une carte graphique NVIDIA et que la fenêtre distante se ferme immédiatement après la connexion, l'installation du pilote Nouveau et le choix d'utiliser le rendu du logiciel peuvent aider. Un redémarrage du logiciel est requis."),
        ("Always use software rendering", "Utiliser toujours le rendu logiciel"),
        ("config_input", "Afin de contrôler le bureau à distance avec le clavier, vous devez accorder à RustDesk l'autorisation \"Surveillance de l’entrée\"."),
        ("config_microphone", "Pour discuter à distance, vous devez accorder à RustDesk les autorisations « Enregistrer l'audio »."),
        ("request_elevation_tip", "Vous pouvez également demander une augmentation des privilèges s'il y a quelqu'un du côté distant."),
        ("Wait", "En cours"),
        ("Elevation Error", "Erreur d'augmentation des privilèges"),
        ("Ask the remote user for authentication", "Demander à l'utilisateur distant de s'authentifier"),
        ("Choose this if the remote account is administrator", "Choisissez ceci si le compte distant est le compte d'administrateur"),
        ("Transmit the username and password of administrator", "Transmettre le nom d'utilisateur et le mot de passe de l'administrateur"),
        ("still_click_uac_tip", "Nécessite toujours que l'utilisateur distant confirme par la fenêtre UAC de RustDesk en cours d'éxécution."),
        ("Request Elevation", "Demande d'augmentation des privilèges"),
        ("wait_accept_uac_tip", "Veuillez attendre que l'utilisateur distant accepte la boîte de dialogue UAC."),
        ("Elevate successfully", "Augmentation des privilèges avec succès"),
        ("uppercase", "majuscule"),
        ("lowercase", "minuscule"),
        ("digit", "chiffre"),
        ("special character", "caractère spécial"),
        ("length>=8", "longueur>=8"),
        ("Weak", "Faible"),
        ("Medium", "Moyen"),
        ("Strong", "Fort"),
        ("Switch Sides", "Inverser la prise de contrôle"),
        ("Please confirm if you want to share your desktop?", "Veuillez confirmer le partager de votre bureau ?"),
        ("Display", "Affichage"),
        ("Default View Style", "Style de vue par défaut"),
        ("Default Scroll Style", "Style de défilement par défaut"),
        ("Default Image Quality", "Qualité d'image par défaut"),
        ("Default Codec", "Codec par défaut"),
        ("Bitrate", "Débit"),
        ("FPS", "FPS"),
        ("Auto", "Auto"),
        ("Other Default Options", "Autres options par défaut"),
        ("Voice call", "Appel voix"),
        ("Text chat", "Conversation textuelle"),
        ("Stop voice call", "Stopper l'appel voix"),
        ("relay_hint_tip", "Il se peut qu'il ne doit pas possible de se connecter directement, vous pouvez essayer de vous connecter via un relais. \nEn outre, si vous souhaitez utiliser directement le relais, vous pouvez ajouter le suffixe \"/r\" à l'ID ou sélectionner l'option \"Toujours se connecter via le relais\" dans la fiche appareils distants."),
        ("Reconnect", "Se reconnecter"),
        ("Codec", "Codec"),
        ("Resolution", "Résolution"),
        ("No transfers in progress", "Pas de transfert en cours"),
        ("Set one-time password length", "Définir la longueur du mot de passe à usage unique"),
        ("RDP Settings", "Configuration RDP"),
        ("Sort by", "Trier par"),
        ("New Connection", "Nouvelle connexion"),
        ("Restore", "Restaurer"),
        ("Minimize", "Minimiser"),
        ("Maximize", "Maximiser"),
        ("Your Device", "Votre appareil"),
        ("empty_recent_tip", "Oups, pas de sessions récentes !\nIl est temps d'en prévoir une nouvelle."),
        ("empty_favorite_tip", "Vous n'avez pas encore d'appareils distants favorits ?\nTrouvons quelqu'un avec qui vous connecter et ajoutez-les à vos favoris !"),
        ("empty_lan_tip", "Oh non, il semble que nous n'ayons pas encore d'appareils réseau local découverts."),
        ("empty_address_book_tip", "Ouh là là ! il semble qu'il n'y ait actuellement aucun appareil distant répertorié dans votre carnet d'adresses."),
        ("eg: admin", "ex: admin"),
        ("Empty Username", "Nom d'utilisation non spécifié"),
        ("Empty Password", "Mot de passe non spécifié"),
        ("Me", "Moi"),
        ("identical_file_tip", "Ce fichier est identique à celui de l'appareil distant."),
        ("show_monitors_tip", "Afficher les moniteurs dans la barre d'outils"),
        ("View Mode", "Mode vue"),
        ("login_linux_tip", "Se connecter au compte Linux distant"),
        ("verify_rustdesk_password_tip", "Vérifier le mot de passe RustDesk"),
        ("remember_account_tip", "Se souvenir de ce compte"),
        ("os_account_desk_tip", "Ce compte est utilisé pour se connecter au système d'exploitation distant et activer la session de bureau en mode sans affichage"),
        ("OS Account", "Compte système d'exploitation"),
        ("another_user_login_title_tip", "Un autre utilisateur est déjà connecté"),
        ("another_user_login_text_tip", "Déconnexion"),
        ("xorg_not_found_title_tip", "Xorg introuvable"),
        ("xorg_not_found_text_tip", "Veuillez installer Xorg"),
        ("no_desktop_title_tip", "Aucun gestionaire de bureau n'est disponible"),
        ("no_desktop_text_tip", "Veuillez installer le gestionaire de bureau GNOME"),
        ("No need to elevate", "Pas besoin de permissions administrateur"),
        ("System Sound", "Son système"),
        ("Default", "Défaut"),
        ("New RDP", "Nouvel RDP"),
        ("Fingerprint", "Empreinte digitale"),
        ("Copy Fingerprint", "Copier empreinte digitale"),
        ("no fingerprints", "Pas d'empreintes digitales"),
        ("Select a peer", "Sélectionnez l'appareil distant"),
        ("Select peers", "Sélectionnez des appareils distants"),
        ("Plugins", "Plugins"),
        ("Uninstall", "Désinstaller"),
        ("Update", "Mise à jour"),
        ("Enable", "Activé"),
        ("Disable", "Desactivé"),
        ("Options", "Options"),
        ("resolution_original_tip", "Résolution d'origine"),
        ("resolution_fit_local_tip", "Adapter la résolution local"),
        ("resolution_custom_tip", "Résolution personnalisée"),
        ("Collapse toolbar", "Réduire la barre d'outils"),
        ("Accept and Elevate", "Accepter et autoriser l'augmentation des privilèges"),
        ("accept_and_elevate_btn_tooltip", "Accepter la connexion l'augmentation des privilèges UAC."),
        ("clipboard_wait_response_timeout_tip", "Expiration du délai d'attente presse-papiers."),
        ("Incoming connection", "Connexion entrante"),
        ("Outgoing connection", "Connexion sortante"),
        ("Exit", "Quitter"),
        ("Open", "Ouvrir"),
        ("logout_tip", "Êtes-vous sûr de vouloir vous déconnecter ?"),
        ("Service", "Service"),
        ("Start", "Lancer"),
        ("Stop", "Stopper"),
        ("exceed_max_devices", "Vous avez atteint le nombre maximal d'appareils gérés."),
        ("Sync with recent sessions", "Synchroniser avec les sessions récentes"),
        ("Sort tags", "Trier les étiquettes"),
        ("Open connection in new tab", "Ouvrir la connexion dans un nouvel onglet"),
        ("Move tab to new window", "Déplacer l'onglet vers une nouvelle fenêtre"),
        ("Can not be empty", "Ne peux pas être vide"),
        ("Already exists", "Existe déjà"),
        ("Change Password", "Changer le mot de passe"),
        ("Refresh Password", "Actualiser le mot de passe"),
        ("ID", "VOTRE IDENTIFIANT"),
        ("Grid View", "Vue Grille"),
        ("List View", "Vue Liste"),
        ("Select", "Sélectionner"),
        ("Toggle Tags", "Basculer vers les étiquettes"),
        ("pull_ab_failed_tip", "Impossible d'actualiser le carnet d'adresses"),
        ("push_ab_failed_tip", "Échec de la synchronisation du carnet d'adresses"),
        ("synced_peer_readded_tip", "Les appareils qui étaient présents dans les sessions récentes seront synchronisés avec le carnet d'adresses."),
        ("Change Color", "Modifier la couleur"),
        ("Primary Color", "Couleur primaire"),
        ("HSV Color", "Couleur TSL"),
        ("Installation Successful!", "Installation réussie !"),
        ("Installation failed!", "Échec de l'installation !"),
        ("Reverse mouse wheel", "Inverser le sens de la molette de la souris"),
        ("{} sessions", "{} sessions"),
        ("scam_title", "Vous êtes peut-être victime d'une ESCROQUERIE !"),
        ("scam_text1", "Si vous êtes au téléphone avec quelqu'un QUE VOUS NE CONNAISSEZ PAS et en qui VOUS N'AVEZ PAS CONFIANCE et qui vous a demandé d'utiliser RustDesk et de démarrer le service, ne le faites pas et raccrochez immédiatement."),
        ("scam_text2", "Il s'agit probablement d'un escroc qui tente de vous voler de l'argent ou d'autres informations personnelles."),
        ("Don't show again", "Ne plus montrer"),
        ("I Agree", "J'accepte"),
        ("Decline", "Refuser"),
        ("Timeout in minutes", "Délai d'expiration en minutes"),
        ("auto_disconnect_option_tip", "Fermer automatiquement les sessions entrantes en cas d'inactivité de l'utilisateur"),
        ("Connection failed due to inactivity", "Déconnecté automatiquement pour cause d'inactivité"),
        ("Check for software update on startup", "Vérifier la disponibilité des mises à jour au démarrage"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "Veuillez mettre à jour RustDesk Server Pro avec la version {} ou une version plus récente !"),
        ("pull_group_failed_tip", "Échec de l'actualisation du groupe"),
        ("Filter by intersection", "Filtrer par intersection"),
        ("Remove wallpaper during incoming sessions", "Supprimer le fond d'écran lors des sessions entrantes"),
        ("Test", ""),
        ("display_is_plugged_out_msg", "L'écran est débranché, passez au premier écran."),
        ("No displays", "Aucun affichage"),
        ("elevated_switch_display_msg", "Basculez vers l’affichage principal, car plusieurs affichages ne sont pas pris en charge en mode utilisateur avec privilèges."),
        ("Open in new window", "Ouvrir dans une nouvelle fenêtre"),
        ("Show displays as individual windows", "Montrer les affichages sous forme de fenêtres individuelles"),
        ("Use all my displays for the remote session", "Utiliser tous mes écrans pour la session à distance"),
        ("selinux_tip", "SELinux est activé sur votre appareil, ce qui peut empêcher RustDesk de fonctionner correctement en tant que machine contrôlé."),
        ("Change view", "Disposition d'affichage"),
        ("Big tiles", "Grandes tuiles"),
        ("Small tiles", "Petites tuiles"),
        ("List", "Liste"),
        ("Virtual display", "Affichage virtuel"),
        ("Plug out all", "Déconnecter tout"),
        ("True color (4:4:4)", "Couleur réelle (4:4:4)"),
        ("Enable blocking user input", "Activer le blocage des entrées utilisateur"),
        ("id_input_tip", "Vous pouvez saisir un ID, une adresse IP directe ou un nom de domaine avec un port (<domaine>:<port>).\nSi vous souhaitez accéder à un appareil sur un autre serveur, veuillez ajouter l'adresse du serveur (<id><adresse_serveur>?key=<valeur_key>), par exemple,\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nSi vous souhaitez accéder à un appareil sur un serveur public, veuillez saisir \"<id>@public\" , la clé n'est pas nécessaire pour le serveur public"),
        ("privacy_mode_impl_mag_tip", "Mode 1"),
        ("privacy_mode_impl_virtual_display_tip", "Mode 2"),
        ("Enter privacy mode", "Passer en mode confidentialité"),
        ("Exit privacy mode", "Quitter le mode confidentialité"),
        ("idd_not_support_under_win10_2004_tip", "Le pilote d'affichage indirect n'est pas pris en charge. Windows 10, version 2004 ou plus récente est requise."),
        ("switch_display_elevated_connections_tip", "Le passage à l'affichage non principal n'est pas pris en charge en mode utilisateur avec privilèges lorsqu'il existe plusieurs connexions. Veuillez réessayer après l'installation si vous souhaitez contrôler plusieurs écrans."),
        ("input_source_1_tip", "Source entrée 1"),
        ("input_source_2_tip", "Source entrée 2"),
        ("capture_display_elevated_connections_tip", "La capture de plusieurs écrans n'est pas prise en charge en mode utilisateur avec privilièges. Veuillez réessayer après l'installation si vous souhaitez contrôler plusieurs écrans."),
        ("Swap control-command key", "Échanger la touche de controle-commande"),
        ("swap-left-right-mouse", "Intervertir le bouton gauche et droit de la souris"),
        ("2FA code", ""),
        ("More", ""),
        ("enable-2fa-title", ""),
        ("enable-2fa-desc", ""),
        ("wrong-2fa-code", ""),
        ("enter-2fa-title", ""),
        ("Email verification code must be 6 characters.", ""),
        ("2FA code must be 6 digits.", ""),
        ("Multiple Windows sessions found", ""),
        ("Please select the session you want to connect to", ""),
        ("powered_by_me", ""),
        ("outgoing_only_desk_tip", ""),
        ("preset_password_warning", ""),
        ("Security Alert", ""),
        ("My address book", ""),
        ("Personal", ""),
        ("Owner", ""),
        ("Set shared password", ""),
        ("Exist in", ""),
        ("Read-only", ""),
        ("Read/Write", ""),
        ("Full Control", ""),
        ("share_warning_tip", ""),
        ("Everyone", ""),
        ("ab_web_console_tip", ""),
        ("allow-only-conn-window-open-tip", "Autoriser les connexions uniquement si la fenetre principale est ouverte"),
    ].iter().cloned().collect();
}
