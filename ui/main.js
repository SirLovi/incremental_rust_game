import init, { Game } from '../pkg/incremental_rust_game.js';

let game;
const resourcesDiv = document.getElementById('resources');
const buildingsDiv = document.getElementById('buildings');
const logDiv = document.getElementById('log');

const resourceNames = ['wood', 'stone', 'food', 'iron', 'gold'];
const buildingNames = ['farm', 'lumber_mill', 'quarry', 'mine', 'bakery'];

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
}

function buildButtons() {
    buildingsDiv.innerHTML = '';
    for (const name of buildingNames) {
        const btn = document.createElement('button');
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
