# Frontend Plan: Atmospheres Feature

This document outlines the plan for implementing the frontend components and logic for the "Atmospheres" feature in Ligeia. The backend is fully implemented; this plan focuses on creating the user-facing interface to interact with it.

## 1. Objective

To build a complete, user-friendly interface for creating, saving, loading, and managing "Atmospheres," which are saved presets of soundscapes.

## 2. Key Features

- **View Atmospheres**: Display a list of all saved atmospheres, grouped by category, in the sidebar.
- **Save Atmosphere**: Save the current mix of active sounds (including their individual volume and loop settings) as a new atmosphere.
- **Load Atmosphere**: Load a saved atmosphere, replacing the current soundscape with the one from the atmosphere.
- **Delete Atmosphere**: Remove an atmosphere from the library.
- **UI Feedback**: Provide clear notifications for all actions (save, load, delete).

## 3. Component & File Modifications

### 3.1. `index.html` - HTML Structure

- **Atmosphere Panel**: A new section will be added to the sidebar (`<div class="sidebar-section">`) to display the list of atmospheres.
  ```html
  <!-- Inside the sidebar -->
  <div class="sidebar-section">
      <h3>Atmospheres</h3>
      <div id="atmosphereList" class="atmosphere-list">
          <!-- Atmospheres will be dynamically rendered here -->
      </div>
  </div>
  ```
- **Save Atmosphere Button**: A new button will be added to the main header controls.
  ```html
  <!-- Inside the header controls -->
  <button id="saveAtmosphere" class="btn btn-primary">💾 Save Atmosphere</button>
  ```
- **Save Atmosphere Modal**: A new modal will be added to the end of the `<body>` for capturing atmosphere details.
  ```html
  <div id="saveAtmosphereModal" class="modal-overlay" style="display:none;">
      <div class="modal-container">
          <h2>Save Atmosphere</h2>
          <form id="saveAtmosphereForm">
              <label for="atmosphereName">Name:</label>
              <input type="text" id="atmosphereName" required>
              
              <label for="atmosphereDescription">Description:</label>
              <textarea id="atmosphereDescription"></textarea>
              
              <label for="atmosphereCategory">Category:</label>
              <select id="atmosphereCategory"></select>
              
              <div class="modal-actions">
                  <button type="button" id="cancelSaveAtmosphere">Cancel</button>
                  <button type="submit">Save</button>
              </div>
          </form>
      </div>
  </div>
  ```

### 3.2. `src/services/DatabaseService.js` - Backend Integration

This service will be extended with new methods to call the Rust backend's atmosphere commands.

- `getAtmosphereCategories()`: Fetches all categories for the save modal dropdown.
- `getAllAtmospheres()`: Gets the list of all saved atmospheres.
- `getAtmosphereWithSounds(id)`: Retrieves a single atmosphere and its associated sound data.
- `saveAtmosphere(atmosphere)`: Saves a new or updates an existing atmosphere.
- `deleteAtmosphere(id)`: Deletes an atmosphere.
- `addSoundToAtmosphere(atmosphereId, audioFileId, volume, isLooping)`
- `removeSoundFromAtmosphere(atmosphereId, audioFileId)`

### 3.3. `src/ui/UIController.js` - DOM Manipulation

New methods will be added to manage the UI for the atmospheres feature.

- `renderAtmosphereList(atmospheres)`: Renders the list of atmospheres in the sidebar. Each item will have "Load" and "Delete" buttons.
- `showSaveAtmosphereModal(categories)`: Displays the modal and populates the category dropdown.
- `hideSaveAtmosphereModal()`: Hides the modal and resets the form.
- Event listeners for the new buttons (`saveAtmosphere`, `loadAtmosphere`, `deleteAtmosphere`).

### 3.4. `src/AmbientMixerApp.js` - Application Logic

The main controller will be updated to handle the state and logic.

- **New State**: `this.atmospheres = []` will be added to store the list of atmospheres.
- **Initialization**: In `initialize()`, the app will call `databaseService.getAllAtmospheres()` and then `uiController.renderAtmosphereList()` to display them on startup.
- **New Handlers**:
    - `handleShowSaveAtmosphereModal()`: Gathers active sounds and shows the save modal.
    - `handleSaveAtmosphere(formData)`: Constructs the atmosphere object from the form data and the current sound pad states, calls the database service, and refreshes the UI.
    - `handleLoadAtmosphere(atmosphereId)`:
        1. Stops all currently playing sounds.
        2. Calls `databaseService.getAtmosphereWithSounds(atmosphereId)`.
        3. For each sound in the loaded atmosphere, finds the corresponding `SoundPad`.
        4. Sets the volume and loop status on the `SoundPad`.
        5. Plays the `SoundPad`.
    - `handleDeleteAtmosphere(atmosphereId)`: Calls the database service to delete the atmosphere and then re-renders the list.

## 4. Implementation Roadmap

1.  **Step 1: Backend Integration (`DatabaseService.js`)**
    - Add all the new `invoke` functions for every atmosphere command exposed by the Rust backend.

2.  **Step 2: HTML Scaffolding (`index.html`)**
    - Add the HTML for the sidebar panel, the "Save Atmosphere" button, and the save modal as described above.

3.  **Step 3: UI Rendering (`UIController.js`)**
    - Implement `renderAtmosphereList` to generate the HTML for each atmosphere item, including Load/Delete buttons with appropriate `data-atmosphere-id` attributes.
    - In `AmbientMixerApp.js`, fetch the atmospheres during `initialize()` and call the new render function.

4.  **Step 4: Save Functionality**
    - In `AmbientMixerApp.js`, create `handleShowSaveAtmosphereModal` and `handleSaveAtmosphere`.
    - In `UIController.js`, implement `showSaveAtmosphereModal` and `hideSaveAtmosphereModal`.
    - Wire up the "Save Atmosphere" button and the modal form submission to the new handlers. The save logic should collect all active `SoundPad`s and their states.

5.  **Step 5: Load Functionality**
    - In `AmbientMixerApp.js`, implement `handleLoadAtmosphere`. This is the most complex part, as it involves stopping the current sounds and starting new ones based on the loaded data.
    - Add an event listener in `UIController.js` that delegates clicks on the "Load" buttons in the atmosphere list to this handler.

6.  **Step 6: Delete Functionality**
    - In `AmbientMixerApp.js`, implement `handleDeleteAtmosphere`.
    - Add a click delegate in `UIController.js` for the "Delete" buttons.

7.  **Step 7: Styling (`styles.css`)**
    - Add CSS rules to style the new atmosphere list, buttons, and the save modal to match the application's existing theme.

This plan provides a clear path to integrating the atmospheres feature into the frontend in a way that is consistent with the existing application architecture.
