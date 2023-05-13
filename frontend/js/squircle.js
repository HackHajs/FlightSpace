const canvas = document.getElementById('myCanvas');
const ctx = canvas.getContext('2d');
const x = 100; // x-coordinate of center of squircle
const y = 100; // y-coordinate of center of squircle
const size = 50; // size of the squircle
const radius = size / 2; // radius of the squircle
const cornerRadius = size / 4; // radius of the rounded corners

ctx.beginPath();
ctx.moveTo(x + radius, y + cornerRadius);

// draw top-right corner
ctx.arcTo(x + size - cornerRadius, y, x + size, y + cornerRadius, cornerRadius);

// draw bottom-right corner
ctx.arcTo(x + size, y + size - cornerRadius, x + size - cornerRadius, y + size, cornerRadius);

// draw bottom-left corner
ctx.arcTo(x + cornerRadius, y + size, x, y + size - cornerRadius, cornerRadius);

// draw top-left corner
ctx.arcTo(x, y + cornerRadius, x + radius, y, cornerRadius);

ctx.closePath();
ctx.fillStyle = '#FFFFFF'; ctx.fill();
