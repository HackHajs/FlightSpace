function wait(){
    // Adds a class to visible an element and remove another, wait 0.5s before redirecting to a site.
    document.getElementById("PreloadTitle").classList.add("visible");
    document.getElementById("PreloadTitle").classList.remove("hidden");
    setTimeout(function(){
        window.location.href = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    }
    , 500);
}