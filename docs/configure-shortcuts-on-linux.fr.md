# Configurer les raccourcis sous Linux

Sous Linux Wayland, les raccourcis globaux peuvent être gérés de deux façons : via le **XDG Portal** (Murmure enregistre les raccourcis via l'interface GlobalShortcuts de `xdg-desktop-portal`) ou via le mode **CLI** (Murmure n'enregistre rien, et vous configurez des Custom Shortcuts au niveau OS qui appellent directement le binaire `murmure`).

Le mode actif se configure dans **Paramètres > Système > Gestion des raccourcis**. Un redémarrage est nécessaire après tout changement.

| Mode | Quand l'utiliser |
| ---- | ---------------- |
| **XDG Portal** | KDE Plasma 6, Hyprland, Sway. Le portal fonctionne de façon fiable sur ces compositeurs. |
| **CLI** | GNOME (par défaut), ou tout compositeur où les raccourcis portal sont instables. |

Sur GNOME, le mode CLI est le défaut car l'implémentation portal de Mutter présente des problèmes connus de latence et de fiabilité qui rendent les raccourcis XDG Portal peu prévisibles.

## Référence des commandes CLI

En mode CLI, Murmure expose les commandes suivantes. Chacune peut être assignée à un Custom Shortcut OS.

| Commande | Effet |
| -------- | ----- |
| `murmure --transcription` | Toggle la transcription standard ON/OFF |
| `murmure --transcription-llm` | Toggle la transcription en mode LLM |
| `murmure --transcription-command` | Toggle la transcription en mode Command |
| `murmure --paste-last` | Colle la dernière transcription |
| `murmure --cancel` | Annule l'enregistrement en cours et revient en idle |
| `murmure --voice-mode` | Toggle le Voice Mode ON/OFF |
| `murmure --llm-mode 1` | Bascule vers le mode LLM 1 |
| `murmure --llm-mode 2` | Bascule vers le mode LLM 2 |
| `murmure --llm-mode 3` | Bascule vers le mode LLM 3 |
| `murmure --llm-mode 4` | Bascule vers le mode LLM 4 |

!!! warning "Limitation Push-to-talk"
    Les Custom Shortcuts OS se déclenchent à l'appui de la touche, pas au relâchement. Seul le **mode toggle** est donc utilisable. Le Push-to-talk (maintenir pour enregistrer, relâcher pour arrêter) ne peut pas être implémenté avec des Custom Shortcuts OS.

## GNOME

GNOME utilise Mutter comme compositeur. L'implémentation portal XDG GlobalShortcuts de Mutter est peu fiable (latence, événements perdus), donc Murmure passe par défaut en mode CLI sur GNOME.

### Ajouter un Custom Shortcut sur GNOME

1. Ouvrez **Paramètres > Clavier > Voir et personnaliser les raccourcis > Raccourcis personnalisés**.
2. Cliquez sur le bouton **+** pour ajouter un nouveau raccourci.
3. Renseignez :
   - **Nom** : `Murmure transcription` (ou le libellé de votre choix)
   - **Commande** : `murmure --transcription`
   - **Raccourci** : appuyez sur la combinaison souhaitée (ex. `Ctrl+Super+Espace`)
4. Cliquez sur **Ajouter**.

Répétez l'opération pour les autres commandes que vous souhaitez lier (par exemple `murmure --paste-last` sur un second raccourci).

### Vérifier que murmure est dans le PATH

Si GNOME ne trouve pas le binaire `murmure`, le raccourci échoue silencieusement. Vérifiez que le binaire est dans le PATH :

```bash
which murmure
```

S'il n'est pas trouvé, utilisez le chemin complet dans le champ Commande, par exemple `/usr/local/bin/murmure --transcription`.

## KDE Plasma 6

KDE Plasma 6 embarque un backend XDG GlobalShortcuts portal fonctionnel. Murmure passe par défaut en mode **XDG Portal** sur KDE, c'est la configuration recommandée, aucun réglage manuel n'est nécessaire.

### Passer en mode CLI sur KDE (utilisateurs avancés)

1. Dans Murmure, allez dans **Paramètres > Système > Gestion des raccourcis** et sélectionnez **CLI**.
2. Redémarrez Murmure.
3. Ouvrez **Paramètres système > Raccourcis > Raccourcis personnalisés**.
4. Cliquez sur **Éditer > Nouveau > Raccourci global > Commande/URL**.
5. Dans l'onglet **Déclencheur**, assignez votre combinaison de touches.
6. Dans l'onglet **Action**, saisissez la commande `murmure --transcription`.
7. Appliquez et fermez.

## Hyprland

Ajoutez des bindings dans `~/.config/hypr/hyprland.conf`. Remplacez `SUPER, Y` par votre modificateur et touche préférés.

```ini
# ~/.config/hypr/hyprland.conf

bind = SUPER, Y, exec, murmure --transcription
bind = SUPER SHIFT, Y, exec, murmure --paste-last
bind = SUPER ALT, Y, exec, murmure --cancel
```

Rechargez Hyprland pour appliquer (`hyprctl reload` ou déconnexion/reconnexion). Hyprland supporte aussi le portal XDG GlobalShortcuts, donc vous pouvez garder Murmure en mode **XDG Portal** si vous préférez gérer les raccourcis depuis l'interface Paramètres de Murmure.

## Sway

Ajoutez des bindings dans `~/.config/sway/config`. Remplacez `$mod+y` par la combinaison de votre choix.

```
# ~/.config/sway/config

bindsym $mod+y exec murmure --transcription
bindsym $mod+Shift+y exec murmure --paste-last
bindsym $mod+Control+y exec murmure --cancel
```

Rechargez Sway pour appliquer (`swaymsg reload`). Comme Hyprland, Sway supporte le portal XDG, vous pouvez donc aussi utiliser le mode **XDG Portal** et gérer les raccourcis depuis Murmure.

## Dépannage

### Le raccourci ne fait rien sur GNOME

- Vérifiez que `murmure` est dans le PATH (lancez `which murmure` dans un terminal).
- Assurez-vous que Murmure est déjà lancé en arrière-plan avant d'appuyer sur le raccourci. Les commandes CLI communiquent avec l'instance en cours d'exécution.
- Vérifiez qu'aucune autre application n'a revendiqué la même combinaison dans Paramètres > Clavier.

### Le raccourci se déclenche mais rien ne se passe dans Murmure

- Ouvrez un terminal et lancez `murmure --transcription` manuellement. Si le message "no running instance found" s'affiche, Murmure n'est pas démarré. Lancez-le d'abord (il démarre dans la barre système).
- Si la commande s'exécute sans erreur mais la transcription ne démarre pas, vérifiez que Murmure est en mode **CLI** dans Paramètres > Système > Gestion des raccourcis.

### Escape hatch : forcer XWayland

Si vous avez besoin de XWayland (par exemple un compositeur plus ancien sans support portal), démarrez Murmure avec la variable d'environnement `GDK_BACKEND` :

```bash
GDK_BACKEND=x11 murmure
```

Il s'agit d'une variable GTK standard. Murmure ne la définit plus automatiquement. En mode XWayland, les raccourcis globaux ne se déclenchent que lorsque la fenêtre Murmure a le focus.

### Les raccourcis XDG Portal fonctionnent sur Hyprland mais pas après un redémarrage

La session portal peut ne pas être enregistrée au démarrage. Vérifiez que `xdg-desktop-portal-hyprland` est installé et démarré :

```bash
systemctl --user status xdg-desktop-portal-hyprland
```
