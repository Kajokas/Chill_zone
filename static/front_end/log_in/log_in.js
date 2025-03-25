const SignUp = () =>{
    var login = document.getElementById("login").value;
    var psw = document.getElementById("psw").value;

    if(!login || !psw){
        alert("Missing fields");
    } else {
        var LogIn = {
            login: login,
            psw: psw
        }

        fetch("/login", {
            method: "POST",
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(LogIn),
        })
        .then ((response) => {
            if (!response.ok){
                alert("Something went wrong!");
            } else {
                response.json().then(data => {
                    console.log("DATA: ", data);
                    window.location.href = '/';
                }).catch(err => {
                    console.error("Recieved error: ", err);
                });
            }
        })
        .catch((error) => {
            console.log("Error with fetch");
        })
    }
};
