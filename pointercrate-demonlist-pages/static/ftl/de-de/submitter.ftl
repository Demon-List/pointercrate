submitter-banned = Gebannt
    .yes = Ja
    .no = Nein

## Record submitter
record-submission-panel = Rekorde einreichen
  .info = Bitte reiche keinen Blödsinn ein, das macht es für uns alle nur schwieriger und führt potenziell zu einem Ausschluss von der Demonlist. Beachte ebenfalls, dass das Formula mehrfach gestellte Rekorde automatisch ablehnt.
  .button = Rekord einreichen!

# .note will prefix all notes in the record submission panel
# (not to be confused with record notes)
#
# { $guidelines-link } will be replaced by .guidelines-link,
# which is turned into a clickable link to the submission guidelines
record-submission = Rekord einreichen
    .note = Beachte

    .demon = Demon
    .demon-info = Der Demon, auf dem der Rekord erreicht wurde. Nur Demons in der Top {$list-size} werden angenommen. Dies schließt Legacy Demons aus!

    .demon-validator-valuemissing = Bitte gebe einen Demon an

    .holder = Rekordhalter
    .holder-info = Der Rekordhalter. Beginn mit der Eingabe, um Vorschläge für bestehende Spieler zu sehen. Sollte dies deine erste Einreichung sein, trage deinen Namen so in das Textfeld ein, wie er auf der Webseite erscheinen soll (und ignoriere dabei alle Vorschläge)

    .holder-input-placeholder = Fange an zu schreiben...

    .holder-validator-valuemissing = Aufgrund von Geometry Dash's technischen Einschränkungen, weiß ich, dass keiner einen so langen Namen hat

    .progress = Fortschritt
    .progress-info = Fortschritt als Prozentanzahl. Nur Werte größer oder gleich der Mindestanforderungen und kleiner oder gleich 100 werden akzeptiert!
    .progress-placeholder = z.B. '50', '98'

    .progress-validator-valuemissing = Bitte gebe den Rekordfortschritt an
    .progress-validator-rangeunderflow = Rekordfortschritt kann nicht negativ sein
    .progress-validator-rangeoverflow = Rekordfortscchritt kann nicht größer als 100% sein
    .progress-validator-badinput = Rekordfortschritt muss eine ganze Zahl sein
    .progress-validator-stepmismatch = Rekordfortschritt darf keine Kommazahl sein

    .video = Video
    .video-info = Ein Beweisvideo, dass die Legitimität des Rekords bestätigt. Wurde der Rekord in einem Stream erzielt, aber nirgendwo anders hochgeladen, gebe bitte einen Twitch-Link zu diesem Stream an
    .video-note = Achte bitte darauf, nur gültige URL's einzureichen!
    .video-placeholder = z.B. https://youtu.be/cHEGAqOgddA

    .video-validator-valuemissing = Bitte gebe ein Video an
    .video-validator-typemismatch = Bitte gebe eine valide Video URL an

    .raw-footage = Rohaufnahmen
    .raw-footage-info-a = Das unbearbeitet und ungeschittene Video für diesen Rekord, auf ein nicht-komprimierenden Host wie Google Drive hochgeladen (also nicht YouTube!). Hinweis: Spielern kann eine einmalige Ausnahme gewährt werden, sodass ein Stream-VOD als Rohaufnahme gilt.
    .raw-footage-info-b = Alle in den Rohaufnahmen möglicherweise enthaltenen personenbezogenen Daten (z.B. Namen, vertrauliche Gespräche) werden streng vertraulich behandelt und nicht außerhalb des Demonlist-Teams geteilt. Du erkennst an, dass durch die Bereitstellung von der Rohaufnahme unbeabsichtigt solche Informationen weitergegeben werden könnten. Du hast das Recht, die Löschung deines Rekordes zu beantragen, indem du dich an einen Listenadministrator wendest.
    .raw-footage-note = Dies ist für jede Einreichung in die Liste erforderlich!

    .raw-footage-validator-typemismatch = Bitte gebe eine valide URL an

    .notes = Notizen oder Kommentarde
    .notes-info = Füge jegliche zusätzliche Anmerkungen hinzu, die du dem Listenmoderator mitteilen möchstest.
    .notes-placeholder = Deine Hoffnungen und Träume für diese Einreichung... oder so

    .guidelines = Mit der Einreichung dieses Rekordes, nimmst du die { $guidelines-link } wahr.
    .guidelines-link = Einreichungsrichtlinien

    .submit = Rekord einreichen

    .submission-success = Rekord wurde erfolgreich eingereicht.
    .submission-success-queue = Rekord wurde erfolgreich eingereicht. Es ist Platz { $queue-position } in der Warteschlange!

## Submitters tab
submitters = Einreichen

submitter-manager = Einreicherverwaltung

submitter-viewer = Einreicher #
    .welcome = Klicke links auf einen Einreicher um anzufangen!

    .info-a = Willkommen zum Einreicherverwalter. Hier kannst du Einreicher bannen / entbannen mit einer absolut revolutionären Benutzeroberfläche, welche definitiv KEINE Kopie der Spielerverwaltung (aber noch leerer) ist.
    .info-b = Einen Einreicher bannen, löscht alle seine Rekorde, welche er bereits eingereicht hat und sich im Status "Eingereicht" befinden. Jegliche Rekorde im Bereich "Angenommen", "Abgelehnt" oder "Ausstehend" bleiben unberührt.

    .records-redirect = Rekord in Rekordverwaltung anzeigen

submitter-listed = Einreicher #{ $submitter-id }

submitter-idsearch-panel = Einricher nach ID suchen
    .info = Einreicher können durch ihre ID eindeutig identifiziert werden. Durch die Eingabe der Einreicher ID wird dieser links ausgewählt (sofern er existiert)
    .id-field = Einreicher ID:

    .submit = Finde nach ID

    .id-validator-valuemissing = Einricher ID erforderlich