statsviewer = Stats anschauen
    .rank = { $list } Rang
    .score = { $list } Punkte
    .stats = { $list } Stats
    .hardest = Schwerster Demon

    .completed = Demons geschafft
    .completed-main = Hauptliste Demons geschafft
    .completed-extended = Erweiterte Liste Demons geschafft
    .completed-legacy = Legacy Liste Demons geschafft

    .created = Demons erstellt
    .published = Demons veröffentlich
    .verified = Demons verifiziert
    .progress = Fortschritt auf

    .stats-value = { $main } Haupt, { $extended } Erweitert, { $legacy } Legacy
    .value-none = Keine

statsviewer-individual = Individuell
    .welcome = Klicke auf einen Benutzernamen links um anzufangen!

    .option-international = International

statsviewer-nation = National
    .welcome = Klicke auf einen Land links um anzufangen!

    .players = Spieler
    .unbeaten = Nicht geschaffte Demons

    .created-tooltip = (Co-)erstellt von { $players } { $players ->
            [one] player
            *[other] players
        } in diesem Land:
    .published-tooltip = Veröffentlicht von:
    .verified-tooltip = Verifiziert von:
    .beaten-tooltip = Geschafft von { $players } { $players ->
            [one] player
            *[other] players
        } in diesem Land:
    .progress-tooltip = Fortschritt von { $players } { $players ->
            [one] player
            *[other] players
        } in diesem Land:

demon-sorting-panel = Demonreihenfolge
    .info = Die Reihenfolge der hier aufgelisteten Demons

    .option-alphabetical = Alphabetisch
    .option-position = Position

continent-panel = Kontinent
    .info = Wähle einen Kontinent aus, um die Stats darauf zu fokussieren. Wähle 'Alle' aus um die Auswahl zurückzusetzen.

    .option-all = Alle

    .option-asia = Asien
    .option-europe = Europa
    .option-australia = Australien
    .option-africa = Afrika
    .option-northamerica = Nordamerika
    .option-southamerica = Südamerika
    .option-centralamerica = Mittelamerika

toggle-subdivision-panel = Zeige Unterteilungen
    .info = Ob die Karte politische Unterteilungen darstellen soll.

    .option-toggle = Zeige Unterteilungen

# { $countries } will be replaced with .info-countries, which will be
# turned into a tooltip listing all of the selectable countries
subdivision-panel = Politische Unterteilung
    .info = Für die { $countries } kannst du einen Staat / Provinz / Land auswählen, um die Stats darauf zu fokussieren.
    .info-countries = folgenden Länder

    .option-none = Keine
