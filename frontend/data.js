const playersURL = "http://localhost:8080/"
function httpGet(playersURL)
{
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.open( "GET", playersURL, false ); // false for synchronous request
    xmlHttp.send( null );
    return xmlHttp.responseText;
}