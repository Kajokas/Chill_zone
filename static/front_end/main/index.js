const RedirectToLogIN = () =>{
    window.location.href = '/loginPage';
};

const OnLoadPage = () => {
    let a = document.cookie
    .split("; ")
    .find((row) => row.startsWith("usr="))
    ?.split("=")[1];

    console.log(a);

    if (a !== undefined){
        let topPageElement = document.getElementById("top_page");
        let log_in_btn = document.getElementById("logInBtn");

        topPageElement.removeChild(log_in_btn);

        let log_out_btn = document.createElement("button");
        log_out_btn.textContent = "Log out";
        log_out_btn.onclick = LogOut;

        topPageElement.appendChild(log_out_btn);
    }
};

const LogOut = () => {
   fetch("/logout", {
       method: "GET",
    })
   .then((response) => {
       if (!response.ok){
           console.log("Unable to log out?");
        }
        return response.text();
    })
   .then((text) => {
       console.log(text);
       window.location.href = '/';
    })
};
