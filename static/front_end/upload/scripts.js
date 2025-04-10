const MusicHasBeenSelected = () => {
    const fileInput = document.getElementById('music_file_input');
    const fileNameDisplay = document.getElementById('musicLabel');

    fileNameDisplay.textContent = 'Selected music: ' + fileInput.files[0].name;
};

const CoverHasBeenSelected = () => {
    const fileInput = document.getElementById('music_cover_input');
    const fileNameDisplay = document.getElementById('coverLabel');

    fileNameDisplay.textContent = 'Selected cover: ' + fileInput.files[0].name;
};

const UploadMusic = () => {
    var title = document.getElementById('music_title').value;
    var MusicFile = document.getElementById('music_file_input').files[0];
    var CoverFile = document.getElementById('music_cover_input').files[0];

    if(!title||!MusicFile||!CoverFile){
        alert("All fields are mandatory!!!");
    } else {
        var data = new FormData();

        data.append('title', title);
        data.append('cover_file', CoverFile);
        data.append('music_file', MusicFile);

        fetch("/upload", {
            method:"POST",
            body: data
        })
        .then((response) => {
            if (!response.ok){
                switch(response.status){
                    default:
                        alert("Something went wrong!");
                        break;
                    case 401:
                        alert("You must be logged in to upload")
                        window.location.href = `/loginPage`;
                        break;
                    case 422:
                        alert("Wrong formats (cover must be jpeg and audio must be MP3)");
                        break;
                    case 413:
                        alert("Files are to big");
                        break;
                }
            } else {
                console.log("Success");
                window.location.href = `/`;
            }
        })
    };
};
