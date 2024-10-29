use rustyline::DefaultEditor;
use std::collections::HashMap;

fn main() {
    println!("Hare 0.1.0");
    let mut rl = DefaultEditor::new().unwrap();
    let scope = &scope();
    loop {
        let line = rl.readline("> ").unwrap().trim().to_string();
        if !line.is_empty() {
            rl.add_history_entry(&line).unwrap_or_default();
            let program = parse_expr(line, scope);

            println!("{:#?}", &program);
            println!("{}", program.eval(scope).display(scope));
        }
    }
}

fn scope() -> Scope {
    HashMap::from([
        (
            "Number".to_string(),
            Object {
                raw_data: 0f64.to_ne_bytes().to_vec(),
                methods: HashMap::from([
                    (
                        "+".to_string(),
                        Function::BuiltIn(|args, scope| {
                            let n1 =
                                f64::from_ne_bytes(args[0].raw_data.clone().try_into().unwrap());
                            let n2 =
                                f64::from_ne_bytes(args[1].raw_data.clone().try_into().unwrap());
                            let mut ins = scope.get("Number").unwrap().clone();
                            ins.raw_data = (n1 + n2).to_ne_bytes().to_vec();
                            ins
                        }),
                    ),
                    (
                        "-".to_string(),
                        Function::BuiltIn(|args, scope| {
                            let n1 =
                                f64::from_ne_bytes(args[0].raw_data.clone().try_into().unwrap());
                            let n2 =
                                f64::from_ne_bytes(args[1].raw_data.clone().try_into().unwrap());
                            let mut ins = scope.get("Number").unwrap().clone();
                            ins.raw_data = (n1 - n2).to_ne_bytes().to_vec();
                            ins
                        }),
                    ),
                    (
                        "*".to_string(),
                        Function::BuiltIn(|args, scope| {
                            let n1 =
                                f64::from_ne_bytes(args[0].raw_data.clone().try_into().unwrap());
                            let n2 =
                                f64::from_ne_bytes(args[1].raw_data.clone().try_into().unwrap());
                            let mut ins = scope.get("Number").unwrap().clone();
                            ins.raw_data = (n1 * n2).to_ne_bytes().to_vec();
                            ins
                        }),
                    ),
                    (
                        "/".to_string(),
                        Function::BuiltIn(|args, scope| {
                            let n1 =
                                f64::from_ne_bytes(args[0].raw_data.clone().try_into().unwrap());
                            let n2 =
                                f64::from_ne_bytes(args[1].raw_data.clone().try_into().unwrap());
                            let mut ins = scope.get("Number").unwrap().clone();
                            ins.raw_data = (n1 / n2).to_ne_bytes().to_vec();
                            ins
                        }),
                    ),
                    (
                        "__display__".to_string(),
                        Function::BuiltIn(|args, scope| {
                            let mut ins = scope.get("String").unwrap().clone();
                            ins.raw_data =
                                f64::from_ne_bytes(args[0].raw_data.clone().try_into().unwrap())
                                    .to_string()
                                    .as_bytes()
                                    .to_vec();
                            ins
                        }),
                    ),
                ]),
            },
        ),
        (
            "String".to_string(),
            Object {
                raw_data: "".to_string().as_bytes().to_vec(),
                methods: HashMap::from([
                    (
                        "+".to_string(),
                        Function::BuiltIn(|args, scope| {
                            let s1 = String::from_utf8(args[0].raw_data.clone()).unwrap();
                            let s2 = String::from_utf8(args[1].raw_data.clone()).unwrap();
                            let mut ins = scope.get("String").unwrap().clone();
                            ins.raw_data = (s1 + &s2).as_bytes().to_vec();
                            ins
                        }),
                    ),
                    (
                        "*".to_string(),
                        Function::BuiltIn(|args, scope| {
                            let s = String::from_utf8(args[0].raw_data.clone()).unwrap();
                            let n =
                                f64::from_ne_bytes(args[1].raw_data.clone().try_into().unwrap());
                            let mut ins = scope.get("String").unwrap().clone();
                            ins.raw_data = (s.repeat(n as usize)).as_bytes().to_vec();
                            ins
                        }),
                    ),
                    (
                        "__display__".to_string(),
                        Function::BuiltIn(|args, _| args[0].clone()),
                    ),
                ]),
            },
        ),
        (
            "Error".to_string(),
            Object {
                raw_data: vec![],
                methods: HashMap::from([(
                    "__display__".to_string(),
                    Function::BuiltIn(|_, scope| {
                        let mut ins = scope.get("String").unwrap().clone();
                        ins.raw_data = "Error!".to_string().as_bytes().to_vec();
                        ins
                    }),
                )]),
            },
        ),
    ])
}

fn parse_expr(source: String, scope: &Scope) -> Node {
    let tokens = tokenize(source);
    if tokens.len() >= 2 {
        let object = parse_object(tokens[0].clone(), scope);
        let method = tokens[1].to_string();
        let args = tokens[2..]
            .iter()
            .map(|i| {
                if i.starts_with('(') | i.ends_with(')') {
                    parse_expr(i[1..i.len() - 1].to_string(), scope)
                } else {
                    Node::Object(parse_object(i.clone(), scope))
                }
            })
            .collect();
        Node::Expr(Expr {
            object,
            method,
            args,
        })
    } else {
        Node::Object(parse_object(tokens[0].clone(), scope))
    }
}

fn parse_object(source: String, scope: &Scope) -> Object {
    if let Ok(n) = source.parse::<f64>() {
        let mut ins = scope.get("Number").unwrap().clone();
        ins.raw_data = n.to_ne_bytes().to_vec();
        ins
    } else if source.starts_with('"') | source.ends_with('"') {
        let mut ins = scope.get("String").unwrap().clone();
        ins.raw_data = source[1..source.len() - 1].to_string().as_bytes().to_vec();
        ins
    } else {
        scope.get("Error").unwrap().clone()
    }
}

fn tokenize(input: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '(' | '{' | '[' if !in_quote => {
                current_token.push(c);
                in_parentheses += 1;
            }
            ')' | '}' | ']' if !in_quote => {
                current_token.push(c);
                if in_parentheses > 0 {
                    in_parentheses -= 1;
                }
            }
            ' ' | '　' | '\t' if !in_quote => {
                if in_parentheses != 0 {
                    current_token.push(c);
                } else if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            '"' => {
                in_quote = !in_quote;
                current_token.push(c);
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if in_parentheses == 0 && !current_token.is_empty() {
        tokens.push(current_token.clone());
        current_token.clear();
    }

    tokens
}

type Scope = HashMap<String, Object>;

#[derive(Debug, Clone)]
enum Function {
    BuiltIn(fn(Vec<Object>, &Scope) -> Object),
}

#[derive(Debug, Clone)]
struct Object {
    raw_data: Vec<u8>,
    methods: HashMap<String, Function>,
}

impl Object {
    fn call(&self, method_name: String, args: Vec<Object>, scope: &Scope) -> Object {
        if let Some(func) = self.methods.get(&method_name) {
            match func {
                Function::BuiltIn(func) => func([vec![self.clone()], args].concat(), scope),
            }
        } else {
            scope.get("Error").unwrap().clone()
        }
    }

    fn display(&self, scope: &Scope) -> String {
        if self.methods.contains_key("__display__") {
            String::from_utf8(self.call("__display__".to_string(), vec![], scope).raw_data).unwrap()
        } else {
            format!("{:?}", self)
        }
    }
}

#[derive(Debug, Clone)]
enum Node {
    Expr(Expr),
    Object(Object),
}

impl Node {
    fn eval(&self, scope: &Scope) -> Object {
        match self {
            Node::Expr(expr) => expr.eval(scope),
            Node::Object(obj) => obj.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Expr {
    object: Object,
    method: String,
    args: Vec<Node>,
}

impl Expr {
    fn eval(&self, scope: &Scope) -> Object {
        let args: Vec<Object> = self.args.iter().map(|i| i.eval(scope)).collect();
        self.object.call(self.method.clone(), args, scope)
    }
}
