# Frontend Plan: Atmospheres Feature (Enhanced)

Rich preset management ("Atmospheres") for saving, loading, updating, and organizing soundscapes with smooth UX, resilience, and future extensibility.

---
## 1. Objectives
Create a robust, low-friction interface to:
- Save current active pads (volume, loop, mute) as an Atmosphere
- Load & optionally crossfade into saved Atmospheres
- Update or duplicate existing Atmospheres
- Organize by category and quickly browse
- Lay groundwork for future tagging, search, export, versioning

Success criteria:
- Load time < 200ms JS orchestration (excluding audio decode)
- Crossfade optional & cancelable
- Partial-load tolerated (missing files) with warning
- No regression to existing mixer/tag features

---
## 2. Data Model (Frontend Shape)
Atmosphere:
```
{
    id, name, description?, category?, subcategory?, created_at, updated_at,
    background_image?, author_image?, is_public?,
    sounds: [ { audio_file_id, volume (0–1), is_looping, is_muted } ]
}
```
Local structures:
- `this.atmospheres: Atmosphere[]`
- `this.atmosphereIndex: Map<number, Atmosphere>`
- `this.activeAtmosphereId: number | null`

Normalization: Audio file metadata remains in existing `audioFiles` Map; sounds array references by id.

---
## 3. Service Layer (`DatabaseService.js`)
Add invoke wrappers (map snake_case -> camelCase on return if needed):
- `getAtmosphereCategories()`
- `getAllAtmospheres()`
- `getAtmosphereWithSounds(id)`
- `saveAtmosphere(atmosphere)` (create/update by presence of `id`)
- `deleteAtmosphere(id)`
- (Future) `duplicateAtmosphere(id, overrides?)`

All methods return Promises; errors surface with standardized message to UI controller.

---
## 4. UI / UX Design
Sidebar Section:
- Collapsible categories (accordion); uncategorized bucket `_Other`
- Each item: name, sound-count badge, actions: Load ▶, Edit ✎, Delete 🗑
- Active atmosphere highlighted (accent border or glow)

Save / Update Modal:
- Fields: Name*, Description, Category (select + “+” inline add), Include muted sounds (checkbox), Crossfade on future load (checkbox, persisted in localStorage), Buttons: Cancel / Save (or Update / Save As New)
- Mode aware: create vs edit vs “save as new”

Load Behavior:
- Option to crossfade (default ON, configurable) over N ms (default 1200ms)
- Graceful cancellation if a new load triggers mid-fade
- Missing files reported; others still load

Delete Confirmation:
- Lightweight inline confirm (small overlay or inline expansion) before permanent removal

Keyboard & A11y:
- Focus trap in modal; Esc closes; Enter submits
- Button labels have aria-labels describing action (e.g. “Load atmosphere Forest Dawn”)

Notifications:
- Success, warning (partial load), error standardized via existing notification system

---
## 5. Application Logic (`AmbientMixerApp`)
New state:
```
this.atmospheres = []
this.atmosphereIndex = new Map()
this.activeAtmosphereId = null
```
Core methods:
- `refreshAtmospheres()` – fetch + rebuild index + render
- `buildCurrentAtmospherePayload(formData)` – derive sounds list from active pads
- `handleShowSaveAtmosphereModal(mode)` – open with context
- `handleSaveAtmosphere(formData, mode)` – create/update/save-as-new
- `handleLoadAtmosphere(id, { crossfadeMs })`
- `handleDeleteAtmosphere(id)` – optimistic removal with rollback on error
- `applyAtmosphereSounds(atmo, options)` – executes load/crossfade logic
- `setActiveAtmosphere(id)` – update highlight & state

Crossfade outline:
1. Snapshot current playing pads (with current volume)
2. Prepare target pads (ensure instances exist; set starting volume 0 if fading in)
3. Animate over `crossfadeMs` using `requestAnimationFrame` (avoid setInterval jitter)
4. On completion, stop pads not in target (unless user aborted mid-fade)
5. If a new load starts, mark previous fade token canceled

Edge cases:
- Atmosphere has 0 sounds → just stop others & notify
- Missing audio file → log + warning badge
- Duplicate name on create → warn; user can confirm overwrite OR choose “Save As New”

---
## 6. UI Controller Additions (`UIController.js`)
Methods:
- `renderAtmosphereList(grouped)`
- `highlightActiveAtmosphere(id)`
- `showSaveAtmosphereModal({ mode, categories, initial })`
- `hideSaveAtmosphereModal()`
- `bindAtmosphereEvents(handlers)` – delegates clicks for Load/Edit/Delete

Rendering responsibility separation: grouping done in app logic; UI method only transforms to DOM.

---
## 7. Styling (`styles.css`)
Add classes:
- `.atmosphere-section`, `.atmo-category`, `.atmo-category-header`, `.atmo-items`
- `.atmo-item`, `.atmo-item.active`, `.atmo-item-actions`
- `.atmo-badge` (sound count)
- Modal reuse existing pattern plus small adjustments for new fields

---
## 8. Error & Resilience Strategy
| Operation | Failure Modes | Strategy |
|-----------|---------------|----------|
| Fetch list | Backend down | Show cached (if available) + retry button |
| Save | Validation, DB err | Inline field error + toast |
| Load | Missing files | Partial apply + warning listing missing names |
| Delete | Race condition | Reload list if not found; toast on failure |
| Crossfade | Audio interruption | Fallback to hard switch |

All service calls wrapped; errors pass normalized shape `{ action, message, details? }`.

---
## 9. Performance & Scalability
- Batch DOM updates (fragment + single insert)
- Avoid full re-render on highlight change
- Debounce sequential saves (300ms)
- Prepare for virtualization if atmospheres > 200 (future hook point)
- Crossfade: minimize allocations; reuse arrays

---
## 10. Testing / Verification
Scenarios:
1. Create new atmosphere (active pads >0) → appears in list
2. Update existing (change volume) → reload & verify volumes match ±1%
3. Load with crossfade ON vs OFF
4. Partial load (simulate missing file) → warning + remaining load ok
5. Delete active atmosphere → highlight cleared
6. Rapid successive loads → only last applies; no JS errors
7. Cancel modal → no side effects

Manual smoke scripts + optional lightweight dev helper (log diff between current pads & atmosphere).

---
## 11. Implementation Phases
1. Service methods + empty sidebar scaffold
2. Fetch & render list (no load logic yet)
3. Save modal (create only)
4. Load (basic hard switch)
5. Delete + confirmation
6. Update / Save As New modes
7. Crossfade implementation & cancellation
8. UX polish (category accordion, active highlight, badges)
9. Edge handling & notifications refinement
10. Testing & small performance passes

Each phase leaves app functional; feature flags (simple booleans) can gate incomplete steps if needed.

---
## 12. Future Extensions (Planned but Deferred)
- Atmosphere tagging & search
- Export/import embedded with library export
- Favorites & pinning
- Recent atmospheres quick bar
- Version history & diffing
- Scheduling / sequencing atmospheres

---
## 13. Open Questions (Track & Resolve During Implementation)
- Should pad ordering be saved/restored? (Current: NOT stored; option to add later.)
- Persist crossfade preference per user? (Plan: localStorage key `atmoCrossfadeMs`)
- Include global master volume in atmosphere? (Initial: NO; maybe optional later.)

---
## 14. Summary
This enhanced plan adds richer UX, resilience, performance awareness, and a phased path. It builds on existing architecture minimizing refactors while setting clear seams for future growth.

Next actionable step: implement Phase 1 (service methods + scaffold) followed by incremental UI integration.
