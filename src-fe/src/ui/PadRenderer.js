// Unified PadRenderer - context-aware sound pad HTML generator
// Supports multiple contexts: mixer, atmosphere, etc.

// Context configurations
const CONTEXTS = {
  mixer: {
    controls: ['toggle', 'loop', 'mute', 'volume', 'edit-tags'],
    cssClasses: ['sound-pad'],
    showArtist: true,
    showTags: true,
    layout: 'grid'
  },
  atmosphere: {
    controls: ['toggle', 'loop', 'mute', 'volume', 'remove'],
    cssClasses: ['sound-pad', 'atmosphere-pad'],
    showArtist: false,
    showTags: false,
    layout: 'compact'
  }
};

export function renderSoundPad(audioFile, padState, options = {}) {
  const { 
    escapeHtml, 
    context = 'mixer',
    origin = null // for tracking where pad originated 
  } = options;
  
  // Get context configuration
  const config = CONTEXTS[context] || CONTEXTS.mixer;
  
  const isPlaying = padState?.isPlaying || false;
  const isLooping = padState?.isLooping || false;
  const isMuted = padState?.isMuted || false;
  const volumePercent = Math.round(((padState?.volume) ?? 0.5) * 100);

  const title = audioFile.title || fileNameFromPath(audioFile.file_path);
  const artist = audioFile.artist || 'Unknown Artist';
  const rpgTags = audioFile.rpgTags || [];

  // Generate CSS classes
  const cssClasses = [
    ...config.cssClasses,
    isPlaying ? 'active' : '',
    isMuted ? 'muted' : ''
  ].filter(Boolean).join(' ');

  // Generate control buttons based on context
  const controlButtons = generateControlButtons(config.controls, {
    isPlaying,
    isLooping,
    isMuted
  });

  // Build data attributes
  const dataAttrs = [
    `data-file-path="${escapeHtml(audioFile.file_path)}"`,
    `data-audio-id="${audioFile.id ?? ''}"`,
    `data-context="${context}"`,
    origin ? `data-origin="${origin}"` : ''
  ].filter(Boolean).join(' ');

  return `
    <div class="${cssClasses}" ${dataAttrs} draggable="true">
      <div class="sound-pad-header">
        <div class="sound-pad-info">
          <div class="sound-pad-title" title="${escapeHtml(title)}">${escapeHtml(title)}</div>
          ${config.showArtist ? `<div class="sound-pad-meta">
            <span class="sound-pad-artist">${escapeHtml(artist)}</span>
            ${config.controls.includes('edit-tags') ? `<button class="edit-tags-btn" data-action="edit-tags" title="Edit Tags" draggable="false">✏️</button>` : ''}
          </div>` : ''}
        </div>
        <div class="sound-pad-status">${isPlaying ? '▶️' : '⏸️'}</div>
      </div>
      <div class="sound-pad-controls">
        <div class="sound-pad-buttons">
          ${controlButtons}
        </div>
        ${config.controls.includes('volume') ? `<div class="volume-control-pad">
          <input type="range" class="volume-slider-pad" min="0" max="100" value="${volumePercent}" data-action="volume" aria-label="Pad Volume" draggable="false">
          <span class="volume-display-pad">${volumePercent}%</span>
        </div>` : ''}
      </div>
      ${config.showTags && rpgTags.length ? `<div class="sound-pad-tags">${rpgTags.map(t => `<span class="tag-chip tag-${t.tagType}">${escapeHtml(t.tagValue)}</span>`).join('')}</div>` : ''}
    </div>`;
}

function generateControlButtons(controls, state) {
  const buttons = [];
  
  if (controls.includes('toggle')) {
    buttons.push(`<button class="pad-btn ${state.isPlaying ? 'active' : ''}" data-action="toggle" title="${state.isPlaying ? 'Stop' : 'Play'}" draggable="false">${state.isPlaying ? '⏸️' : '▶️'}</button>`);
  }
  
  if (controls.includes('loop')) {
    buttons.push(`<button class="pad-btn ${state.isLooping ? 'active' : ''}" data-action="loop" title="Loop" draggable="false">🔁</button>`);
  }
  
  if (controls.includes('mute')) {
    buttons.push(`<button class="pad-btn ${state.isMuted ? 'active' : ''}" data-action="mute" title="Mute" draggable="false">${state.isMuted ? '🔇' : '🔊'}</button>`);
  }
  
  if (controls.includes('remove')) {
    buttons.push(`<button class="pad-btn" data-action="remove" title="Remove from atmosphere" draggable="false">✕</button>`);
  }
  
  return buttons.join('');
}

function fileNameFromPath(path) {
  return path.split(/[/\\]/).pop()?.replace(/\.[^/.]+$/, '') || 'Unknown';
}
