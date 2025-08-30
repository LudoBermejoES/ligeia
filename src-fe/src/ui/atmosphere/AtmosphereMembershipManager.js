/**
 * AtmosphereMembershipManager - Core manager for atmosphere membership operations
 */
import logger from '../../utils/logger.js';

export class AtmosphereMembershipManager {
    constructor(service, libraryManager) {
        this.service = service;
        this.libraryManager = libraryManager;
        this.atmosphere = null;
        this.members = new Map(); // audioId -> { volume, is_looping, is_muted, min_seconds, max_seconds }
        this.persistTimer = null;
        this.detailLoaded = false;
    }

    /**
     * Set current atmosphere and load its membership
     * @param {Object} atmosphere - Atmosphere object
     */
    async setAtmosphere(atmosphere) {
        this.atmosphere = atmosphere;
        this.members.clear();
        this.detailLoaded = false;
        
        if (atmosphere) {
            logger.debug('membership', `Loading atmosphere: ${atmosphere.name}`);
            await this.loadAtmosphereMembers();
        }
    }

    /**
     * Load atmosphere members from service
     */
    async loadAtmosphereMembers() {
        if (!this.atmosphere) return;
        
        try {
            const details = await this.service.getAtmosphereWithSounds(this.atmosphere.id);
            this.members.clear();
            
            
            if (details.sounds) {
                details.sounds.forEach(sound => {
                    this.members.set(sound.audio_file_id, {
                        volume: sound.volume ?? 0.5,
                        is_looping: sound.is_looping ?? false,
                        is_muted: sound.is_muted ?? false,
                        min_seconds: sound.min_seconds ?? 0,
                        max_seconds: sound.max_seconds ?? 0
                    });
                });
            }
            
            this.detailLoaded = true;
            logger.debug('membership', `Loaded ${this.members.size} members for atmosphere ${this.atmosphere.name}`);
        } catch (error) {
            logger.error('membership', 'Failed to load atmosphere members:', error);
            throw error;
        }
    }

    /**
     * Add sound to atmosphere
     * @param {number} audioId - Audio file ID
     */
    addSound(audioId) {
        const numericAudioId = Number(audioId);
        
        if (this.members.has(numericAudioId)) {
            logger.debug('membership', 'Audio already exists in atmosphere', { audioId: numericAudioId });
            return { exists: true, audioId: numericAudioId };
        }

        // Add with default settings
        this.members.set(numericAudioId, {
            volume: 0.5,
            is_looping: false,
            is_muted: false,
            min_seconds: 0,
            max_seconds: 0
        });

        logger.debug('membership', `Added audio ${numericAudioId} to atmosphere`);
        this.schedulePersist();
        
        return { exists: false, audioId: numericAudioId };
    }

    /**
     * Remove sound from atmosphere
     * @param {number} audioId - Audio file ID
     */
    removeSound(audioId) {
        const numericAudioId = Number(audioId);
        
        if (this.members.has(numericAudioId)) {
            this.members.delete(numericAudioId);
            logger.debug('membership', `Removed audio ${numericAudioId} from atmosphere`);
            this.schedulePersist();
            return true;
        }
        
        return false;
    }

    /**
     * Update member metadata
     * @param {number} audioId - Audio file ID
     * @param {Object} updates - Updates to apply
     */
    updateMember(audioId, updates) {
        const numericAudioId = Number(audioId);
        const existing = this.members.get(numericAudioId);
        
        if (existing) {
            const updated = { ...existing, ...updates };
            this.members.set(numericAudioId, updated);
            this.schedulePersist();
            logger.debug('membership', `Updated member ${numericAudioId}:`, updates);
            return updated;
        }
        
        return null;
    }

    /**
     * Update delay values for a member
     * @param {number} audioId - Audio file ID
     * @param {number} minSeconds - Minimum delay seconds
     * @param {number} maxSeconds - Maximum delay seconds
     */
    updateDelayValues(audioId, minSeconds, maxSeconds) {
        const numericAudioId = Number(audioId);
        const updates = {
            min_seconds: Math.max(0, Math.min(60, Number(minSeconds) || 0)),
            max_seconds: Math.max(0, Math.min(60, Number(maxSeconds) || 0))
        };
        
        // If delay is set, force looping
        if (updates.min_seconds > 0 || updates.max_seconds > 0) {
            updates.is_looping = true;
        }
        
        return this.updateMember(numericAudioId, updates);
    }

    /**
     * Get member data
     * @param {number} audioId - Audio file ID
     * @returns {Object|null} Member data
     */
    getMember(audioId) {
        return this.members.get(Number(audioId)) || null;
    }

    /**
     * Get all members
     * @returns {Map} Members map
     */
    getAllMembers() {
        return new Map(this.members);
    }

    /**
     * Get member count
     * @returns {number} Number of members
     */
    getMemberCount() {
        return this.members.size;
    }

    /**
     * Check if atmosphere has members
     * @returns {boolean} True if has members
     */
    hasMembers() {
        return this.members.size > 0;
    }

    /**
     * Schedule persistence with debounce
     * @param {number} delay - Delay in milliseconds
     */
    schedulePersist(delay = 600) {
        if (this.persistTimer) {
            clearTimeout(this.persistTimer);
        }

        this.persistTimer = setTimeout(() => {
            this.persistChanges();
        }, delay);
    }

    /**
     * Persist changes to backend
     */
    async persistChanges() {
        if (!this.atmosphere) {
            logger.warn('membership', 'No atmosphere to persist');
            return;
        }

        try {
            // Get current atmosphere details to preserve metadata
            let detail;
            if (this.detailLoaded && this.atmosphere.id) {
                try {
                    detail = await this.service.getAtmosphereWithSounds(this.atmosphere.id);
                } catch (inner) {
                    logger.warn('membership', 'Persist re-fetch failed; using local state', { id: this.atmosphere.id, error: inner.message });
                    detail = { atmosphere: this.atmosphere, sounds: [] };
                }
            } else {
                detail = { atmosphere: this.atmosphere, sounds: [] };
            }

            // Map sounds with correct structure for saveAtmosphere
            const sounds = Array.from(this.members.entries()).map(([audio_file_id, meta]) => {
                const numericAudioFileId = Number(audio_file_id);
                
                // Skip sounds with invalid audio_file_id
                if (!Number.isInteger(numericAudioFileId) || numericAudioFileId <= 0) {
                    logger.warn('membership', `Skipping sound with invalid audio_file_id: ${audio_file_id} (converted to ${numericAudioFileId})`);
                    return null;
                }
                
                return {
                    // Omit id field since it's optional and null causes serialization issues
                    atmosphere_id: Number(this.atmosphere.id),
                    audio_file_id: numericAudioFileId,
                    volume: Number(meta.volume),
                    is_looping: Boolean(meta.is_looping),
                    is_muted: Boolean(meta.is_muted),
                    min_seconds: Number(meta.min_seconds || 0),
                    max_seconds: Number(meta.max_seconds || 0),
                    created_at: new Date().toISOString()
                };
            }).filter(sound => sound !== null);  // Remove null entries

            // Create payload but ensure i64 fields are not null
            const payload = { 
                ...detail.atmosphere, 
                sounds,
                // Force proper types for critical fields that might be null
                id: detail.atmosphere.id ? Number(detail.atmosphere.id) : undefined,
                default_crossfade_ms: Number(detail.atmosphere.default_crossfade_ms || 2500)
            };
            
            // Single comprehensive debug log
            const debugInfo = {
                soundCount: sounds.length,
                firstSound: sounds[0] || 'none',
                atmosphereObject: detail.atmosphere,
                nullFields: Object.entries(payload).filter(([key, value]) => value === null).map(([key]) => key),
                i64Fields: {
                    id: { value: payload.id, type: typeof payload.id },
                    default_crossfade_ms: { value: payload.default_crossfade_ms, type: typeof payload.default_crossfade_ms }
                },
                payloadKeys: Object.keys(payload),
                fullPayload: payload
            };
            
            console.log('🔍 ATMOSPHERE PAYLOAD DEBUG:', JSON.stringify(debugInfo, null, 2));
            
            await this.service.saveAtmosphere(payload);
            
            logger.debug('membership', `Persisted ${sounds.length} members for atmosphere ${this.atmosphere.name}`);
        } catch (error) {
            logger.error('membership', 'Failed to persist atmosphere changes:', error);
            throw error;
        }
    }

    /**
     * Clear all members
     */
    clear() {
        this.members.clear();
        this.atmosphere = null;
        this.detailLoaded = false;
        
        if (this.persistTimer) {
            clearTimeout(this.persistTimer);
            this.persistTimer = null;
        }
    }

    /**
     * Get debug information
     * @returns {Object} Debug info
     */
    getDebugInfo() {
        return {
            atmosphere: this.atmosphere?.name || 'None',
            memberCount: this.members.size,
            detailLoaded: this.detailLoaded,
            hasPersistTimer: !!this.persistTimer
        };
    }
}