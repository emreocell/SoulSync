//! User-facing CLI text in English, German and Spanish.

use soulsync_core::Lang;

pub struct Text {
    pub usage: &'static str,
    pub trainer: &'static str,
    pub badges: &'static str,
    pub party: &'static str,
    pub boxes: &'static str,
    pub slot_party: &'static str,
    pub slot_box: &'static str,
    pub level: &'static str,
    pub hp: &'static str,
    pub met: &'static str,
    pub location: &'static str,
    pub egg: &'static str,
    pub error: &'static str,
    pub watching: &'static str,
    pub ev_caught: &'static str,
    pub ev_fainted: &'static str,
    pub ev_evolved: &'static str,
    pub ev_level: &'static str,
    pub ev_to_party: &'static str,
    pub ev_to_box: &'static str,
    pub ev_gone: &'static str,
    pub ev_encountered: &'static str,
    pub ev_badges: &'static str,
    pub run_created: &'static str,
    pub run_share: &'static str,
    pub connecting: &'static str,
    pub connected: &'static str,
    pub disconnected: &'static str,
    pub sent: &'static str,
    pub server_error: &'static str,
    pub generation: &'static str,
    pub rom_unknown: &'static str,
    pub rom_unsupported: &'static str,
    pub rom_rates: &'static str,
    pub rom_rates_failed: &'static str,
}

const EN: Text = Text {
    usage: "\
SoulSync: track Pokémon Soul Link runs live

Usage:
  soulsync inspect <save> [--json] [--lang en|de|es]   Show party, boxes and badges
  soulsync watch   <save> [--json] [--lang en|de|es]   Watch the save and report changes
  soulsync rom     <rom> [--json]                      Tell which game a ROM is and check its catch rates
  soulsync new     [--server <url>]                    Create a run and get its code
  soulsync sync    <save> --code <code> --name <name> [--rom <rom>] [--server <url>]
                                                       Sync your save with the run

--rom reads the catch rates from your own ROM (a randomizer may have changed
them). The ROM stays on your machine.

<save> is the .dsv from DeSmuME's Battery folder, or a .sav file.
Supported: Pokémon Black 2 / White 2.
Server: --server, or SOULSYNC_SERVER (default http://localhost:3000).\nLanguage: --lang, or the SOULSYNC_LANG / LANG environment variables.",
    trainer: "Trainer",
    badges: "badges",
    party: "Party",
    boxes: "Boxes",
    slot_party: "Party",
    slot_box: "Box",
    level: "Lv",
    hp: "HP",
    met: "met at",
    location: "location",
    egg: "egg",
    error: "Error",
    watching: "Watching {path} ({party} in party, {boxes} in boxes). Save in-game to see changes. Ctrl+C to stop.",
    ev_caught: "New",
    ev_fainted: "Fainted (0 HP)",
    ev_evolved: "Evolved",
    ev_level: "is now Lv",
    ev_to_party: "joined the party",
    ev_to_box: "went to box",
    ev_gone: "No longer in the save",
    ev_encountered: "First encounter",
    ev_badges: "Badges",
    run_created: "Run created: {code}",
    run_share: "Send this code to your partner. Both of you then run:\n  soulsync sync <save> --code {code} --name <your name>{server}\nDashboard: {dashboard}",
    connecting: "Connecting to {url} ...",
    connected: "Connected as {name}. Watching {path}. Save in-game to sync. Ctrl+C to stop.",
    disconnected: "Connection lost, retrying in {secs} s ...",
    sent: "Synced",
    server_error: "Server",
    generation: "Generation",
    rom_unknown: "this does not look like a Nintendo DS ROM",
    rom_unsupported: "not supported yet",
    rom_rates: "Catch rates read: {changed} of {total} differ from the original game",
    rom_rates_failed: "Could not read catch rates from the ROM, using the original values",
};

const DE: Text = Text {
    usage: "\
SoulSync: Soul-Link-Runs live verfolgen

Benutzung:
  soulsync inspect <spielstand> [--json] [--lang en|de|es]   Team, Boxen und Orden anzeigen
  soulsync watch   <spielstand> [--json] [--lang en|de|es]   Spielstand beobachten und Änderungen melden
  soulsync rom     <rom> [--json]                            Spiel einer ROM erkennen und Fangraten prüfen
  soulsync new     [--server <url>]                          Run erstellen und Code erhalten
  soulsync sync    <spielstand> --code <code> --name <name> [--rom <rom>] [--server <url>]
                                                             Spielstand mit dem Run synchronisieren

--rom liest die Fangraten aus deiner eigenen ROM (ein Randomizer kann sie
geändert haben). Die ROM bleibt auf deinem Rechner.

<spielstand> ist die .dsv aus dem Battery-Ordner von DeSmuME oder eine .sav.
Unterstützt: Pokémon Schwarz 2 / Weiß 2.
Server: --server oder SOULSYNC_SERVER (Standard http://localhost:3000).\nSprache: --lang oder die Umgebungsvariablen SOULSYNC_LANG / LANG.",
    trainer: "Trainer",
    badges: "Orden",
    party: "Team",
    boxes: "Boxen",
    slot_party: "Team",
    slot_box: "Box",
    level: "Lv",
    hp: "KP",
    met: "gefangen in",
    location: "Ort",
    egg: "Ei",
    error: "Fehler",
    watching: "Beobachte {path} ({party} im Team, {boxes} in Boxen). Speichere im Spiel, um Änderungen zu sehen. Strg+C beendet.",
    ev_caught: "Neu",
    ev_fainted: "Besiegt (0 KP)",
    ev_evolved: "Entwicklung",
    ev_level: "ist jetzt Lv",
    ev_to_party: "ist jetzt im Team",
    ev_to_box: "liegt jetzt in Box",
    ev_gone: "Nicht mehr im Spielstand",
    ev_encountered: "Erste Begegnung",
    ev_badges: "Orden",
    run_created: "Run erstellt: {code}",
    run_share: "Schick den Code deinem Partner. Danach startet ihr beide:\n  soulsync sync <spielstand> --code {code} --name <dein Name>{server}\nDashboard: {dashboard}",
    connecting: "Verbinde mit {url} ...",
    connected: "Verbunden als {name}. Beobachte {path}. Speichere im Spiel, um zu synchronisieren. Strg+C beendet.",
    disconnected: "Verbindung verloren, neuer Versuch in {secs} s ...",
    sent: "Synchronisiert",
    server_error: "Server",
    generation: "Generation",
    rom_unknown: "das sieht nicht nach einer Nintendo-DS-ROM aus",
    rom_unsupported: "noch nicht unterstützt",
    rom_rates: "Fangraten gelesen: {changed} von {total} weichen vom Originalspiel ab",
    rom_rates_failed: "Fangraten konnten nicht aus der ROM gelesen werden, es gelten die Originalwerte",
};

const ES: Text = Text {
    usage: "\
SoulSync: sigue en directo tus runs de Pokémon Soul Link

Uso:
  soulsync inspect <guardado> [--json] [--lang en|de|es]   Muestra equipo, cajas y medallas
  soulsync watch   <guardado> [--json] [--lang en|de|es]   Vigila el guardado y avisa de los cambios
  soulsync rom     <rom> [--json]                          Detecta qué juego es una ROM y comprueba sus ratios de captura
  soulsync new     [--server <url>]                        Crea una run y da su código
  soulsync sync    <guardado> --code <código> --name <nombre> [--rom <rom>] [--server <url>]
                                                          Sincroniza tu guardado con la run

--rom lee los ratios de captura de tu propia ROM (un randomizer puede haberlos
cambiado). La ROM se queda en tu ordenador.

<guardado> es el .dsv de la carpeta Battery de DeSmuME, o un archivo .sav.
Compatible: Pokémon Negro 2 / Blanco 2.
Servidor: --server, o SOULSYNC_SERVER (por defecto http://localhost:3000).\nIdioma: --lang, o las variables de entorno SOULSYNC_LANG / LANG.",
    trainer: "Entrenador",
    badges: "medallas",
    party: "Equipo",
    boxes: "Cajas",
    slot_party: "Equipo",
    slot_box: "Caja",
    level: "Nv",
    hp: "PS",
    met: "capturado en",
    location: "lugar",
    egg: "huevo",
    error: "Error",
    watching: "Vigilando {path} ({party} en el equipo, {boxes} en las cajas). Guarda en el juego para ver los cambios. Ctrl+C para salir.",
    ev_caught: "Nuevo",
    ev_fainted: "Debilitado (0 PS)",
    ev_evolved: "Evolución",
    ev_level: "ahora es Nv",
    ev_to_party: "ahora está en el equipo",
    ev_to_box: "ahora está en la caja",
    ev_gone: "Ya no está en el guardado",
    ev_encountered: "Primer encuentro",
    ev_badges: "Medallas",
    run_created: "Run creada: {code}",
    run_share: "Envía este código a tu compañero. Después, cada uno ejecuta:\n  soulsync sync <guardado> --code {code} --name <tu nombre>{server}\nPanel: {dashboard}",
    connecting: "Conectando con {url} ...",
    connected: "Conectado como {name}. Vigilando {path}. Guarda en el juego para sincronizar. Ctrl+C para salir.",
    disconnected: "Conexión perdida, reintentando en {secs} s ...",
    sent: "Sincronizado",
    server_error: "Servidor",
    generation: "Generación",
    rom_unknown: "esto no parece una ROM de Nintendo DS",
    rom_unsupported: "aún no es compatible",
    rom_rates: "Ratios de captura leídos: {changed} de {total} difieren del juego original",
    rom_rates_failed: "No se pudieron leer los ratios de captura de la ROM; se usan los valores originales",
};

pub fn text(lang: Lang) -> &'static Text {
    match lang {
        Lang::En => &EN,
        Lang::De => &DE,
        Lang::Es => &ES,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every placeholder the English text uses must survive translation,
    /// otherwise the command would print a literal `{code}` or lose a value.
    #[test]
    fn translations_keep_placeholders() {
        let en = text(Lang::En);
        for lang in [Lang::De, Lang::Es] {
            let tr = text(lang);
            for (name, a, b) in [
                ("watching", en.watching, tr.watching),
                ("run_created", en.run_created, tr.run_created),
                ("run_share", en.run_share, tr.run_share),
                ("connecting", en.connecting, tr.connecting),
                ("connected", en.connected, tr.connected),
                ("disconnected", en.disconnected, tr.disconnected),
                ("rom_rates", en.rom_rates, tr.rom_rates),
            ] {
                for ph in [
                    "{path}",
                    "{party}",
                    "{boxes}",
                    "{code}",
                    "{server}",
                    "{dashboard}",
                    "{url}",
                    "{name}",
                    "{secs}",
                    "{changed}",
                    "{total}",
                ] {
                    assert_eq!(
                        a.matches(ph).count(),
                        b.matches(ph).count(),
                        "{lang:?} {name} {ph}"
                    );
                }
            }
        }
    }

    #[test]
    fn usage_lists_every_language() {
        for lang in [Lang::En, Lang::De, Lang::Es] {
            assert!(text(lang).usage.contains("--lang en|de|es"), "{lang:?}");
        }
    }
}
