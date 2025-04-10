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
                switch (response.status){
                    default:
                        alert("Something went wrong!");
                        break;
                    case 401:
                        alert("Incorrect login or password")
                        break;
                }
            } else {
                window.location.href = '/';
            }
        })
    }
};
