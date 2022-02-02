Bot telegram 
********************

Criado com Rust para estudos sobre programação de bots do telegram. Utilizado chamada para uma API que recebendo uma frase em japonês retorna a análise morfológica

https://labs.goo.ne.jp/api/en/morphological-analysis/

Exemplo
========
Input: 日本に留学生したい！

Output: 

.. code-block:: text

    [
        Morpheme {
            word: "日本",
            class: "名詞",
        },
        Morpheme {
            word: "に",
            class: "格助詞",
        },
        Morpheme {
            word: "留学生",
            class: "名詞",
        },
        Morpheme {
            word: "し",
            class: "動詞語幹",
        },
        Morpheme {
            word: "たい",
            class: "動詞接尾辞",
        },
        Morpheme {
            word: "！",
            class: "句点",
        },
    ]
