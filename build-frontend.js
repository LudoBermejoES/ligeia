#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

console.log('Building frontend distribution...');

// Create dist directory
const distDir = path.join(__dirname, 'dist');
if (fs.existsSync(distDir)) {
    fs.rmSync(distDir, { recursive: true, force: true });
}
fs.mkdirSync(distDir, { recursive: true });

// Copy function that preserves directory structure
function copyRecursive(src, dest, excludeDirs = []) {
    const stats = fs.statSync(src);
    
    if (stats.isDirectory()) {
        const basename = path.basename(src);
        if (excludeDirs.includes(basename)) {
            console.log(`Skipping directory: ${src}`);
            return;
        }
        
        if (!fs.existsSync(dest)) {
            fs.mkdirSync(dest, { recursive: true });
        }
        
        const entries = fs.readdirSync(src);
        for (const entry of entries) {
            const srcPath = path.join(src, entry);
            const destPath = path.join(dest, entry);
            copyRecursive(srcPath, destPath, excludeDirs);
        }
    } else {
        fs.copyFileSync(src, dest);
        console.log(`Copied: ${src} -> ${dest}`);
    }
}

// Copy all src-fe contents except node_modules
const srcFe = path.join(__dirname, 'src-fe');
copyRecursive(srcFe, distDir, ['node_modules']);

console.log('Frontend distribution built successfully!');
console.log(`Distribution created at: ${distDir}`);