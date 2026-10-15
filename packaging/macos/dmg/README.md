# Janela do DMG no macOS

Esta pasta guarda os dois arquivos que o Finder usa para desenhar a janela do DMG. O `package.sh`
apenas os copia para a imagem; nada é gerado durante a compilação.

| Arquivo | O que é |
|---|---|
| `background.tiff` | Imagem de fundo da janela (1x e 2x, HiDPI). Vai para `.background/background.tiff`. |
| `dmg-layout.DS_Store` | Ajustes de visualização do volume (tamanho da janela, ícones, fundo). Vai para `.DS_Store`. |

Os dois são gerados por `suite-fisilab/scripts/gerar_mac.py`, no repositório
[mrcandrade/larfic](https://github.com/mrcandrade/larfic). Para mudar o visual, altere o script lá e
copie os arquivos novos para cá.

## Nomes que o layout espera

- Nome do volume: `Texto` (exatamente, sem versão; o Finder acha o fundo pelo nome do volume).
- Aplicativo: `Texto.app`.
- Atalho para a pasta de aplicativos: `Aplicativos` (`ln -s /Applications`).
