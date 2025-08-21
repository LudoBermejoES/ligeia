// Draggable initialization adapter for membership panel using @shopify/draggable
// Uses global Draggable (loaded via CDN) to enable dropping mixer pads into membership panel.
// Assumes script tag: @shopify/draggable draggable.bundle.legacy.js has been loaded.
export function initMembershipDraggable({ containerSelector = '#membershipPanelBody', sourceSelector = '.sound-pad', onDrop }) {
  if (!window.Draggable) { console.warn('Draggable CDN not loaded'); return null; }
  const { Droppable } = window.Draggable;
  const container = document.querySelector(containerSelector);
  if (!container) return null;
  const dr = new Droppable(container, {
    draggable: sourceSelector,
    dropzone: containerSelector,
    mirror: { constrainDimensions: true }
  });
  dr.on('droppable:dropped', (evt) => {
    const pad = evt.dragEvent?.source;
    const id = pad?.dataset?.audioId;
    if (id && onDrop) onDrop(Number(id));
  });
  return dr;
}
