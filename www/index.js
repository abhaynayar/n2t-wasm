import * as wasm from "n2t-wasm";
import { Emu } from "n2t-wasm";
import { memory } from "n2t-wasm/n2t_wasm_bg";

// -------------------- Painting on the canvas ----------------------- //

const height = 256;
const width = 512;
const scale = 1;

const canvas = document.getElementById("n2t-wasm-canvas");
canvas.width = width*scale;
canvas.height = height*scale;

const ctx = canvas.getContext('2d');
ctx.imageSmoothingEnabled= false;
ctx.fillStyle = "#000000";

// ------------------- JavaScript Helper functions ------------------- //

function dec2bin(dec){
    var n = (dec >>> 0).toString(2);
    n = "0000000000000000".substr(n.length) + n;
    return n;
}

// -------------- Export JS Functions called from Rust --------------- //

export function put_xy(address, value) {

    // 16 bits per address:
    // - width  => 512 => 32 cols
    // - height => 256 => 16 rows

    // Screen starts at address 0x4000
    var screen = address - 0x4000;
    var x = screen % 32;
    var y = screen / 32;

    for (var i=15; i>=0; --i) {
        var set = value & (1<<i);
        if (set != 0) ctx.fillStyle = "#000000";
        else ctx.fillStyle = "#FFFFFF";
        ctx.fillRect(((x*16)+i)*scale, y*scale, scale, scale);
    }
}

export function put_op(x) {
    document.getElementById("opcodeText").innerHTML = x;
}

export function put_regs(x) {
    document.getElementById("regsText").innerHTML = x;
}

// --------------------- Calling Rust functions ---------------------- //

const emu = Emu.new();

function loadRom() {
    var x = document.getElementById("inputRom").value;
    emu.reset();
    emu.load_rom(x);
    console.log("Loaded ROM");
}

function loadRam() {
    var x = document.getElementById("ram_address").value;
    document.getElementById("ram_value").value = emu.load_ram(x);
    console.log("Loaded RAM");
}

function storeRam() {
    var x = document.getElementById("ram_address").value;
    var y = document.getElementById("ram_value").value;
    emu.store_ram(x,y);
    console.log("Stored value: " + y + ", in address: " + x);
}

var pause = true;
function playFn() { pause = false }
function pauseFn() { pause = true; }
function resetFn() { emu.reset(); console.log("ASDF"); }

const renderLoop = () => {
    if (!pause) {
        emu.continue_execution();
    }
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

// Event listeners:
document.getElementById("loadRomBtn").addEventListener("click", loadRom);
document.getElementById("loadRamBtn").addEventListener("click", loadRam);
document.getElementById("storeRamBtn").addEventListener("click", storeRam);
document.getElementById("playBtn").addEventListener("click", playFn);
document.getElementById("pauseBtn").addEventListener("click", pauseFn);
document.getElementById("resetBtn").addEventListener("click", resetFn);
