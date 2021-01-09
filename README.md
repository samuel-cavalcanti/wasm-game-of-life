# The Game of Life WebAssembly

![](game_of_life.gif)

Toy project  usando Rust + WebAssembly para compreender como é programar em rust pensando em desenvolvimento web( WebAssembly).

## dependências

- rustc 1.48.0
- wasm-pack 0.9.1

### compilação  

```shell
chmod +x build.sh
./build
```

### subi o server

```shell
cd www
npm run start
```

### Verdades sobre Rust e seu ecossistema

- __boas__: gerenciador de pacotes (cargo) e crates.io são muito
bons, testar Código em rust é ridiculamente fácil, declaração de variáveis é delicia. Ter um compilador por mais que chato te ajudando
a lembrar e cuidando em partes do ciclo de vída da variável é algo positivo. Result e Option é melhor que try-catch.

- __ruins__: Não possuí polimorfismo, rust não resolve o gerenciamento de memória pra você, ele te ajuda, rust te incentiva
a fazer cópias inteiras ao invés de passar a referência e ter que lidar com o life cycle dela.  

## Conclusão

"Rust é mais fácil que C++", "Rust é C++ só que bom",
"Rust é a melhor linguagem do mundo". Tudo mentira. Não existe
cura para péssimo desenvolvedor a não ser estudar. Um compilador
Mágico não foi inventado.
