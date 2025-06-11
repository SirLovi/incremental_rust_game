import init, { Game } from '../../pkg/incremental_rust_game.js';
import { wasm_base64 } from '../../pkg/wasm_base64.js';
import { el, button } from './components.js';

const resourceNames = ['wood','stone','food','iron','gold','energy','science','mana'];
const buildingNames = ['farm','lumber_mill','quarry','mine','bakery','generator','lab','shrine'];

const resDiv = document.getElementById('resources');
const bldDiv = document.getElementById('buildings');
const logDiv = document.getElementById('log');
const achDiv = document.getElementById('achievements');
const saveStamp = document.getElementById('save-stamp');
const buildingButtons = {};

function formatCost(cost){return resourceNames.map(r=>cost[r]>0?`${cost[r].toFixed(1)} ${r}`:'').filter(Boolean).join(', ')}
function log(msg){const p=document.createElement('p');p.textContent=msg;logDiv.appendChild(p);logDiv.scrollTop=logDiv.scrollHeight;Toastify({text:msg,duration:3000}).showToast();}

function updateResources(){resDiv.innerHTML='';resourceNames.forEach(r=>{const val=Game.get_resource(r);resDiv.appendChild(el('div',{class:'mb-1'},el('span',{class:'mr-1'},`${r}: ${val.toFixed(1)}`)));});
    buildingNames.forEach(name=>{const cost=JSON.parse(Game.building_cost(name));const btn=buildingButtons[name];if(!btn)return;btn.textContent=`Build ${name}`;btn.title=formatCost(cost);btn.disabled=!resourceNames.every(r=>Game.get_resource(r)>=cost[r]);});}

function buildUI(){bldDiv.innerHTML='';buildingNames.forEach(name=>{const btn=button(`Build ${name}`,()=>{if(Game.build(name)){log(`Built ${name}`);}else{log(`Cannot build ${name}`);}updateResources();});buildingButtons[name]=btn;bldDiv.appendChild(btn);});}

function tick(){Game.tick(Date.now()/1000);updateResources();let msg=Game.pop_log();while(msg){log(msg);msg=Game.pop_log();}achDiv.textContent=JSON.parse(Game.achievements()).join('\n');}

function autosave(){localStorage.setItem('idle-save',Game.save());saveStamp.textContent=new Date().toLocaleTimeString();}

function decodeWasm(b64){const bin=atob(b64);const arr=new Uint8Array(bin.length);for(let i=0;i<bin.length;i++)arr[i]=bin.charCodeAt(i);return arr;}
async function run(){await init(decodeWasm(wasm_base64));new Game();buildUI();updateResources();setInterval(tick,1000);setInterval(autosave,60000);document.addEventListener('keydown',e=>{if(e.key==='s')autosave();if(e.key==='l'){const d=localStorage.getItem('idle-save');if(d){Game.load(d);log('Loaded');updateResources();}}});const d=localStorage.getItem('idle-save');if(d){Game.load(d);}autosave();}
run();
