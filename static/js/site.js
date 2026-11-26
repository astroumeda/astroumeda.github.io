// Small, dependency-free interactions. Everything here is optional decoration:
// the page works without JavaScript.
(() => {
  "use strict";
  const root = document.documentElement;
  const calm = matchMedia("(prefers-reduced-motion: reduce)").matches;
  const isDark = () =>
    root.dataset.theme ? root.dataset.theme === "dark" : matchMedia("(prefers-color-scheme: dark)").matches;
  const css = (name) => getComputedStyle(root).getPropertyValue(name).trim();

  // ---------------------------------------------------------------- theme toggle
  document.querySelector(".theme-toggle")?.addEventListener("click", () => {
    root.dataset.theme = isDark() ? "light" : "dark";
    localStorage.setItem("theme", root.dataset.theme);
  });

  // ---------------------------------------------------------------- email: un-obfuscate on load
  const mail = document.querySelector(".email-link");
  if (mail) {
    const addr = `${mail.dataset.user}@${mail.dataset.domain}`;
    mail.textContent = addr;
    mail.href = `mailto:${addr}`;
    const copy = document.querySelector(".copy-btn");
    if (copy && navigator.clipboard) {
      copy.hidden = false;
      copy.addEventListener("click", async () => {
        await navigator.clipboard.writeText(addr);
        copy.textContent = "Copied ✓";
        setTimeout(() => (copy.textContent = "Copy"), 1600);
      });
    }
  }

  // ---------------------------------------------------------------- hero starfield
  // Twinkling stars that drift slightly with the mouse. Stars are drawn in the
  // text colour, so they show in both light and dark mode.
  const canvas = document.querySelector(".starfield");
  if (canvas) {
    const ctx = canvas.getContext("2d");
    let stars = [], w = 0, h = 0, mx = 0, my = 0, t = 0, running = true;
    const resize = () => {
      const dpr = Math.min(devicePixelRatio || 1, 2);
      w = canvas.clientWidth; h = canvas.clientHeight;
      canvas.width = w * dpr; canvas.height = h * dpr;
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      const n = Math.round((w * h) / 5200);
      stars = Array.from({ length: n }, () => ({
        x: Math.random() * w, y: Math.random() * h,
        r: Math.random() ** 3 * 1.6 + 0.3,       // mostly faint, a few bright
        z: Math.random() * 0.8 + 0.2,            // depth, for parallax
        p: Math.random() * Math.PI * 2,          // twinkle phase
        s: Math.random() * 0.02 + 0.005,         // twinkle speed
      }));
    };
    const draw = () => {
      ctx.clearRect(0, 0, w, h);
      // the hero is always dark (photo background), so read its own colour
      const color = getComputedStyle(canvas).getPropertyValue("--text").trim();
      for (const s of stars) {
        const a = 0.3 + 0.5 * s.z * (0.5 + 0.5 * Math.sin(s.p + t * s.s * 60));
        ctx.globalAlpha = Math.max(0, Math.min(1, a));
        ctx.fillStyle = color;
        ctx.beginPath();
        ctx.arc(s.x + mx * s.z * 14, s.y + my * s.z * 14, s.r, 0, Math.PI * 2);
        ctx.fill();
      }
      ctx.globalAlpha = 1;
    };
    const loop = () => { if (!running) return; t += 1 / 60; draw(); requestAnimationFrame(loop); };
    resize(); draw();
    addEventListener("resize", () => { resize(); draw(); });
    if (!calm) {
      canvas.parentElement.addEventListener("pointermove", (e) => {
        const r = canvas.getBoundingClientRect();
        mx = (e.clientX - r.left) / r.width - 0.5;
        my = (e.clientY - r.top) / r.height - 0.5;
      });
      // only animate while the hero is on screen
      new IntersectionObserver(([e]) => {
        running = e.isIntersecting;
        if (running) loop();
      }).observe(canvas);
    }
  }

  // ---------------------------------------------------------------- "I'm a ..." rotator
  const rot = document.querySelector(".rotator");
  if (rot && !calm) {
    const words = rot.dataset.words.split("|");
    let i = 0;
    const type = async (word) => {
      for (let k = rot.textContent.length; k >= 0; k--) { rot.textContent = rot.textContent.slice(0, k); await sleep(28); }
      for (let k = 1; k <= word.length; k++) { rot.textContent = word.slice(0, k); await sleep(55); }
    };
    const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
    (async () => { for (;;) { await sleep(2600); i = (i + 1) % words.length; await type(words[i]); } })();
  }

  // ---------------------------------------------------------------- Lyα forest dividers
  // A fake quasar spectrum: flat continuum with random absorption lines.
  document.querySelectorAll(".forest").forEach((el) => {
    const W = 1200, H = 40, N = 600;
    let d = "";
    const lines = Array.from({ length: 70 }, () => ({ x: Math.random() * W, depth: Math.random() ** 0.7, width: 1.5 + Math.random() * 5 }));
    for (let i = 0; i <= N; i++) {
      const x = (i / N) * W;
      let f = 1;
      for (const l of lines) f *= 1 - l.depth * Math.exp(-(((x - l.x) / l.width) ** 2));
      const y = 6 + (1 - f) * (H - 10) + (Math.random() - 0.5) * 1.2;
      d += (i ? "L" : "M") + x.toFixed(1) + " " + y.toFixed(1);
    }
    el.innerHTML = `<svg viewBox="0 0 ${W} ${H}" preserveAspectRatio="none"><path d="${d}"/></svg>`;
  });

  // ---------------------------------------------------------------- cosmic clock
  const clock = document.querySelector("[data-cosmic-age]");
  if (clock) {
    const t0 = performance.now();
    const tick = () => {
      const s = Math.floor((performance.now() - t0) / 1000);
      const mm = String(Math.floor(s / 60)).padStart(2, "0"), ss = String(s % 60).padStart(2, "0");
      clock.textContent = `13.787 Gyr + ${mm}:${ss}`;
    };
    tick(); setInterval(tick, 1000);
  }

  // ---------------------------------------------------------------- easter egg 1: press "z" to redshift the page
  // Each press adds Δz = 1; "0" or Esc brings it back to z = 0.
  let z = 0;
  const badge = document.createElement("div");
  badge.className = "z-badge mono";
  badge.setAttribute("role", "status");
  document.body.append(badge);
  const setZ = (nz) => {
    z = Math.max(0, Math.min(nz, 12));
    // stretch hue toward the red: a cartoon, not a real (1+z) shift
    const k = Math.min(z / 4, 1);
    document.querySelector("main").style.filter = z
      ? `sepia(${0.9 * k}) saturate(${1 + 3 * k}) hue-rotate(${-35 * k}deg) brightness(${1 - 0.3 * k})`
      : "";
    document.querySelector("main").style.transition = "filter .4s";
    badge.textContent = z ? `z = ${z}   (press 0 to come home)` : "";
    badge.classList.toggle("show", z > 0);
  };

  // ---------------------------------------------------------------- easter egg 2: Konami code → reionization
  const konami = ["ArrowUp", "ArrowUp", "ArrowDown", "ArrowDown", "ArrowLeft", "ArrowRight", "ArrowLeft", "ArrowRight", "b", "a"];
  let pos = 0;
  addEventListener("keydown", (e) => {
    if (e.target.closest("input, textarea, [contenteditable]") || e.metaKey || e.ctrlKey || e.altKey) return;
    pos = e.key === konami[pos] ? pos + 1 : e.key === konami[0] ? 1 : 0;
    if (pos === konami.length) { pos = 0; reionize(); return; }
    if (e.key === "z") setZ(z + 1);
    else if (e.key === "0" || e.key === "Escape") setZ(0);
  });

  function reionize() {
    if (document.querySelector(".reion")) return;
    const c = document.createElement("canvas");
    c.className = "reion";
    document.body.append(c);
    const cap = document.createElement("div");
    cap.className = "reion-caption mono";
    document.body.append(cap);
    const x = c.getContext("2d");
    const W = (c.width = innerWidth), H = (c.height = innerHeight);
    // ionizing sources switch on one after another; their bubbles grow and overlap
    const src = Array.from({ length: 28 }, (_, i) => ({ x: Math.random() * W, y: Math.random() * H, on: i * 70 + Math.random() * 300, r: 0 }));
    const fog = isDark() ? "rgba(14,16,28,1)" : "rgba(60,62,78,1)";
    const t0 = performance.now(), Tmax = 4200;
    const frame = (now) => {
      const t = now - t0;
      x.globalCompositeOperation = "source-over";
      x.fillStyle = fog; x.fillRect(0, 0, W, H);
      x.globalCompositeOperation = "destination-out";
      let lit = 0;
      for (const s of src) {
        if (t < s.on) continue;
        lit++;
        s.r = ((t - s.on) / Tmax) ** 1.4 * Math.hypot(W, H) * 0.9;
        const g = x.createRadialGradient(s.x, s.y, s.r * 0.8, s.x, s.y, s.r);
        g.addColorStop(0, "rgba(0,0,0,1)"); g.addColorStop(1, "rgba(0,0,0,0)");
        x.fillStyle = g; x.beginPath(); x.arc(s.x, s.y, s.r, 0, Math.PI * 2); x.fill();
      }
      const zNow = Math.max(6, 15 - (t / Tmax) * 9).toFixed(1);
      const xhi = Math.max(0, 1 - t / Tmax);
      cap.textContent = xhi > 0 ? `z = ${zNow}   x_HI = ${xhi.toFixed(2)}   sources: ${lit}` : "z ≈ 6 — reionization complete ✨";
      if (t < Tmax * 1.05) requestAnimationFrame(frame);
      else { c.classList.add("fade"); setTimeout(() => { c.remove(); cap.remove(); }, 2600); }
    };
    requestAnimationFrame(frame);
  }

  // ---------------------------------------------------------------- hello, fellow developer
  console.log(
    "%c✦ Hi there! %cThis site is built with Zola (Rust). Source: https://github.com/astroumeda/astroumeda.github.io\nTry the Konami code, or press z.",
    "font-weight:bold;font-size:14px", "font-size:12px"
  );
})();
