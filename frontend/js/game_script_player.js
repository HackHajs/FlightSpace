//////////////////////////////////INIT CANVAS/////////////////////////////////////////
const canvas = document.getElementById('canvas2');
const ctx = canvas.getContext('2d');
const radius = 25;
const speed = 50;
let x = 640;
let y = 275;
console.log("GSP initted");
//////////////////////////////////DRAW POINT/////////////////////////////////////////
ctx.beginPath();
ctx.arc(x, y, radius, 0, 2 * Math.PI);
ctx.fillStyle = '#5972D8'; ctx.fill();
//////////////////////////////////CLICKETY CLICK/////////////////////////////////////////
canvas.addEventListener('click', (event) => {
    const rect = canvas.getBoundingClientRect();
    const targetX = event.clientX - rect.left; const targetY = event.clientY - rect.top;
    const dx = targetX - x; const dy = targetY - y;
    const distance = Math.sqrt(dx*dx + dy*dy);
    const frames = Math.ceil(distance / speed); let frame = 1;
    const incrementX = dx / frames; const incrementY = dy / frames;
    const animate = () => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = '#B5FF87';
        x += incrementX; y += incrementY;
        ctx.beginPath(); ctx.arc(x, y, radius, 0, 2 * Math.PI);
        ctx.fillStyle = '#5972D8'; ctx.fill();
        if (frame < frames) {frame++;
            requestAnimationFrame(animate);}};
    animate();});

////////////////////////////////END CANVAS/////////////////////////////////////////
