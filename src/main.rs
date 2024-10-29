use rustyline::DefaultEditor;
use std::collections::HashMap;

fn main() {
    println!("Hare 0.1.0");
    let mut rl = DefaultEditor::new().unwrap();
    let scope = &mut scope();
    loop {
        let line = rl.readline("> ").unwrap().trim().to_string();
        if !line.is_empty() {
            rl.add_history_entry(&line).unwrap_or_default();
            println!("{}", run_program(line, scope).display(scope));
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
                        "%".to_string(),
                        Function::BuiltIn(|args, scope| {
                            let n1 =
                                f64::from_ne_bytes(args[0].raw_data.clone().try_into().unwrap());
                            let n2 =
                                f64::from_ne_bytes(args[1].raw_data.clone().try_into().unwrap());
                            let mut ins = scope.get("Number").unwrap().clone();
                            ins.raw_data = (n1 % n2).to_ne_bytes().to_vec();
                            ins
                        }),
                    ),
                    (
                        "^".to_string(),
                        Function::BuiltIn(|args, scope| {
                            let n1 =
                                f64::from_ne_bytes(args[0].raw_data.clone().try_into().unwrap());
                            let n2 =
                                f64::from_ne_bytes(args[1].raw_data.clone().try_into().unwrap());
                            let mut ins = scope.get("Number").unwrap().clone();
                            ins.raw_data = n1.powf(n2).to_ne_bytes().to_vec();
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
        (
            "None".to_string(),
            Object {
                raw_data: vec![],
                methods: HashMap::from([(
                    "__display__".to_string(),
                    Function::BuiltIn(|_, scope| {
                        let mut ins = scope.get("String").unwrap().clone();
                        ins.raw_data = "None".to_string().as_bytes().to_vec();
                        ins
                    }),
                )]),
            },
        ),
    ])
}

fn run_program(source: String, scope: &mut Scope) -> Object {
    let mut result = scope.get("None").unwrap().clone();
    for line in tokenize_program(source) {
        if line.len() == 2 {
            result = parse_expr(line[1].to_string(), scope).eval(scope);
            scope.insert(line[0].trim().to_string(), result.clone());
        } else {
            result = parse_expr(line[0].to_string(), scope).eval(scope);
        }
    }
    result
}

fn parse_expr(source: String, scope: &Scope) -> Node {
    let tokens = tokenize_expr(source);
    if tokens.len() >= 2 {
        let object = parse_object(tokens[0].clone(), scope);
        let method = tokens[1].to_string();
        let args = tokens[2..]
            .iter()
            .map(|i| parse_object(i.clone(), scope))
            .collect();
        Node::Expr(Expr {
            object: Box::new(object),
            method,
            args,
        })
    } else {
        parse_object(tokens[0].clone(), scope)
    }
}

fn parse_object(source: String, scope: &Scope) -> Node {
    let source = source.trim().to_string();
    if let Ok(n) = source.parse::<f64>() {
        let mut ins = scope.get("Number").unwrap().clone();
        ins.raw_data = n.to_ne_bytes().to_vec();
        Node::Object(ins)
    } else if source.starts_with('"') | source.ends_with('"') {
        let mut ins = scope.get("String").unwrap().clone();
        ins.raw_data = source[1..source.len() - 1].to_string().as_bytes().to_vec();
        Node::Object(ins)
    } else if source.starts_with('(') | source.ends_with(')') {
        parse_expr(source[1..source.len() - 1].to_string(), scope)
    } else {
        Node::Variable(source)
    }
}

fn tokenize_program(input: String) -> Vec<Vec<String>> {
    let mut tokens: Vec<Vec<String>> = Vec::new();
    let mut current_token = String::new();
    let mut after_equal = String::new();
    let mut is_equal = false;
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;
    let mut escape = false;

    for c in input.chars() {
        match c {
            '(' | '{' | '[' if !in_quote => {
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
                in_parentheses += 1;
            }
            ')' | '}' | ']' if !in_quote => {
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
                in_parentheses -= 1;
            }
            '"' => {
                if !escape {
                    in_quote = !in_quote;
                    escape = false;
                }
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
            }
            ';' if !in_quote => {
                if in_parentheses != 0 {
                    if is_equal {
                        after_equal.push(c);
                    } else {
                        current_token.push(c);
                    }
                } else {
                    if !current_token.is_empty() {
                        if is_equal {
                            is_equal = false;
                            tokens.push(vec![current_token.clone(), after_equal.clone()]);
                            current_token.clear();
                            after_equal.clear();
                        } else {
                            tokens.push(vec![current_token.clone()]);
                            current_token.clear();
                        }
                    }
                }
            }
            '=' if !in_quote => {
                if in_parentheses != 0 {
                    if is_equal {
                        after_equal.push(c);
                    } else {
                        current_token.push(c);
                    }
                } else {
                    is_equal = true;
                }
            }
            '\\' => {
                escape = true;
            }
            _ => {
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
            }
        }
    }

    if in_parentheses == 0 && !current_token.is_empty() {
        if is_equal {
            tokens.push(vec![current_token.clone(), after_equal]);
            current_token.clear();
        } else {
            tokens.push(vec![current_token.clone()]);
            current_token.clear();
        }
    }
    tokens
}

fn tokenize_expr(input: String) -> Vec<String> {
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
    Variable(String),
}

impl Node {
    fn eval(&self, scope: &Scope) -> Object {
        match self {
            Node::Expr(expr) => expr.eval(scope),
            Node::Object(obj) => obj.clone(),
            Node::Variable(v) => scope.get(v).unwrap_or(scope.get("Error").unwrap()).clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Expr {
    object: Box<Node>,
    method: String,
    args: Vec<Node>,
}

impl Expr {
    fn eval(&self, scope: &Scope) -> Object {
        let args: Vec<Object> = self.args.iter().map(|i| i.eval(scope)).collect();
        self.object
            .eval(scope)
            .call(self.method.clone(), args, scope)
    }
}
