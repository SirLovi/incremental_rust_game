import init, { Game } from '../pkg/incremental_rust_game.js';

let game;
const resourcesDiv = document.getElementById('resources');
const buildingsDiv = document.getElementById('buildings');
const logDiv = document.getElementById('log');

const resourceNames = ['wood', 'stone', 'food', 'iron', 'gold'];
const buildingNames = ['farm', 'lumber_mill', 'quarry', 'mine', 'bakery'];
const buildingButtons = {};

function formatCost(cost) {
    const parts = [];
    for (const r of resourceNames) {
        const v = cost[r];
        if (v > 0) {
            parts.push(`${v.toFixed(1)} ${r}`);
        }
    }
    return parts.join(', ');
}

function log(msg) {
    const p = document.createElement('p');
    p.textContent = msg;
    logDiv.appendChild(p);
    logDiv.scrollTop = logDiv.scrollHeight;
}

function updateResources() {
    resourcesDiv.innerHTML = '';
    for (const name of resourceNames) {
        const span = document.createElement('span');
        span.textContent = `${name}: ${game.get_resource(name).toFixed(1)}`;
        resourcesDiv.appendChild(span);
    }

    for (const name of buildingNames) {
        const cost = JSON.parse(game.building_cost(name));
        const btn = buildingButtons[name];
        if (!btn) continue;
        btn.textContent = `Build ${name} (${formatCost(cost)})`;
        let affordable = true;
        for (const r of resourceNames) {
            if (game.get_resource(r) < cost[r]) {
                affordable = false;
                break;
            }
        }
        btn.disabled = !affordable;
    }
}

function buildButtons() {
    buildingsDiv.innerHTML = '';
    for (const name of buildingNames) {
        const btn = document.createElement('button');
        buildingButtons[name] = btn;
        btn.textContent = `Build ${name}`;
        btn.className = 'btn';
        btn.onclick = () => {
            if (game.build(name)) {
                log(`Built ${name}`);
            } else {
                log(`Cannot build ${name}`);
            }
            updateResources();
        };
        buildingsDiv.appendChild(btn);
    }
}

async function run() {
    await init();
    game = new Game();
    buildButtons();
    updateResources();
    let tickRateInput = document.getElementById('tick-rate');

    setInterval(() => {
        game.tick(Date.now() / 1000);
        updateResources();
    }, 1000);

    tickRateInput.onchange = () => {
        game.set_tick_rate(parseFloat(tickRateInput.value));
    };

    document.getElementById('save').onclick = () => {
        const data = game.save();
        localStorage.setItem('idle-save', data);
        log('Game saved');
    };

    document.getElementById('load').onclick = () => {
        const data = localStorage.getItem('idle-save');
        if (data) {
            game.load(data);
            log('Game loaded');
            updateResources();
        }
    };
}

run();
