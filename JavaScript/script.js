document.addEventListener('DOMContentLoaded', () => {
    const arrayInputDiv = document.getElementById('arrayInput');
    const addNumberButton = document.getElementById('addNumberButton');
    const removeNumberButton = document.getElementById('removeNumberButton');
    const dataForm = document.getElementById('dataForm');
    const loadingDiv = document.getElementById('loading');
    const responseDiv = document.getElementById('response');

    addNumberButton.addEventListener('click', () => {
        const newInput = document.createElement('input');
        newInput.type = 'number';
        newInput.name = 'arrayInput[]';
        newInput.required = true;
        arrayInputDiv.appendChild(newInput);
    });

    removeNumberButton.addEventListener('click', () => {
        const inputs = arrayInputDiv.getElementsByTagName('input');
        if (inputs.length > 1) {
            arrayInputDiv.removeChild(inputs[inputs.length - 1]);
        }
    });

    dataForm.addEventListener('submit', (event) => {
        event.preventDefault();

        loadingDiv.style.display = 'block';
        responseDiv.textContent = '';

        const textInput = document.getElementById('textInput').value;
        const arrayInputs = arrayInputDiv.getElementsByTagName('input');
        const arrayValues = Array.from(arrayInputs).map(input => parseInt(input.value));

        const data = {
            text: textInput,
            array: arrayValues
        };

        fetch('http://127.0.0.1:8101', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        })
        .then(response => response.text())
        .then(data => {
            loadingDiv.style.display = 'none';
            responseDiv.textContent = `Ответ сервера: ${data}`;
        })
        .catch((error) => {
            loadingDiv.style.display = 'none';
            responseDiv.textContent = `Ошибка: ${error}`;
        });
    });
});
