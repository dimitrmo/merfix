import init, {
    version,
    remove_exif,
    detect_image_mime_type,
    detect_image_extension,
    supported_extensions,
    supported_mime_types
} from 'https://cdn.jsdelivr.net/npm/merfix@0.1.33/merfix.min.js';

async function setup() {
    await init(); // Load WebAssembly module

    console.log('>> supported extensions', supported_extensions());
    console.log('>> supported mime types', supported_mime_types());

    document.getElementById("reset-btn").addEventListener("click", async () => {
        const link = document.getElementById("download-link");
        if (link) {
            link.style.display = "none";
            link.textContent = "Download";
        }

        const fileInput = document.getElementById("file-input");
        if (fileInput) {
            fileInput.value = ""
        }
    })

    document.getElementById("process-btn").addEventListener("click", async () => {
        const fileInput = document.getElementById("file-input");
        if (!fileInput.files.length) {
            return alert("Select an image");
        }

        const file = fileInput.files[0];
        const fileName = file.name;
        const fileExtension = fileName.split('.').pop();

        console.log('>> file name', fileName);
        console.log('>> file extension', fileExtension);

        const arrayBuffer = await file.arrayBuffer();
        const raw = new Uint8Array(arrayBuffer);
        const result = remove_exif(raw, fileExtension);

        const detectedType = detect_image_mime_type(raw);
        console.log('>> detected type', detectedType);

        const detectedExtension = detect_image_extension(raw);
        console.log('>> detected extension', detectedExtension);

        if (result.is_error()) {
            console.error(">> error", result.get_error());
        }

        const processedBytes = result.get_data();

        // Create downloadable image
        const blob = new Blob([processedBytes], { type: file.type });
        const url = URL.createObjectURL(blob);

        const link = document.getElementById("download-link");
        link.href = url;
        link.download = `image_no_exif.${fileExtension}`;
        link.style.display = "block";
        link.textContent = "Download Image";
    });
}

setup().then(_ => console.log('>> setup ready', version()));
