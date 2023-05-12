const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const radius = 10;

// set initial position
let x = 50;
let y = 50;

// draw circle at initial position
ctx.beginPath();
ctx.arc(x, y, radius, 0, 2 * Math.PI);
ctx.fillStyle = 'blue';
ctx.fill();

// add event listener for mouse click
canvas.addEventListener('click', (event) => {
    // get click coordinates relative to canvas
    const rect = canvas.getBoundingClientRect();
    const mouseX = event.clientX - rect.left;
    const mouseY = event.clientY - rect.top;

    // clear canvas and redraw circle at new position
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    x = mouseX;
    y = mouseY;
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, 2 * Math.PI);
    ctx.fillStyle = 'blue';
    ctx.fill();
});


