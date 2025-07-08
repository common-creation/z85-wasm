#!/usr/bin/env node

const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

// Check if wasm-pack is installed
function checkWasmPack() {
  return new Promise((resolve) => {
    const check = spawn('wasm-pack', ['--version'], { shell: true });
    check.on('error', () => resolve(false));
    check.on('close', (code) => resolve(code === 0));
  });
}

// Install wasm-pack
function installWasmPack() {
  console.log('Installing wasm-pack...');
  return new Promise((resolve, reject) => {
    const install = spawn('npm', ['install', '-g', 'wasm-pack'], { 
      shell: true,
      stdio: 'inherit'
    });
    install.on('close', (code) => {
      if (code === 0) {
        console.log('wasm-pack installed successfully!');
        resolve();
      } else {
        reject(new Error('Failed to install wasm-pack'));
      }
    });
  });
}

// Build the WASM module
function buildWasm() {
  console.log('Building WASM module...');
  return new Promise((resolve, reject) => {
    const build = spawn('wasm-pack', ['build', '--release', '--target', 'web', '--out-dir', 'pkg'], {
      shell: true,
      stdio: 'inherit'
    });
    build.on('close', (code) => {
      if (code === 0) {
        console.log('WASM module built successfully!');
        resolve();
      } else {
        reject(new Error('Failed to build WASM module'));
      }
    });
  });
}

// Main build process
async function main() {
  try {
    // Check if wasm-pack is installed
    const hasWasmPack = await checkWasmPack();
    
    if (!hasWasmPack) {
      console.log('wasm-pack not found.');
      console.log('Please install wasm-pack manually:');
      console.log('  npm install -g wasm-pack');
      console.log('or');
      console.log('  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh');
      process.exit(1);
    }

    // Build the WASM module
    await buildWasm();

    // Create TypeScript declaration file if it doesn't exist
    const pkgPath = path.join(__dirname, '..', 'pkg');
    const dtsPath = path.join(pkgPath, 'z85_wasm.d.ts');
    
    if (!fs.existsSync(dtsPath)) {
      console.log('Creating TypeScript declarations...');
      // wasm-pack should generate these automatically
    }

    console.log('Build completed successfully!');
  } catch (error) {
    console.error('Build failed:', error.message);
    process.exit(1);
  }
}

// Run the build
main();