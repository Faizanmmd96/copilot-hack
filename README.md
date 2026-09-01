# Image Enhancer & Recolorer

A powerful desktop application built with **Tauri** and **React** for enhancing and recoloring multiple images simultaneously.

## Features

### Image Enhancement
- **Brightness Adjustment** - Control image brightness (0.5x to 2x)
- **Contrast Adjustment** - Adjust contrast levels (0.5x to 2x)
- **Saturation Control** - Modify color saturation (0 to 2x)
- **Sharpening** - Enhance image details
- **Blur Effects** - Apply blur filters

### Image Recoloring
- **Hue Shift** - Rotate colors around the color wheel (0-360°)
- **Color Replacement** - Replace specific colors
- **Grayscale Conversion** - Convert images to black and white
- **Colorize** - Add color to grayscale images
- **Sepia Effect** - Classic sepia tone effect
- **Color Correction** - Fine-tune overall color balance

### Batch Processing
- Select and process multiple images at once
- Apply the same adjustments to all selected images
- Real-time preview of adjustments
- Progress tracking during processing

## Tech Stack

- **Frontend**: React + TypeScript
- **Build Tool**: Vite
- **Desktop Framework**: Tauri
- **Image Processing**: Rust (using `image` crate)
- **Styling**: CSS3 with modern gradients and animations

## Project Structure

```
.
├── src/                          # React frontend source
│   ├── components/
│   │   └── ImageEditor.tsx       # Main image editor component
│   ├── styles/
│   │   ├── index.css             # Global styles
│   │   ├── App.css               # App layout styles
│   │   └── ImageEditor.css       # Editor component styles
│   ├── App.tsx                   # Root React component
│   └── main.tsx                  # React entry point
├── src-tauri/                    # Tauri backend (Rust)
│   ├── src/
│   │   └── main.rs              # Backend image processing logic
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
├── index.html                    # HTML template
├── package.json                  # NPM dependencies
├── tsconfig.json                 # TypeScript configuration
├── vite.config.ts               # Vite configuration
└── README.md                     # This file
```

## Installation

### Prerequisites
- Node.js (v16 or higher)
- npm or yarn
- Rust (for building the desktop app)

### Setup

1. **Clone the repository**
```bash
git clone <repository-url>
cd image-enhancer
```

2. **Install dependencies**
```bash
npm install
```

3. **Install Tauri CLI** (if not already installed)
```bash
npm install --save-dev @tauri-apps/cli
```

## Development

### Start Development Server

Run both the React dev server and Tauri in development mode:

```bash
npm run tauri-dev
```

This will:
- Start the Vite dev server on `localhost:5173`
- Open the Tauri desktop window
- Hot-reload changes as you edit files

### Build for Development

```bash
npm run dev        # Run React dev server
npm run build      # Build React for production
npm run tauri      # Use Tauri CLI
```

## Building

### Build for Production

```bash
npm run tauri-build
```

This will create a production-ready desktop application bundle for your platform.

### Build Artifacts

- **Windows**: `src-tauri/target/release/` (installer and portable exe)
- **macOS**: `src-tauri/target/release/` (dmg and app bundle)
- **Linux**: `src-tauri/target/release/` (AppImage and deb)

## Usage

1. **Launch the application** - Run the desktop app
2. **Add Images** - Click the "Add Images" button to select one or more images
3. **Adjust Settings** - Use the sliders on the left to adjust:
   - Brightness
   - Contrast
   - Saturation
   - Hue Shift
   - Colorize Hue
4. **Preview** - See thumbnails of selected images in the grid
5. **Process All** - Click "Process All" to apply adjustments to all images
6. **Find Processed Images** - Enhanced images are saved in the same directory with `_enhanced.png` suffix

## Image Processing Commands

The following Rust functions handle image processing:

- `adjust_brightness(input_path, output_path, factor)` - Adjust brightness
- `adjust_contrast(input_path, output_path, factor)` - Adjust contrast
- `adjust_saturation(input_path, output_path, factor)` - Adjust saturation
- `convert_grayscale(input_path, output_path)` - Convert to grayscale
- `apply_sepia(input_path, output_path)` - Apply sepia tone
- `shift_hue(input_path, output_path, degrees)` - Shift hue
- `colorize_grayscale(input_path, output_path, hue)` - Colorize grayscale images

## Performance

- Rust backend ensures fast image processing
- Asynchronous operations keep UI responsive
- Parallel processing support through `rayon` crate

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Add Images | Ctrl/Cmd + O |
| Process All | Ctrl/Cmd + Enter |
| Clear All | Escape |

## Troubleshooting

### Images not processing?
- Ensure the image format is supported (PNG, JPEG, BMP, etc.)
- Check that you have write permissions in the image directory
- Try with a smaller image file

### Application won't start?
- Make sure Rust is installed: `rustc --version`
- Clear cache: `cargo clean` in `src-tauri/` directory
- Rebuild: `npm run tauri-build`

## Future Enhancements

- [ ] Batch export options
- [ ] Undo/Redo functionality
- [ ] Custom filter creation
- [ ] Before/After comparison view
- [ ] Drag-and-drop image support
- [ ] Additional filters (blur, sharpen, emboss)
- [ ] Profile/preset management
- [ ] Batch scheduling

## License

This project is open source and available under the MIT License.

## Support

For issues, questions, or suggestions, please open an issue on the GitHub repository.

---

Made with ❤️ using Tauri and React
