const maro = document.getElementById("answer");
const dmv = maro.getContext("2d");
dmv.fillStyle = '#FFFFFF'; dmv.fillRect(0,0,683,100);
dmv.fillStyle = '#000000'; dmv.font = "20px serif"; dmv.fillText("Are you an informed tourist? Let's check with this quick quiz!", 27, 35 , 600);
/////////////////////////////////////////////////////////////////////////////////////////////////
setInterval(function() {
    setTimeout(function() {
        console.log("Waiting 100ms");
    }, 100);
    console.log(answera);
            console.log(answerb);
            dmv.clearRect(0, 0, maro.width, maro.height);
            dmv.fillStyle = '#FFFFFF'; dmv.fillRect(0,0,2000,50);
            dmv.fillStyle = '#000000'; dmv.fillText(question, 20, 35 , 600);
}, 10000);
