// PadRenderer - stateless helper to generate sound pad HTML
// Keeps presentation separate from UIController orchestration.
export function renderSoundPad(audioFile, padState, { escapeHtml }) {
  const isPlaying = padState?.isPlaying || false;
  const isLooping = padState?.isLooping || false;
  const isMuted = padState?.isMuted || false;
  const volumePercent = Math.round(((padState?.volume) ?? 0.5) * 100);

  const title = audioFile.title || fileNameFromPath(audioFile.file_path);
  const artist = audioFile.artist || 'Unknown Artist';
  const rpgTags = audioFile.rpgTags || [];

  return `
    <div class="sound-pad ${isPlaying ? 'active' : ''} ${isMuted ? 'muted' : ''}" data-file-path="${escapeHtml(audioFile.file_path)}" data-audio-id="${audioFile.id ?? ''}" draggable="true">
      <div class="sound-pad-header">
        <div class="sound-pad-info">
          <div class="sound-pad-title">${escapeHtml(title)}</div>
          <div class="sound-pad-meta">
            <span class="sound-pad-artist">${escapeHtml(artist)}</span>
            <button class="edit-tags-btn" data-action="edit-tags" title="Edit Tags" draggable="false">✏️</button>
          </div>
        </div>
        <div class="sound-pad-status">${isPlaying ? '▶️' : '⏸️'}</div>
      </div>
      <div class="sound-pad-controls">
        <div class="sound-pad-buttons">
          <button class="pad-btn ${isPlaying ? 'active' : ''}" data-action="toggle" draggable="false">${isPlaying ? 'Stop' : 'Play'}</button>
          <button class="pad-btn ${isLooping ? 'active' : ''}" data-action="loop" draggable="false">Loop</button>
          <button class="pad-btn ${isMuted ? 'active' : ''}" data-action="mute" draggable="false">Mute</button>
        </div>
        <div class="volume-control-pad">
          <input type="range" class="volume-slider-pad" min="0" max="100" value="${volumePercent}" data-action="volume" aria-label="Pad Volume" draggable="false">
          <span class="volume-display-pad">${volumePercent}%</span>
        </div>
      </div>
      ${rpgTags.length ? `<div class="sound-pad-tags">${rpgTags.map(t => `<span class="tag-chip tag-${t.tagType}">${escapeHtml(t.tagValue)}</span>`).join('')}</div>` : ''}
    </div>`;
}

function fileNameFromPath(path) {
  return path.split(/[/\\]/).pop()?.replace(/\.[^/.]+$/, '') || 'Unknown';
}
