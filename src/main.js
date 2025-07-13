// Function to find the correct invoke function
function getTauriInvoke() {
  if (window.__TAURI__.invoke) {
    return window.__TAURI__.invoke;
  } else if (window.__TAURI__.core && window.__TAURI__.core.invoke) {
    return window.__TAURI__.core.invoke;
  } else if (window.__TAURI__.tauri && window.__TAURI__.tauri.invoke) {
    return window.__TAURI__.tauri.invoke;
  } else {
    // Search through all properties
    for (const key in window.__TAURI__) {
      const obj = window.__TAURI__[key];
      if (obj && typeof obj === 'object' && obj.invoke) {
        console.log(`Using invoke from window.__TAURI__.${key}.invoke`);
        return obj.invoke;
      }
    }
    return null;
  }
}

// Simple direct approach for Tauri v2
let selectedFilePath = null;
let selectedFileContent = null;

document.addEventListener('DOMContentLoaded', () => {
  console.log('DOM Content Loaded - Initializing application...');  // Check if Tauri is available
  if (!window.__TAURI__) {
    console.error('Tauri API not available');
    alert('Erro: Esta aplicação deve ser executada através do Tauri.');
    return;
  }

  console.log('Tauri API found:', window.__TAURI__);
  console.log('Available properties:', Object.keys(window.__TAURI__));

  // Check for different possible invoke locations
  if (window.__TAURI__.invoke) {
    console.log('invoke found at window.__TAURI__.invoke');
  } else if (window.__TAURI__.core && window.__TAURI__.core.invoke) {
    console.log('invoke found at window.__TAURI__.core.invoke');
  } else if (window.__TAURI__.tauri && window.__TAURI__.tauri.invoke) {
    console.log('invoke found at window.__TAURI__.tauri.invoke');
  } else {
    console.log('invoke function not found, searching...');
    // Search through the object structure
    for (const key in window.__TAURI__) {
      const obj = window.__TAURI__[key];
      if (obj && typeof obj === 'object') {
        console.log(`Checking ${key}:`, Object.keys(obj));
        if (obj.invoke) {
          console.log(`invoke found at window.__TAURI__.${key}.invoke`);
        }
      }
    }
  }

  const fileInput = document.getElementById('fileInput');
  const uploadBtn = document.getElementById('uploadBtn');
  const convertBtn = document.getElementById('convertBtn');

  // Check if all elements exist
  if (!fileInput) {
    console.error('fileInput element not found');
    return;
  }
  if (!uploadBtn) {
    console.error('uploadBtn element not found');
    return;
  }
  if (!convertBtn) {
    console.error('convertBtn element not found');
    return;
  }

  console.log('All elements found, setting up event listeners...');

  // Drag and drop functionality
  const fileInfo = document.getElementById('fileInfo');

  fileInfo.addEventListener('dragover', (e) => {
    e.preventDefault();
    fileInfo.classList.add('drag-over');
  });

  fileInfo.addEventListener('dragleave', (e) => {
    e.preventDefault();
    fileInfo.classList.remove('drag-over');
  });

  fileInfo.addEventListener('drop', (e) => {
    e.preventDefault();
    fileInfo.classList.remove('drag-over');

    const files = e.dataTransfer.files;
    if (files.length > 0) {
      const file = files[0];
      if (file.name.toLowerCase().endsWith('.xml')) {
        handleFileSelection(file);
      } else {
        document.getElementById('messageText').textContent = 'Por favor, selecione apenas arquivos XML.';
        document.getElementById('message').style.display = 'block';
      }
    }
  });

  // Function to handle file selection (both from input and drag-drop)
  async function handleFileSelection(file) {
    console.log('File selected:', file.name);
    try {
      selectedFilePath = file.name;
      selectedFileContent = await file.text();

      document.getElementById('fileName').textContent = file.name;
      document.getElementById('fileStatus').textContent = 'Arquivo pronto para conversão';
      document.getElementById('convertBtn').disabled = false;

      // Add visual feedback that file is loaded
      document.getElementById('fileInfo').classList.add('has-file');
      document.getElementById('message').style.display = 'none';

    } catch (error) {
      console.error('Erro ao ler o arquivo:', error);
      document.getElementById('fileName').textContent = 'Erro ao selecionar arquivo';
      document.getElementById('fileStatus').textContent = 'Tente novamente';
    }
  }

  uploadBtn.addEventListener('click', async (e) => {
    console.log('Upload button clicked');
    e.preventDefault();
    fileInput.click();
  });

  fileInput.addEventListener('change', async (event) => {
    console.log('File input changed');
    const file = event.target.files[0];
    if (file) {
      await handleFileSelection(file);
    } else {
      console.log('No file selected');
      document.getElementById('fileName').textContent = 'Nenhum arquivo selecionado';
      document.getElementById('fileStatus').textContent = 'Clique em "Anexar XML" ou arraste um arquivo XML aqui';
      document.getElementById('convertBtn').disabled = true;
      selectedFilePath = null;
      selectedFileContent = null;
      document.getElementById('fileInfo').classList.remove('has-file');
    }
  });

  convertBtn.addEventListener('click', async (e) => {
    console.log('Convert button clicked');
    e.preventDefault();
    try {
      if (!selectedFileContent) {
        document.getElementById('messageText').textContent = 'Nenhum arquivo selecionado.';
        document.getElementById('message').style.display = 'block';
        return;
      }

      console.log('Starting conversion...');
      // Mostrar indicador de carregamento
      document.getElementById('loading').style.display = 'block';
      document.getElementById('message').style.display = 'none';

      // Chamar comando Rust para converter o XML e abrir diálogo de salvamento
      console.log('Invoking convert_and_save_xml_with_dialog...');

      const tauriInvoke = getTauriInvoke();
      if (!tauriInvoke) {
        throw new Error('Função invoke do Tauri não encontrada');
      }

      const savedPath = await tauriInvoke('convert_and_save_xml_with_dialog', {
        xmlContent: selectedFileContent,
        originalFileName: selectedFilePath
      });
      console.log('Conversion and save successful:', savedPath);

      // Ocultar indicador de carregamento
      document.getElementById('loading').style.display = 'none';
      document.getElementById('message').style.display = 'block';

      // Mostrar mensagem de sucesso com o caminho onde foi salvo
      document.getElementById('messageText').textContent = `Arquivo salvo com sucesso em: ${savedPath}`;

    } catch (error) {
      console.error('Erro na conversão:', error);
      document.getElementById('loading').style.display = 'none';
      document.getElementById('message').style.display = 'block';

      // Verificar se foi cancelado pelo usuário
      if (error.toString().includes('cancelada')) {
        document.getElementById('messageText').textContent = 'Operação cancelada.';
      } else {
        document.getElementById('messageText').textContent = `Erro: ${error}`;
      }
    }
  });

  console.log('Application initialized successfully');
});
