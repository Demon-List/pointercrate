demon-name = Demon Name
    .validator-valuemissing = Bitte spezifiziere einen Namen

demon-password = Level Passwort

demon-id = Level ID
    .validator-rangeunderflow = Level ID muss positiv sein

demon-length = Level Länge

demon-objects = Objektanzahl

demon-difficulty = Schwierigkeitsgrad

demon-gdversion = Erstellt in

demon-ngsong = Newgrounds Song

demon-score = Demonlist Punkte ({$percent}%)

demon-video = Verifizierungsvideo
    .validator-typemismatch = Bitte gebe eine gültige URL ein

demon-thumbnail = Thumbnail
    .validator-typemismatch = Bitte gebe eine gültige URL ein
    .validator-valuemissing = Bitte gebe eine URL ein

demon-position = Position
    .validator-rangeunderflow = Demon Position muss mindestens 1 sein
    .validator-badinput = Demon Position muss eine gültige Zahl sein
    .validator-stepmismatch = Demon Position darf keine Kommazahl sein
    .validator-valuemissing = Bitte gebe eine Position an

# still very icky about this word
demon-requirement = Mindestanforderung
    .validator-rangeunderflow = Mindestanforderung darf nicht negativ sein
    .validator-rangeoverflow = Mindestanforderung darf nicht größer als 100% sein
    .validator-badinput = Mindestanforderung muss eine gültige Zahl sein
    .validator-stepmismatch = Mindestanforderung darf keine Kommazahl sein
    .validator-valuemissing = Bitte gebe eine Mindestanforderung an

demon-publisher = Veröffentlicher
    .validator-valuemissing = Bitte gebe einen Veröffentlicher an

demon-verifier = Verifier
    .validator-valuemissing = Bitte gebe eine Verifizierer an

# leave rated not translated?
demon-rated = Bewertung
    .yes = Ja (bewertet)
    .no = Nein (nicht bewertet)

demon-creators = Ersteller

demon-headline-by = von { $creator }
demon-headline-verified-by = verifiziert von { $verifier }
demon-headline-published-by = veröffentlicht von { $publisher }

demon-headline = von { $creator }
    .same-verifier-publisher = verifiziert und veröffentlicht von { $publisher }
    .unique-verifier-publisher = { demon-headline-published-by }, { demon-headline-verified-by }

    .no-creators = von Unbekannt, { $verified-and-published }

    .one-creator = { demon-headline-by }, { $verified-and-published }
    .one-creator-is-publisher = { demon-headline-by }, verifiziert von { $verifier }
    .one-creator-is-verifier = { demon-headline-by }, veröffentlicht von { $publisher }

    .two-creators = von { $creator1 } und { $creator2 }, { $verified-and-published }

    .more-creators = { demon-headline-by } und { $more }, { $verified-and-published }
    .more-creators-tooltip = mehr

movements = Positionsgeschichte
    .date = Datum
    .change = Änderung

movements-newposition = Neue Position
    .legacy = Legacy
# probably also not translatable

movements-reason = Grund
    .added = Zur Liste hinzugefügt
    .addedabove = { $demon } wurde darüber platziert
    .moved = Bewegt
    .movedabove = { $demon } wurde über diesen Demon bewegt
    .movedbelow = { $demon } wurde unter diesen Demon bewegt
    .rated = Rated
    .unrated = Nicht Rated
    .otherrated = { $demon } wurde bewertet
    .otherunrated = { $demon } wurde der Bewertung entzogen
                            # entwertet ist hier falsch
                            # genitiv spricht sich hier aber auch sehr komisch?

demon-records = Rekorde
    .none = Keine Rekorde!
    .none-yet = Keine Rekorde bisher! Sei der erste!

demon-records-qualify = {$percent}% { $percent ->
    [100] benötigt, um sich zu qualifizieren
    *[other] oder mehr, um sich zu qualifizieren
}

demon-records-total = {$num-records} { $num-records ->
    [one] Rekord registriert
    *[other] Rekorde registriert
}, von denen {$num-completions} { $num-completions ->
    [one] 100% ist
    *[other] 100% sind
}

demons = Demons
demon-manager = Demon Verwaltung

demon-listed = {$demon} (ID: {$demon-id})
    .publisher = von {$publisher}

demon-viewer = Demon #
    .welcome = Klicke auf einen Demon links um anzufangen!

    .video-field = { demon-video }:
    .thumbnail-field = { demon-thumbnail }:
    .position-field = { demon-position }:
    .requirement-field = { demon-requirement }:
    .publisher-field = { demon-publisher }:
    .verifier-field = { demon-verifier }:
    .rated-field = { demon-rated }:
    .creators-field = { demon-creators }:
    .levelid-field = { demon-id }:

demon-add-panel = Demon hinzufügen
    .button = Füge einen Demon hinzu!

demon-add-form = Demon hinzufügen
    .name-field = { demon-name }:
    .name-validator-valuemissing = Bitte gebe einen Demonnamen an

    .levelid-field = Geometry Dash Level ID:
    .position-field = { demon-position } ({ list-ratedplus }):
    .requirement-field = { demon-requirement }:
    .verifier-field = { demon-verifier }:
    .publisher-field = { demon-publisher }:
    .video-field = { demon-video }:
    .rated-field = { demon-rated }
    .creators-field = { demon-creators }:

    .submit = Demon hinzufügen

    .edit-success = Demon erfolgreich hinzugefügt!

demon-video-dialog = Verifikationsvideo ändern
    .info = Ändere das Verifikationsvideo für diesen Rekord. Lasse das Feld leer um es zu entfernen.
    .video-field = Video link:
    .submit = Bearbeite

demon-name-dialog = Demon Namen ändern
    .info = Ändere den Demon Namen. Mehrere Demons mit den selben Namen SIND unterstützt!
    .name-field = Name:
    .submit = Bearbeite

demon-thumbnail-dialog = Thumbnail ändern
    .info = Ändere den Thumbnail Link für diesen Rekord. Um es auf ein Youtube-Thumbnail zu ändern, setze es zu { $video-id }.
    .info-videoid = VIDEO_ID

    .thumbnail-field = Thumbnail link:
    .submit = Bearbeite

demon-position-dialog = Position ändern
    .info = Ändere die Position vom Demon. Muss größer als 0 und mindestens so groß wie die zurzeitige Listengröße sein.
    .position-field = Position ({ list-ratedplus }):
    .submit = Bearbeite

demon-requirement-dialog = Mindestanforderung ändern
    .info = Ändere die Mindestanforderung für diesen Demon. Muss zwischen 0 und 100 (einschließlich) liegen.
    .requirement-field = Mindestanforderung:
    .submit = Bearbeite

demon-publisher-dialog = Veröffentlicher ändern
    .info = Schreibe den neuen Veröffentlicher in das untere Textfeld. Sollte der Spieler bereits existieren, so wird er dir vorgeschlagen und du kannst draufdrücken.
    .submit = Bearbeite

demon-verifier-dialog = Verifizierer ändern
    .info = Schreibe den neuen Verifizierer in das untere Textfeld. Sollte der Spieler bereits existieren, so wird er dir vorgeschlagen und du kannst draufdrücken.
    .submit = Bearbeite

demon-levelid-dialog = Level ID ändern
    .info = Leer lassen um die Level ID zu entfernen.
    .submit = Bearbeite

demon-creator-dialog = Ersteller hinzufügen
    .info = Schreibe den neuen Ersteller in das untere Textfeld. Sollte der Spieler bereits existieren, so wird er dir vorgeschlagen und du kannst draufdrücken.
    .submit = Ersteller hinzufügen

    .edit-success = Ersteller erfolgreich hinzugefügt!