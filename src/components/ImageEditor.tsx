import { useState, useRef } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import '../styles/ImageEditor.css'

interface ImageFile {
  id: string
  file: File
  preview: string
  processing: boolean
}

export default function ImageEditor() {
  const [images, setImages] = useState<ImageFile[]>([])
  const [brightness, setBrightness] = useState(1)
  const [contrast, setContrast] = useState(1)
  const [saturation, setSaturation] = useState(1)
  const [hueShift, setHueShift] = useState(0)
  const [colorizeHue, setColorizeHue] = useState(0)
  const [processingAll, setProcessingAll] = useState(false)
  const fileInputRef = useRef<HTMLInputElement>(null)

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.currentTarget.files
    if (!files) return

    Array.from(files).forEach(file => {
      if (file.type.startsWith('image/')) {
        const preview = URL.createObjectURL(file)
        setImages(prev => [...prev, {
          id: Date.now() + Math.random().toString(),
          file,
          preview,
          processing: false
        }])
      }
    })
  }

  const removeImage = (id: string) => {
    setImages(prev => prev.filter(img => img.id !== id))
  }

  const processImage = async (imageFile: ImageFile, effects: {
    brightness: number
    contrast: number
    saturation: number
    hueShift: number
  }) => {
    try {
      setImages(prev => prev.map(img =>
        img.id === imageFile.id ? { ...img, processing: true } : img
      ))

      const inputPath = imageFile.file.path || imageFile.file.name
      const outputName = imageFile.file.name.replace(/\.[^/.]+$/, '') + '_enhanced.png'
      const outputPath = inputPath.replace(/[^\/\\]+$/, outputName)

      if (effects.brightness !== 1) {
        await invoke('adjust_brightness', {
          inputPath,
          outputPath,
          factor: effects.brightness
        })
      }

      if (effects.contrast !== 1) {
        await invoke('adjust_contrast', {
          inputPath: outputPath || inputPath,
          outputPath: outputPath || inputPath,
          factor: effects.contrast
        })
      }

      if (effects.saturation !== 1) {
        await invoke('adjust_saturation', {
          inputPath: outputPath || inputPath,
          outputPath: outputPath || inputPath,
          factor: effects.saturation
        })
      }

      if (effects.hueShift !== 0) {
        await invoke('shift_hue', {
          inputPath: outputPath || inputPath,
          outputPath: outputPath || inputPath,
          degrees: effects.hueShift
        })
      }

      setImages(prev => prev.map(img =>
        img.id === imageFile.id ? { ...img, processing: false } : img
      ))
    } catch (error) {
      console.error('Processing error:', error)
      setImages(prev => prev.map(img =>
        img.id === imageFile.id ? { ...img, processing: false } : img
      ))
    }
  }

  const processAllImages = async () => {
    setProcessingAll(true)
    const effects = {
      brightness,
      contrast,
      saturation,
      hueShift
    }

    for (const img of images) {
      await processImage(img, effects)
    }
    setProcessingAll(false)
  }

  return (
    <div className="image-editor">
      <div className="editor-container">
        <div className="controls-panel">
          <h2>Adjustment Controls</h2>

          <div className="control-group">
            <label>
              Brightness
              <input
                type="range"
                min="0.5"
                max="2"
                step="0.1"
                value={brightness}
                onChange={(e) => setBrightness(parseFloat(e.target.value))}
              />
              <span className="value">{brightness.toFixed(1)}</span>
            </label>
          </div>

          <div className="control-group">
            <label>
              Contrast
              <input
                type="range"
                min="0.5"
                max="2"
                step="0.1"
                value={contrast}
                onChange={(e) => setContrast(parseFloat(e.target.value))}
              />
              <span className="value">{contrast.toFixed(1)}</span>
            </label>
          </div>

          <div className="control-group">
            <label>
              Saturation
              <input
                type="range"
                min="0"
                max="2"
                step="0.1"
                value={saturation}
                onChange={(e) => setSaturation(parseFloat(e.target.value))}
              />
              <span className="value">{saturation.toFixed(1)}</span>
            </label>
          </div>

          <div className="control-group">
            <label>
              Hue Shift (degrees)
              <input
                type="range"
                min="0"
                max="360"
                step="1"
                value={hueShift}
                onChange={(e) => setHueShift(parseFloat(e.target.value))}
              />
              <span className="value">{hueShift.toFixed(0)}°</span>
            </label>
          </div>

          <div className="control-group">
            <label>
              Colorize Hue (degrees)
              <input
                type="range"
                min="0"
                max="360"
                step="1"
                value={colorizeHue}
                onChange={(e) => setColorizeHue(parseFloat(e.target.value))}
              />
              <span className="value">{colorizeHue.toFixed(0)}°</span>
            </label>
          </div>

          <div className="button-group">
            <button
              onClick={() => fileInputRef.current?.click()}
              className="btn btn-primary"
            >
              + Add Images
            </button>
            <button
              onClick={processAllImages}
              disabled={images.length === 0 || processingAll}
              className="btn btn-success"
            >
              {processingAll ? 'Processing...' : 'Process All'}
            </button>
          </div>

          <input
            ref={fileInputRef}
            type="file"
            multiple
            accept="image/*"
            onChange={handleFileSelect}
            style={{ display: 'none' }}
          />
        </div>

        <div className="images-panel">
          <h2>Images ({images.length})</h2>
          <div className="images-grid">
            {images.length === 0 ? (
              <div className="empty-state">
                <p>No images selected. Click "Add Images" to get started!</p>
              </div>
            ) : (
              images.map(img => (
                <div key={img.id} className="image-item">
                  <div className="image-preview">
                    <img src={img.preview} alt="preview" />
                    {img.processing && <div className="processing-overlay">Processing...</div>}
                  </div>
                  <div className="image-info">
                    <p className="image-name">{img.file.name}</p>
                    <button
                      onClick={() => removeImage(img.id)}
                      className="btn btn-sm btn-danger"
                      disabled={img.processing}
                    >
                      Remove
                    </button>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>
      </div>
    </div>
  )
}
