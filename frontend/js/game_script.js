const halfwidth= 640;
const height   = 550;
////////////////////////////////////DATA PULL/////////////////////////////////////////
setInterval(function() {
    fetch('http://localhost:8080/players')
        .then(response => response.json())
        .then(data => {
            console.log(data.length);
            let player1=data.Player1; let x1=player1.x_pos; let y1=player1.y_pos;
            let player2=data.Player2; let x2=player2.x_pos; let y2=player2.y_pos;
            let player3=data.Player3; let x3=player3.x_pos; let y3=player3.y_pos;
            let player4=data.Player4; let x4=player4.x_pos; let y4=player4.y_pos;
            let player5=data.Player5; let x5=player5.x_pos; let y5=player5.y_pos;





































        })
        .catch(error => console.error(error));
}, 500);
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
        ctx.fillStyle = '#FFCC00'; ctx.fillRect(0,0,halfwidth,height);
        ctx.fillStyle = '#4D4D4D'; ctx.fillRect(halfwidth,0,halfwidth,height);
        ctx.fillStyle = '#B5FF87';
        x += incrementX; y += incrementY;
        ctx.beginPath(); ctx.arc(x, y, radius, 0, 2 * Math.PI);
        ctx.fillStyle = '#5972D8'; ctx.fill();
        if (frame < frames) {frame++;
            requestAnimationFrame(animate);}};
    animate();});

////////////////////////////////END CANVAS/////////////////////////////////////////