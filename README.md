# impiler
imp compiler created using rust


Instruções

### Instalando o rust
* 1- Faça o download do rust através do link https://www.rust-lang.org/pt-BR/install.html  
* 2- (Windows) Instale o Visual Studio Installer https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=15  
  * 2.1- Instale as ferramentas de build do Visual C++, marcando a opção indicada na foto (Conjunto de ferramentas do VC++ 2015.3)
  ![Passo 2.1](https://i.imgur.com/NejaFP3.png)
* 3- Instale o Cargo o gerenciador de pacotes do rust https://doc.rust-lang.org/cargo/getting-started/installation.html  
  * 3.1- (Windows) Caso esteja utilizando o VS Code e ao tentar instalar receba o erro: ``` could not remove 'rustup-bi
n' ``` feche o VS Code e tente novamente.
  
### Rodando o código
* 1- Use o Cargo para compilar e executar o código.   
  * 1.1 - ```cargo build```: Recupera dependências e compila o código gerando um .exe.  
  * 1.2 - ```cargo run```: compila e executa o código.  

### Rodando o código pelo docker
* 1- Instalar o docker  
  https://docs.docker.com/
* 2- Abrir o Docker Quick Start Terminal
* 3- Docker funcionando ?  
    ```docker info``` 

* 4- Construir a imagem do docker  
    ```docker build -t impiler .```

* 5- Rodar o projeto  
    ```docker run -it impiler```  