use std::collections::HashMap;
use std::io;

fn sistema_departamento()
{
    let mut departamento_pessoas = HashMap::new();

    loop
    {
        let mut comando = String::new();
        println!("Digite o comando do tipo: \"Adicione <pessoa> ao <departamento>\":");
        io::stdin().read_line(& mut comando).expect("comando error");
        let comando = comando.trim();
        let mut token_comando = comando.split_whitespace();

        let pessoa = match token_comando.nth(1)
        {
            Some(p) => p,
            None =>
            {
                println!("Erro pessoa: comando inválido!");
                continue;
            }
        };

        let departamento = match token_comando.nth(1)
        {
            Some(d) => d,
            None =>
            {
                println!("Erro departamento: comando inválido!");
                continue;
            }
        };

        let empregado = departamento_pessoas.entry(String::from(departamento)).or_insert(vec![]);
        empregado.push(String::from(pessoa));

        println!("{:?}", departamento_pessoas);
    }
}

fn main()
{
    sistema_departamento();
}
