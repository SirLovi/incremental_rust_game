export function el(tag, attrs = {}, ...children) {
    const e = document.createElement(tag);
    for (const [k,v] of Object.entries(attrs)) {
        if (k === 'class') e.className = v; else e.setAttribute(k,v);
    }
    for (const c of children) e.appendChild(c);
    return e;
}

export function button(label, onclick, tooltip='') {
    const b = el('button', {class:'btn-primary px-2 py-1 m-1', title:tooltip});
    b.textContent = label;
    b.onclick = onclick;
    return b;
}
