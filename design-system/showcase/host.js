// Runs the Dodai showcase in the page: the server build answers `GET /view` and `POST /act`;
// here the same answers come from the WebAssembly module, by wrapping `fetch`.
(async () => {
  const dir = new URL('.', document.currentScript.src).href;
  const { instance } = await WebAssembly.instantiateStreaming(fetch(dir + 'showcase.wasm'));
  const x = instance.exports;
  const dec = new TextDecoder(), enc = new TextEncoder();
  const clock = () => x.set_clock(Date.now() / 1000, -new Date().getTimezoneOffset());
  const out = () => dec.decode(new Uint8Array(x.memory.buffer, x.out_ptr(), x.out_len()));
  const copy = () => dec.decode(new Uint8Array(x.memory.buffer, x.copy_ptr(), x.copy_len()));

  // Shader pictures were rendered beforehand (see gen): `chain` or `preset` names the file.
  const picture = (q) => {
    const p = new URLSearchParams(q);
    return dir + 'shaders/' + (p.get('preset') || p.get('chain').split(',')[0]) + '.jpg';
  };

  const real = window.fetch.bind(window);
  window.fetch = async (url, init) => {
    const u = String(url);
    if (u === '/view') { clock(); x.view(); return new Response(out(), { headers: { 'Content-Type': 'text/html' } }); }
    if (u.startsWith('/act')) {
      clock();
      const a = enc.encode(init.body), p = x.alloc(a.length);
      new Uint8Array(x.memory.buffer, p, a.length).set(a);
      x.act(p, a.length, u.includes('part=side') ? 1 : 0);
      x.dealloc(p, a.length);
      const c = copy();
      return new Response(out(), { headers: c ? { 'X-Copy': encodeURIComponent(c) } : {} });
    }
    return real(url, init);
  };

  const mount = () => document.querySelectorAll('img[src^="/shader?"]').forEach((i) => { i.src = picture(i.dataset.q); });
  const seen = new MutationObserver(mount);

  clock(); x.view();
  document.body.innerHTML = out();
  mount();
  seen.observe(document.body, { childList: true, subtree: true });
  const s = document.createElement('script');
  s.src = dir + 'app.js';
  document.body.append(s);
})();
