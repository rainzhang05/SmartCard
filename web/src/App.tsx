import { useState } from 'react'
import './App.css'

function App() {
  const [status, setStatus] = useState('Disconnected')

  const connect = () => {
    setStatus('Connecting...')

    // Send message to extension
    window.postMessage({
      webcard: 'request',
      i: crypto.randomUUID(),
      c: 'list_readers',
      p: {}
    }, window.location.origin)

    // Listen for response
    const handleResponse = (event: MessageEvent) => {
      if (event.source !== window) return
      if (event.data.webcard === 'response') {
        const response = event.data
        if (response.s === 'success') {
          const readers = response.d as string[]
          if (readers.length > 0) {
            setStatus(`Connected: ${readers[0]}`)
          } else {
            setStatus('No readers found')
          }
        } else {
          setStatus(`Error: ${response.e}`)
        }
        window.removeEventListener('message', handleResponse)
      }
    }

    window.addEventListener('message', handleResponse)
  }

  return (
    <div className="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center p-4">
      <h1 className="text-4xl font-bold mb-8 bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent">
        PIV Management Platform
      </h1>

      <div className="bg-gray-800 p-8 rounded-xl shadow-lg border border-gray-700 max-w-md w-full">
        <div className="flex items-center justify-between mb-6">
          <span className="text-gray-400">Status</span>
          <span className={`px-3 py-1 rounded-full text-sm ${status.includes('Connected') ? 'bg-green-500/20 text-green-400' : 'bg-red-500/20 text-red-400'
            }`}>
            {status}
          </span>
        </div>

        <button
          onClick={connect}
          className="w-full bg-blue-600 hover:bg-blue-500 text-white font-medium py-3 px-4 rounded-lg transition-colors duration-200 mb-6"
        >
          Connect to Device
        </button>

        <div className="grid grid-cols-2 gap-4 pt-6 border-t border-gray-700">
          <a href="#" className="flex flex-col items-center p-4 bg-gray-700/50 rounded-lg hover:bg-gray-700 transition-colors">
            <span className="text-sm font-medium text-gray-300">Native Host</span>
            <span className="text-xs text-gray-500 mt-1">Download Installer</span>
          </a>
          <a href="#" className="flex flex-col items-center p-4 bg-gray-700/50 rounded-lg hover:bg-gray-700 transition-colors">
            <span className="text-sm font-medium text-gray-300">Extension</span>
            <span className="text-xs text-gray-500 mt-1">Download CRX</span>
          </a>
        </div>
      </div>
    </div>
  )
}

export default App
