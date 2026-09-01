import { useState } from 'react'
import ImageEditor from './components/ImageEditor'
import './styles/App.css'

function App() {
  return (
    <div className="app">
      <header className="app-header">
        <h1>🎨 Image Enhancer & Recolorer</h1>
        <p>Enhance and recolor multiple images with ease</p>
      </header>
      <main className="app-main">
        <ImageEditor />
      </main>
    </div>
  )
}

export default App
