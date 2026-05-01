"use strict";

function escapeHtml(s) {
  return s
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

const LINTS = JSON.parse(document.getElementById("lint-json").textContent);
const tbody = document.getElementById("lint-body");
const q = document.getElementById("q");
const cat = document.getElementById("cat");

const categories = [...new Set(LINTS.map((l) => l.category))].sort();
for (const c of categories) {
  const opt = document.createElement("option");
  opt.value = c;
  opt.textContent = c;
  cat.appendChild(opt);
}

function render() {
  const fq = q.value.trim().toLowerCase();
  const fc = cat.value;
  tbody.replaceChildren();
  for (const l of LINTS) {
    if (fc && l.category !== fc) continue;
    const hay = (l.id + l.category + l.default_level + l.doc).toLowerCase();
    if (fq && !hay.includes(fq)) continue;
    const tr = document.createElement("tr");
    tr.id = l.id;
    const tdId = document.createElement("td");
    tdId.innerHTML = `<a href="#${escapeHtml(l.id)}"><code>${escapeHtml(l.id)}</code></a>`;
    const tdCat = document.createElement("td");
    tdCat.innerHTML = `<code>${escapeHtml(l.category)}</code>`;
    const tdDef = document.createElement("td");
    tdDef.innerHTML = `<code>${escapeHtml(l.default_level)}</code>`;
    const tdDoc = document.createElement("td");
    tdDoc.className = "doc";
    tdDoc.textContent = l.doc;
    tr.append(tdId, tdCat, tdDef, tdDoc);
    tbody.appendChild(tr);
  }
}

q.addEventListener("input", render);
cat.addEventListener("change", render);
render();
