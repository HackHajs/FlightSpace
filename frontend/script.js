const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

function drawPoint(x, y) {
    ctx.fillStyle = "black";
    ctx.fillRect(x, y, 1, 1);
}

canvas.addEventListener("click", (event) => {
    const x = event.offsetX;
    const y = event.offsetY;
    drawPoint(x, y);
});
