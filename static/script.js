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
            li.textContent = file.name;
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
        files = e.dataTransfer.files;
        updateFileList();
    });

    dropArea.addEventListener('click', () => fileInput.click());
    fileInput.addEventListener('change', (e) => {
        files = e.target.files;
        updateFileList();
    });

    uploadButton.addEventListener('click', async () => {
        if (files.length === 0) {
            alert('No file selected');
            return;
        }

        const formData = new FormData();
        for (const file of files) {
            formData.append('file', file);
        }

        const response = await fetch('/upload', {
            method: 'POST',
            body: formData,
        });

        if (response.ok) {
            progressDiv.style.color = 'green';
            progressDiv.innerText = 'Envoi réussi !';
            files = [];
            updateFileList();
        } else {
            progressDiv.style.color = 'red';
            progressDiv.innerText = 'Echec de l\'envoi !';
        }
    });
});
