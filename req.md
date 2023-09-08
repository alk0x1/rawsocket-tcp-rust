## Servidor
- [X] Escolher um porta para receber as conexões (maior que 1024)
- [X] Aceitar a conexão do cliente
- [X] Criar uma thread com a conexão do cliente (para cada cliente). Na thread:
    - [X] Receber dados enviado pelo cliente
    - [X] Armazenar os dados.
    - Verificar se contém:
      - [X] “Sair”
          se sim: fechar a conexão.
          Finalizar a thread.
      - [X] “Arquivo”:
        - [X] Abrir o arquivo.
        - [X] Calcular o Hash do arquivo com SHA (Procure um exemplo de uso do SHA)
          - [X] Escolher a ordem para enviar (se necessário)
              Nome do arquivo
              Tamanho
              CRC
              Dados
              Status (ok, nok, etc…)
        Senão:
        - [X] Imprimir os dados recebidos.
        - [X] Reenviar os dados recebido de volta.

## Cliente 
- [X] Fazer a conexão para o endereço da máquina e porta escolhida para o servidor
- [X] Enviar uma das opções tratadas no servidor (Requisição), escolhida pelo usuário.
- [X] Receber os dados do servidor: (Resposta)
- “Sair”
  - [X] Fechar a conexão.
  - [X] Finalizar a thread.
- “Arquivo”:
  - [X] Receber os dados de acordo com a ordem escolhida. (Acabou de criar um protocolo!)
  - [X] Abrir o arquivo.
  - [X] Verificar o Hash
- Senão:
 - [X] Imprimir os dados recebidos com “Resposta:” antes dos dados.


## O trabalho deve:
- [X] Usar Sockets TCP Multi-thread
  - [X] Cliente e Servidor

No cliente:
  - [X] O usuário escolher a opção para se comunicar com o servidor.
  - [X] Enviar as requisições para o servidor.
    - [X] Texto.
    - [X] Pedido de arquivo.
    - [X] Receber as resposta do servidor e fazer o esperado.
    - [X] Texto. (Imprimir)
    - [X] Arquivo. (verificar e salvar)
    - [X] Fazer verificação de integridade do arquivo recebido (Verificar se Hash é igual).
No Servidor:
- [X] Receber requisições do Cliente
- [X] Tratar corretamente as requisições e fazer o esperado.
  - [X] Texto.
  - [X] Pedido de arquivo.