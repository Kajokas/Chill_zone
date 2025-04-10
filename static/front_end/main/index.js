let timesLoaded = 0;
let isOnLoadingtimeOut = false;

const RedirectToLogIN = () =>{
    window.location.href = '/loginPage';
};

const CheckLogin = () => {
    fetch("/getUser", {
        method: "GET",
        credentials: 'include',
    })
    .then((response) => {
        if(!response.ok){
            switch (response.status){
                default:
                    console.log("Something went wrong");
                    break;
                case 401:
                    console.log("Failed to authenticate");
                    break;
            }

        } else {
            let topPageElement = document.getElementById("Button_Container");
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
    })
};

const OnLoadPage = () => {
    CheckLogin();

    LoadMainSongs();
};

const LogOut = () => {
   fetch("/logout", {
       method: "GET",
    })
   .then((response) => {
       if (!response.ok){
           //don't know how we would get here but just in case
           console.log("Unable to log out?");
        }else{
            window.location.href = '/';
        }
    })
};

const LoadMainSongs = () => {
    fetch(`/loadMainPageSongs?f=${timesLoaded}`, {
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
                AddASongDiv(song.id, song.title, song.artist, song.thumbnail);
            });
        });
    })
};

function AddASongDiv(id, title, author, thumb){
    let container = document.getElementById("contents");

    let songDiv = document.createElement("div");
    songDiv.classList.add("songDiv");
    songDiv.onclick = () => {
        console.log("ID: ", id);
        ListenToSong(id);
    };

    let songCover = document.createElement("img");
    songCover.src = "songsDir/" + thumb;
    songCover.width = 500;
    songCover.height = 300;

    let titleText = document.createElement("p");
    titleText.id = "titleText";
    titleText.innerText = title;
    if (title.length <= 30){
        titleText.innerText = title;
    } else {
        titleText.innerText = title.substring(0,30) + "...";
    }

    let artistName = document.createElement("p");
    artistName.id = "artist";
    artistName.innerText = author;


    songDiv.appendChild(songCover);
    songDiv.appendChild(titleText);
    songDiv.appendChild(artistName);

    container.appendChild(songDiv);
};

const ListenToSong = (songId) => {
    window.location.href =`/listen?l=${songId}`;
};

window.addEventListener('scroll', () => {
    const { scrollTop, scrollHeight, clientHeight } = document.documentElement;

    const isAtBottom = scrollTop + clientHeight >= scrollHeight - 5;

    if (isAtBottom && !isOnLoadingtimeOut) {
        timesLoaded++;
        isOnLoadingtimeOut = true;
        LoadMainSongs();

        setTimeout(() => {
           isOnLoadingtimeOut = false;
        }, 3000);
    }
});
