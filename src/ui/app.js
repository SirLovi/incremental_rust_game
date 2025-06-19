import init, { Game } from '../../pkg/incremental_rust_game.js';
import { wasm_base64 } from '../../pkg/wasm_base64.js';
import { el, button, displayName } from './components.js';

const resourceNames = ['wood','stone','food','iron','gold','energy','science','mana'];
const buildingNames = ['farm','lumber_mill','quarry','mine','bakery','generator','lab','shrine'];

const resDiv = document.getElementById('resources');
const bldDiv = document.getElementById('buildings');
const logDiv = document.getElementById('log');
const achDiv = document.getElementById('achievements');
const saveStamp = document.getElementById('save-stamp');
const tickInput = document.getElementById('tick-rate');
const saveBtn = document.getElementById('save');
const loadBtn = document.getElementById('load');
const resetBtn = document.getElementById('reset');
const buildingButtons = {};
let lastSave = 0;
let currentToast = null;

function updateLoadButton(){
    loadBtn.disabled = !localStorage.getItem('idle-save');
}

function updateAchievements(){
    achDiv.innerHTML='';
    const list=JSON.parse(Game.achievements());
    if(list.length===0) return;
    const ul=el('ul');
    list.forEach(id=>{
        ul.appendChild(el('li',{title:id},`✅ ${displayName(id)}`));
    });
    achDiv.appendChild(ul);
}

function formatCost(cost){
    return resourceNames
        .map(r=>cost[r]>0?`${cost[r].toFixed(1)} ${displayName(r)}`:'')
        .filter(Boolean)
        .join(', ');
}
function log(msg){
    const atBottom=logDiv.scrollTop+logDiv.clientHeight>=logDiv.scrollHeight-5;
    const p=document.createElement('p');
    p.textContent=msg;
    logDiv.appendChild(p);
    if(atBottom) logDiv.scrollTop=logDiv.scrollHeight;
    toast(msg);
    while(logDiv.children.length>100) logDiv.removeChild(logDiv.firstChild);
}

function toast(msg, color='green'){
    if(currentToast) currentToast.hideToast();
    currentToast = Toastify({text:msg,duration:3000,style:{background:color}});
    currentToast.showToast();
}

function updateResources(){
    resDiv.innerHTML='';
    resourceNames.forEach(r=>{
        const val=Game.get_resource(r);
        const rate=Game.get_resource_rate(r);
        const color=rate>0?'text-green-400':rate<0?'text-red-400':'text-gray-400';
        const rateStr=`(${rate>=0?'+':''}${rate.toFixed(1)}/s)`;
        resDiv.appendChild(
            el('div',{class:'mb-1 mx-2'},
                el('span',{},`${displayName(r)} ${val.toFixed(1)}`),
                el('span',{class:`ml-1 ${color}`},rateStr)
            )
        );
    });
    buildingNames.forEach(name=>{
        const cost=JSON.parse(Game.building_cost(name));
        const btn=buildingButtons[name];
        if(!btn) return;
        const count=Game.building_count(name);
        btn.textContent=`Build ${displayName(name)} (${count}) – ${formatCost(cost)}`;
        btn.title=formatCost(cost);
        const affordable=resourceNames.every(r=>Game.get_resource(r)>=cost[r]);
        btn.disabled=!affordable;
    });
}

function buildUI(){
    bldDiv.innerHTML='';
    buildingNames.forEach(name=>{
        const cost=JSON.parse(Game.building_cost(name));
        const btn=button(`Build ${displayName(name)} (${Game.building_count(name)}) – ${formatCost(cost)}`,
            ()=>{
                if(Game.build(name)){
                    log(`Built ${displayName(name)}`);
                }else{
                    log(`Cannot build ${displayName(name)}`);
                }
                updateResources();
            },
            formatCost(cost)
        );
        buildingButtons[name]=btn;
        bldDiv.appendChild(btn);
    });
}

function tick(){
    Game.tick(Date.now()/1000);
    updateResources();
    let msg=Game.pop_log();
    while(msg){
        log(msg);
        msg=Game.pop_log();
    }
    updateAchievements();
}

function saveGame(showToast=true){
    lastSave=Date.now();
    localStorage.setItem('idle-save',Game.save());
    saveStamp.textContent=new Date(lastSave).toLocaleTimeString();
    updateLoadButton();
    if(showToast) toast('Saved');
}

function loadGame(){
    const d=localStorage.getItem('idle-save');
    if(d){
        Game.load(d);
        updateResources();
        toast('Loaded');
        updateAchievements();
        updateLoadButton();
    }
}

function autosave(){
    if(Date.now()-lastSave<60000) return;
    saveGame(false);
}

function decodeWasm(b64){
    const bin=atob(b64);
    const arr=new Uint8Array(bin.length);
    for(let i=0;i<bin.length;i++) arr[i]=bin.charCodeAt(i);
    return arr;
}

async function run(){
    try{
        await init(decodeWasm(wasm_base64));
    }catch(err){
        console.error(err);
        toast('Failed to load WASM', 'red');
        return;
    }

    new Game();
    buildUI();
    updateResources();
    setInterval(tick,1000);
    setInterval(autosave,60000);

    saveBtn.onclick=()=>saveGame(true);
    loadBtn.onclick=loadGame;
    resetBtn.onclick=()=>{ localStorage.clear(); location.reload(); };
    tickInput.oninput=()=>{
        Game.set_tick_rate(parseFloat(tickInput.value));
        localStorage.setItem('tick-rate', tickInput.value);
    };

    document.addEventListener('keydown',e=>{
        if(e.target.tagName==='INPUT' || e.target.tagName==='TEXTAREA') return;
        if(e.ctrlKey||e.metaKey||e.altKey||e.shiftKey) return;
        if(e.key==='Escape' && currentToast){ currentToast.hideToast(); currentToast=null; return; }
        if(e.key==='s'){ e.preventDefault(); saveGame(true);}
        if(e.key==='l'){ e.preventDefault(); loadGame(); }
    });

    const savedRate=localStorage.getItem('tick-rate');
    if(savedRate){
        tickInput.value=savedRate;
        Game.set_tick_rate(parseFloat(savedRate));
    }

    const d=localStorage.getItem('idle-save');
    if(d){ Game.load(d); }
    updateLoadButton();
    updateAchievements();
    saveGame(false);
}

document.addEventListener('DOMContentLoaded',run);
