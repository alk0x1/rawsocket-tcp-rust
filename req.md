## Servidor
- [X] Escolher um porta para receber as conexões (maior que 1024)
- [X] Aceitar a conexão do cliente
- [X] Criar uma thread com a conexão do cliente (para cada cliente). Na thread:
    - [X] Receber dados enviado pelo cliente
    - [X] Armazenar os dados.
    - Verificar se contém:
      - [] “Sair”
          se sim: fechar a conexão.
          Finalizar a thread.
      - [] “Arquivo”:
        - [] Abrir o arquivo.
        - [] Calcular o Hash do arquivo com SHA (Procure um exemplo de uso do SHA)
          - [] Escolher a ordem para enviar (se necessário)
              Nome do arquivo
              Tamanho
              CRC
              Dados
              Status (ok, nok, etc…)
        Senão:
        - [] Imprimir os dados recebidos.
        - [] Reenviar os dados recebido de volta.

## Cliente 
- [] Fazer a conexão para o endereço da máquina e porta escolhida para o servidor
- [] Enviar uma das opções tratadas no servidor (Requisição), escolhida pelo usuário.
- [] Receber os dados do servidor: (Resposta)
- “Sair”
  - [] Fechar a conexão.
  - [] Finalizar a thread.
- “Arquivo”:
  - [] Receber os dados de acordo com a ordem escolhida. (Acabou de criar um protocolo!)
  - [] Abrir o arquivo.
  - [] Verificar o Hash
- Senão:
 - [] Imprimir os dados recebidos com “Resposta:” antes dos dados.


## O trabalho deve:
- [] Usar Sockets TCP Multi-thread
  - [] Cliente e Servidor

No cliente:
  - [] O usuário escolher a opção para se comunicar com o servidor.
  - [] Enviar as requisições para o servidor.
    - [] Texto.
    - [] Pedido de arquivo.
    - [] Receber as resposta do servidor e fazer o esperado.
    - [] Texto. (Imprimir)
    - [] Arquivo. (verificar e salvar)
    - [] Fazer verificação de integridade do arquivo recebido (Verificar se Hash é igual).
No Servidor:
- [] Receber requisições do Cliente
- [] Tratar corretamente as requisições e fazer o esperado.
  - [] Texto.
  - [] Pedido de arquivo.