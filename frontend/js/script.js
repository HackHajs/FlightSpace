const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const radius = 25;
const speed = 50; // pixels per frame
let x = 50;
let y = 50;
ctx.fillStyle = 'red';
ctx.fillRect(0,0,640,550);
ctx.fillStyle = 'blue';
ctx.fillRect(640,0,640,550);
ctx.beginPath();
ctx.arc(x, y, radius, 0, 2 * Math.PI);
ctx.fillStyle = 'black';
ctx.fill();
canvas.addEventListener('click', (event) => {
    // get click coordinates relative to canvas
    const rect = canvas.getBoundingClientRect();
    const targetX = event.clientX - rect.left;
    const targetY = event.clientY - rect.top;
    console.log(event.clientX)
    const dx = targetX - x;
    const dy = targetY - y;
    const distance = Math.sqrt(dx*dx + dy*dy);
    const frames = Math.ceil(distance / speed);
    const incrementX = dx / frames;
    const incrementY = dy / frames;
    let frame = 1;
    const animate = () => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = 'red';
        ctx.fillRect(0,0,640,550);
        ctx.fillStyle = 'blue';
        ctx.fillRect(640,0,640,550);
        ctx.fillStyle = 'black';
        x += incrementX;
        y += incrementY;
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, 2 * Math.PI);
        ctx.fillStyle = 'black';
        ctx.fill();
        if (frame < frames) {
            frame++;
            requestAnimationFrame(animate);}};
    animate();});