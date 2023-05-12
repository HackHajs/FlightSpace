// Get the current time
function getCurrentTime() {
    var date = new Date();
    var hours = date.getHours();
    var minutes = date.getMinutes();
    var seconds = date.getSeconds();
    
    // Format the time values to ensure two digits
    hours = ('0' + hours).slice(-2);
    minutes = ('0' + minutes).slice(-2);
    seconds = ('0' + seconds).slice(-2);
    
    return hours + ':' + minutes + ':' + seconds;
  }
  
  // Update the element with the current time
  function updateTime() {
    var element = document.getElementById("PreloadTitle");
    if (element) {
      element.innerHTML = getCurrentTime();
    }
  }
  
  // Call the updateTime function every second
  setInterval(updateTime, 1000);
  