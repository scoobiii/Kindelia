import React, { createContext, useContext, useEffect, useState } from "react";

/**
 * Kindelia Frontend - App.jsx
 * Fully integrated prototype for Kindelia Energy
 * - Multi-user types: residential, commercial, industrial, plant
 * - Wallet + Grandma Mode
 * - Marketplace & investor dashboard
 * - IoT production dashboard (mocked)
 * - Smart contracts HVM integration (mocked)
 * - Accessibility: UX60, TTS, high contrast
 */

// ---------- Mock API helpers (replace with real RPC calls) ----------
async function apiPostTransaction(payload, signature) {
  await new Promise((r) => setTimeout(r, 400));
  return { ok: true, txid: `tx_${Math.random().toString(36).slice(2, 9)}` };
}

async function apiGetBalance(address) {
  await new Promise((r) => setTimeout(r, 200));
  return { ok: true, balance: 1000 };
}

// ---------- Wallet Mock ----------
function deriveKeysFromSeed(seed) {
  const hash = Array.from(seed + "::kindelia").reduce((acc, c) => acc + c.charCodeAt(0), 0);
  const pub = `KIND-PUB-${hash.toString(36)}`;
  const priv = `KIND-PRIV-${hash.toString(36)}-${seed.length}`;
  return { pubkey: pub, privkey: priv };
}

function signMock(privkey, data) {
  const s = privkey + ":" + btoa(data).slice(0, 16);
  return s;
}

// ---------- Wallet Context ----------
const WalletContext = createContext(null);

export function useWallet() {
  return useContext(WalletContext);
}

export function WalletProvider({ children }) {
  const [seed, setSeed] = useState(localStorage.getItem("kind_seed") || "seed-grandma");
  const [keys, setKeys] = useState(deriveKeysFromSeed(seed));
  const [address, setAddress] = useState(keys.pubkey.slice(0, 12));
  const [balance, setBalance] = useState(0);
  const [history, setHistory] = useState([]);

  useEffect(() => setKeys(deriveKeysFromSeed(seed)), [seed]);
  useEffect(() => setAddress(keys.pubkey.slice(0, 12)), [keys]);
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

  async function sendContractTx(contract, method, params) {
    const payload = JSON.stringify({ contract, method, params });
    const sig = signMock(keys.privkey, payload);
    const res = await apiPostTransaction(payload, sig);
    return res;
  }

  const api = { seed, setSeed, keys, address, balance, history, sendSimple, sendContractTx };
  return <WalletContext.Provider value={api}>{children}</WalletContext.Provider>;
}

// ---------- Accessibility / UX60 utilities ----------
function useTTS() {
  const speak = (text) => {
    if (typeof window === "undefined") return;
    if (window.speechSynthesis) {
      const u = new SpeechSynthesisUtterance(text);
      u.rate = 0.95; u.pitch = 1.0;
      window.speechSynthesis.cancel();
      window.speechSynthesis.speak(u);
    } else console.log("TTS: ", text);
  };
  return { speak };
}

function useSTT(onIntent) {
  useEffect(() => {
    if (!("webkitSpeechRecognition" in window || "SpeechRecognition" in window)) return;
    const ctor = window.SpeechRecognition || window.webkitSpeechRecognition;
    const rec = new ctor();
    rec.lang = "pt-BR"; rec.interimResults = false; rec.maxAlternatives = 1;
    rec.onresult = (ev) => {
      const t = ev.results[0][0].transcript.toLowerCase();
      const pt = /enviar\s+(\d+)\s+(?:para|p/)\s+([^\s]+)/i.exec(t);
      if (pt) return onIntent({ type: "send", to: pt[2], amount: parseInt(pt[1], 10) });
      const en = /send\s+(\d+)\s+to\s+([^\s]+)/i.exec(t);
      if (en) return onIntent({ type: "send", to: en[2], amount: parseInt(en[1], 10) });
      onIntent({ type: "unknown", raw: t });
    };
    rec.onerror = (e) => console.warn("STT error", e);
    (window as any)._kindelia_recognition = rec;
  }, [onIntent]);
}

// ---------- UI Components ----------
function UserTypeSelector({ type, setType }) {
  return (
    <div className="mb-3">
      <label>Tipo de usuário</label>
      <select value={type} onChange={e => setType(e.target.value)} className="w-full p-2 border rounded">
        <option value="residential">Residencial</option>
        <option value="commercial">Comercial</option>
        <option value="industrial">Industrial</option>
        <option value="plant">Usina</option>
      </select>
    </div>
  );
}

function GrandmaModePanel({ userType }) {
  const { address, balance, sendSimple, keys } = useWallet();
  const { speak } = useTTS();
  const [to, setTo] = useState(""); const [amount, setAmount] = useState(1);
  const [message, setMessage] = useState(""); const [processing, setProcessing] = useState(false);

  useEffect(() => speak("Modo Avó ativado. Olá, bem-vinda."), []);

  async function doOneClick() {
    setProcessing(true); setMessage("Enviando..."); speak("Enviando. Por favor aguarde.");
    const res = await sendSimple(to || "friend", amount || 1);
    if (res.ok) { setMessage(`Enviado ✅ ID: ${res.txid}`); speak("Transação enviada com sucesso."); }
    else { setMessage("Erro ao enviar"); speak("Erro ao enviar transação."); }
    setProcessing(false);
  }

  return (
    <section aria-labelledby="grandma-title" className="p-4 rounded-lg shadow-md bg-white">
      <h2 id="grandma-title" className="text-2xl font-bold mb-2">Modo Avó — Enviar em 1 clique</h2>
      <p>Tipo de usuário: <strong>{userType}</strong></p>
      <p>Endereço: <strong>{address}</strong></p>
      <p>Saldo aproximado: <strong>{balance}</strong></p>

      <div className="mb-3">
        <label>Enviar para</label>
        <input value={to} onChange={(e) => setTo(e.target.value)} className="w-full p-3 border rounded" placeholder="Nome ou endereço" />
      </div>

      <div className="mb-3">
        <label>Quantidade</label>
        <input type="number" value={amount} onChange={(e) => setAmount(Number(e.target.value))} className="w-36 p-3 border rounded" />
      </div>

      <div className="flex items-center gap-3">
        <button onClick={doOneClick} disabled={processing} className="bg-blue-600 text-white px-4 py-3 rounded-lg">{processing ? "Enviando..." : "Enviar (1-clique)"}</button>
      </div>

      <div className="mt-3 text-sm text-gray-600">{message}</div>
      <div className="mt-4 text-xs text-gray-500">Chave pública (resumo): {keys.pubkey.slice(0,12)}</div>
    </section>
  );
}

function Marketplace() {
  const { balance } = useWallet();
  const [listings, setListings] = useState([]);
  useEffect(() => setListings([{ id: "tx1", energy: 100, price: 50 }]), []);
  return (
    <section className="p-4 rounded bg-white shadow mt-6">
      <h3 className="font-bold mb-2">Marketplace de Energia</h3>
      <ul>
        {listings.map(l => (
          <li key={l.id} className="mb-2">
            Energia: {l.energy} kWh — Preço: {l.price} • <button className="bg-blue-500 text-white px-2 py-1 rounded">Comprar</button>
          </li>
        ))}
      </ul>
      <div className="mt-2 text-sm text-gray-500">Saldo atual: {balance} tokens</div>
    </section>
  );
}

function ProductionDashboard({ plantId }) {
  const [data, setData] = useState({ power: 0, energyToday: 0 });
  useEffect(() => {
    const interval = setInterval(() => setData({ power: Math.floor(Math.random() * 1000), energyToday: Math.floor(Math.random() * 5000) }), 2000);
    return () => clearInterval(interval);
  }, [plantId]);
  return (
    <section className="p-4 rounded bg-white shadow mt-6">
      <h3 className="font-bold mb-2">Dashboard de Produção</h3>
      <div>Potência atual: {data.power} kW</div>
      <div>Energia hoje: {data.energyToday} kWh</div>
    </section>
  );
}

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

function AccessibilityControls({ onToggleLarge }) {
  const [big, setBig] = useState(localStorage.getItem("ui:big") === "1");
  const [contrast, setContrast] = useState(localStorage.getItem("ui:contrast") === "high");

  function toggleBig() { const n = !big; setBig(n); localStorage.setItem("ui:big", n ? "1" : "0"); onToggleLarge(n); }
  function toggleContrast() { const n = !contrast; setContrast(n); localStorage.setItem("ui:contrast", n ? "high" : "normal"); document.documentElement.style.setProperty("--kind-bg", n ? "#111827" : "#ffffff"); document.documentElement.style.setProperty("--kind-fg", n ? "#ffffff" : "#111827"); }

  return (
    <div className="p-3 flex gap-3 items-center">
      <label className="flex items-center gap-2"><input type="checkbox" checked={big} onChange={toggleBig} /><span>Fonte grande</span></label>
      <label className="flex items-center gap-2"><input type="checkbox" checked={contrast} onChange={toggleContrast} /><span>Alto contraste</span></label>
    </div>
  );
}

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

// ---------- Main App ----------
export default function App() {
  const [large, setLarge] = useState(localStorage.getItem("ui:big") === "1");
  const [intent, setIntent] = useState(null);
  const [userType, setUserType] = useState("residential");

  function handleIntent(i) {
    setIntent(i);
    if (i && i.type === "send") {
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
            <AccessibilityControls onToggleLarge={(v) => { setLarge(v); document.documentElement.style.fontSize = v ? "120%" : "100%"; }} />
          </header>

          <main style={{ display: "grid", gridTemplateColumns: "1fr 420px", gap: 20 }}>
            <div>
              <UserTypeSelector type={userType} setType={setUserType} />
              <GrandmaModePanel userType={userType} />
              <Marketplace />
              {userType === "plant" && <ProductionDashboard plantId="plant_001" />}
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
