# 🚀 TimeLocky Quick Launcher

**Simple batch script for launching TimeLocky programs**

---

## 📋 Usage

**Double-click:** `quick_launcher.bat`

---

## 🎯 Menu Options

```
[1] Launch Telegram Bot (Console)      ← Opens in CMD window
[2] Launch Telegram Bot (Background)   ← Silent, no window
[3] Launch TimeLocky App                ← Start GUI
[4] Open TimeLocky Folder              ← Explorer
[5] Add to Windows Startup             ← Auto-start on boot
[Q] Quit
```

---

## ⚡ Quick Setup

### Auto-start Telegram Bot on Login:
```
1. Double-click quick_launcher.bat
2. Press [5] — Add to Windows Startup
3. Press [1] — Add Telegram Bot (Background)
4. Done! Bot runs automatically on every boot
```

### Create Desktop Shortcut with Hotkey:
```
1. Right-click quick_launcher.bat → Send to → Desktop
2. Right-click desktop shortcut → Properties
3. Set "Shortcut key": Ctrl+Alt+L (your choice)
4. Click OK
```

Now press `Ctrl+Alt+L` anywhere to open the launcher!

---

## 🔧 Startup Management

### Add to Startup:
- Option [5] in main menu
- Choose what to auto-start:
  - Telegram Bot (Background) — Silent
  - Telegram Bot (Console) — Visible window
  - TimeLocky App — GUI

### Remove from Startup:
- Option [5] → [4] "Remove from Startup"
- Removes all TimeLocky startup items

---

## 📂 Manual Startup Access

```
Win+R  →  shell:startup  →  Enter
```

This opens the startup folder where shortcuts are created.

---

## 💡 Tips

**Silent Bot**: Use "Background" option for no console window

**Visible Bot**: Use "Console" option to see bot status/errors

**Desktop Shortcut**: Fastest way to access launcher

**Startup Folder**: Shortcuts are created in:
```
%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup
```

---

**No dependencies required — pure Windows batch script! 🎯**
