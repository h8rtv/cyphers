# Relatório:

## Faça a criptanálise da mensagem cifrada com o cifrador de César e mostre a chave usada. Qual é o texto criptografado?

Pouco conhecimento faz com que as pessoas se sintam
orgulhosas. Muito conhecimento, que se sintam
humildes. Eh assim que as espigas sem graos erguem
desdenhosamente a cabeca para o ceu, enquanto as
cheias as baixam para a terra, sua mae.
Leonardo Da Vinci.

## O algoritmo de Vernam é vulnerável à análise de frequências? Justifique.

Não. Se o "one time pad" for gerado competentemente e utilizado somente uma vez,
o algoritmo não é vulnerável a análise de frequências. Caso ele seja utilizado 
diversas vezes, ou uma chave menor que a mensagem seja usada em parcelas da
mensagem, uma vulnerabilidade é criada.

### Como será feita a geração da chave?

De maneira pseudoaleatória e única, utilizando um bom algoritmo gerador de números
aleatórios.

### É possivel usar o algoritmo de Vernam para cifrar uma base de dados? Justifique.

Não é uma boa aplicação do algoritmo. Para ele ser seguro, uma chave do tamanho
do banco de dados deve ser gerada, o que duplica a quantidade dados para armazenar.
Também, para não ser vulnerável a análise de frequências, cada alteração do banco 
deverá gerar uma nova chave e cifrar todo o banco novamente.

## O algoritmo RC4 é vulnerável à análise de frequências? Justifique.

Não. Assim como o Vernam, a saída do algoritmo é embaralhada de forma que não há
uma distribuição das frequências dos bytes que possa ser analisado e que um possível
ataque possa tomar uma decisão informada com base na cifra como ocorre na cifra
de cesar.

