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
        log_out_btn.id = "logOutBtn";
        log_out_btn.textContent = "Log out";
        log_out_btn.onclick = LogOut;

        let uploadPopUpBtn = document.createElement("button");
        uploadPopUpBtn.id = "UploadBtn";
        uploadPopUpBtn.textContent = "Upload";
        uploadPopUpBtn.onclick = () => {window.location.href = "/uploadPage"};

        topPageElement.appendChild(log_out_btn);
        topPageElement.appendChild(uploadPopUpBtn);
    }

    LoadMainSongs();
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

const OpenUploadPopUp = () => {
    console.log("Works");
};


const LoadMainSongs = () => {
    fetch("/loadMainPageSongs", {
        method: "GET",
    })
    .then((response) => {
        if (!response.ok){
            alert("Something went wrong!");
        }
        return response.json().then(data => {
            var songs = data;

            songs.forEach((song, index) => {
                console.log(`${index}: ${song.artist}`);
                AddASongDiv(song.id, song.title, song.author, song.thumbnail);
            });
        }).catch(err => {
            console.error("Recieved error: ", err);
        });
    })
};

function AddASongDiv(id, title, author, thumb){
    let container = document.getElementById("contents");

    let songDiv = document.createElement("div");
    songDiv.classList.add("songDiv");
    songDiv.onclick = () => {
        console.log("ID: ", id);
    };

    let songCover = document.createElement("img");
    songCover.src = "songsDir/" + thumb;
    songCover.width = 500;
    songCover.height = 300;

    let titleText = document.createElement("p");
    titleText.id = "titleText";
    titleText.innerText = title;

    let artistName = document.createElement("p");
    artistName.id = "artist";
    artistName.innerText = author;


    songDiv.appendChild(songCover);
    songDiv.appendChild(titleText);
    songDiv.appendChild(artistName);

    container.appendChild(songDiv);
};
