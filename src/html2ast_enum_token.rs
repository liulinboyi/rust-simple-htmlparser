use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

extern crate serde;
extern crate serde_json;

type Ref = Rc<RefCell<Vec<Rc<RefCell<Node>>>>>;
type NodeRef = Vec<Rc<RefCell<Node>>>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Token {
    NullString = 0,
    LeftSlash = 1,
    LeftAngleBracket = 2,
    RightAngleBracket = 3,
    NodeString = 4,
    Doctype = 5,
    BlankString = 6,
    EofString = 7,
    EqualSring = 8,
    TextString = 9,
}

use std::sync::Mutex;

lazy_static! {
    static ref MAP: HashMap<Token, String> = {
        let mut map = HashMap::new();
        map.insert(Token::NullString, String::from(""));
        map.insert(Token::LeftSlash, String::from("/"));
        map.insert(Token::LeftAngleBracket, String::from("<"));
        map.insert(Token::RightAngleBracket, String::from(">"));
        map.insert(Token::NodeString, String::from("node"));
        map.insert(Token::Doctype, String::from("DOCTYPE"));
        map.insert(Token::BlankString, String::from(" "));
        map.insert(Token::EofString, String::from("EOF"));
        map.insert(Token::EqualSring, String::from("="));
        map.insert(Token::TextString, String::from("text"));
        map
    };
    // static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub r#type: String,
    pub content: Option<String>,
    pub index: u32,
    pub tag: Option<String>,
    pub children: Ref,
    pub close_tag: Option<bool>,
    pub self_close: Option<bool>,
    pub attrs: Option<Vec<Attr>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attr {
    key: String,
    value: String,
}

pub fn generate_str(str: &str) -> String {
    return String::from(str);
}

// pub fn run(f: &String) -> Result<(), Box<dyn std::error::Error>> {
pub fn run(f: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // let mut file: Vec<String> = vec![];
    // for item in f.chars() {
    //     file.push(String::from(item));
    // }
    let mut file = f;

    let mut index = 0;
    let root = Node {
        r#type: generate_str("root"),
        children: Rc::new(RefCell::new(vec![])),
        content: None,
        index: 0,
        close_tag: None,
        self_close: None,
        tag: None,
        attrs: None,
    };

    let mut stack: NodeRef = vec![Rc::new(RefCell::new(root))];
    // let NodeString = String::from("node");
    // let EofString = String::from("EOF");

    while index < file.len() {
        let item = file.get(index..index + 1).ok_or("start err")?;
        let token = lexer(&item[0], index, &file, &stack)?;
        let rc_token = Rc::new(RefCell::new(token));
        // println!("{:?}", rc_token.clone());
        index = rc_token.borrow().index as usize;
        if rc_token.borrow().r#type == MAP[&Token::EofString] {
            // return stack[0];
            // println!("{:?}", stack[0]);
            // let serialized = serde_json::to_string(&stack[0]).unwrap();
            // println!("serialized = {}", serialized);
            // std::fs::write("res.json", serialized)?; // 写入文件
            break;
        }
        if rc_token.borrow().close_tag == Some(false) || rc_token.borrow().close_tag == None {
            // 非闭合标签
            let len = stack.len();
            let cur = stack.get(len - 1).ok_or("no close tag err")?;

            // stack[len - 1].children.push(token); // 放入栈顶children处
            cur.borrow_mut()
                .children
                .borrow_mut()
                .push(rc_token.clone());
            if rc_token.borrow().r#type == MAP[&Token::NodeString]
                && rc_token.borrow().self_close == Some(false)
            {
                // 如果是标签节点，则放入栈中
                stack.push(rc_token.clone());
            }
        } else {
            // 闭合标签，栈顶标签出栈
            let len = stack.len();
            let cur = stack.get(len - 1).ok_or("no close tag err")?;
            if cur.borrow().r#type != generate_str("root") {
                stack.pop().ok_or("stack pop err")?;
            }
        }
    }
    Ok(())
}

pub fn lexer(
    item: &String,
    mut index: usize,
    file: &Vec<String>,
    stack: &NodeRef,
) -> Result<Node, Box<dyn std::error::Error>> {
    fn is_end(file: &Vec<String>, index: usize) -> bool {
        return file.len() - 1 <= index;
    }

    // 使用闭包，捕获上下文中变量
    let is_comment_end =
        |hanlder: &Vec<String>, index: usize| -> Result<bool, Box<dyn std::error::Error>> {
            let mut count = index;
            let target = String::from("-->");
            let mut sour = MAP[&Token::NullString].clone();
            while count < index + 3 {
                sour += &hanlder.get(count..count + 1).ok_or("is_comment_end err")?[0];
                count += 1;
            }
            Ok(target == sour)
        };

    if *item == MAP[&Token::LeftAngleBracket] {
        // <
        index += 1;
        if file.get(index..index + 1).ok_or("< err")?[0] == MAP[&Token::LeftSlash] {
            // </
            index += 1;
            let mut tag = MAP[&Token::NullString].clone();
            let mut cur = file.get(index..index + 1).ok_or("</ err")?;
            while cur[0] != MAP[&Token::RightAngleBracket] && !is_end(&file, index) {
                // </xx>
                tag += &cur[0];
                index += 1;
                cur = file.get(index..index + 1).ok_or("</xx> err")?;
            }

            return Ok(Node {
                r#type: MAP[&Token::NodeString].clone(),
                tag: Some(tag),
                index: index as u32,
                close_tag: Some(true),
                self_close: Some(false),
                content: None,
                children: Rc::new(RefCell::new(vec![])),
                attrs: None,
            });
        } else if file.get(index..index + 1).ok_or("comment ! err")?[0] == String::from("!") {
            index += 1;
            let mut cur = file
                .get(index..index + 1)
                .ok_or("comment content start err")?;
            let mut count = 2;
            let cache_index = index;
            while count != 0 {
                // <!--
                if cur[0] != String::from("-") {
                    // console.assert("fail")
                    // println!("fail");
                    index = cache_index;
                    break;
                }
                index += 1;
                cur = file
                    .get(index..index + 1)
                    .ok_or("comment content inner err")?;
                count -= 1;
            }
            // DOCTYPE
            let doc = file
                .get(index..index + MAP[&Token::Doctype].len())
                .ok_or("DOCTYPE err")?;
            let temp_doc = String::from(doc.join(""));
            // println!("{:?}", doc);
            // println!("{:?}", temp_doc);
            if temp_doc == MAP[&Token::Doctype] && !is_end(&file, index) {
                index += MAP[&Token::Doctype].len(); // 跳过DOCTYPE
                cur = file
                    .get(index..index + 1)
                    .ok_or("DOCTYPE content inner err")?;
                while cur[0] == MAP[&Token::BlankString] {
                    index += 1;
                    cur = file
                        .get(index..index + 1)
                        .ok_or("DOCTYPE content inner err")?;
                }
                cur = file
                    .get(index..index + 1)
                    .ok_or("DOCTYPE content inner err")?;
                let mut content = MAP[&Token::NullString].clone();
                while cur[0] != MAP[&Token::RightAngleBracket] {
                    content += &cur[0];
                    index += 1;
                    cur = file
                        .get(index..index + 1)
                        .ok_or("DOCTYPE content inner err")?;
                }
                return Ok(Node {
                    r#type: MAP[&Token::Doctype].clone(),
                    tag: None,
                    index: index as u32,
                    close_tag: None,
                    self_close: None,
                    content: Some(content),
                    children: Rc::new(RefCell::new(vec![])),
                    attrs: None,
                });
            }

            // -->结束
            let mut content = MAP[&Token::NullString].clone();
            cur = file
                .get(index..index + 1)
                .ok_or("comment content end err")?;
            let mut is_c_end = false;

            let mut end = false;
            while !end {
                is_c_end = is_comment_end(&file, index)?;
                if is_c_end {
                    break;
                }
                content += &cur[0];
                index += 1;
                end = is_end(&file, index);
                if end {
                    return Ok(Node {
                        r#type: MAP[&Token::EofString].clone(),
                        tag: None,
                        index: index as u32,
                        close_tag: None,
                        self_close: None,
                        content: None,
                        children: Rc::new(RefCell::new(vec![])),
                        attrs: None,
                    });
                }
                cur = file.get(index..index + 1).ok_or("comment err")?;
            }
            if is_c_end {
                // -->
                index += 3
            }

            return Ok(Node {
                r#type: String::from("comment"),
                tag: None,
                index: index as u32,
                close_tag: None,
                self_close: None,
                content: Some(content),
                children: Rc::new(RefCell::new(vec![])),
                attrs: None,
            });
        } else {
            // <
            let mut tag = MAP[&Token::NullString].clone();
            let mut cur = file.get(index..index + 1).ok_or("< tag err")?;
            while cur[0] != MAP[&Token::BlankString]
                && cur[0] != MAP[&Token::RightAngleBracket]
                && !is_end(&file, index)
            {
                tag += &cur[0];
                index += 1;
                cur = file.get(index..index + 1).ok_or("after attrs err")?;
            }
            let mut attrs: Vec<Attr> = vec![];
            if cur[0] == MAP[&Token::BlankString] {
                // 删除空格
                while file.get(index..index + 1).ok_or("inner attrs err")?[0]
                    == MAP[&Token::BlankString]
                {
                    index += 1;
                }
                let mut key = MAP[&Token::NullString].clone();
                let mut value = MAP[&Token::NullString].clone();
                cur = file.get(index..index + 1).ok_or("attrs start err")?;
                while cur[0] != MAP[&Token::RightAngleBracket] && !is_end(&file, index) {
                    // 删除空格
                    if cur[0] == MAP[&Token::BlankString]
                        && cur[0] != MAP[&Token::RightAngleBracket]
                    {
                        while file.get(index..index + 1).ok_or("attrs blank err")?[0]
                            == MAP[&Token::BlankString]
                        {
                            index += 1;
                            cur = file.get(index..index + 1).ok_or("attrs blank inner err")?;
                        }
                    }
                    if cur[0] != MAP[&Token::EqualSring]
                        && cur[0] != MAP[&Token::RightAngleBracket]
                        && cur[0] != MAP[&Token::LeftSlash]
                    {
                        key += &cur[0];
                    } else if cur[0] == MAP[&Token::EqualSring]
                        && cur[0] != MAP[&Token::RightAngleBracket]
                    {
                        index += 1;
                        cur = file.get(index..index + 1).ok_or("attrs start value err")?;
                        while cur[0] != MAP[&Token::BlankString]
                            && !is_end(&file, index)
                            && cur[0] != MAP[&Token::RightAngleBracket]
                        {
                            if cur[0] == String::from('"') {
                                index += 1;
                                cur = file.get(index..index + 1).ok_or("quote start err")?;
                            } else {
                                value += &cur[0];
                                index += 1;
                                cur = file.get(index..index + 1).ok_or("quote inner err")?;
                            }
                        }
                        attrs.push(Attr { key, value });
                        key = MAP[&Token::NullString].clone();
                        value = MAP[&Token::NullString].clone();
                        index -= 1;
                    } else if cur[0] == MAP[&Token::LeftSlash] {
                        index += 1;
                        cur = file.get(index..index + 1).ok_or("/ inner err")?;
                        if cur[0] == MAP[&Token::RightAngleBracket] {
                            // <xx />
                            return Ok(Node {
                                r#type: MAP[&Token::NodeString].clone(),
                                tag: Some(tag),
                                index: index as u32,
                                close_tag: None,
                                self_close: Some(true),
                                content: None,
                                children: Rc::new(RefCell::new(vec![])),
                                attrs: Some(attrs),
                            });
                        }
                    } else if cur[0] == MAP[&Token::RightAngleBracket] {
                        // 这里没加1
                        break;
                    }
                    index += 1;
                    cur = file.get(index..index + 1).ok_or("attr end err")?;
                }
            }
            cur = file.get(index..index + 1).ok_or("blank start err")?;
            while cur[0] != MAP[&Token::RightAngleBracket] && !is_end(&file, index) {
                index += 1;
                cur = file.get(index..index + 1).ok_or("blank inner err")?;
            }
            index += 1; // >
            if file.get(index..index + 1).ok_or("<xx></xx> err")?[0]
                == MAP[&Token::LeftAngleBracket]
            {
                // <xx></xx>
                index += 1;
            }

            return Ok(Node {
                r#type: MAP[&Token::NodeString].clone(),
                tag: Some(tag),
                index: index as u32,
                close_tag: None,
                self_close: Some(false),
                content: None,
                children: Rc::new(RefCell::new(vec![])),
                attrs: Some(attrs),
            });
        }
    } else if *item == MAP[&Token::RightAngleBracket] {
        // >
        index += 1;
        let mut content = MAP[&Token::NullString].clone();
        if is_end(&file, index) {
            return Ok(Node {
                r#type: MAP[&Token::EofString].clone(),
                tag: None,
                index: index as u32,
                close_tag: None,
                self_close: None,
                content: None,
                children: Rc::new(RefCell::new(vec![])),
                attrs: None,
            });
        }
        let mut cur = file.get(index..index + 1).ok_or("> err")?;
        let mut end = is_end(&file, index);
        let top_stack = stack.get(stack.len() - 1).ok_or("top_stack err")?;
        let is_script = top_stack.borrow().tag == Some(generate_str("script"))
            || top_stack.borrow().tag == Some(generate_str("noscript"));
        while (cur[0] != MAP[&Token::LeftAngleBracket] || is_script) && !end {
            content += &cur[0];
            index += 1;
            end = is_end(&file, index);
            if end {
                return Ok(Node {
                    r#type: MAP[&Token::EofString].clone(),
                    tag: None,
                    index: index as u32,
                    close_tag: None,
                    self_close: None,
                    content: None,
                    children: Rc::new(RefCell::new(vec![])),
                    attrs: None,
                });
            }
            cur = file.get(index..index + 1).ok_or("> content err")?;
        }

        return Ok(Node {
            r#type: MAP[&Token::TextString].clone(),
            tag: None,
            index: index as u32,
            close_tag: None,
            self_close: None,
            content: Some(content),
            children: Rc::new(RefCell::new(vec![])),
            attrs: None,
        });
    } else {
        // content
        let mut content = MAP[&Token::NullString].clone();
        let mut cur = file.get(index..index + 1).ok_or("content start err")?;
        let mut end = false;
        while cur[0] != MAP[&Token::LeftAngleBracket] && !end {
            content += &cur[0];
            index += 1;
            end = is_end(&file, index);
            if end {
                return Ok(Node {
                    r#type: MAP[&Token::EofString].clone(),
                    tag: None,
                    index: index as u32,
                    close_tag: None,
                    self_close: None,
                    content: None,
                    children: Rc::new(RefCell::new(vec![])),
                    attrs: None,
                });
            }
            cur = file.get(index..index + 1).ok_or("content end err")?;
        }

        return Ok(Node {
            r#type: MAP[&Token::TextString].clone(),
            tag: None,
            index: index as u32,
            close_tag: None,
            self_close: None,
            content: Some(content),
            children: Rc::new(RefCell::new(vec![])),
            attrs: None,
        });
    }
}
