function wait(){
    // Adds a class to visible an element and remove another, wait 0.5s before redirecting to a site.
    var element = document.getElementById("PreloadBar");
// Verifica si el elemento existe
    if (element) {
  // Elimina el elemento utilizando el método remove()
        element.remove();
    } else{
        console.log("ERROR: MANIPULATION DETECTED");
        // Redirect to error page
        window.location.href = "/error/error.html";

    }
    document.getElementById("Spinner").classList.remove("hidden");
    setTimeout(function(){
        window.location.href = "/frontend/lang/";
    }
    , 250);
}