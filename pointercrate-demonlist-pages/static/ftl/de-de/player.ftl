player-banned = Gebannt
    .yes = Ja
    .no = Nein

player-nationality = Nationalität
    .info = Beachte, dass dies ausschließlich als "Land mit rechtsmäßigem Wohnsitz" gilt, ohne Ausnahmen!
    .none = Keine

player-subdivision = Politische Gliederung
    .none = Keine

## Players tab
players = Spieler

player-manager = Spielerverwaltung

player-viewer = Spieler #
    .welcome = Klick auf einen Spieler links um anzufangen!

    .info = Willkommen zur Spielerverwaltung. Hier kannst du Spieler bannen / entbannen. Einen Spieler bannen entfernt alle seine eingereichten oder ausstehenden Rekorde. Alle bereits angenommen Rekorde werden stattdessen auf den Status "abgelehnt" gesetzt.
    .records-redirect = Zeige Rekorde an

player-idsearch-panel = Spieler nach ID suchen
    .info = Spieler können durch ihre ID eindeutig identifiziert werden. Durch die Eingabe der Spieler ID wird dieser links ausgewählt (sofern er existiert)
    .id-field = Spieler ID:

    .submit = Finde nach ID

    .id-validator-valuemissing = Spieler ID benötigt

player-name-dialog = Spielername ändern
    .info = Ändere diesen Spielernamen. Dies wird den Namen des Spielers auf jedem Rekord aktualisieren. Sollte ein Spieler bereits mit diesem Namen existieren, so werden alle seine Objekte zusammengefasst, wobei das neue Objekt die ID des Spielers erhält, welches gerade bearbeitet wird. In diesem Fall werden die Rekorde der Spieler zusammengefasst und ihre Ersteller- / Verifizierer- / Veröffentlicherinformationen werden aktualisiert. Intern wird jeder Rekord zum neuen Spieler verschoben und bei Konflikten gelten dieselben Regeln wie beim Bearbeiten eines Spielers von einem Rekord.
    .name-field = Name:

    .submit = Ändern

    .name-validator-valuemissing = Bitte gebe einen Namen für den Spieler an

## List integration tab
list-integration = Listenintegration

claimed-player = Spieleranspruch
    .verified = Verifiziert
    .unverified = Nicht verifiziert

# .info-api-link is turned into a clickable link to the geolocation API
# pointercrate uses, and replaces { $info-api-link }
claim-geolocate = Flagge geolocaten
    .info = Durch das Drücken des Knopfes, lässt sich die Flagge des ausgewählten Spielers IP-Geolocaten. Damit dies funktioniert, wird deine IP möglicherweise an Drittanbieter weitergegeben. Weitere Informationen findest du in der Datenschutzerklärung (Privacy Policy)!

    .submit = Los

    .edit-success = Setze Nationalität auf: { $nationality }
    .edit-success-subdivision = Setze Nationalität auf: { $nationality }/{ $subdivision }

claim-lock-submissions = Einreichungen sperren
    .info = Ob Einreichungen für deinen beanspruchten Spieler gesperrt werden sollen, sodass nur du Rekorde einreichen kannst (solange du mit dem Konto angemeldet bist, welche den Anspruch besitzt)

    .edit-success = Änderung erfolgreich durchgeführt

claim-records = Rekorde deines beanspruchten Spielers
    .info = Eine Liste der Rekorde deines beanspruchten Spielers, inklusive aller ausstehenden und abgelehnten Einreichungen. Nutze dies, um deine Einreichungen zu verfolgen. Durch Anklicken auf ein Rekord werden alle öffentlichen Notizen angezeigt, die ein Listenmoderator hinterlassen hat. Die Hintergrundfarbe eines Rekordes zeigt an, ob dieser { $record-approved-styled }, { $record-submitted-styled }, { $record-rejected-styled } wurde, oder { $record-underconsideration-styled } ist.

    .record-notes = Notizen für Rekord ID { $record-id }:
    .record-notes-none = Keine öffentlichen Notizen vorhanden!

claim-manager = Ansprüche verwalten
    .info-a = Verwalte deine Ansprüche hier. Die Liste kann mithilfe der Felder auf der rechten Seite entweder nach Spieler oder Benutzer gefiltert werden. Ungültige Ansprüche sollten mit dem Papierkorbsymbol gelöscht werden.
    .info-b = Um einen Anspruch zu verifizieren, klicke auf das Häkchen. Verifiziere nur Ansprüche, von denen du belegen kannst, dass sie richtig sind (dies bedeutet höchstwahrscheinlich mit dem Spieler in Kontakt zu treten und zu fragen ob der Anspruch selbst gestellt wurde oder nicht).
    .info-c = Sobald ein Anspruch auf einen Spieler verifiziert wurde, werden alle anderen nicht verifizierten Ansprüche auf diesem Spieler automatisch gelöscht. Benutzer können danach, keine neuen Ansprüche auf diesen Spieler erheben.
    .info-d = Ein Anspruch mit grüner Hintergrundfarbe ist verifiziert, ein Anspruch mit blauem Hintergrund ist nicht verifiziert bzw. ungeprüft.

    .claim-no-records = Der beanspruchte Spieler ({ $player-id }) hat keinen genehmigten Eintrag auf der Liste.

claim-listed-user = Anspruch von Nutzer:
claim-listed-player = Anspruch auf Spieler:

claim-initiate-panel = Anspruch einreichen
    .info = Wähle den beanspruchten Spieler aus

# { $discord } is replaced by .info-discord, which is turned into a
# clickable link to Pointercrate Central by default (this can be modified
# in pointercrate-example/src/main.rs)
claim-info-panel = Anspruch einreichen 101
    .info-a = Spielerbeanspruchung ist der Prozess, einen Demonlist Spieler mit einem Pointercrate Benutzerkonto zu verbinden.
    .info-b = Um einen Anspruch einzureichen, klicke auf den Stift links neben "Spieleranspruch". Sobald ausgewählt, besteht ein unverifizierter Anspruch auf einen Spieler. Diese Ansprüche werden anschließend manuell von Mitgliedern des Pointercrate-Teams überprüft. Die Überprüfung kann im Discord-Server beantragt werden.
    .info-c = Du kannst keinen Anspruch auf einen Spieler erheben, welcher bereits einen verifizierten Anspruch von einem anderen Benutzer hat.

    .info-discord = dieser Discord-Server

claim-video-panel = Rekordvideo
    .info = Durch Anklicken eines Anspruchs in "Ansprüche verwalten" wird ein zufällig ausgewähltes Video eines angenommenen Rekords des Spielers angezeigt.