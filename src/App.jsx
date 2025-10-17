import React, { createContext, useContext, useEffect, useState } from "react";

/**
 * Kindelia Frontend - Single File React App
 * - Implements Wallet UI (Grandma Mode / UX60)
 * - Uses Web Speech API for TTS and (optional) STT
 * - Mocked backend API calls to /api/* endpoints (replace with real endpoints)
 * - Accessible: large fonts, high contrast, ARIA labels
 *
 * To use:
 * 1) Create a new Vite React app: `npm create vite@latest kindelia-frontend -- --template react`
 * 2) Install (optional) Tailwind and configure or use the inline classes
 * 3) Replace src/App.jsx with this file and run `npm install && npm run dev`
 *
 * This file is intentionally self-contained and avoids third-party dependencies.
 */

// ---------- Mock API helpers (replace with real RPC calls) ----------
async function apiPostTransaction(payload, signature) {
  // Mock delay
  await new Promise((r) => setTimeout(r, 400));
  // Return mock txid
  return { ok: true, txid: `tx_${Math.random().toString(36).slice(2, 9)}` };
}

async function apiGetBalance(address) {
  await new Promise((r) => setTimeout(r, 200));
  return { ok: true, balance: 1000 };
}

// ---------- Wallet Mock (mirrors Rust mocks) ----------
function deriveKeysFromSeed(seed) {
  // very small deterministic mock (not secure). Replace with real crypto.
  const hash = Array.from(seed + "::kindelia").reduce((acc, c) => acc + c.charCodeAt(0), 0);
  const pub = `KIND-PUB-${hash.toString(36)}`;
  const priv = `KIND-PRIV-${hash.toString(36)}-${seed.length}`;
  return { pubkey: pub, privkey: priv };
}

function signMock(privkey, data) {
  // return a pseudo-signature
  const s = privkey + ":" + btoa(data).slice(0, 16);
  return s;
}

// ---------- Wallet Context ----------
const WalletContext = createContext(null);

export function useWallet() {
  return useContext(WalletContext);
}

export function WalletProvider({ children }) {
  const [seed, setSeed] = useState(() => localStorage.getItem("kind_seed") || "seed-grandma");
  const [keys, setKeys] = useState(() => deriveKeysFromSeed(seed));
  const [address, setAddress] = useState(() => keys.pubkey.slice(0, 12));
  const [balance, setBalance] = useState(0);
  const [history, setHistory] = useState([]);

  useEffect(() => {
    setKeys(deriveKeysFromSeed(seed));
  }, [seed]);

  useEffect(() => {
    setAddress(keys.pubkey.slice(0, 12));
  }, [keys]);

  useEffect(() => {
    let mounted = true;
    apiGetBalance(address).then((r) => {
      if (mounted && r.ok) setBalance(r.balance);
    });
    return () => (mounted = false);
  }, [address]);

  async function sendSimple(to, amount) {
    const payload = `send:${to}:${amount}`;
    const sig = signMock(keys.privkey, payload);
    const res = await apiPostTransaction(payload, sig);
    if (res.ok) {
      const txid = res.txid;
      const entry = { txid, to, amount, date: new Date().toISOString() };
      setHistory((h) => [entry, ...h]);
      return { ok: true, txid };
    }
    return { ok: false, err: "post-failed" };
  }

  const api = { seed, setSeed, keys, address, balance, history, sendSimple };

  return <WalletContext.Provider value={api}>{children}</WalletContext.Provider>;
}

// ---------- Accessibility / UX60 utilities ----------
function useTTS() {
  const speak = (text) => {
    if (typeof window === "undefined") return;
    if (window.speechSynthesis) {
      const u = new SpeechSynthesisUtterance(text);
      // prefer local language, moderate rate
      u.rate = 0.95;
      u.pitch = 1.0;
      window.speechSynthesis.cancel();
      window.speechSynthesis.speak(u);
    } else {
      console.log("TTS: ", text);
    }
  };
  return { speak };
}

function useSTT(onIntent) {
  useEffect(() => {
    if (!("webkitSpeechRecognition" in window || "SpeechRecognition" in window)) return;
    const ctor = window.SpeechRecognition || window.webkitSpeechRecognition;
    const rec = new ctor();
    rec.lang = "pt-BR";
    rec.interimResults = false;
    rec.maxAlternatives = 1;
    rec.onresult = (ev) => {
      const t = ev.results[0][0].transcript;
      // basic parse: "enviar 10 para joao" or "send 5 to alice"
      const lower = t.toLowerCase();
      const pt = /enviar\s+(\d+)\s+(?:para|p/)\s+([^\s]+)/i.exec(lower);
      if (pt) {
        onIntent({ type: "send", to: pt[2], amount: parseInt(pt[1], 10) });
        return;
      }
      const en = /send\s+(\d+)\s+to\s+([^\s]+)/i.exec(lower);
      if (en) {
        onIntent({ type: "send", to: en[2], amount: parseInt(en[1], 10) });
        return;
      }
      onIntent({ type: "unknown", raw: t });
    };
    rec.onerror = (e) => console.warn("STT error", e);
    // expose for manual control
    (window as any)._kindelia_recognition = rec;
  }, [onIntent]);
}

// ---------- Grandma Mode UI Component ----------
function GrandmaModePanel() {
  const { address, balance, sendSimple, keys } = useWallet();
  const { speak } = useTTS();
  const [to, setTo] = useState("");
  const [amount, setAmount] = useState(1);
  const [message, setMessage] = useState("");
  const [processing, setProcessing] = useState(false);

  useEffect(() => {
    // on mount greet
    speak("Modo Avó ativado. Olá, bem-vinda.");
  }, []);

  async function doOneClick() {
    setProcessing(true);
    setMessage("Enviando...");
    speak("Enviando. Por favor aguarde.");
    const res = await sendSimple(to || "friend", amount || 1);
    if (res.ok) {
      setMessage(`Enviado ✅ ID: ${res.txid}`);
      speak("Transação enviada com sucesso.");
    } else {
      setMessage("Erro ao enviar");
      speak("Erro ao enviar transação.");
    }
    setProcessing(false);
  }

  return (
    <section aria-labelledby="grandma-title" className="p-4 rounded-lg shadow-md bg-white" style={{ maxWidth: 720 }}>
      <h2 id="grandma-title" className="text-2xl font-bold mb-2">Modo Avó — Enviar em 1 clique</h2>
      <p className="mb-3">Endereço: <strong>{address}</strong></p>
      <p className="mb-3">Saldo aproximado: <strong>{balance}</strong></p>

      <div className="mb-3">
        <label className="block mb-1" htmlFor="to">Enviar para</label>
        <input id="to" aria-label="destinatario" value={to} onChange={(e) => setTo(e.target.value)} className="w-full p-3 border rounded" placeholder="Nome ou endereço" />
      </div>

      <div className="mb-3">
        <label className="block mb-1" htmlFor="amount">Quantidade</label>
        <input id="amount" aria-label="quantidade" type="number" value={amount} onChange={(e) => setAmount(Number(e.target.value))} className="w-36 p-3 border rounded" />
      </div>

      <div className="flex items-center gap-3">
        <button onClick={doOneClick} disabled={processing} className="bg-blue-600 text-white px-4 py-3 rounded-lg" aria-live="polite">{processing ? "Enviando..." : "Enviar (1-clique)"}</button>
        <button onClick={() => { speak("Confirmar envio: " + amount + " unidades para " + (to || "amigo")); setMessage("Confirme no seu dispositivo"); }} className="border px-3 py-2 rounded">Falar</button>
      </div>

      <div className="mt-3">
        <p className="text-sm text-gray-600">{message}</p>
      </div>

      <div className="mt-4 text-xs text-gray-500">Chave pública (resumo): {keys.pubkey.slice(0,12)}</div>
    </section>
  );
}

// ---------- Voice & STT Controller ----------
function VoiceController({ onIntent }) {
  const { speak } = useTTS();
  useSTT(onIntent);
  const startListening = () => {
    const rec = (window as any)._kindelia_recognition;
    if (rec) rec.start();
    else alert("Reconhecimento de voz não suportado no navegador");
  };
  return (
    <div className="p-3">
      <button onClick={() => { speak("Assistente ativado"); startListening(); }} className="bg-green-600 text-white px-3 py-2 rounded">Ativar Assistente por Voz</button>
    </div>
  );
}

// ---------- Accessibility Controls ----------
function AccessibilityControls({ onToggleLarge }) {
  const [big, setBig] = useState(localStorage.getItem("ui:big") === "1");
  const [contrast, setContrast] = useState(localStorage.getItem("ui:contrast") === "high");

  function toggleBig() {
    const n = !big;
    setBig(n);
    localStorage.setItem("ui:big", n ? "1" : "0");
    onToggleLarge(n);
  }

  function toggleContrast() {
    const n = !contrast;
    setContrast(n);
    localStorage.setItem("ui:contrast", n ? "high" : "normal");
    document.documentElement.style.setProperty("--kind-bg", n ? "#111827" : "#ffffff");
    document.documentElement.style.setProperty("--kind-fg", n ? "#ffffff" : "#111827");
  }

  return (
    <div className="p-3 flex gap-3 items-center">
      <label className="flex items-center gap-2">
        <input type="checkbox" checked={big} onChange={toggleBig} />
        <span>Fonte grande</span>
      </label>
      <label className="flex items-center gap-2">
        <input type="checkbox" checked={contrast} onChange={toggleContrast} />
        <span>Alto contraste</span>
      </label>
    </div>
  );
}

// ---------- Main App ----------
export default function App() {
  const [large, setLarge] = useState(localStorage.getItem("ui:big") === "1");
  const [intent, setIntent] = useState(null);

  function handleIntent(i) {
    setIntent(i);
    // auto-trigger grandma mode send for voice intents
    if (i && i.type === "send") {
      // broadcast a message that Grandma Mode panel can listen to
      const ev = new CustomEvent("kindelia_voice_intent", { detail: i });
      window.dispatchEvent(ev);
    }
  }

  useEffect(() => {
    document.documentElement.style.setProperty("--kind-bg", localStorage.getItem("ui:contrast") === "high" ? "#111827" : "#ffffff");
    document.documentElement.style.setProperty("--kind-fg", localStorage.getItem("ui:contrast") === "high" ? "#ffffff" : "#111827");
  }, []);

  return (
    <WalletProvider>
      <div style={{ background: "var(--kind-bg)", color: "var(--kind-fg)", minHeight: "100vh", padding: 20 }}>
        <div style={{ maxWidth: 1100, margin: "0 auto" }}>
          <header style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 20 }}>
            <h1 style={{ fontSize: large ? 36 : 24, margin: 0 }}>Kindelia — Wallet (Modo Avó)</h1>
            <div>
              <AccessibilityControls onToggleLarge={(v) => { setLarge(v); document.documentElement.style.fontSize = v ? "120%" : "100%"; }} />
            </div>
          </header>

          <main style={{ display: "grid", gridTemplateColumns: "1fr 420px", gap: 20 }}>
            <div>
              <GrandmaModePanel />

              <section className="mt-6 p-4 rounded bg-white shadow">
                <h3 className="font-bold mb-2">Assistente de Voz</h3>
                <VoiceController onIntent={handleIntent} />
                <div className="mt-2 text-sm text-gray-600">Diga: "Enviar 10 para joao" (pt) ou "Send 5 to alice" (en)</div>
              </section>

              <section className="mt-6 p-4 rounded bg-white shadow">
                <h3 className="font-bold mb-2">Histórico de Transações</h3>
                <TransactionHistory />
              </section>
            </div>

            <aside>
              <ProfileCard />
              <section className="mt-6 p-4 rounded bg-white shadow">
                <h4 className="font-bold mb-2">Preferências</h4>
                <p className="text-sm">Ative TTS no navegador para feedback de voz.</p>
              </section>
            </aside>
          </main>

          <footer style={{ marginTop: 30, fontSize: 12, color: "#9CA3AF" }}>
            Kindelia — protótipo de wallet inclusiva • Não use em produção
          </footer>
        </div>
      </div>
    </WalletProvider>
  );
}

// ---------- Small Components ----------
function TransactionHistory() {
  const { history } = useWallet();
  if (!history || history.length === 0) return <div className="p-3">Nenhuma transação ainda</div>;
  return (
    <ul className="p-3">
      {history.map((h) => (
        <li key={h.txid} className="mb-2">
          <div className="font-semibold">ID: {h.txid}</div>
          <div className="text-sm">To: {h.to} • Amount: {h.amount}</div>
          <div className="text-xs text-gray-500">{new Date(h.date).toLocaleString()}</div>
        </li>
      ))}
    </ul>
  );
}

function ProfileCard() {
  const { address, balance, keys } = useWallet();
  return (
    <div className="p-4 rounded bg-white shadow">
      <h3 className="font-bold mb-2">Conta</h3>
      <div className="mb-2">Endereço: <strong>{address}</strong></div>
      <div className="mb-2">Saldo: <strong>{balance}</strong></div>
      <div className="text-xs text-gray-500">Pubkey: {keys.pubkey.slice(0, 24)}</div>
    </div>
  );
}
