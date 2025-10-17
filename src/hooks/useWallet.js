// src/hooks/useWallet.js
import { useState } from 'react'

export function useWallet() {
  const [address, setAddress] = useState(null)

  const login = async (privateKey) => {
    const res = await fetch('http://localhost:42000/api/login', {
      method: 'POST',
      body: JSON.stringify({ privateKey }),
      headers: { 'Content-Type': 'application/json' },
    })
    const data = await res.json()
    setAddress(data.address)
  }

  return { address, login }
}
