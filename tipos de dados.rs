/* 
Tabela de Tipos de Dados em Rust
Tipo de Dado	Descrição
Inteiros	Números inteiros, podem ser positivos, negativos ou zero.
i8	Inteiro de 8 bits (-128 a 127)
i16	Inteiro de 16 bits (-32768 a 32767)
i32	Inteiro de 32 bits (-2147483648 a 2147483647)
i64	Inteiro de 64 bits (-9223372036854775808 a 9223372036854775807)
isize	Inteiro do tamanho do ponteiro (geralmente i32 ou i64)
Unsigned Integers	Números inteiros não negativos.
u8	Inteiro sem sinal de 8 bits (0 a 255)
u16	Inteiro sem sinal de 16 bits (0 a 65535)
u32	Inteiro sem sinal de 32 bits (0 a 4294967295)
u64	Inteiro sem sinal de 64 bits (0 a 18446744073709551615)
usize	Inteiro sem sinal do tamanho do ponteiro (geralmente u32 ou u64)
Ponto flutuante	Números com casas decimais.
f32	Ponto flutuante de precisão simples (32 bits)
f64	Ponto flutuante de precisão dupla (64 bits)
Booleano	Verdadeiro ou falso.
bool	Valor booleano
Caracteres	Letras, números e símbolos.
char	Um caractere Unicode
Cadeias de caracteres	Sequências de caracteres.
&str	Uma fatia de string literal
String	Uma string mutável
Tuplas	Coleções ordenadas de valores de tipos diferentes.
(T1, T2, ..., Tn)	Tupla com n elementos de tipos T1, T2, ..., Tn
Arrays	Coleções ordenadas de valores do mesmo tipo.
[T; n]	Array de n elementos do tipo T
Slices	Seções de arrays.
&[T]	Uma fatia de um array de elementos do tipo T
Vetores	Coleções dinâmicas de valores do mesmo tipo.
Vec<T>	Vetor de elementos do tipo T
Referências	Ponteiros para outros valores.
&T	Uma referência para um valor do tipo T
Mutáveis	Indica que um valor pode ser modificado.
mut T	Um valor mutável do tipo T
Outros	Existem outros tipos de dados, como structs, enums, closures, etc.
*/

// Inteiro
let numero: i32 = 10;

// Ponto flutuante
let pi: f64 = 3.14;

// Booleano
let is_true: bool = true;

// Caractere
let letra: char = 'a';

// Cadeia de caracteres
let frase: &str = "Olá, mundo!";

// Tupla
let tupla: (i32, f64, &str) = (1, 2.5, "teste");

// Array
let array: [i32; 5] = [1, 2, 3, 4, 5];

//

/*
Em Rust, a diferença entre mutável e imutável diz respeito à capacidade de modificação de um valor após sua criação.

Mutável (mutável): indica que um valor pode ser alterado após sua criação.

É declarado usando a palavra-chave mut antes do tipo de dado.
Exemplo: let mut x: i32 = 5; (o valor de x pode ser alterado posteriormente).
Imutável (imutável): indica que um valor não pode ser alterado após sua criação.

É o padrão em Rust, não sendo necessário explicitar a imutabilidade.
Exemplo: let x: i32 = 5; (o valor de x não pode ser alterado posteriormente).
Essa característica é fundamental em Rust, pois prioriza a segurança de memória evitando alterações acidentais em dados importantes.

Veja alguns pontos para te ajudar a lembrar:

Por padrão, tudo é imutável: a menos que você declare explicitamente como mut, o valor não poderá ser modificado.
Modificações em valores imutáveis: se você tentar modificar um valor imutável, o compilador irá gerar um erro.
Quando usar mutável: use mut apenas quando precisa mesmo modificar o valor.
Benefícios da imutabilidade: a imutabilidade ajuda a prevenir bugs e torna o código mais fácil de entender e raciocinar.
 */

 /*
 
Tuplas em Rust
Em Rust, uma tupla é uma coleção ordenada de valores de tipos diferentes. É como uma lista, mas com um tamanho fixo e tipos de dados específicos para cada elemento.

Declaração:

Para declarar uma tupla, você usa parênteses e especifica os tipos de dados e valores dos elementos separados por vírgulas.

Rust
// Tupla com 3 elementos: um inteiro, um ponto flutuante e uma string
let tupla: (i32, f64, &str) = (1, 2.5, "Olá, mundo!");
Use o código com cuidado.
Acesso aos elementos:

Para acessar um elemento da tupla, você usa o índice do elemento entre colchetes.

Rust
// Acessando o segundo elemento da tupla
let segundo_elemento = tupla.1; // 2.5

// Acessando o último elemento da tupla
let ultimo_elemento = tupla.2; // "Olá, mundo!"
Use o código com cuidado.
Utilização:

As tuplas são usadas em diversas situações em Rust, como:

Retornar múltiplos valores de uma função: Uma função pode retornar uma tupla contendo os valores desejados.
Agrupar dados relacionados: As tuplas podem ser usadas para agrupar dados relacionados, como coordenadas de um ponto ou nome e idade de uma pessoa.
Passar múltiplos argumentos para uma função: Você pode passar uma tupla como argumento para uma função que espera múltiplos valores.
Desestruturação: As tuplas podem ser desestruturadas em variáveis individuais para facilitar o acesso aos seus elementos.
Exemplo:

Rust
// Função que retorna uma tupla com a área e o perímetro de um círculo
fn calcular_circulo(raio: f64) -> (f64, f64) {
    let area = 3.14 * raio * raio;
    let perimetro = 2.0 * 3.14 * raio;
    (area, perimetro)
}

// Usando a função e desestruturando a tupla
let (area, perimetro) = calcular_circulo(5.0);

println!("Área: {}", area);
println!("Perímetro: {}", perimetro);
Use o código com cuidado.
Vantagens:

As tuplas são simples de usar e eficientes em termos de memória.
Permitem agrupar dados de forma concisa.
São compatíveis com muitos recursos da linguagem Rust, como desestruturação e iteração.
Desvantagens:

O tamanho da tupla é fixo no momento da criação.
Não é possível adicionar ou remover elementos de uma tupla.
Conclusão:

As tuplas são uma ferramenta poderosa e versátil em Rust para agrupar e manipular dados de diferentes tipos.
  */

  