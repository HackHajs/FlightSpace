const canvass = document.getElementById('canvas2');
const cty = canvass.getContext('2d');
// set initial position
let a = 75;
let b = 75;

// draw circle at initial position
cty.beginPath();
cty.arc(a, b, radius, 0, 2 * Math.PI);
cty.fillStyle = 'red';
cty.fill();
// add event listener for mouse click
canvass.addEventListener('click', (event) => {
    // get click coordinates relative to canvass
    const rect = canvass.getBoundingClientRect();
    const targeta = Math.floor(Math.random() * 100);
    const targetb = Math.floor(Math.random() * 100);

    // calculate distance to target
    const da = targeta - a;
    const db = targetb - b;
    const distance = Math.sqrt(da*da + db*db);

    // calculate number of frames needed to reach target
    const frames = Math.ceil(distance / speed);

    // calculate a and b increments per frame
    const incrementa = da / frames;
    const incrementb = db / frames;

    // start animation
    let frame = 1;
    const animate = () => {
        // clear canvass
        cty.clearRect(0, 0, canvass.width, canvass.height);

        // update position
        a += incrementa;
        b += incrementb;

        // draw circle at new position
        cty.beginPath();
        cty.arc(a, b, radius, 0, 2 * Math.PI);
        cty.fillStyle = 'red';
        cty.fill();

        // check if animation is finished
        if (frame < frames) {
            frame++;
            requestAnimationFrame(animate);
        }
    };
    animate();
});
