//////////////////////////////////INIT CANVAS/////////////////////////////////////////
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const radius = 25;
const speed = 35;
let x = 640;
let y = 275;
const halfwidth= 640;
const height   = 550;
//////////////////////////////////DRAW BG/////////////////////////////////////////
ctx.fillStyle = '#FFCC00'; ctx.fillRect(0,0,640,550);
ctx.fillStyle = '#4D4D4D'; ctx.fillRect(640,0,640,550);
//////////////////////////////////DRAW POINT/////////////////////////////////////////
ctx.beginPath();
ctx.arc(x, y, radius, 0, 2 * Math.PI);
ctx.fillStyle = '#5972D8'; ctx.fill();
//////////////////////////////////CLICKETY CLICK/////////////////////////////////////////
const move = () => {
    const rect = canvas.getBoundingClientRect();
    const targetX = Math.random() * canvas.width; const targetY = Math.random() * canvas.height;
    const dx = targetX - x; const dy = targetY - y;
    const distance = Math.sqrt(dx*dx + dy*dy);
    const frames = Math.ceil(distance / speed); let frame = 1;
    const incrementX = dx / frames; const incrementY = dy / frames;
    const animate = () => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = '#FFCC00'; ctx.fillRect(0,0,halfwidth,height);
        ctx.fillStyle = '#4D4D4D'; ctx.fillRect(halfwidth,0,halfwidth,height);
        ctx.fillStyle = '#B5FF87';
        x += incrementX; y += incrementY;
        ctx.beginPath(); ctx.arc(x, y, radius, 0, 2 * Math.PI);
        ctx.fillStyle = '#5972D8'; ctx.fill();
        if (frame < frames) {frame++;
            requestAnimationFrame(animate);}};
    animate();};
setInterval(function() {
    move();
}, 500);

////////////////////////////////END CANVAS/////////////////////////////////////////