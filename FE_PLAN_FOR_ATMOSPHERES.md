# Atmospheres Feature: Full Frontend + Backend + Database Plan (v2.3)

Comprehensive specification for implementing "Atmospheres" (persistent soundscape presets) with scalability toward pro‑grade ambient engines (inspired by tools like Syrinscape) while fitting existing Ligeia architecture.

---
## 1. Goals & Non‑Goals
Status (v2.3): Phase 1 delivered. Phase 2 crossfade engine + cancellation + curve options delivered. Phase 3 (initial) enhancements implemented:
 - Progress event emission (start/progress/almost_complete/complete) wired to notifications (visual progress bar pending).
 - Diff computation + basic summary notification before load (no confirm overlay yet).
 - Real sound count badge + basic integrity indicator (missing file warning) now in list.
 - Client-side atmosphere search/filter with highlighting (backend search endpoint still pending).
 - Crossfade defaults & curve persistence live (migration previously completed).
Remaining near-term tasks: duplicate endpoint + UI, backend search & integrity endpoints, progress bar UI, diff confirmation overlay.
Goals:
- Save current active sound pads (volume, loop, mute) as reusable presets.
- Load presets with optional smooth crossfades & cancellation.
- Organize atmospheres via hierarchical categories + keywords.
- Support incremental enhancement: variation sets, one‑shots, macros later.
- Maintain robustness (partial loads, missing audio) without blocking UX.
- Provide export/import continuity with existing library export.

Non‑Goals (initial): real‑time EQ per layer, scheduling/sequencing, multi‑user sync.

Success Metrics:
- Atmosphere list fetch < 80ms (cached) / < 150ms cold.
- Crossfade CPU steady (<5% main thread spikes) for ≤ 32 active layers.
- Atmosphere load ≥ 95% success even with missing files (graceful degrade).

---
## 2. Current Backend State (Rust) Summary
Existing structs (from `models.rs` now include new fields):
```
Atmosphere { id?, name, title, description, category, subcategory, subsubcategory?, keywords[], background_image?, author_image?, is_public, default_crossfade_ms, fade_curve, created_at, updated_at }
AtmosphereSoundMapping { id?, atmosphere_id, audio_file_id, volume, is_looping, is_muted, created_at }
AtmosphereWithSounds { atmosphere, sounds, audio_files }
AtmosphereCategory { id, name, parent_id? }
```
Handlers already expose save, get, delete, add/remove/update sounds, categories.

Gaps vs Desired Capability:
- No crossfade metadata or default settings.
- No versioning or integrity summary.
- No variation sets / one‑shot randomization / grouping.
- Keywords exist but no search endpoint.
- No duplication command.

---
## 3. Proposed Data Model Extensions
Add server-side (Rust) fields / tables incrementally, feature-flagged.

### 3.1 Atmosphere Table Additions
| Field | Type | Purpose | Default |
|-------|------|---------|---------|
| version | INTEGER | Schema version for migrations | 1 |
| default_crossfade_ms | INTEGER | Preferred crossfade when loading this atmosphere | 1200 |
| fade_curve | TEXT | 'linear'|'equal_power'|'exp' | 'equal_power' |
| persistent_groups | TEXT | JSON array of group names to persist across loads | '[]' |
| flags | TEXT | JSON object for future booleans (e.g., {"protect":false}) | '{}' |

Implementation: Add nullable columns with defaults; migration sets version=1 for legacy rows.

### 3.2 New Table: atmosphere_variation_sets
```
id INTEGER PK
atmosphere_id INTEGER FK
name TEXT
created_at TEXT
```

### 3.3 New Table: atmosphere_variation_items
```
id INTEGER PK
variation_set_id INTEGER FK
audio_file_id INTEGER FK
weight REAL DEFAULT 1.0
```

### 3.4 Extend AtmosphereSoundMapping
Add columns:
| Column | Type | Purpose |
|--------|------|---------|
| role | TEXT | 'loop'|'oneshot'|'bed'|'spot' (playback semantics) |
| group | TEXT | Logical grouping (e.g., 'weather','fauna') |
| fade_in_ms | INTEGER | Optional per-layer entry fade |
| fade_out_ms | INTEGER | Optional per-layer exit fade |
| random_min_delay_s | REAL | For oneshot scheduling |
| random_max_delay_s | REAL | For oneshot scheduling |
| probability | REAL | 0–1 chance inclusion on load |
| variation_set_id | INTEGER NULL | Link to variation set (if using variations) |

All added as nullable for backward compatibility.

### 3.5 Macros (Deferred)
Future tables: atmosphere_macros (id, atmosphere_id, name, actions JSON) not in MVP.

### 3.6 Integrity Summary (Derived, not stored)
Backend endpoint can compute missing audio_file_ids to send to client.

---
## 4. Backend API Additions
New Tauri commands (phased):
Phase 1:
- get_all_atmospheres (already)
- get_atmosphere_with_sounds (already)
- save_atmosphere (already) – extend to accept new optional fields
- delete_atmosphere (already)
- get_atmosphere_categories (already)

Phase 2:
- search_atmospheres(query: String, category?: String, keywords?: Vec<String>)
- duplicate_atmosphere(id: i64, overrides?: AtmosphereOverride)

Phase 3:
- save_variation_set(set: VariationSetPayload)
- list_variation_sets(atmosphere_id: i64)
- delete_variation_set(id: i64)

Phase 4:
- compute_atmosphere_integrity(id: i64) -> { missing_ids: Vec<i64> }

Payload / Return Normalization: Use snake_case in Rust; frontend maps to camelCase.

---
## 5. Migration Strategy
1. Add new columns with `ALTER TABLE` guarded by `IF NOT EXISTS` (SQLite tolerant pattern: check pragma table_info first).
2. Set default_crossfade_ms=1200 for all rows.
3. Backfill version=1 where NULL.
4. Create new variation tables only when feature flag `ENABLE_VARIATIONS` true (compile‑time cfg or runtime setting).
5. Provide a lightweight `migrate_atmospheres()` function invoked at startup.

---
## 6. Frontend Data Structures (Updated)
Implemented modules:
- AmbientMixerApp (orchestrator)
- LibraryManager (audio files + SoundPad creation)
- SoundPad (now with fadeTo() & cancelFades())
- AtmosphereService (CRUD wrappers)
- AtmosphereManager (list/create/load/delete; delegates crossfade)
- AtmosphereUIController (sidebar + save/edit modal; crossfade + curve fields)
- AtmosphereEngine (linear crossfade + cancellation token)

State (current):
```
atmospheres: AtmosphereSummary[]
activeAtmosphereId: number|null
AtmosphereManager.engine: AtmosphereEngine
AtmosphereEngine.currentToken: { id: Symbol, cancelled: boolean } | null
```
Planned additions:
```
atmosphereDetailCache: Map<number, AtmosphereDetail>
variation / oneshot scheduling data structures
persisted default_crossfade_ms & fade_curve per atmosphere
```
Layer runtime object (future extended):
```
{ audioFileId, volume, isLooping, isMuted, role?, group?, variationSetId?,
  fadeInMs?, fadeOutMs?, rand?: { minDelay, maxDelay, probability }, resolvedFileId? }
```

---
## 7. AtmosphereEngine (Frontend) (Implemented + Roadmap)
Implemented:
- crossfadeTo(detail, soundPads, { durationMs, curve='linear' })
- Cancellation token (new load cancels prior)
- Uses SoundPad.fadeTo for individual ramps

Pending:
- Progress events
- Snapshot/diff preview
- Group persistence (persistGroups)
- Oneshot scheduler + variation resolution

---
## 8. Crossfade Algorithm Detail (Current vs Planned)
Current (v2.2):
1. Identify removals -> fadeTo(0) with stopWhenZero.
2. Additions: ensure pad, start ~0 volume, play, fade to target.
3. Existing volume changes: fade from current to target.
4. Linear ramps via Web Audio linearRampToValueAtTime; fallback JS stepping.
5. Cancellation sets token.cancelled; subsequent steps abort; existing ramps finish harmlessly.

Planned Enhancements:
- Central diff planner + curve function application.
- Per-layer fade overrides (fade_in_ms / fade_out_ms).
- Equal-power & exponential curve math.
- Preview diff summary for UI.

---
## 9. UI / UX Specification
Sidebar Panel (Atmospheres Section):
- Category accordion; collapsed state persisted (localStorage key `atmoCatState`).
- Item row: name, count badge (# active layers), warning icon if integrity.missing>0, actions (Load, Edit, ⋯ menu). Active atmosphere row has glow.
- Quick actions bar: + New, Duplicate (disabled until selection), Search field.

Save / Edit Modal:
- Fields: Name*, Description, Category, Subcategory, Keywords (token input), Include muted?, Include paused (non‑playing)?, Default crossfade ms, Curve select, Persist groups (multi-select built from groups present in current pad set).
- Buttons: Cancel, Save / Update, Save As New.
- Validation inline (name required, crossfade >=0, curve valid).

Load Flow:
- Click Load -> if crossfade enabled: shows unobtrusive progress bar; if missing layers: toast warning 'Loaded with 2 missing layers'.
- Option: context menu "Preview" executes dry run (computes differences; shows summary overlay) -> Confirm.

Deletion:
- Inline confirm row expansion: "Delete ‘Forest Dawn’?" [Cancel] [Delete].

Search:
- Fuzzy on name + keywords + category; highlight matches.

Accessibility:
- All interactive elements tabindex, aria-labels, role="button" where needed.
- Modal focus trap + Escape support.

Notifications:
- Standard UIController.showNotification for success/warn/error with consistent prefixes: Atmosphere: Saved, Atmosphere: Partial Load, etc.

---
## 10. Frontend Module Changes (Status)
Delivered:
- AtmosphereService (CRUD)
- AtmosphereManager (list/create/load/delete; crossfade delegation)
- AtmosphereUIController (list + create/edit modal + crossfade/curve fields + search/filter + badges)
- AtmosphereEngine (crossfade + cancellation + progress events + diff compute)
- SoundPad fade utilities (linear, equal_power, exp curves)

Pending:
- Duplicate flow (FE + BE)
- Backend search endpoint (client-side filter currently sufficient for small sets)
- Variation & oneshot UI surfaces
- Enhanced integrity (server verification + tooltip listing missing assets)
- Interactive diff confirmation & visual progress bar

---
## 11. Backend Implementation Steps (Status)
Done:
1. Added columns default_crossfade_ms, fade_curve with migration/backfill.
2. Extended Atmosphere struct & repository save/select.
3. Manager load uses persisted defaults.

Pending / Upcoming:
4. Search endpoint.
5. Duplicate endpoint.
6. Integrity computation endpoint.
7. Variation set tables & CRUD.
1. Migration module (Rust): check/add columns + new tables.
2. Extend Atmosphere struct (serde default for new fields) & implement From<Row> mapping for new columns.
3. Update save_atmosphere to write new fields (with fallback defaults).
4. Add variation set structs + CRUD (feature gated).
5. Implement search_atmospheres (LIKE on name/title; JSON keywords extraction) optionally full-text later.
6. Duplicate: fetch detail, insert new atmosphere (new id, updated name "Copy" suffix), duplicate mappings.
7. Integrity computation: join missing audio_file_ids (LEFT JOIN where audio_files.id IS NULL).
8. Ensure all new commands added to `main.rs` and annotated with #[tauri::command].

---
## 12. Error & Resilience Strategy
| Operation | Possible Failure | Mitigation | User Feedback |
|-----------|------------------|------------|---------------|
| Migration | Locked DB | Retry w/ backoff, abort with clear log | Startup error toast |
| Save | Validation | Validate client + server; return 400-like error string | Inline field + toast |
| Load | Missing files | Skip & mark missing count | Warning toast + icon |
| Crossfade | Cancelled | Token cancellation path | No error, silent abort |
| Variation pick | Empty set | Fallback to first mapping | Warn in log |

Standard error normalization: `{ action, message, cause? }` on frontend.

---
## 13. Performance & Memory
- Constrain simultaneous decode to 3; queue remainder.
- Release audio nodes of removed layers post fadeOut.
- Reuse Float32Array buffers for ramp calculations (avoid GC churn).
- Defer oneshot scheduler activation until first oneshot layer exists.
- Cache last 10 loaded atmospheres detail in Map (LRU discard beyond).

---
## 14. Testing Matrix
Unit (JS):
- Crossfade curve numeric correctness vs expected values.
- Diff calculation (current vs target) edge: empty, identical, disjoint.
- Variation weighted selection (monte-carlo approximation test).

Integration (App):
- Save -> reload -> verify volumes within ±1%.
- Load with some files deleted -> missing count accurate.
- Rapid load spam (5 loads in <1s) -> only last applied; no uncaught errors.
- Cancel mid crossfade (load new) -> old fade stops.

Backend (Rust tests):
- Migration idempotency.
- Search returns expected subset.
- Duplicate preserves mapping counts but new ids.

---
## 15. Implementation Phases (Refined Status)
✅ Phase 1: CRUD + list + save modal + create from current pads.
✅ Phase 2 (core): Crossfade engine (linear) + cancellation + configurable duration + curve options stored & applied.
✅ Phase 3 (foundation): Progress events, diff summary, search UI, sound count + integrity badges, persisted crossfade defaults.
🟡 Phase 3 polish: visual progress bar, diff confirmation overlay.
⬜ Phase 4: Role/group fields + persistGroups logic.
⬜ Phase 5: Backend integrity computation + enhanced indicators.
⬜ Phase 6: Duplicate + backend search + keyword UX refinements.
⬜ Phase 7: Variation sets + weighted selection.
⬜ Phase 8: One‑shot scheduling + probability & delay params.
⬜ Phase 9: Performance tuning (decode queue, buffer reuse, cleanup).
⬜ Phase 10: Advanced diff viewer, favorites, accessibility, export integration.

---
## 16. Future / Deferred
- Macros & action scripting.
- Scheduling/sequencing timeline.
- Per-layer DSP (EQ, filters).
- Multi-scene transitions & persistent ambience layer across a playlist.
- Version history & diff viewer.

---
## 17. Open Questions
1. Duplicate of both name & title fields—do we need both? (Potential consolidation: keep `name`, drop `title`.)
2. Should atmosphere save include non-playing but loaded pads? (Currently configurable via checkbox.)
3. Variation sets global vs per-atmosphere? (Planned: per-atmosphere; reconsider if reuse emerges.)
4. Export: embed all variation set metadata? (Yes, full fidelity.)
5. Security: any need to sanitize background_image paths? (Validate extension + disallow remote HTTP until sandboxing.)

---
## 18. Summary / Next Actions (Updated)
Baseline now includes: creation, listing, smooth crossfades with cancellation & curve selection, persisted defaults, progress events, diff summary notification, client-side search, badges for count & basic integrity.

Immediate Next Actions (toward v2.4):
1. Duplicate action (backend command + FE button, name suffix "(Copy)").
2. Replace notification spam with inline/inline-row progress bar tied to progress events.
3. Diff preview confirmation overlay (accept / cancel) before initiating crossfade.
4. Backend search_atmospheres endpoint (optional for scale); integrate when available.
5. Backend integrity endpoint returning explicit missing audio_file_ids; upgrade badge to show tooltip list.
6. Accessibility polish (ARIA labels on action buttons, badges, search input) & keyboard navigation in list.

Stretch (if time permits):
- Duplicate with selective sound inclusion.
- Persist search term across sessions.

Exit criteria v2.4: duplicate working, visual progress bar present, confirmable diff overlay, enhanced integrity (server-backed or documented fallback), accessible list interactions.

---
Document version: 2.3
