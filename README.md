# ShortCheat

Dieses Repository enthält meine persönlichen Shortcuts für verschiedene Anwendungen und Umgebungen.  Es verwendet YAML-Dateien zur Datenspeicherung und ein Skript zur Generierung eines PNG-Cheat-Sheets, das ich dann per Kommandozeile ein- und ausblenden kann.

## Struktur

*   `shortcuts/`
    *   `kde.yaml` (KDE-spezifische Shortcuts)
    *   `hyprland.yaml` (Hyprland-spezifische Shortcuts)
    *   `vscode.yaml`, `firefox.yaml`, ... (Anwendungsspezifische Shortcuts)
*   `scripts/`
    *   `generate_png.py` (Python-Skript zur PNG-Generierung)
*   `cheat_sheet.png` (Das generierte Cheat-Sheet-Bild)
*   `README.md` (Diese Datei)

## YAML-Format

Die YAML-Dateien verwenden folgendes Format:

```yaml
app: "Anwendungsname"
description: "Beschreibung der Anwendung"
sections:
  - name: "Sektionstitel"
    shortcuts:
      - keys: "Ctrl + S"
        description: "Speichern"
      - keys: "Ctrl + Shift + S"
        description: "Speichern unter"
      - ...
```

*   `app`: Der Name der Anwendung.
*   `description`: Eine kurze Beschreibung der Anwendung.
*   `sections`: Eine Liste von Sektionen (z.B. "Navigation", "Bearbeitung").
*   `name`: Der Titel der Sektion.
*   `shortcuts`: Eine Liste von Shortcuts.
    *   `keys`: Die Tasten-Kombination (als Text, z.B. "Ctrl + Shift + A").
    *   `description`: Die Beschreibung der Funktion.

## PNG-Generierung (Beispiel mit Python)

Das Skript `scripts/generate_png.py` nimmt die YAML-Dateien und generiert ein PNG-Cheat-Sheet.  Hier ist ein **Beispiel** (du musst es anpassen, um es zu erstellen):

```python
# scripts/generate_png.py
# (Beispiel, muss angepasst werden!)
import yaml
from PIL import Image, ImageDraw, ImageFont

# Konfiguration
font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"  # Pfad zu deiner Schriftart
font_size = 18
bg_color = (30, 30, 30)  # Dunkles Hintergrundgrau
text_color = (255, 255, 255)  # Weißer Text
padding = 10
line_height = font_size + padding
image_width = 800
image_height = 0  # Wird dynamisch berechnet

def load_shortcuts(yaml_file):
    with open(yaml_file, 'r') as f:
        return yaml.safe_load(f)

def generate_image(shortcuts_data, output_file):
    global image_height
    image_height = padding # Startpadding
    for app_data in shortcuts_data:
        image_height += line_height * 2 # App-Titel
        for section in app_data['sections']:
            image_height += line_height # Sektionsüberschrift
            image_height += len(section['shortcuts']) * line_height # Shortcuts
        image_height += padding # Zwischenraum
    
    image = Image.new("RGB", (image_width, image_height), bg_color)
    draw = ImageDraw.Draw(image)
    font = ImageFont.truetype(font_path, font_size)

    y_offset = padding # Startpadding
    for app_data in shortcuts_data:
        # App-Titel
        draw.text((padding, y_offset), app_data['app'], fill=text_color, font=font)
        y_offset += line_height
        for section in app_data['sections']:
            # Sektionsüberschrift
            draw.text((padding + 20, y_offset), section['name'], fill=text_color, font=font)
            y_offset += line_height
            for shortcut in section['shortcuts']:
                # Shortcuts
                draw.text((padding + 40, y_offset), f"{shortcut['keys']}: {shortcut['description']}", fill=text_color, font=font)
                y_offset += line_height
        y_offset += padding # Zwischenraum
                
    image.save(output_file)

# Hauptteil
shortcuts_data = []
shortcuts_data.append(load_shortcuts('shortcuts/kde.yaml'))
shortcuts_data.append(load_shortcuts('shortcuts/hyprland.yaml'))
shortcuts_data.append(load_shortcuts('shortcuts/vscode.yaml'))
# ... mehr laden ...

generate_image(shortcuts_data, "cheat_sheet.png")

```

**Wichtiger Hinweis:**  Dies ist ein *Grundgerüst*.  Du musst das Skript anpassen, um die YAML-Dateien zu parsen, die Texte zu formatieren und das PNG zu erstellen (z.B. mit der Bibliothek `Pillow`).  Du musst die Pfade zur Schriftart, Farben und das Layout anpassen.  Am besten, du startest mit einem einfachen Beispiel (nur einem Shortcut) und erweiterst es dann.  Du könntest auch eine Markdown-Bibliothek nutzen um die Tabellen direkt zu erstellen und diese dann in ein Bild umzuwandeln.

### Installation der Python Bibliotheken

```bash
pip install PyYAML Pillow
```

## Verwendung

1.  **YAML-Dateien erstellen/bearbeiten:**  Lege die YAML-Dateien in `shortcuts/` an oder bearbeite sie.
2.  **PNG generieren:**  Führe das Python-Skript aus:

    ```bash
    python scripts/generate_png.py
    ```

    Dadurch wird `cheat_sheet.png` generiert.
3.  **Cheat-Sheet anzeigen (Debian/KDE):**  In KDE, erstelle einen globalen Shortcut (Systemeinstellungen -> Shortcuts -> Globale Shortcuts), der den Befehl ausführt:

    ```bash
    pqiv -c -i -t --hide-cursor-at-bins cheat_sheet.png
    ```

    Installiere `pqiv` (falls noch nicht geschehen): `sudo apt install pqiv`.
4.  **Cheat-Sheet anzeigen (Arch/Hyprland):**  Füge in deiner Hyprland-Konfiguration (in der `~/.config/hypr/hyprland.conf`) einen Shortcut hinzu:

    ```ini
    bind = SUPER, F1, exec, pqiv -c -i -t --hide-cursor-at-bins cheat_sheet.png
    ```

    Oder einen Befehl in deinem `keyd` (oder ähnlichen Tastatur-Mapping-Tool).

## Tipps & Tricks

*   **Schriftart:**  Experimentiere mit verschiedenen Schriftarten, um die Lesbarkeit zu optimieren.
*   **Layout:** Passe die Abstände, Farben und Schriftgrößen im Python-Skript an, um das Aussehen zu verbessern.  Du kannst auch zusätzliche Elemente (z.B. Rahmen) einfügen.
*   **GitHub Actions:**  Richte eine GitHub Action ein, die das Skript bei jeder Änderung an den YAML-Dateien automatisch ausführt und das PNG aktualisiert.  So hast du immer ein aktuelles Cheat-Sheet.

## Nächste Schritte

1.  **Skript verbessern:** Verbessere das Python-Skript, um das Layout, die Formatierung und die Fehlerausgabe zu optimieren.
2.  **GitHub Actions:** Konfiguriere eine GitHub Action zur automatischen PNG-Generierung.
3.  **Erweiterungen:** Füge weitere Anwendungen und Shortcuts hinzu.
4.  **Design:**  Verfeinere das Design des Cheat-Sheets (Farben, Rahmen, Icons...).
5.  **Integration:**  Erstelle ein Skript, das dir automatisch die Konfiguration des Shortcuts (für KDE/Hyprland) in die Zwischenablage legt, um das Einrichten zu vereinfachen.
6.  **Nutze die KI:** Nutze die KI, um das Python-Skript zu optimieren und die Erstellung des Cheat Sheets zu vereinfachen.

**Dieses `README.md` ist eine Vorlage.  Passe sie an deine Bedürfnisse an und nutze die KI, um dir bei der Umsetzung zu helfen!**

**Wichtig:** Kopiere diesen Text in eine `README.md`-Datei in deinem GitHub-Repository.  Passe die Pfade, Schriftarten und Farben an deine Bedürfnisse an.  Und lass dich von der KI unterstützen!
