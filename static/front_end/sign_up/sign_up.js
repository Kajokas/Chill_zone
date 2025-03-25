const SignUp = () => {
    var name = document.getElementById("username").value;
    var mail = document.getElementById("mail").value;
    var psw = document.getElementById("psw").value;

    if (!name||!mail||!psw){
        alert("All fields are mandatory");
    } else {
        console.log(name + " " + mail + " " + psw);
        var User = {
            id: 1,
            username: name,
            email: mail,
            psw: psw
        }

        fetch("/signup",{
            method: "POST",
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(User),
        })
        .then((response) =>{
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
