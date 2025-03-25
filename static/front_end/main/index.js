const TestFunk = () =>{
    fetch("/testAPI", {
        method: "GET",
    }).then((response) => {
        if (!response.ok){
            console.log("broken");
        }
        return response.text();
    })
    .then((text) => {
        alert(text);
    })
};

const RedirectToLogIN = () =>{
    window.location.href = '/login';
};
