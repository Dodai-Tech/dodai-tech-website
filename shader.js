// Plays the pre-rendered dodai-shaders frames behind the page, back and forth.
(() => {
  const c = document.querySelector('canvas.shader');
  if (!c) return;
  const n = +c.dataset.frames, base = c.dataset.shader, g = c.getContext('2d');
  const frames = Array.from({ length: n }, (_, i) => { const im = new Image(); im.src = `${base}${String(i).padStart(2, '0')}.jpg`; return im; });
  const draw = i => { const im = frames[i]; if (im.complete && im.naturalWidth) { c.width = im.naturalWidth; c.height = im.naturalHeight; g.drawImage(im, 0, 0); } };
  frames[0].onload = () => draw(0);
  if (matchMedia('(prefers-reduced-motion: reduce)').matches) return;
  let i = 0, d = 1, last = 0;
  const tick = t => { if (t - last > 90) { last = t; i += d; if (i >= n - 1 || i <= 0) d = -d; draw(i); } requestAnimationFrame(tick); };
  requestAnimationFrame(tick);
})();
