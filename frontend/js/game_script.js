////////////////////////////////////DATA PULL/////////////////////////////////////////
fetch('http://localhost:8080/players')
    .then(response => response.json()) // Convert response to J
    .then(data => {
        console.log(data); // Log the data to the console
        var numPlayers=data.length;
        var player1=data[0].name;
        var player2=data[1].name;
        var player3=data[2].name;
    })
    .catch(error => console.error(error)); // Log any errors to the console

//////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////INIT CANVAS/////////////////////////////////////////
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const radius = 25;
const speed = 50;
let x = 640;
let y = 275;
//////////////////////////////////DRAW BG/////////////////////////////////////////
ctx.fillStyle = '#FFCC00'; ctx.fillRect(0,0,640,550);
ctx.fillStyle = '#4D4D4D'; ctx.fillRect(640,0,640,550);
//////////////////////////////////DRAW POINT/////////////////////////////////////////
ctx.beginPath();
ctx.arc(x, y, radius, 0, 2 * Math.PI);
ctx.fillStyle = 'black';
ctx.fill();
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
        ctx.fillStyle = '#FFCC00'; ctx.fillRect(0,0,640,550);
        ctx.fillStyle = '#4D4D4D'; ctx.fillRect(640,0,640,550);
        ctx.fillStyle = 'black';
        x += incrementX; y += incrementY;
        ctx.beginPath(); ctx.arc(x, y, radius, 0, 2 * Math.PI);
        ctx.fillStyle = 'black'; ctx.fill();
        if (frame < frames) {
            frame++;
            requestAnimationFrame(animate);}};
    animate();});
//////////////////////////////////END CANVAS/////////////////////////////////////////