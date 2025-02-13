function formatFileSize(sizeInBytes) {
    if (sizeInBytes < 1024) {
        return `${sizeInBytes} B`;
    } else if (sizeInBytes < 1024 ** 2) {
        return `${(sizeInBytes / 1024).toFixed(2)} KB`;
    } else if (sizeInBytes < 1024 ** 3) {
        return `${(sizeInBytes / (1024 ** 2)).toFixed(2)} MB`;
    } else {
        return `${(sizeInBytes / (1024 ** 3)).toFixed(2)} GB`;
    }
}

document.addEventListener("DOMContentLoaded", () => {
    const dropArea = document.getElementById('drop-area');
    const fileInput = document.getElementById('file-input');
    const uploadButton = document.getElementById('upload-button');
    const progressDiv = document.getElementById('progress');
    const fileList = document.getElementById('file-list');
    let files = [];

    function updateFileList() {
        fileList.innerHTML = '';
        
        for (const file of files) {
            const li = document.createElement('li');
            
            const fileName = document.createElement('span');
            fileName.classList.add('file-name');
            fileName.textContent = file.name;
            
            const fileSize = document.createElement('span');
            fileSize.classList.add('file-size');
            fileSize.textContent = formatFileSize(file.size);

            const removeIcon = document.createElement('span');
            removeIcon.classList.add('remove-icon');
            removeIcon.textContent = '❌';
            removeIcon.onclick = () => {
                files = files.filter(f => f !== file);
                updateFileList();
            };

            li.appendChild(fileName);
            li.appendChild(fileSize);
            li.appendChild(removeIcon);
            fileList.appendChild(li);
        }
    }

    dropArea.addEventListener('dragover', (e) => {
        e.preventDefault();
        dropArea.classList.add('highlight');
    });

    dropArea.addEventListener('dragleave', () => {
        dropArea.classList.remove('highlight');
    });

    dropArea.addEventListener('drop', (e) => {
        e.preventDefault();
        dropArea.classList.remove('highlight');
        files = Array.from(e.dataTransfer.files);
        updateFileList();
    });

    dropArea.addEventListener('click', () => fileInput.click());
    fileInput.addEventListener('change', (e) => {
        files = Array.from(e.target.files);
        updateFileList();
    });

    uploadButton.addEventListener('click', () => {
        if (files.length === 0) {
            alert('No file selected');
            return;
        }

        const formData = new FormData();
        for (const file of files) {
            formData.append('file', file);
        }

        const xhr = new XMLHttpRequest();
        xhr.open('POST', '/upload', true);

        progressDiv.innerHTML = '<div class="progress-bar"><div class="progress-fill"></div></div>';
        const progressFill = document.querySelector(".progress-fill");

        xhr.upload.onprogress = (event) => {
            if (event.lengthComputable) {
                let percent = (event.loaded / event.total) * 100;
                progressFill.style.width = percent + '%';
            }
        };

        const displayErrorMessage = () => {
            progressDiv.innerHTML += "<p style='color:red'>Échec de l'envoi !</p>";
        }

        xhr.onload = () => {
            if (xhr.status == 200) {
                progressDiv.innerHTML += '<p style="color:green">Envoi réussi !</p>';
                files = [];
                updateFileList();
            } else displayErrorMessage();
        };
        xhr.onerror = displayErrorMessage;

        xhr.send(formData);
    });
});
