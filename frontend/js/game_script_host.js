fetch('http://localhost:8080/questions/')
    .then(response => response.json()) // Convert response to JSON
    .then(data => {
        console.log(data); // Log the data to the console
    })
    .catch(error => console.error(error)); // Log any errors to the console
