const { invoke } = window.__TAURI__.core;
const { dialog } = window.__TAURI__.shell;

document.getElementById('uploadBtn').addEventListener('click', async () => {
  console.log('Botão upload clicado');
  const selected = await open({ filters: [{ name: 'XML', extensions: ['xml'] }] });
  if (!selected) {
    document.getElementById('filePath').textContent = 'Nenhum arquivo selecionado.';
    document.getElementById('convertBtn').disabled = true;
    return;
  }
  document.getElementById('filePath').textContent = `Arquivo selecionado: ${selected}`;
  document.getElementById('convertBtn').disabled = false;
});

document.getElementById('convertBtn').addEventListener('click', async () => {
  const filePath = document.getElementById('filePath').textContent.replace('Arquivo selecionado: ', '');
  const savePath = await open({ directory: false, multiple: false, filters: [{ name: 'XML', extensions: ['xml'] }] });
  if (!savePath) return;
  const result = await invoke('convert_and_save_xml', { inputPath: filePath, savePath });
  document.getElementById('result').textContent = `Arquivo salvo em: ${result}`;
});
