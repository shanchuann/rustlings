// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // 创建一个向量来存储结果
        let mut output = Vec::new();

        // 迭代输入向量中的每一个元组
        for (text, command) in input {
            // 根据命令对字符串进行相应的操作
            let result = match command {
                Command::Uppercase => text.to_uppercase(),
                Command::Trim => text.trim().to_string(),
                Command::Append(times) => {
                    let mut appended = text.clone();
                    for _ in 0..times {
                        appended.push_str("bar");
                    }
                    appended
                },
            };
            // 将结果添加到输出向量中
            output.push(result);
        }

        output
    }
}

fn main() {
    // 您可以在此处进行实验
}

#[cfg(test)]
mod tests {
    // 导入 `transformer` 函数
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}