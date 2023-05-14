const maro = document.getElementById("answer");
const dmv = maro.getContext("2d");
dmv.fillStyle = '#FFFFFF'; dmv.fillRect(0,0,683,100);
console.log("ANSWER");
/////////////////////////////////////////////////////////////////////////////////////////////////
setInterval(function() {
    fetch('http://localhost:8080/questionid/' + questnum)
        .then(response => response.json())
        .then(data => {
            var answera = data.a;
            var answerb = data.b;
            console.log(answera);
            dmv.clearRect(0, 0, maro.width, maro.height); dmv.font = "20px serif";
            dmv.fillStyle = '#FFFFFF'; dmv.fillRect(0, 0, 640, 100);
            dmv.fillStyle = '#000000'; dmv.fillText(answera, 27, 35, 600);
            dmv.fillStyle = '#FFFFFF'; dmv.fillRect(640, 0, 640, 100);
            dmv.fillStyle = '#000000'; dmv.fillText(answerb, 697, 35, 600);
            questnum=questnum+1;
        })
        .catch(error => console.error(error)); // Log any errors to the console}, 10000);
}, 10000);