//////////////////////////////////INIT CANVIS/////////////////////////////////////////
const canvis = document.getElementById('player');
const cta = canvis.getContext('2d');
const radius = 25;
const speed = 50;
let x = 640;
let y = 275;
console.log("GSP initted");
//////////////////////////////////DRAW POINT/////////////////////////////////////////
cta.beginPath();
cta.arc(x, y, radius, 0, 2 * Math.PI);
cta.fillStyle = '#5972D8'; cta.fill();
//////////////////////////////////CLICKETY CLICK/////////////////////////////////////////
canvis.addEventListener('click', (event) => {
    console.log("clicc")
    const rect = canvis.getBoundingClientRect();
    const targetX = event.clientX - rect.left; const targetY = event.clientY - rect.top;
    const dx = targetX - x; const dy = targetY - y;
    const distance = Math.sqrt(dx*dx + dy*dy);
    const frames = Math.ceil(distance / speed); let frame = 1;
    const incrementX = dx / frames; const incrementY = dy / frames;
    const animate = () => {
        cta.clearRect(0, 0, canvis.width, canvis.height);
        cta.fillStyle = '#B5FF87';
        x += incrementX; y += incrementY;
        cta.beginPath(); cta.arc(x, y, radius, 0, 2 * Math.PI);
        cta.fillStyle = '#5972D8'; cta.fill();
        if (frame < frames) {frame++;
            requestAnimationFrame(animate);}};
    animate();});
////////////////////////////////END CANVIS/////////////////////////////////////////
