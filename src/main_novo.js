import { open, save } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';

let selectedFilePath = null;

document.addEventListener('DOMContentLoaded', () => {
  document.getElementById('uploadBtn').addEventListener('click', async () => {
    try {
      console.log('Botão upload clicado');

      // Abrir diálogo para selecionar arquivo XML
      const selected = await open({
        filters: [{ name: 'XML Files', extensions: ['xml'] }],
        multiple: false
      });

      if (!selected) {
        document.getElementById('filePath').textContent = 'Nenhum arquivo selecionado.';
        document.getElementById('convertBtn').disabled = true;
        return;
      }

      selectedFilePath = selected;
      const fileName = selected.split(/[\\/]/).pop();
      document.getElementById('filePath').textContent = `Arquivo selecionado: ${fileName}`;
      document.getElementById('convertBtn').disabled = false;

    } catch (error) {
      console.error('Erro ao abrir o diálogo de upload:', error);
      document.getElementById('filePath').textContent = 'Erro ao selecionar o arquivo.';
    }
  });

  document.getElementById('convertBtn').addEventListener('click', async () => {
    try {
      if (!selectedFilePath) {
        document.getElementById('result').textContent = 'Nenhum arquivo selecionado.';
        return;
      }

      // Abrir diálogo para salvar arquivo convertido
      const savePath = await save({
        filters: [{ name: 'XML Files', extensions: ['xml'] }],
        defaultPath: selectedFilePath.replace(/\.xml$/i, '-convertido.xml')
      });

      if (!savePath) {
        return; // Usuário cancelou
      }

      // Chamar comando Rust para converter e salvar
      const result = await invoke('convert_and_save_xml', {
        inputPath: selectedFilePath,
        savePath: savePath
      });

      document.getElementById('result').textContent = `Arquivo salvo em: ${result}`;

    } catch (error) {
      console.error('Erro na conversão:', error);
      document.getElementById('result').textContent = `Erro: ${error}`;
    }
  });
});
