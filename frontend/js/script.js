const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const radius = 25;
const speed = 50; // pixels per frame

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
    const targetX = event.clientX - rect.left;
    const targetY = event.clientY - rect.top;

    // calculate distance to target
    const dx = targetX - x;
    const dy = targetY - y;
    const distance = Math.sqrt(dx*dx + dy*dy);

    // calculate number of frames needed to reach target
    const frames = Math.ceil(distance / speed);

    // calculate x and y increments per frame
    const incrementX = dx / frames;
    const incrementY = dy / frames;

    // start animation
    let frame = 1;
    const animate = () => {
        // clear canvas
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // update position
        x += incrementX;
        y += incrementY;

        // draw circle at new position
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, 2 * Math.PI);
        ctx.fillStyle = 'blue';
        ctx.fill();

        // check if animation is finished
        if (frame < frames) {
            frame++;
            requestAnimationFrame(animate);
        }
    };
    animate();
});
