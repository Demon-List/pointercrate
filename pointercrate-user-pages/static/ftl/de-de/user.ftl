user-username = Benutzername:
user-displayname = Anzeigename:
    .none = Nichts
user-id = Benutzer ID:

user-permissions = Berechtigungen:
    .moderator = Moderator
    .administrator = Administrator

    .list-helper = Listenhelfer
    .list-moderator = Listenmoderator
    .list-administrator = Listenadministrator

## Auth input fields
auth-username = Benutzername:
    .validator-valuemissing = Benutzername erforderlich
    .validator-tooshort = Benutzername zu kurz. Es muss mindestens 3 Zeichen lang sein.
    .error-alreadytaken = Dieser Benutzername ist bereits vergeben. Bitte wähle einen anderen.
auth-password = Passwort:
    .validator-valuemissing = Passwort benötigt
    .validator-tooshort = Password zu kurz. Es muss mindestens 10 Zeichen lang sein.
auth-repeatpassword = Password wiederholen:
    .validator-notmatching = Passwords stimmen nicht überein!

## Login/registration forms
#
# The .redirect-link attributes will be turned into
# clickable link, which will replace { $redirect-link }
# in the .redirect attributes
# If you have linked your pointercrate account with a Google account, you must sign in via Google oauth by clicking the button below:
login = Einloggen
.oauth-info = Solltest du deinen Pointercrate Konto mit einem Google Konto verlinkt haben, musst du dich mit dem unteren Google oauth Knopf einloggen:

    .methods-separator = sonst

    .info = Logge dich mit Benutzername und Passwort ein. Loginversuche sind auf 3 alle 30 Minuten beschränkt.
    .submit = Einloggen

    .error-invalidcredentials = Falsche Login Daten

    .redirect = Hast du bereits ein Pointercrate Konto? { $redirect-link } stattdessen.
    .redirect-link = Einloggen

register = Registrierung
    .info = Erstelle ein neues Konto. Bitte achte darauf, dass der Benutzername nach der Registrierung NICHT mehr geändert werden kann!
    .submit = Registriere

    .redirect = Noch kein Pointercrate Konto? { $redirect-link } für eins!
    .redirect-link = Registriere

register-oauth = Wähle deinen Benutzername:
    .submit = Registriere!

## Users tab
users = Benutzer

user-viewer = Pointercrate Kontoverwaltung
    .welcome = Klicke auf einen User links um anzufangen!
    .delete-user = Benutzer löschen
    .edit-user = Benutzer bearbeiten

    .edit-success = Benutzer erfolgreich bearbeitet!
    .edit-notmodified = Keine Änderungen vorgenommen!
    .delete-success = Benutzer erfolgreich gelöscht!

    .own-account = Dies ist dein Konto. Du kannst dein eigenes Konto mithilfe diesem Dashboard nicht ändern!

user-listed = ID: { $user-id }
    .displayname = Anzeigename:

user-idsearch-panel = Benutzer suchen
    .info = Benutzer können eindeutig durch Name und ID gefunden werden. Um ein Benutzerkonto zu ändern, brauchst du ihre ID. Kennst du weder noch, suche in der Liste unten!
    .id-field = Benutzer ID:

    .submit = Per ID suchen

    .id-validator-valuemissing = Benutzer ID benötigt

## Profile tab
profile = Profil
    .header = Profil - {$username}

profile-username = Benutzername
    .info = Der Name unter dem zu registriert bist und dich bei Pointercrate mit einloggst. Dieser Name ist einzigartig und kann nicht verändert werden.

profile-display-name = Anzeigename
    .info = Falls gesetzt, wird dieser Name anstatt dein Benutzername gezeigt. Anzeigenamen sind nicht einzigartig und können nicht zum einloggen verwendet werden.

    .dialog-header = Anzeigename ändern
    .dialog-newname = Neuer Anzeigename:

    .dialog-submit = Ändern

profile-youtube = YouTube Kanal
    .info = Ein Link zu deinem YouTube Kanal, falls du einen hast. Falls gesetzt, werden alle Erwähnungen deines Names zu deinem YouTube Kanal umgeleitet.

    .dialog-header = Youtube Kanal Link ändern
    .dialog-newlink = Neuer YouTube Kanal Link:

    .dialog-submit = Ändern

    .newlink-validator-typemismatch = Bitte gebe eine gültige URL ein

profile-permissions = Berechtigungen
    .info = Die Berechtigungen, welche du auf Pointercrate hast. 'Listen...' bedeutet, dass du Teil des Demonlist Teams bist. 'Moderator' und 'Administrator' bedeutet, dass du Teil des Pointercrate Mitarbeiterteam bist.

profile-delete-account = Konto löschen
    .dialog-header = Konto löschen
    .dialog-info = Dein Konto zu löschen ist unwiderruflich!
    .dialog-submit = Löschen

profile-change-password = Passwort ändern
    .dialog-header = Passwort ändern
    .dialog-info = Um Änderungen an deinem Profil vorzunehmen, musst du dein Passwort erneut eingeben. Falls du dein Passwort änderst, musst du dich neu anmelden. Desweiteren werden alle deine Zugrifftokens ungültg.

    .dialog-newpassword = Neues Passwort:
    .dialog-repeatnewpassword = Passwort wiederholen:
    .dialog-authenticate = Authentifiziere:

    .dialog-submit = Ändern

    .authenticate-validator-valuemissing = Passwort benötigt
    .authenticate-validator-tooshort = Password zu kurz. Es muss mindestens 10 Zeichen lang sein.

    .newpassword-validator-tooshort = Password zu kurz. Es muss mindestens 10 Zeichen lang sein.

    .repeatnewpassword-validator-tooshort = Password zu kurz. Es muss mindestens 10 Zeichen lang sein.
    .repeatnewpassword-validator-notmatching = Passwörter stimmen nicht überein!

profile-logout = Ausloggen
    .info = Logge dich aus deinem Pointercrate Konto von diesem Browser aus.
    .button = Ausloggen

profile-get-token = Erhalte Zugriffstoken
    .info = Dein Zugriffstoken erlaubt dir (oder authentifizierte Programme), API Anfragen zu stellen. Sie erlauben jedoch keine Änderungen an deinem Konto.
    .button = Erhalte Zugriffstoken

    .view-header = Dein Zugriffstoken ist

profile-invalidate-tokens = Token ungültig machen
    .info = Sollten deine Zugriffstoken veröffentlicht worden sein, kannst du sie hier ungültig machen. Das ungültig machen all deiner Zugriffstoken, lässt sie aufhören zu funktionieren. Du musst dich hiernach erneut einloggen!
    .button = Mache alle Zugriffstoken ungültig

profile-oauth = Mit Google verlinken
    .info = Aktiviere das Einloggen in dein Konto via Google oauth. Google oauth ist sicherer als das herkömmliche einloggen mit deinem Passwort und präventiert das Vergessen und die subsequente Aussperrung aus deinem Konto. Ein Google Account zu verlinken ist unwiderruflich und kann später NICHT geändert werden!
