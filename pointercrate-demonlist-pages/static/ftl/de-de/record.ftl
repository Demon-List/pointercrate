## Commonly referenced record data
record-submitted = Eingereicht
record-underconsideration = Ausstehend
record-approved = Angenommen
record-rejected = Abgelehnt

record-videolink = Videolink
record-videoproof = Videobeweis
record-rawfootage = Rohaufnahme
record-demon = Demon
record-holder = Rekordhalter
record-progress = Fortschritt
record-submitter = Einreicher ID
record-date = Einreichungsdatum

## Records tab (user area)
records = Rekorde
record-manager = Rekordverwalter
    .all-option = Alle Demons

record-listed = Rekord #{ $record-id }
    .progress = { $percent }% auf { $demon }

record-viewer = Rekord #
    .welcome = Klicke auf einen Rekord links um anzufangen!
    .delete = Rekord löschen

    .copy-data-success = Rekorddaten in Zwischenablage kopiert!
    .copy-data-error = Fehler beim Kopieren in Zwischenablage

    .confirm-delete = Bist du sicher? Der Rekord und seine Notizen werden unwiderruflich gelöscht!

record-note = Notiz hinzufügen
    .placeholder = Füge eine Notiz hier hinzu. Drücke auf 'Hinzufügen' wenn fertig!
    .public-checkbox = Öffentliche Notiz

    .submit = Hinzufügen

record-note-listed = Rekordnotiz #{ $note-id }
    .confirm-delete = Diese Aktion löscht die Notiz unwiderruflich. Fortfahren?

    .author = Diese Notiz wurde von { $author } verfasst.
    .author-submitter = Diese Notiz wurde als Kommentar vom Einreicher verfasst.
    .editors = Die Notiz wurde daraufhin von folgenden Personen bearbeitet: { $editors }.
    .transferred = Diese Notiz wurde ursprünglich nicht auf diesem Rekord hinterlassen.
    .public = Diese Notiz ist public.

record-status-filter-panel = Filter
    .info = Filter nach Rekordstatus

record-status-filter-all = Alle

record-idsearch-panel = Rekord nach ID suchen
    .info = Rekorde können durch ihre ID eindeutig identifiziert werden. Durch die Eingabe der Rekord ID wird dieser links ausgewählt (sofern er existiert)
    .id-field = Rekord ID:

    .submit = Finde nach ID

    .id-validator-valuemissing = Rekord ID erforderlich

record-playersearch-panel = Nach Spieler filtern
    .info = Spieler können durch ihre ID und ihren Namen eindeutig identifiziert werden. Durch Eingabe eines dieser Felder an der entsprechenden Stelle wird die Ansicht links gefiltert. "Finde ..." setzt die Ansicht zurück, sofern das Textfeld leer ist.

    .id-field = Spieler ID:
    .id-submit = Finde nach ID

    .name-field = Spielername:
    .name-submit = Finde nach Name

# Record viewer dialogs
record-videolink-dialog = Videolink ändern
    .info = Ändere den Videolink für diesen Rekord. Beachte, dass du als Listenmoderator, das Textfeld leer lassen kannst, um das Video von diesem Rekord zu entfernen.
    .videolink-field = Videolink:

    .submit = Ändern

    .videolink-validator-typemismatch = Gebe eine gültige URL ein

record-demon-dialog = Rekorddemon ändern
    .info = Ändere den assoziierten Demon von diesem Rekord. Suche den Demon unten, dem dieser Rekord zugeordnet werden sollte. Klicke es, um den Rekord zu bearbeiten

record-holder-dialog = Rekordhalter ändern
    .info = Gebe den neuen Rekordhalter unten ein. Sollte der Spieler bereits existieren, wird er unten vorgeschlagen.
    .submit = Ändern

record-progress-dialog = Rekordfortschritt ändern
    .info = Ändere den Rekordfortschritt von diesem Rekord. Dieser muss zwischen der Mindestanforderung des Demons, und 100 (inklusive) liegen.
    .progress-field = Fortschritt:

    .submit = Ändern

    .progress-validator-rangeunderflow = Fortschritt darf nicht negativ sein
    .progress-validator-rangeoverflow = Fortschritt darf nicht größer als 100% sein
    .progress-validator-badinput = Fortschritt muss eine gültige Zahl sein
    .progress-validator-stepmismatch = Fortschritt darf keine Kommazahl sein
    .progress-validator-valuemissing = Bitte gebe einen Fortschritt an

record-date-dialog = Einreichungsdatum ändern
    .info = Ändere das Einreichungsdatum des Rekords. Dieser kann so angepasst werden, um Rekorde auf den Demonseiten anders anzuordnen.
    .date-field = Datum:

    .submit = Ändern

# The giant information box below the record manager, split
# into different sections here
#
# Each section (except .a and .b) will begin with a bolded version of
# the appropriate record state, or a bolded version of .note for .note-a/b
# attributes
#
record-manager-help = Rekorde verwalten
    .a = Verwende die Liste links, um Rekorde anzusehen / zu bearbeiten. Verwende den Bereich rechts, um nach Status, Spieler, etc. zu filtern. Durch "{ record-status-filter-all }" kann nach Demon gefiltert werden.

    .b = Ein Rekord kann sich in vier möglichen Zuständen befinden: { record-rejected }, { record-approved }, { record-submitted } und { record-underconsideration }. Der Einfachheit halber gehen wir davon aus, dass Kevin ein Spieler ist und Cataclysm ein Demon ist, auf dem er ein Rekord hat.

    .rejected = Sollte der Rekord { record-rejected } sein, heißt das, dass Kevin keinen anderen Rekord auf Cataclysm hat und Kevin keine weiteren Rekorde auf Cataclysm einreichen kann. Umgekehrt bedeutet das, dass falls Kevin einen Rekord auf Cataclysm hat, welcher nicht abgelehnt wurde, wir direkt wissen, dass kein abgelehnter Rekord für Kevin auf Cataclysm existiert.
    Sollte ein Rekord von Kevin auf Cataclysm abgelehnt werden, werden alle anderen Rekorde von Kevin auf Cataclysm gelöscht, um die oben genannte Eindeutigkeit zu gewährleisten.

    .approved = Sollte der Rekord { record-approved } sein, bedeutet das, dass keine Rekorde mit weniger Fortschritt als der { record-approved }e Rekord existieren oder erlaubt sind.
    Einen Rekord auf den Status { record-approved } zu ändern, löscht alle anderen Einreichungen für Kevin auf Cataclysm mit weniger Fortschritt.

    .submitted = Ist der Rekord { record-submitted }, bestehen keine weiteren Einschränkungen hinsichtlich der Eindeutigkeit. Dies bedeutet, dass mehrere Einreichungen von Kevin auf Cataclysm möglich sind, solange diese alle verschiedene Videolinks besitzen. Aus den oben genannten Gründen, werden jedoch alle Duplikate gelöscht, sobald einer der Einreichungen angenommen oder abgelehnt wurde.

    .underconsideration = Sollte der Rekord noch { record-underconsideration } sein, ist er im Prinzip immer noch eine Einreichung. Jedoch sind keine weiteren Einreichungen von Kevin auf Cataclysm erlaubt.

    .note = Beachte

    .note-a = Sollte ein Spieler gebannt sein, darf er keine { record-approved }en / { record-submitted }e Rekorde haben. Alle Rekorde mit dem Status { record-submitted } werden gelöscht, und alle anderen werden auf { record-rejected } verändert.

    .note-b = Ein Einreicher zu bannen, löscht alle seine Einreichungen mit dem Status { record-submitted }. Rekorde, welche von ihm mit dem Status { record-approved }/{ record-rejected } eingereicht wurden, bleiben unberührt.