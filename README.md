# Introdução

Este repositório contém a implementação do clássico [Jogo da Vida](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) em Rust.

A implementação imprime o "tabuleiro" diretamente em um terminal linux.


# Utilização

A implementação suporta parametros de execução e para mais detalhes basta executar o comando:

```shell script
$ cargo run -- --help
```

Exemplo da saída do comando de ajuda:

```shell script
game_of_life 1.0.0

USAGE:
    game_of_life [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --columns <columns>          Number of columns [default: 60]
    -d, --delay <delay>              Refresh delay in miliseconds (The higher the faster) [default: 100]
        --layout <layout>            Layout file
    -p, --padding <padding>          Board padding [default: 5]
        --population <population>    Starting alive population percentage [default: 15]
    -r, --rows <rows>                Number of rows [default: 30]


```

Para executar o programa sem nenhum parametro, basta utilizar o comando abaixo

```shell script
 # Execucao com parametros padrão
 $ cargo run

```

Também é possível informar um arquivo de layout para com a população inicial, os exemplos  encontram-se na pasta `data/`
