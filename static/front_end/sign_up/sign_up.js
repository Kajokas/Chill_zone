const SignUp = () => {
    var name = document.getElementById("username").value;
    var mail = document.getElementById("mail").value;
    var psw = document.getElementById("psw").value;

    if(!document.getElementById("mail").checkValidity()){
        alert("Must be a valid email!");
        return;
    }

    if (!name||!mail||!psw){
        alert("All fields are mandatory");
    } else {
        console.log(name + " " + mail + " " + psw);
        var User = {
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
                switch (response.status){
                    default:
                        alert("Something went wrong!");
                        break;
                    case 400:
                        response.text().then(text => {
                            alert(text);
                        })
                        break;
                }
            } else {
                response.json().then(data => {
                    console.log("DATA: ", data);
                    window.location.href = '/';
                });
            }
        })
    }
};
