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
    var musicFile = MusicFile.name;
    var coverFile = CoverFile.name;

    if(!title||!musicFile||!coverFile){
        alert("All fields are mandatory!!!");
    } else {
        var data = new FormData();

        data.append('Title', title);
        data.append('CoverArtFile', coverFile);
        data.append('MusicFile', musicFile);
        data.append('coverFile', CoverFile);
        data.append('musicFile', MusicFile);

        fetch("/upload", {
            method:"POST",
            body: data
        })
        .then((response) => {
            if (!response.ok){
                alert("Something went wrong!");
            } else {
                console.log("Works");
            }
        })
        .catch((error) => {
            console.log("Error with fetch");
        })
    };
};
