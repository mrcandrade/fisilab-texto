# Modificações do Texto em relação ao WordCraft

O Texto é uma versão modificada do [WordCraft](https://github.com/storytold/wordcraft)
(© 2026 ArtCraft Team e contribuidores do WordCraft, MIT OR Apache-2.0), feita pelo
**FisiLab — LARFIC · UFRGS** em parceria com o **CTA — Centro de Tecnologia Acadêmica** para a
Suíte FisiLab. As modificações seguem os mesmos termos (MIT OR Apache-2.0).

## O que mudou e por quê

### Nome e marca
- O produto passou a se chamar **Texto** em todo texto visível: título da janela, nome do
  aplicativo, janela "Sobre", diálogos, mensagens de status e de erro, tela inicial, Opções,
  textos de ajuda da linha de comando e do servidor MCP, e nos catálogos de tradução
  (`crates/ui-egui/src/i18n/*.tsv`, chaves e traduções atualizadas juntas). Onde "Texto" sozinho
  se confundiria com a palavra comum (por exemplo "Procurar no FisiLab Texto…", "Mostrar a tela
  inicial ao abrir o FisiLab Texto", o nome no instalador e no servidor MCP), o programa é chamado
  de **FisiLab Texto**. Os metadados gravados nos arquivos exportados (aplicativo do `.docx`,
  criador/produtor do PDF, gerador de ODT/RTF/HTML/LaTeX), o título dos trabalhos de impressão e o
  nome de autor padrão (`FisiLab Texto User`) também dizem FisiLab Texto.
- **Motivo:** o NOTICE do WordCraft diz que o nome, o logotipo e as marcas ArtCraft não estão
  cobertos pela licença aberta. Por isso a pasta `docs/brand/` (logotipos e wordmark ArtCraft) foi
  removida e nenhuma marca ArtCraft é exibida ou embutida. O crédito em texto ao ArtCraft Team e ao
  WordCraft foi mantido (e é exigido).

### Identidade visual da Suíte FisiLab
- Novo crate `crates/fisilab-brand` (compartilhado pela suíte): paleta, tela de abertura com a
  logo do CTA (cerca de 3 s, pode ser pulada com um clique ou tecla; `FISILAB_NO_SPLASH=1`
  desliga) e a janela **"Sobre a Suíte FisiLab"** com créditos, manifesto e logos da UFRGS e do CTA.
- Ícone novo do aplicativo (`assets/fisilab/fisilab-texto*.png`, `fisilab-texto.ico`) na janela, na barra
  de tarefas e no executável.
- Paleta original do WordCraft, sem cores da marca nos temas. O tema escuro original aparece como
  **Noite** e o claro original (o padrão, como no WordCraft) como **Dia**; os dois vêm primeiro em
  Exibir › Tema da interface e em Arquivo › Opções, seguidos de "Usar configuração do sistema".
  O comando `ui.theme` aceita `noite`/`night` e `dia`/`day`, além de `dark`, `light` e `system`.
  Foram acrescentados aos tokens do tema `accent_hover` e `link`, com os mesmos valores originais.

### Ajuda, links e comunidade
- Guia **Ajuda**: o botão "Community Discord" foi trocado por **"Sobre a Suíte FisiLab"**, e um novo
  grupo "Suíte FisiLab" traz **Site da Suíte FisiLab**, **Manifesto pelas tecnologias livres**,
  **Projeto original (WordCraft)** e **Reportar um problema**
  (`https://github.com/mrcandrade/fisilab-texto/issues`). Novos comandos de interface: `ui.fisilab`,
  `ui.fisilabSite`, `ui.manifesto`, `ui.upstream`, `ui.reportIssue` (o `ui.discord` foi removido).
- O botão "Discord" da barra de título virou o botão **FisiLab**, que abre a janela "Sobre a Suíte
  FisiLab" (pode ser escondido em Arquivo › Opções › Geral).
- No Backstage (Arquivo), o link "Discord community" virou "Sobre a Suíte FisiLab".
- A janela "Sobre o Texto" mantém as abas de contribuidores e modelos do WordCraft (crédito aos
  autores originais), com os links da suíte no lugar do site e do Discord da ArtCraft.
- O documento de exemplo, o servidor MCP e o cabeçalho LaTeX deixaram de apontar para
  getartcraft.com/discord.

### Comunicação com servidores externos
- O WordCraft não tem verificação de atualizações, telemetria nem envio de relatórios de falha:
  não há cliente HTTP no projeto. As únicas conexões de rede são locais (canal de controle e
  ponte MCP em `127.0.0.1`, e a integração com o Zotero instalado no computador). Nada precisou
  ser desligado; os links que abriam páginas da ArtCraft foram redirecionados (acima).

### Idioma
- Uma instalação nova abre em **português do Brasil** (o aplicativo desktop define `pt-br` só quando ainda não há `ui.json`; o padrão da biblioteca é o original, "automático"); a escolha
  em Arquivo › Opções (incluindo "automático") continua funcionando e é guardada. Com a interface em
  português, documentos novos são escritos em pt-BR.

### Executáveis, identificador e pastas
- Executáveis: `fisilab-texto.exe` (aplicativo) e `fisilab-texto-cli.exe` (linha de comando), definidos com
  `[[bin]]` nos crates `apps/wordcraft` e `apps/wordcraft-cli` (os nomes dos pacotes Cargo não
  mudaram: `cargo run -p wordcraft`).
- Identificador do aplicativo: `br.ufrgs.fisilab.texto` (janela, arquivo `.desktop`,
  metainfo e MIME do Linux).
- Pasta de configurações: `%APPDATA%\FisiLab Texto` (Windows), `~/Library/Application Support/FisiLab Texto`
  (macOS) e `~/.config/fisilab-texto` (Linux) — não se mistura com uma instalação do WordCraft.
  Dicionários do usuário: `%LOCALAPPDATA%\FisiLab Texto\dictionaries`.
- Recursos do Windows (`apps/wordcraft/build.rs`): ícone `assets/fisilab/fisilab-texto.ico`,
  ProductName, FileDescription, CompanyName e LegalCopyright da Suíte FisiLab.
- "Tornar padrão" no Windows abre Configurações › Aplicativos padrão em
  `registeredAppMachine=FisiLab.Texto`.
- `packaging/windows/wordcraft.wxs` e `package.ps1` foram adaptados (nomes, novo UpgradeCode e GUIDs,
  ProgIds `FisiLab.Texto.*`, descrições em português). O MSI instala em
  `%ProgramFiles%\Suite FisiLab\Texto` e registra as Capabilities em
  `HKLM\Software\FisiLab\Texto\Capabilities` (entrada `FisiLab.Texto` em RegisteredApplications).
  O zip portátil contém a pasta `fisilab-texto-<versão>-windows-<arq>-portable`. O instalador
  oficial da suíte é gerado separadamente (Inno Setup).

### Empacotamento Linux

- O empacotamento para Linux (AppImage e .deb) usa os novos nomes, ícones e identificadores:
  `FisiLab-Texto-<versão>-x86_64.AppImage`, `fisilab-texto_<versão>_amd64.deb`, binários
  `fisilab-texto` e `fisilab-texto-cli` e o identificador `br.ufrgs.fisilab.texto`.

### Repositório
- Removida a pasta `.github/` (fluxos de CI, release e lint do projeto original, que rodariam
  contra este fork).
- `README.md` novo, em português; o original está em `README.upstream.md`.
- `NOTICE` ganhou um cabeçalho do Texto; o texto original segue abaixo dele, sem alterações.

## Segurança e privacidade
- O AutoSave agora vem **desligado** em uma instalação nova (`crates/ui-egui/src/lib.rs` e
  `crates/engine/src/session.rs`). Antes vinha ligado e, depois de um Salvar, reescrevia o arquivo
  do aluno a cada alteração, sem deixar a versão anterior. A preferência não é gravada em disco; o
  usuário continua podendo ligá-lo pelo botão AutoSave ou em Opções, e a recuperação de documentos
  não salvos (`file.recover`) não depende dele.

## O que não mudou
- Todo o editor: formatos de arquivo, diagramação, comandos, atalhos, revisão, mala direta,
  automação (CLI, canal de controle, MCP) e as traduções existentes.
- Nomes de crates, módulos e identificadores internos (`wordcraft-*`), variáveis de ambiente
  (`WORDCRAFT_*`), o identificador do servidor MCP (`wordcraft`), URIs `wordcraft://`, e os
  comentários e a documentação histórica em `docs/`.
- `LICENSE-MIT`, `LICENSE-APACHE` e `ATTRIBUTION.md`.
- Os temas originais Claro, Escuro e Sistema, e os scripts de empacotamento para macOS e web (não adaptados;
  a suíte distribui para Windows e Linux).
