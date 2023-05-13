//////////////////////////////////INIT canvos/////////////////////////////////////////
const canvos = document.getElementById('canvas1');
const ctb = canvos.getContext('2d');
const radius = 50;
const speed = 50;
let x = 640;
let y = 275;
console.log("GSH initted");
//////////////////////////////////DRAW BG/////////////////////////////////////////
ctb.fillStyle = '#FFCC00'; ctb.fillRect(0,0,640,550);
ctb.fillStyle = '#4D4D4D'; ctb.fillRect(640,0,640,550);
//////////////////////////////////DRAW POINT/////////////////////////////////////////
ctb.beginPath();
ctb.arc(x, y, radius, 0, 2 * Math.PI);
ctb.fillStyle = '#D85999EA'; ctb.fill();
//////////////////////////////////CLICKETY CLICK/////////////////////////////////////////
canvos.addEventListener('click', (event) => {
    const rect = canvos.getBoundingClientRect();
    const targetX = event.clientX - rect.left; const targetY = event.clientY - rect.top;
    const dx = targetX - x; const dy = targetY - y;
    const distance = Math.sqrt(dx*dx + dy*dy);
    const frames = Math.ceil(distance / speed); let frame = 1;
    const incrementX = dx / frames; const incrementY = dy / frames;
    const animate = () => {
        ctb.clearRect(0, 0, canvos.width, canvos.height);
        ctb.fillStyle = '#FFCC00'; ctb.fillRect(0,0,640,550);
        ctb.fillStyle = '#4D4D4D'; ctb.fillRect(640,0,640,550);
        ctb.fillStyle = '#D85999EA';
        x += incrementX; y += incrementY;
        ctb.beginPath(); ctb.arc(x, y, radius, 0, 2 * Math.PI);
        ctb.fillStyle = '#D85999EA'; ctb.fill();
        if (frame < frames) {frame++;
            requestAnimationFrame(animate);}};
    animate();});

////////////////////////////////END canvos/////////////////////////////////////////