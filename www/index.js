import init, { Emu } from "./pkg/n2t_wasm.js";

await init();

const height = 256;
const width = 512;
const scale = 1;

const canvas = document.getElementById("n2t-wasm-canvas");
canvas.width = width * scale;
canvas.height = height * scale;

const ctx = canvas.getContext("2d");
ctx.imageSmoothingEnabled = false;
ctx.fillStyle = "#000000";

export function put_xy(address, value) {
  // 16 bits per address:
  // - width  => 512 => 32 cols
  // - height => 256 => 16 rows

  // Screen starts at address 0x4000
  const screen = address - 0x4000;
  const x = screen % 32;
  const y = Math.floor(screen / 32);

  for (let i = 15; i >= 0; --i) {
    const set = value & (1 << i);
    if (set !== 0) ctx.fillStyle = "#000000";
    else ctx.fillStyle = "#FFFFFF";
    ctx.fillRect(((x * 16) + i) * scale, y * scale, scale, scale);
  }
}

const emu = Emu.new();

let paused = true;
function play() {
  paused = false;
  console.log("Started Emulator");
}

const renderLoop = () => {
  if (!paused) emu.run();
  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

// Event listeners:
document.getElementById("playBtn").addEventListener("click", play);
document.getElementById("inputfile").addEventListener("change", function () {
  const fr = new FileReader();
  fr.onload = function () {
    emu.load_rom(fr.result);
  };
  fr.readAsText(this.files[0]);
});
