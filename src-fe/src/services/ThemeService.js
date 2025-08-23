/**
 * ThemeService - Manages theme loading and switching
 */
export class ThemeService {
    constructor() {
        this.currentTheme = null;
        this.availableThemes = new Map();
        this.loadedThemes = new Set();
        this.themeStyleElement = null;
    }

    async initialize() {
        try {
            // Load available themes
            await this.loadAvailableThemes();
            
            // Load default theme
            await this.loadTheme('default');
            
            console.log('ThemeService initialized successfully');
            return true;
        } catch (error) {
            console.error('Failed to initialize ThemeService:', error);
            return false;
        }
    }

    async loadAvailableThemes() {
        try {
            // For now, we'll manually define available themes
            // In the future, this could scan the themes directory
            const themes = [
                {
                    slug: 'default',
                    name: 'Default',
                    description: 'The default Ligeia theme',
                    category: 'general'
                },
                {
                    slug: 'fantasy',
                    name: 'Fantasy',
                    description: 'A rich fantasy theme inspired by classic tabletop RPG aesthetics',
                    category: 'rpg'
                },
                {
                    slug: 'horror',
                    name: 'Horror',
                    description: 'A dark, atmospheric horror theme with deep reds, blacks, and eerie styling',
                    category: 'horror'
                },
                {
                    slug: 'superheroes',
                    name: 'Superheroes',
                    description: 'A vibrant superhero theme inspired by classic comic book aesthetics with bold colors, dynamic patterns, and heroic styling',
                    category: 'superheroes'
                }
                // Add more themes here as they're created
            ];

            themes.forEach(theme => {
                this.availableThemes.set(theme.slug, theme);
            });

            console.log(`Loaded ${themes.length} available themes`);
        } catch (error) {
            console.error('Failed to load available themes:', error);
        }
    }

    async loadTheme(themeSlug) {
        try {
            if (!this.availableThemes.has(themeSlug)) {
                throw new Error(`Theme '${themeSlug}' not found`);
            }

            // Load theme configuration
            const configResponse = await fetch(`./themes/${themeSlug}/theme.json`);
            const themeConfig = await configResponse.json();

            // Load theme CSS
            const cssResponse = await fetch(`./themes/${themeSlug}/theme.css`);
            const themeCss = await cssResponse.text();

            // Apply theme CSS
            this.applyThemeCSS(themeCss, themeSlug);

            // Store current theme
            this.currentTheme = {
                slug: themeSlug,
                config: themeConfig,
                css: themeCss
            };

            this.loadedThemes.add(themeSlug);
            console.log(`Loaded theme: ${themeConfig.name}`);

            // Dispatch theme change event
            this.dispatchThemeChangeEvent(themeSlug, themeConfig);

            return true;
        } catch (error) {
            console.error(`Failed to load theme '${themeSlug}':`, error);
            return false;
        }
    }

    applyThemeCSS(css, themeSlug) {
        // Remove existing theme style element
        if (this.themeStyleElement) {
            this.themeStyleElement.remove();
        }

        // Create new style element
        this.themeStyleElement = document.createElement('style');
        this.themeStyleElement.id = `theme-${themeSlug}`;
        this.themeStyleElement.textContent = css;

        // Insert into document head
        document.head.appendChild(this.themeStyleElement);
    }

    async switchTheme(themeSlug) {
        if (this.currentTheme?.slug === themeSlug) {
            console.log(`Theme '${themeSlug}' is already active`);
            return true;
        }

        return await this.loadTheme(themeSlug);
    }

    getCurrentTheme() {
        return this.currentTheme;
    }

    getAvailableThemes() {
        return Array.from(this.availableThemes.values());
    }

    isThemeLoaded(themeSlug) {
        return this.loadedThemes.has(themeSlug);
    }

    dispatchThemeChangeEvent(themeSlug, themeConfig) {
        const event = new CustomEvent('themeChanged', {
            detail: {
                slug: themeSlug,
                config: themeConfig,
                previousTheme: this.currentTheme?.slug
            }
        });
        document.dispatchEvent(event);
    }

    // Theme-specific helper methods
    getThemeColor(colorPath) {
        if (!this.currentTheme?.config?.colors) return null;
        
        const pathParts = colorPath.split('.');
        let value = this.currentTheme.config.colors;
        
        for (const part of pathParts) {
            value = value?.[part];
            if (value === undefined) return null;
        }
        
        return value;
    }

    getThemeVariable(variableName) {
        return getComputedStyle(document.documentElement)
            .getPropertyValue(`--${variableName}`)
            .trim();
    }

    setThemeVariable(variableName, value) {
        document.documentElement.style.setProperty(`--${variableName}`, value);
    }
}