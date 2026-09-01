# Setup Guide for Image Enhancer & Recolorer

This guide will help you set up the development environment for the Image Enhancer & Recolorer application.

## Prerequisites

Before you start, ensure you have the following installed:

### Required Software

1. **Node.js and npm**
   - Download from: https://nodejs.org/
   - Recommended version: LTS (16.x or higher)
   - Verify installation:
   ```bash
   node --version
   npm --version
   ```

2. **Rust**
   - Download from: https://www.rust-lang.org/tools/install
   - Windows: Run `rustup-init.exe`
   - macOS/Linux: Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - Verify installation:
   ```bash
   rustc --version
   cargo --version
   ```

3. **Git**
   - Download from: https://git-scm.com/
   - Verify installation:
   ```bash
   git --version
   ```

## Installation Steps

### Step 1: Clone and Navigate to Repository

```bash
git clone <your-repo-url>
cd image-enhancer
```

### Step 2: Install Node Dependencies

```bash
npm install
```

This will install all JavaScript/TypeScript dependencies including:
- React
- React-DOM
- Tauri
- TypeScript
- Vite
- And other required packages

### Step 3: Verify Rust Setup

```bash
rustc --version
cargo --version
```

If Rust is not installed, follow the instructions at https://www.rust-lang.org/tools/install

### Step 4: Install Tauri CLI (if needed)

```bash
npm install --save-dev @tauri-apps/cli
```

## Development Workflow

### Starting Development Mode

Open your terminal in the project root directory and run:

```bash
npm run tauri-dev
```

This command will:
1. Start the Vite dev server (usually on http://localhost:5173)
2. Build the Rust backend
3. Open the Tauri desktop application window
4. Enable hot-reload for React code changes

### Hot Reload

- **Frontend Changes**: React components automatically reload when you save files
- **Backend Changes**: Rust code requires rebuilding (the dev server will handle this)

## Building for Production

### Create a Production Build

```bash
npm run tauri-build
```

This command will:
1. Build the React app with Vite
2. Compile the Rust backend in release mode
3. Create an installer/bundle for your OS

### Output Locations

- **Windows**: `src-tauri/target/release/bundle/nsis/` (installer)
- **macOS**: `src-tauri/target/release/bundle/macos/` (app bundle and DMG)
- **Linux**: `src-tauri/target/release/bundle/appimage/` (AppImage and deb)

## Troubleshooting

### Issue: Node modules not found

**Solution**: Run `npm install` in the project root directory.

```bash
npm install
```

### Issue: Rust compilation errors

**Solution**: Ensure you have Rust installed and updated.

```bash
rustup update
cargo clean
npm run tauri-dev
```

### Issue: Port 5173 already in use

**Solution**: The Vite dev server will try alternative ports. Check the terminal output for the actual port being used.

Alternatively, you can specify a different port:

```bash
npm run dev -- --port 5174
```

### Issue: "Cannot find tauri" errors

**Solution**: Install Tauri CLI locally.

```bash
npm install --save-dev @tauri-apps/cli
npm run tauri-dev
```

### Issue: Rust build fails with OpenSSL errors (Windows)

**Solution**: Install Visual C++ build tools or use vcpkg.

```bash
rustup toolchain install stable-msvc
rustup default stable-msvc
npm run tauri-dev
```

## Project Structure

```
image-enhancer/
├── src/                    # React TypeScript frontend
│   ├── components/
│   ├── styles/
│   ├── App.tsx
│   └── main.tsx
├── src-tauri/             # Rust backend
│   ├── src/
│   │   └── main.rs       # Tauri/image processing code
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── public/                # Static assets
├── index.html            # HTML entry point
├── package.json          # npm dependencies
├── tsconfig.json         # TypeScript configuration
├── vite.config.ts        # Vite configuration
└── README.md             # Project documentation
```

## Available npm Scripts

```bash
npm run dev           # Start Vite dev server only
npm run build         # Build React app for production
npm run preview       # Preview production build
npm run tauri         # Run Tauri CLI
npm run tauri-dev     # Start development with Tauri (RECOMMENDED)
npm run tauri-build   # Build production application bundle
```

## Testing the Application

### Manual Testing

1. Start the dev server: `npm run tauri-dev`
2. The application window will open
3. Click "Add Images" and select some image files
4. Adjust the sliders for various effects
5. Click "Process All" to apply transformations
6. Check the same directory as your source images for `_enhanced.png` files

### Common Test Cases

- [ ] Add single image
- [ ] Add multiple images
- [ ] Adjust brightness
- [ ] Adjust contrast
- [ ] Adjust saturation
- [ ] Change hue
- [ ] Process all images
- [ ] Remove images from queue
- [ ] Try with different image formats (PNG, JPEG, BMP)

## Next Steps

1. Read the [README.md](./README.md) for feature documentation
2. Check out the React components in `src/components/`
3. Review the Rust backend code in `src-tauri/src/main.rs`
4. Explore Tauri documentation: https://tauri.app/
5. Learn about React: https://react.dev/

## Getting Help

- Check the troubleshooting section above
- Review Tauri docs: https://tauri.app/docs/
- Check Rust docs: https://doc.rust-lang.org/
- Open an issue in the repository

## Environment Variables

Create a `.env` file in the project root if needed:

```env
# Example environment variables
NODE_ENV=development
```

Tauri-specific env vars (for advanced users):
- `TAURI_CONFIG_DIR` - Path to Tauri config directory
- `TAURI_PRIVATE_KEY` - For code signing (production builds)

## Performance Tips

1. **Close unnecessary applications** when building
2. **Use SSD** for better compilation times
3. **Update Rust regularly**: `rustup update`
4. **Clear Cargo cache** if builds are slow: `cargo clean`

---

You're all set! Run `npm run tauri-dev` to start developing! 🚀
