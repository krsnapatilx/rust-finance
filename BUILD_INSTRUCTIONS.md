# 🛑 CRITICAL: Windows SDK Missing

The project cannot build because the **Windows 10 SDK** (or Windows 11 SDK) is missing from your Visual Studio installation. This component contains `kernel32.lib`, which is required for linking.

## 🛠️ Step-by-Step Fix

1.  **Open Visual Studio Installer** (search in Start Menu).
2.  Click **"Modify"** next to "Visual Studio Build Tools 2022".
3.  Go to the **"Individual components"** tab at the top.
4.  Search for **"Windows 10 SDK"** (e.g. `Windows 10 SDK (10.0.19041.0)`).
5.  **Check the box** next to it.
6.  Click **"Modify"** in the bottom right corner.
7.  **Restart your computer** after installation.

## 🏃 Run the Bot
After fixing the SDK, simply run:
```powershell
run_bot.bat
```
