export function el(tag, attrs = {}, ...children) {
    const e = document.createElement(tag);
    for (const [k,v] of Object.entries(attrs)) {
        if (k === 'class') e.className = v; else e.setAttribute(k,v);
    }
    for (const c of children) e.appendChild(c);
    return e;
}

export function displayName(name) {
    return name
        .split('_')
        .map(w => w.charAt(0).toUpperCase() + w.slice(1))
        .join(' ');
}

export function button(label, onclick, tooltip='') {
    const b = el('button', {title: tooltip});
    b.textContent = label;
    b.onclick = onclick;

    const proto = Object.getPrototypeOf(b);
    const desc = Object.getOwnPropertyDescriptor(proto, 'disabled');
    Object.defineProperty(b, 'disabled', {
        get() { return desc.get.call(b); },
        set(v) {
            desc.set.call(b, v);
            if (v) {
                b.className = 'btn-disabled m-1';
            } else {
                b.className = 'btn-primary m-1';
            }
        }
    });
    b.disabled = false;
    return b;
}
