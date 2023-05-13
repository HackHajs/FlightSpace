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
