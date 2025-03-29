const LoadSong = () => {
    const urlParams = new URLSearchParams(window.location.search);
    const l = urlParams.get('l');

    if (l) {
        fetch(`/song?l=${l}`, {
            method: "GET",
        })
        .then((response) => {
            if(!response.ok){
                console.log("No bueno");
            }else{
                response.json().then(data => {
                    console.log("DATA: ", data);
                    let coverImg = document.getElementById("cover");
                    coverImg.src = "songsDir/" + data.thumbnail;

                    let audioSrc = document.getElementById("audio_source");
                    audioSrc.src = "songsDir/" + data.song;

                    let songTitle = document.getElementById("song_title");
                    songTitle.innerText = data.title;

                    let artist = document.getElementById("artist");
                    artist.innerText = data.artist;
                })
            }
        })
    } else {
        console.error('Missing "l" parameter');
    }
}

const playBtnLogic = () => {
    let playBtn = document.getElementById("playbtn");
    let audio_source = document.getElementById("audio_source");

    if(audio_source.paused) {
        audio_source.play();
        playBtn.innerHTML = '❚❚';
    } else {
        audio_source.pause();
        playBtn.innerHTML = '▶';
    }
}

document.getElementById("title_text").addEventListener('click', function(){
    window.location.href = `/`;
});
