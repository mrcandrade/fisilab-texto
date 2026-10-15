<p align="center">
  <img src="assets/fisilab/fisilab-texto-256.png" alt="Ícone do Texto" width="128">
</p>

<h1 align="center">Texto · Suíte FisiLab</h1>

<p align="center">
  <b>Processador de texto</b><br>
  Alternativa livre ao <b>Microsoft Word</b>
</p>

<p align="center">
  Disponibilizado pelo <b>FisiLab — LARFIC · UFRGS</b> em parceria com o <b>CTA — Centro de
  Tecnologia Acadêmica</b>, para democratizar o acesso à tecnologia entre os alunos da UFRGS.
</p>

---

## O que faz

- **Interface familiar:** faixa de opções com as guias, grupos e atalhos que quem usa o Word já
  conhece; a galeria de estilos mostra a prévia ao vivo.
- **Seus arquivos:** abre e salva `.docx`, e também `.odt`, `.rtf`, `.html`, `.md`, `.tex` e
  `.txt`; importa documentos `.doc` do Word 97-2003; exporta PDF com texto selecionável, links e
  marcadores.
- **Formatação completa:** fontes, parágrafos, estilos, listas, tabelas (mesclar, dividir,
  estilos, fórmulas), colunas, quebras de seção, cabeçalhos e rodapés, numeração de páginas, marca
  d'água e bordas de página.
- **Trabalho acadêmico:** sumário, notas de rodapé e de fim, citações e bibliografia (APA, MLA,
  Chicago, IEEE), legendas, referências cruzadas e índice remissivo.
- **Revisão:** ortografia e gramática enquanto você digita, comentários nas margens, controle de
  alterações (aceitar e rejeitar), comparação de documentos e verificador de acessibilidade.
- **Mala direta:** a partir de um CSV, com campos, bloco de endereço, envelopes e etiquetas.
- **Sem sobrescrever sem querer:** o AutoSave vem desligado; ligue-o no botão AutoSave ou em
  Arquivo › Opções se quiser que o arquivo seja regravado a cada alteração.
- **Privado e rápido:** tudo funciona sem internet; o texto é diagramado em Rust nativo, na GPU.
- **Automação:** cada ação é um comando, disponível pela linha de comando (`fisilab-texto-cli`), por
  um canal de controle JSON e por um servidor MCP para agentes de IA.

A interface abre em português do Brasil na primeira execução e pode ser trocada em Arquivo ›
Opções. As cores são as originais do WordCraft, com as opções de tema **Noite** (escuro) e **Dia**
(claro, o padrão).

## Instalação

### Windows

Baixe o instalador `FisiLab-Texto-Setup-<versão>.exe` na [página de downloads da Suíte FisiLab](https://github.com/mrcandrade/larfic/releases/latest) e execute-o.

### Linux

Baixe o `FisiLab-Texto-<versão>-x86_64.AppImage` na [página de downloads da Suíte FisiLab](https://github.com/mrcandrade/larfic/releases/latest), marque-o como executável (`chmod +x` ou Propriedades › Permitir executar) e abra-o. No Ubuntu/Debian também é possível instalar o `fisilab-texto_<versão>_amd64.deb` com `sudo apt install ./fisilab-texto_<versão>_amd64.deb`. Requer Ubuntu 22.04 ou mais recente (ou equivalente).

### Compilar a partir do código

É preciso ter o [Rust](https://www.rust-lang.org/pt-BR/tools/install) instalado.

```sh
git clone https://github.com/mrcandrade/fisilab-texto
cd fisilab-texto
cargo run --release -p wordcraft                  # o aplicativo (fisilab-texto.exe)
cargo run --release -p wordcraft -- --sample      # com o documento de exemplo
cargo run --release -p wordcraft-cli -- --help    # a linha de comando (fisilab-texto-cli.exe)
```

Os executáveis ficam em `target/release/` (`fisilab-texto` e `fisilab-texto-cli`).

## Créditos

O Texto é uma versão modificada do [WordCraft](https://github.com/storytold/wordcraft), criado
pelo ArtCraft Team e contribuidores. Todo o trabalho de edição, formatos de arquivo e ferramentas
vem do projeto original; o FisiLab alterou o nome, a identidade visual, o tema de cores, a tela de
abertura, os links e o empacotamento (veja [MODIFICACOES.md](MODIFICACOES.md)).

Licença: [MIT](LICENSE-MIT) ou [Apache-2.0](LICENSE-APACHE), à sua escolha. Os avisos exigidos
estão em [NOTICE](NOTICE), os materiais de terceiros em [ATTRIBUTION.md](ATTRIBUTION.md) e o
README original em [README.upstream.md](README.upstream.md).

Os nomes e marcas do projeto original pertencem aos seus titulares; esta versão não é afiliada nem
endossada por eles. Microsoft e Microsoft Word são marcas da Microsoft Corporation, citadas apenas
para descrever a compatibilidade.

## Manifesto

Conhecimento produzido em uma universidade pública deve circular livremente — e as ferramentas que
usamos para ensinar, pesquisar e criar também.
Leia o [Manifesto pelas tecnologias livres](https://github.com/mrcandrade/larfic/blob/main/suite-fisilab/MANIFESTO.md).

## Privacidade

O Texto não coleta dados de uso nem envia o conteúdo dos seus arquivos para lugar nenhum: não há
telemetria, estatísticas nem verificação automática de atualizações. Veja o que ele guarda no seu
computador e quando usa a internet no [aviso de privacidade da Suíte FisiLab](https://github.com/mrcandrade/larfic/blob/main/suite-fisilab/PRIVACIDADE.md).

---

<p align="center">
  <a href="https://cta.if.ufrgs.br"><img src="assets/fisilab/cta-logo.webp" alt="CTA — Centro de Tecnologia Acadêmica" height="48"></a>
  &nbsp;&nbsp;&nbsp;
  <a href="https://github.com/mrcandrade/larfic/tree/main/suite-fisilab"><img src="assets/fisilab/larfic-logo.png" alt="LARFIC" height="48"></a>
  <br>
  <sub>Suíte FisiLab · FisiLab — LARFIC · UFRGS · em parceria com o CTA — Centro de Tecnologia Acadêmica</sub>
</p>
