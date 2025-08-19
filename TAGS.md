Alright—here’s a **deep, software-ready taxonomy** for RPG audio. It’s designed so you can drop it straight into a database / tagging UI and scale it later. I’ve split it into:

- Design rules (so tags stay clean)
- Field mapping (so you know where to store them)
- **Massive controlled vocabularies** for **Genre**, **Mood**, **Occasion**, and **Keywords** (with hierarchy + aliases)
- Data shapes (JSON examples) and usage tips

---

# RPG Audio Tagging Spec — Deep Taxonomy v1.0

## 0) Design rules (use these in your software)
- **Controlled vocabulary**: store a canonical `slug` (machine value) and an optional `label` (display value).  
  - Convention: `kebab-case` ASCII for slugs, e.g., `epic`, `dark-ambient`, `space-battle`.
- **Multi-value** fields allowed for all tag groups.
- **Aliases**: keep a `synonyms` array for search (e.g., `“brass-hits”` → [“braams”, “brams”]).
- **Hierarchy**: support parent/child (e.g., `genre: electronic > synthwave`).
- **Weights**: allow an optional `relevance` (0–100) per tag on a track.
- **Localization**: keep canonical slugs English-only; translate labels as needed.
- **Safety cues**: optional flags per track: `sudden-loudness`, `screams`, `gore`, `nsfw-language`.
- **Loopability**: `loopable`, `stinger`, `one-shot`, `intro`, `outro`, `transition`.
- **Time/energy facets** (optional numerics): `bpm`, `energy(0–5)`, `intensity(0–5)`, `tension(0–5)`.

## 1) Field mapping (ID3 & common libs)
- **Genre** → `TCON` (ID3) + internal `genre[]` taxonomy
- **Mood** → `TMOO` (ID3v2.4) *or* `TXXX:Mood`
- **Occasion** → `TXXX:Occasion`
- **Keywords** → `TXXX:Keywords` (freeform but controlled by this spec)
- Optional: `TBPM`, `COMM` (notes), `TXXX:Flags` (`loopable;stinger;…`)

---

## 2) GENRE (hierarchical, broad → specific)
> Use broad + 0–3 specifics. Example: `orchestral`, `orchestral:hybrid`, `orchestral:choir`.

### Orchestral
- `orchestral`
  - `orchestral:cinematic`
  - `orchestral:hybrid`
  - `orchestral:heroic`
  - `orchestral:dark`
  - `orchestral:minimal`
  - `orchestral:romantic`
  - `orchestral:baroque`
  - `orchestral:renaissance`
  - `orchestral:medieval`
  - `orchestral:choral` (alias: `choir`, `gregorian`)
  - `orchestral:percussive`
  - `orchestral:trailer` (alias: `trailer-music`)

### Ambient & Drones
- `ambient`
  - `ambient:dark-ambient`
  - `ambient:space-ambient`
  - `ambient:nature-ambient`
  - `ambient:ritual`
  - `ambient:drone`
  - `ambient:textural`
  - `ambient:new-age`
  - `ambient:lofi-ambient`

### Electronic
- `electronic`
  - `electronic:synthwave` (alias: `retrowave`, `outrun`)
  - `electronic:cyberpunk`
  - `electronic:idm`
  - `electronic:glitch`
  - `electronic:industrial`
  - `electronic:ebm`
  - `electronic:techno`
  - `electronic:trance`
  - `electronic:dnb`
  - `electronic:downtempo`
  - `electronic:chiptune` (alias: `8bit`)
  - `electronic:shoegaze-electronic`

### Rock / Metal / Post-
- `rock`
  - `rock:post-rock`
  - `rock:gothic-rock`
  - `rock:progressive`
- `metal`
  - `metal:power`
  - `metal:symphonic`
  - `metal:black`
  - `metal:doom`
  - `metal:folk-metal`
  - `metal:industrial-metal`
- `post-metal`

### Folk / World / Traditional
- `folk`
  - `folk:celtic`
  - `folk:nordic`
  - `folk:middle-eastern`
  - `folk:mediterranean`
  - `folk:asian-east`
  - `folk:asian-south`
  - `folk:african`
  - `folk:andino`
  - `folk:balkan`
  - `folk:sea-shanty`
  - `folk:wild-west-folk`

### Jazz / Blues / Noir / Lounge
- `jazz`
  - `jazz:noir`
  - `jazz:swing`
  - `jazz:cool`
  - `jazz:latin`
- `blues`
- `lounge`

### Horror / Sound-Design Forward
- `horror`
  - `horror:atonal`
  - `horror:dissonant-strings`
  - `horror:sound-design`
- `sound-design`
  - `sound-design:risers`
  - `sound-design:impacts`
  - `sound-design:whooshes`
  - `sound-design:stingers`
  - `sound-design:booms`
  - `sound-design:braams` (alias: `brass-hits`)

### Historical / Diegetic / Source
- `diegetic`
  - `diegetic:tavern-band`
  - `diegetic:radio`
  - `diegetic:gramophone`
  - `diegetic:street-musician`
- `historical`
  - `historical:baroque`
  - `historical:renaissance`
  - `historical:medieval`
  - `historical:romantic`

### Setting-Driven Blends
- `fantasy`
  - `fantasy:high-fantasy`
  - `fantasy:grimdark`
  - `fantasy:fairy`
- `sci-fi`
  - `sci-fi:space-opera`
  - `sci-fi:hard-sci-fi`
  - `sci-fi:cyberpunk`
  - `sci-fi:biopunk`
  - `sci-fi:post-human`
- `punk-variants`
  - `dieselpunk`, `steampunk`, `atompunk`, `solarpunk`
- `post-apocalyptic`
- `western`
- `mystery-noir`
- `modern-urban`
- `superhero`
- `mythic`
  - `mythic:norse`, `mythic:greco-roman`, `mythic:egyptian`, `mythic:celtic`, `mythic:japanese`, `mythic:mesoamerican`

---

## 3) MOOD (emotional/psychological tone)
> Treat mood as multi-dimensional. Below are curated, distinct labels. Use 1–4 per track.

### Positive / Uplifting
`heroic`, `triumphant`, `noble`, `uplifting`, `hopeful`, `inspiring`, `adventurous`, `confident`, `victorious`, `festive`, `playful`, `whimsical`, `merry`, `lighthearted`, `tender`, `romantic`, `serene`, `pastoral`, `warm`, `comforting`, `nostalgic`, `bittersweet`

### Neutral / Reflective
`mysterious`, `enigmatic`, `curious`, `contemplative`, `dreamlike`, `ethereal`, `mythic`, `arcane`, `otherworldly`, `solemn`, `ceremonial`, `sacred`, `ritualistic`, `austere`, `stoic`

### Dark / Tense / Negative
`ominous`, `foreboding`, `tense`, `suspenseful`, `uneasy`, `eerie`, `creepy`, `unsettling`, `menacing`, `sinister`, `gothic`, `dread`, `grim`, `bleak`, `oppressive`, `claustrophobic`, `tragic`, `melancholic`, `sorrowful`, `desolate`, `lonely`, `fatalistic`, `nihilistic`

### Action Energy / Aggression
`driving`, `relentless`, `frenetic`, `furious`, `aggressive`, `percussive`, `charged`, `urgent`, `high-stakes`, `chaotic`, `volatile`, `explosive`, `brooding-intensity`, `building`, `rising-tension`, `calm-before-storm`

### Subtle Texture / Atmospherics
`airy`, `hazy`, `glacial`, `glitchy`, `noisy`, `grainy`, `organic`, `mechanical`, `rusted`, `industrial`, `neon`, `digital-cold`, `bio-organic`, `wet`, `dry`

### Horror-Specific
`eldritch`, `uncanny`, `body-horror`, `liminal`, `dissonant`, `abhorrent`, `ritual-fear`, `cosmic-dread`

---

## 4) OCCASION (scene/use case)
> The most important driver for real-time selection. Think “when would I play this?”

### Meta / Session Flow
`session-start`, `recap`, `table-chatter`, `break`, `session-end`, `credits`, `level-up`, `quest-complete`, `achievement`, `loot-found`, `character-death`, `epilogue`, `flashback`, `montage`

### Exploration & Travel
`overworld-travel`, `wilderness-exploration`, `urban-exploration`, `dungeon-crawl`, `ruin-delving`, `underdark-journey`, `sewers`, `cave-exploration`, `mountain-pass`, `desert-crossing`, `jungle-trek`, `swamp-march`, `arctic-trek`, `sea-voyage`, `river-journey`, `airship-voyage`, `space-cruise`, `hyperspace-transit`, `derelict-ship-exploration`, `space-station-walk`

### Social / Roleplay
`tavern`, `inn`, `market`, `black-market`, `noble-court`, `audience-with-ruler`, `council-debate`, `negotiation`, `interrogation`, `trial`, `festival`, `wedding`, `funeral`, `ceremony`, `religious-service`, `gambling-den`, `speakeasy`, `noir-club`, `tea-house`

### Investigation / Heist / Stealth
`crime-scene`, `library-research`, `occult-research`, `stakeout`, `tailing`, `surveillance`, `infiltration`, `lockpicking`, `safecracking`, `hacking`, `netrun`, `vault-breach`, `disguise`, `escape`, `extraction`, `clean-getaway`

### Puzzles / Traps / Timers
`riddle-solving`, `mechanism-puzzle`, `arcane-puzzle`, `trap-primed`, `trap-triggered`, `chase-timer`, `bomb-timer`, `reactor-meltdown`, `airlock-timer`

### Combat (phase-aware)
`combat-ambush`, `combat-skirmish`, `combat-duel`, `combat-horde`, `combat-siege`, `combat-naval`, `combat-aerial`, `combat-vehicular`, `combat-mecha`, `combat-space-battle`,  
`boss-intro`, `boss-loop`, `boss-final-phase`, `victory-fanfare`, `defeat-lament`,  
`chase`, `car-chase`, `foot-chase`, `dogfight`, `boarding-action`

### Horror / Supernatural
`haunting`, `possession`, `ritual`, `summoning`, `banishment`, `eldritch-reveal`, `sanity-slip`, `ghost-encounter`, `vampire-lair`, `werewolf-hunt`, `zombie-siege`, `cult-gathering`, `sacrificial-altar`

### Magic / Psionics / Sci-Tech
`spellcasting-prep`, `battle-magic`, `divination`, `telepathy`, `dream-walk`, `astral-travel`, `time-warp`, `portal-crossing`, `teleportation`,  
`lab-experiment`, `biotech-lab`, `nanotech-swarm`, `ai-core`, `cyberdeck-dive`, `cyber-combat`, `drone-control`, `mech-dock`, `warp-jump`, `tractor-beam`, `ship-docking`, `eva-walk`

### Survival / Downtime / Crafting
`campfire`, `short-rest`, `long-rest`, `foraging`, `hunting`, `tracking`, `blacksmithing`, `fletching`, `alchemy`, `enchanting`, `cooking`, `base-building`, `training`, `shopping`, `bargain`, `healing`, `hospital-ward`

### Environment / Events
`sunrise`, `sunset`, `night-watch`, `storm`, `rain`, `blizzard`, `sandstorm`, `earthquake`, `flood`, `eclipse`, `meteor-shower`, `radiation-storm`, `anomaly-event`, `void-rift`, `volcanic-eruption`

### Transitions & UI
`scene-transition`, `reveal-stinger`, `jump-scare`, `mystery-sting`, `map-open`, `map-close`, `quest-accepted`, `quest-failed`, `dice-roll`, `success-cue`, `failure-cue`

---

## 5) KEYWORDS (big bucket, organized by facets)
> Use as many as needed; these power fast filtering. Include parent facet for clarity in your DB.

### 5.1 Biomes & Terrain
`biome:forest`, `biome:ancient-forest`, `biome:rainforest`, `biome:swamp`, `biome:bog`, `biome:marsh`, `biome:desert`, `biome:dunes`, `biome:oasis`, `biome:arctic`, `biome:tundra`, `biome:glacier`, `biome:mountain`, `biome:canyon`, `biome:steppe`, `biome:plains`, `biome:grassland`, `biome:jungle`, `biome:savanna`, `biome:volcanic`, `biome:underdark`, `biome:cave`, `biome:sewers`, `biome:coast`, `biome:open-sea`, `biome:river`, `biome:lake`, `biome:sky`, `biome:astral`, `biome:void`, `biome:otherworld`

### 5.2 Locations & Structures
`loc:castle`, `loc:keep`, `loc:fortress`, `loc:watchtower`, `loc:dungeon`, `loc:catacombs`, `loc:crypt`, `loc:temple`, `loc:shrine`, `loc:monastery`, `loc:library`, `loc:academy`, `loc:mage-tower`, `loc:throne-room`, `loc:market`, `loc:slums`, `loc:harbor`, `loc:mine`, `loc:smithy`, `loc:inn`, `loc:tavern`, `loc:prison`, `loc:arena`, `loc:laboratory`, `loc:biolab`, `loc:reactor`, `loc:spaceport`, `loc:hangar`, `loc:bridge-deck`, `loc:engineering`, `loc:cryosleep-bay`, `loc:cargo-bay`, `loc:derelict`, `loc:ruins`, `loc:ancient-city`, `loc:fairy-glen`

### 5.3 Cultures, Eras & Styles (non-exhaustive, neutral labels)
`style:medieval-european`, `style:renaissance`, `style:baroque`, `style:romantic-era`, `style:ancient-greek`, `style:ancient-roman`, `style:egyptian`, `style:norse`, `style:celtic`, `style:arabesque`, `style:persian`, `style:ottoman`, `style:indian-classical`, `style:japanese-traditional`, `style:chinese-traditional`, `style:korean-traditional`, `style:balinese-gamelan`, `style:andino`, `style:west-african`, `style:native-north-american`, `style:mexican-folk`, `style:iberian-folk`, `style:byzantine-chant`, `style:gothic`, `style:western-frontier`, `style:1920s`, `style:noir-1940s`, `style:cold-war`, `style:cyberpunk-neon`, `style:dieselpunk`, `style:steampunk`, `style:post-apocalyptic`

### 5.4 Creatures & Foes
`creature:dragon`, `creature:drake`, `creature:wyvern`, `creature:giant`, `creature:troll`, `creature:ogre`, `creature:goblin`, `creature:orc`, `creature:kobold`, `creature:gnoll`, `creature:undead`, `creature:zombie`, `creature:ghoul`, `creature:skeleton`, `creature:lich`, `creature:vampire`, `creature:werewolf`, `creature:ghost`, `creature:demon`, `creature:devil`, `creature:angel`, `creature:fae`, `creature:dryad`, `creature:elemental`, `creature:construct`, `creature:golem`, `creature:slime`, `creature:beast`, `creature:wolf`, `creature:bear`, `creature:spider`, `creature:kraken`, `creature:siren`, `creature:aberration`, `creature:eldritch-entity`, `creature:alien`, `creature:parasite`, `creature:machine-swarm`

### 5.5 Factions & NPC Types
`faction:empire`, `faction:rebels`, `faction:mercenary-band`, `faction:thieves-guild`, `faction:mages-guild`, `faction:knightly-order`, `faction:church`, `faction:cult`, `faction:cartel`, `faction:megacorp`, `faction:syndicate`, `faction:raiders`, `faction:mutants`, `faction:androids`, `npc:noble`, `npc:merchant`, `npc:smuggler`, `npc:guard`, `npc:assassin`, `npc:priest`, `npc:witch`, `npc:warlock`, `npc:necromancer`, `npc:ranger`, `npc:alchemist`, `npc:scientist`, `npc:hacker`, `npc:pilot`, `npc:ai`

### 5.6 Magic, Powers & Elements
`magic:abjuration`, `magic:conjuration`, `magic:divination`, `magic:enchantment`, `magic:evocation`, `magic:illusion`, `magic:necromancy`, `magic:transmutation`,  
`element:fire`, `element:ice`, `element:lightning`, `element:wind`, `element:earth`, `element:water`, `element:poison`, `element:acid`, `element:shadow`, `element:light`, `element:void`, `element:metal`, `element:wood`,  
`ritual:blood-magic`, `ritual:summoning`, `ritual:sacrifice`, `ritual:binding`, `ritual:banishment`

### 5.7 Technology & Vehicles
`tech:medieval`, `tech:clockwork`, `tech:steam`, `tech:diesel`, `tech:nuclear`, `tech:fusion`, `tech:antimatter`, `tech:biotech`, `tech:nanotech`, `tech:ai`, `tech:cybernetics`, `tech:synthetic`,  
`vehicle:horse`, `vehicle:carriage`, `vehicle:war-wagon`, `vehicle:train`, `vehicle:subway`, `vehicle:motorbike`, `vehicle:armored-vehicle`, `vehicle:airship`, `vehicle:zeppelin`, `vehicle:airplane`, `vehicle:helicopter`, `vehicle:mech`, `vehicle:tank`, `vehicle:starfighter`, `vehicle:shuttle`, `vehicle:frigate`, `vehicle:battleship`, `vehicle:freighter`

### 5.8 Weather & Natural Phenomena
`weather:clear`, `weather:wind`, `weather:rain`, `weather:thunderstorm`, `weather:snow`, `weather:blizzard`, `weather:heatwave`, `weather:sandstorm`, `weather:fog`, `weather:mist`, `weather:hail`, `weather:aurora`, `weather:eclipse`, `weather:meteor-shower`, `weather:solar-storm`, `weather:radiation-storm`, `weather:anomaly`

### 5.9 Instruments & Timbres (for search refinement)
`timbre:strings-warm`, `timbre:strings-dissonant`, `timbre:low-brass`, `timbre:braams`, `timbre:solo-violin`, `timbre:solo-cello`, `timbre:harp`, `timbre:flute`, `timbre:whistle`, `timbre:bagpipes`, `timbre:nyckelharpa`, `timbre:hurdy-gurdy`, `timbre:oud`, `timbre:sitar`, `timbre:shakuhachi`, `timbre:erhu`, `timbre:guzheng`, `timbre:koto`, `timbre:gamelan`, `timbre:frame-drum`, `timbre:taiko`, `timbre:bodhran`, `timbre:dulcimer`, `timbre:lute`, `timbre:organ`, `timbre:church-choir`, `timbre:male-chant`, `timbre:female-vocalise`, `timbre:child-choir`, `timbre:synthetic-pad`, `timbre:analog-synth`, `timbre:fm-synth`, `timbre:granular`, `timbre:noise-texture`, `timbre:clockwork`, `timbre:metallic-hits`, `timbre:sub-boom`

### 5.10 SFX & Foley (diegetic cues)
`sfx:footsteps`, `sfx:armor-clank`, `sfx:sword-clash`, `sfx:bow-release`, `sfx:gunshot`, `sfx:reload`, `sfx:ricochet`, `sfx:explosion`, `sfx:door-creak`, `sfx:gate-bang`, `sfx:chains`, `sfx:coins`, `sfx:pages-turning`, `sfx:quill-scratch`, `sfx:campfire`, `sfx:wood-crackle`, `sfx:water-drip`, `sfx:river`, `sfx:ocean-surf`, `sfx:thunder`, `sfx:wind`, `sfx:rain`, `sfx:market-crowd`, `sfx:tavern-murmur`, `sfx:church-bells`, `sfx:chant`, `sfx:chant-latin`, `sfx:monster-roar`, `sfx:dragon-breath`, `sfx:zombie-moans`, `sfx:ghost-wail`, `sfx:space-engine-hum`, `sfx:alarm`, `sfx:scanner-beeps`, `sfx:keyboard`, `sfx:hacking-glitches`, `sfx:warp`, `sfx:teleport`, `sfx:magic-whoosh`, `sfx:spell-impact`, `sfx:shield`, `sfx:portal-open`, `sfx:portal-close`

### 5.11 Gamey / UI-ish
`ui:ping`, `ui:notify`, `ui:confirm`, `ui:error`, `ui:upgrade`, `ui:inventory`, `ui:map-open`, `ui:map-close`, `ui:crafting-complete`, `ui:purchase`, `ui:sell`, `ui:quest-update`, `ui:skill-point`, `ui:dice-roll`

### 5.12 Audio Structure & Utility
`util:loopable`, `util:stinger`, `util:intro`, `util:outro`, `util:transition`, `util:bed`, `util:drone`, `util:motif`, `util:theme`, `util:alt-mix`, `util:instrumental`, `util:with-vocals`, `util:diegetic`, `util:non-diegetic`, `util:stem-percussion`, `util:stem-ambient`, `util:stem-melody`

---

## 6) JSON shapes (drop-in examples)

### 6.1 Single track (minimal)
```json
{
  "id": "trk_00123",
  "title": "Into the Ruins",
  "genre": ["ambient", "ambient:dark-ambient"],
  "mood": ["mysterious", "uneasy", "textural"],
  "occasion": ["dungeon-crawl", "ruin-delving", "trap-primed"],
  "keywords": ["biome:underdark", "loc:ruins", "sfx:water-drip", "timbre:low-brass"],
  "flags": ["util:loopable"],
  "bpm": 60,
  "intensity": 2,
  "tension": 4
}
```

### 6.2 Aliases & labels (catalog dictionary)
```json
{
  "tags": {
    "mood": [
      { "slug": "heroic", "label": "Heroic", "synonyms": ["valiant", "gallant"] },
      { "slug": "eldritch", "label": "Eldritch", "synonyms": ["cosmic-horror", "lovecraftian"] }
    ],
    "genre": [
      { "slug": "orchestral:hybrid", "label": "Hybrid Orchestral", "parents": ["orchestral"] }
    ]
  }
}
```

---

## 7) Usage patterns & UI guidance
- **Search UX**: facet chips for `Genre`, `Mood`, `Occasion`, `Keywords`; free text maps to synonyms.
- **Quick picks**: prebuilt filters (“Calm Exploration”, “High-Tension Stealth”, “Boss Final Phase”).
- **Autoplaylists**: rules like `occasion contains "combat" AND intensity >= 4 AND (genre includes "orchestral" OR "electronic:industrial")`.
- **Smart transitions**: suggest `util:stinger` matching current `mood` when occasion changes.
- **Conflict checks**: warn on mutually exclusive moods (`serene` + `furious`) unless user overrides.
- **Content safety**: toggle to hide tracks flagged `sudden-loudness` or `screams`.

---

## 8) Master lists (flat, copy/paste friendly)

### GENRE (flat)
`orchestral`, `orchestral:cinematic`, `orchestral:hybrid`, `orchestral:heroic`, `orchestral:dark`, `orchestral:minimal`, `orchestral:romantic`, `orchestral:baroque`, `orchestral:renaissance`, `orchestral:medieval`, `orchestral:choral`, `orchestral:percussive`, `orchestral:trailer`,  
`ambient`, `ambient:dark-ambient`, `ambient:space-ambient`, `ambient:nature-ambient`, `ambient:ritual`, `ambient:drone`, `ambient:textural`, `ambient:new-age`, `ambient:lofi-ambient`,  
`electronic`, `electronic:synthwave`, `electronic:cyberpunk`, `electronic:idm`, `electronic:glitch`, `electronic:industrial`, `electronic:ebm`, `electronic:techno`, `electronic:trance`, `electronic:dnb`, `electronic:downtempo`, `electronic:chiptune`,  
`rock`, `rock:post-rock`, `rock:gothic-rock`, `rock:progressive`,  
`metal`, `metal:power`, `metal:symphonic`, `metal:black`, `metal:doom`, `metal:folk-metal`, `metal:industrial-metal`, `post-metal`,  
`folk`, `folk:celtic`, `folk:nordic`, `folk:middle-eastern`, `folk:mediterranean`, `folk:asian-east`, `folk:asian-south`, `folk:african`, `folk:andino`, `folk:balkan`, `folk:sea-shanty`, `folk:wild-west-folk`,  
`jazz`, `jazz:noir`, `jazz:swing`, `jazz:cool`, `jazz:latin`, `blues`, `lounge`,  
`horror`, `horror:atonal`, `horror:dissonant-strings`, `horror:sound-design`,  
`sound-design`, `sound-design:risers`, `sound-design:impacts`, `sound-design:whooshes`, `sound-design:stingers`, `sound-design:booms`, `sound-design:braams`,  
`diegetic`, `diegetic:tavern-band`, `diegetic:radio`, `diegetic:gramophone`, `diegetic:street-musician`,  
`historical`, `historical:baroque`, `historical:renaissance`, `historical:medieval`, `historical:romantic`,  
`fantasy`, `fantasy:high-fantasy`, `fantasy:grimdark`, `fantasy:fairy`,  
`sci-fi`, `sci-fi:space-opera`, `sci-fi:hard-sci-fi`, `sci-fi:cyberpunk`, `sci-fi:biopunk`, `sci-fi:post-human`,  
`dieselpunk`, `steampunk`, `atompunk`, `solarpunk`,  
`post-apocalyptic`, `western`, `mystery-noir`, `modern-urban`, `superhero`, `mythic`, `mythic:norse`, `mythic:greco-roman`, `mythic:egyptian`, `mythic:celtic`, `mythic:japanese`, `mythic:mesoamerican`

### MOOD (flat)
`heroic`, `triumphant`, `noble`, `uplifting`, `hopeful`, `inspiring`, `adventurous`, `confident`, `victorious`, `festive`, `playful`, `whimsical`, `merry`, `lighthearted`, `tender`, `romantic`, `serene`, `pastoral`, `warm`, `comforting`, `nostalgic`, `bittersweet`,  
`mysterious`, `enigmatic`, `curious`, `contemplative`, `dreamlike`, `ethereal`, `mythic`, `arcane`, `otherworldly`, `solemn`, `ceremonial`, `sacred`, `ritualistic`, `austere`, `stoic`,  
`ominous`, `foreboding`, `tense`, `suspenseful`, `uneasy`, `eerie`, `creepy`, `unsettling`, `menacing`, `sinister`, `gothic`, `dread`, `grim`, `bleak`, `oppressive`, `claustrophobic`, `tragic`, `melancholic`, `sorrowful`, `desolate`, `lonely`, `fatalistic`, `nihilistic`,  
`driving`, `relentless`, `frenetic`, `furious`, `aggressive`, `percussive`, `charged`, `urgent`, `high-stakes`, `chaotic`, `volatile`, `explosive`, `brooding-intensity`, `building`, `rising-tension`, `calm-before-storm`,  
`airy`, `hazy`, `glacial`, `glitchy`, `noisy`, `grainy`, `organic`, `mechanical`, `rusted`, `industrial`, `neon`, `digital-cold`, `bio-organic`, `wet`, `dry`,  
`eldritch`, `uncanny`, `body-horror`, `liminal`, `dissonant`, `abhorrent`, `ritual-fear`, `cosmic-dread`

### OCCASION (flat)
`session-start`, `recap`, `table-chatter`, `break`, `session-end`, `credits`, `level-up`, `quest-complete`, `achievement`, `loot-found`, `character-death`, `epilogue`, `flashback`, `montage`,  
`overworld-travel`, `wilderness-exploration`, `urban-exploration`, `dungeon-crawl`, `ruin-delving`, `underdark-journey`, `sewers`, `cave-exploration`, `mountain-pass`, `desert-crossing`, `jungle-trek`, `swamp-march`, `arctic-trek`, `sea-voyage`, `river-journey`, `airship-voyage`, `space-cruise`, `hyperspace-transit`, `derelict-ship-exploration`, `space-station-walk`,  
`tavern`, `inn`, `market`, `black-market`, `noble-court`, `audience-with-ruler`, `council-debate`, `negotiation`, `interrogation`, `trial`, `festival`, `wedding`, `funeral`, `ceremony`, `religious-service`, `gambling-den`, `speakeasy`, `noir-club`, `tea-house`,  
`crime-scene`, `library-research`, `occult-research`, `stakeout`, `tailing`, `surveillance`, `infiltration`, `lockpicking`, `safecracking`, `hacking`, `netrun`, `vault-breach`, `disguise`, `escape`, `extraction`, `clean-getaway`,  
`riddle-solving`, `mechanism-puzzle`, `arcane-puzzle`, `trap-primed`, `trap-triggered`, `chase-timer`, `bomb-timer`, `reactor-meltdown`, `airlock-timer`,  
`combat-ambush`, `combat-skirmish`, `combat-duel`, `combat-horde`, `combat-siege`, `combat-naval`, `combat-aerial`, `combat-vehicular`, `combat-mecha`, `combat-space-battle`, `boss-intro`, `boss-loop`, `boss-final-phase`, `victory-fanfare`, `defeat-lament`, `chase`, `car-chase`, `foot-chase`, `dogfight`, `boarding-action`,  
`haunting`, `possession`, `ritual`, `summoning`, `banishment`, `eldritch-reveal`, `sanity-slip`, `ghost-encounter`, `vampire-lair`, `werewolf-hunt`, `zombie-siege`, `cult-gathering`, `sacrificial-altar`,  
`spellcasting-prep`, `battle-magic`, `divination`, `telepathy`, `dream-walk`, `astral-travel`, `time-warp`, `portal-crossing`, `teleportation`, `lab-experiment`, `biotech-lab`, `nanotech-swarm`, `ai-core`, `cyberdeck-dive`, `cyber-combat`, `drone-control`, `mech-dock`, `warp-jump`, `tractor-beam`, `ship-docking`, `eva-walk`,  
`campfire`, `short-rest`, `long-rest`, `foraging`, `hunting`, `tracking`, `blacksmithing`, `fletching`, `alchemy`, `enchanting`, `cooking`, `base-building`, `training`, `shopping`, `bargain`, `healing`, `hospital-ward`,  
`sunrise`, `sunset`, `night-watch`, `storm`, `rain`, `blizzard`, `sandstorm`, `earthquake`, `flood`, `eclipse`, `meteor-shower`, `radiation-storm`, `anomaly-event`, `void-rift`, `volcanic-eruption`,  
`scene-transition`, `reveal-stinger`, `jump-scare`, `mystery-sting`, `map-open`, `map-close`, `quest-accepted`, `quest-failed`, `dice-roll`, `success-cue`, `failure-cue`

### KEYWORDS (facet slugs; sample)
**Biomes/Terrain**: `biome:forest`, `biome:desert`, `biome:arctic`, `biome:mountain`, `biome:jungle`, `biome:swamp`, `biome:underdark`, `biome:cave`, `biome:coast`, `biome:ocean`, `biome:river`, `biome:sky`, `biome:astral`, `biome:void`  
**Locations/Structures**: `loc:castle`, `loc:keep`, `loc:fortress`, `loc:dungeon`, `loc:temple`, `loc:shrine`, `loc:monastery`, `loc:library`, `loc:academy`, `loc:mage-tower`, `loc:throne-room`, `loc:market`, `loc:slums`, `loc:harbor`, `loc:mine`, `loc:smithy`, `loc:inn`, `loc:tavern`, `loc:prison`, `loc:arena`, `loc:laboratory`, `loc:reactor`, `loc:spaceport`, `loc:hangar`, `loc:bridge-deck`, `loc:engineering`, `loc:cryosleep-bay`, `loc:cargo-bay`, `loc:derelict`, `loc:ruins`, `loc:ancient-city`, `loc:fairy-glen`  
**Cultures/Eras/Styles**: `style:medieval-european`, `style:renaissance`, `style:baroque`, `style:romantic-era`, `style:ancient-greek`, `style:ancient-roman`, `style:egyptian`, `style:norse`, `style:celtic`, `style:arabesque`, `style:persian`, `style:ottoman`, `style:indian-classical`, `style:japanese-traditional`, `style:chinese-traditional`, `style:korean-traditional`, `style:gamelan`, `style:andino`, `style:west-african`, `style:noir-1940s`, `style:cyberpunk-neon`, `style:steampunk`, `style:post-apocalyptic`, `style:western-frontier`  
**Creatures**: `creature:dragon`, `creature:giant`, `creature:troll`, `creature:goblin`, `creature:orc`, `creature:undead`, `creature:zombie`, `creature:ghoul`, `creature:skeleton`, `creature:lich`, `creature:vampire`, `creature:werewolf`, `creature:ghost`, `creature:demon`, `creature:devil`, `creature:angel`, `creature:fae`, `creature:dryad`, `creature:elemental`, `creature:golem`, `creature:kraken`, `creature:siren`, `creature:alien`, `creature:eldritch-entity`  
**Factions/NPCs**: `faction:empire`, `faction:rebels`, `faction:thieves-guild`, `faction:mages-guild`, `faction:church`, `faction:cult`, `faction:cartel`, `faction:megacorp`, `faction:raiders`, `faction:mutants`, `npc:merchant`, `npc:noble`, `npc:guard`, `npc:assassin`, `npc:priest`, `npc:witch`, `npc:necromancer`, `npc:ranger`, `npc:alchemist`, `npc:scientist`, `npc:hacker`, `npc:pilot`, `npc:ai`  
**Magic/Elements**: `magic:abjuration`, `magic:conjuration`, `magic:divination`, `magic:enchantment`, `magic:evocation`, `magic:illusion`, `magic:necromancy`, `magic:transmutation`, `element:fire`, `element:ice`, `element:lightning`, `element:wind`, `element:earth`, `element:water`, `element:poison`, `element:acid`, `element:shadow`, `element:light`, `element:void`  
**Tech/Vehicles**: `tech:steam`, `tech:diesel`, `tech:nuclear`, `tech:fusion`, `tech:biotech`, `tech:nanotech`, `tech:ai`, `tech:cybernetics`, `vehicle:horse`, `vehicle:carriage`, `vehicle:train`, `vehicle:motorbike`, `vehicle:airship`, `vehicle:mech`, `vehicle:starfighter`, `vehicle:frigate`  
**Weather/Phenomena**: `weather:clear`, `weather:wind`, `weather:rain`, `weather:thunderstorm`, `weather:snow`, `weather:blizzard`, `weather:fog`, `weather:aurora`, `weather:eclipse`, `weather:meteor-shower`, `weather:solar-storm`, `weather:radiation-storm`  
**Timbres/Instruments**: `timbre:strings-warm`, `timbre:strings-dissonant`, `timbre:low-brass`, `timbre:braams`, `timbre:harp`, `timbre:flute`, `timbre:whistle`, `timbre:bagpipes`, `timbre:nyckelharpa`, `timbre:hurdy-gurdy`, `timbre:oud`, `timbre:sitar`, `timbre:shakuhachi`, `timbre:erhu`, `timbre:guzheng`, `timbre:koto`, `timbre:gamelan`, `timbre:taiko`, `timbre:bodhran`, `timbre:organ`, `timbre:church-choir`, `timbre:male-chant`, `timbre:female-vocalise`, `timbre:analog-synth`, `timbre:fm-synth`, `timbre:granular`, `timbre:noise-texture`, `timbre:clockwork`, `timbre:metallic-hits`, `timbre:sub-boom`  
**SFX/Foley**: `sfx:footsteps`, `sfx:armor-clank`, `sfx:sword-clash`, `sfx:bow-release`, `sfx:gunshot`, `sfx:reload`, `sfx:ricochet`, `sfx:explosion`, `sfx:door-creak`, `sfx:chains`, `sfx:coins`, `sfx:pages-turning`, `sfx:campfire`, `sfx:water-drip`, `sfx:river`, `sfx:ocean-surf`, `sfx:thunder`, `sfx:wind`, `sfx:rain`, `sfx:market-crowd`, `sfx:tavern-murmur`, `sfx:church-bells`, `sfx:chant`, `sfx:monster-roar`, `sfx:dragon-breath`, `sfx:zombie-moans`, `sfx:ghost-wail`, `sfx:space-engine-hum`, `sfx:alarm`, `sfx:scanner-beeps`, `sfx:keyboard`, `sfx:hacking-glitches`, `sfx:warp`, `sfx:teleport`, `sfx:magic-whoosh`, `sfx:spell-impact`, `sfx:shield`, `sfx:portal-open`, `sfx:portal-close`  
**Utility/Structure**: `util:loopable`, `util:stinger`, `util:intro`, `util:outro`, `util:transition`, `util:bed`, `util:drone`, `util:motif`, `util:theme`, `util:alt-mix`, `util:instrumental`, `util:with-vocals`, `util:diegetic`, `util:non-diegetic`, `util:stem-percussion`, `util:stem-ambient`, `util:stem-melody`

---

## 9) Implementation tips
- **Importers**: when scanning existing libraries, map free text to closest canonical slug using a synonym table; log misses for curation.
- **Curation UI**: keyboard-only tagging, recent-tags bar, and quick sliders for `intensity`, `tension`.
- **Playback logic**: when `occasion` changes, fade to track sharing ≥1 `mood` and ≥1 `genre` parent; if none, fall back to `keywords` + `intensity` proximity.
- **Stems**: if you store stems, mirror all parent tags, add `util:stem-*`.
- **QA**: lint new tags against the dictionary; flag unrecognized slugs.

---

If you want, I can package this into a **downloadable `.md`** and/or a **ready-to-use JSON dictionary** for your app (with `slug`, `label`, `parents`, `synonyms`).