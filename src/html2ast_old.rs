use serde_derive::{Deserialize, Serialize};
use std::ops::Index;
use std::{cell::RefCell, rc::Rc};

extern crate serde;
extern crate serde_json;

type Ref = Rc<RefCell<Vec<Rc<RefCell<Node>>>>>;
type NodeRef = Vec<Rc<RefCell<Node>>>;

// #[derive(Debug, PartialEq)]
// pub enum Token {
//     StartTag = 0,
//     // "<"
//     CloseToken = 1,
//     // "/"
//     ClostTag = 2,
//     // ">"
//     CommenStartTag = 3,
//     // "!"
//     CommentLine = 4,
//     //  "-"
//     CommentEndTag = 5,
//     // "-->"
//     Space = 6,
//     // " "
//     Equal = 7,
//     // "="
//     EOF = 8, // "EOF"
// }

// impl From<&str> for Token {
//     fn from(u: &str) -> Self {
//         match u {
//             "EOF" => Token::EOF,
//             _ => unimplemented!(), // u8里面除了这八个符号，还有其他的符号，其他符号程序退出
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub r#type: String,
    pub content: Option<String>,
    pub index: u32,
    pub tag: Option<String>,
    pub children: Ref,
    pub close_tag: Option<bool>,
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

pub fn run(f: &String) -> Result<(), Box<dyn std::error::Error>> {
    let file: Vec<String> = vec![];
    for item in f.chars() {
        file.push(String::from(item));
    }
    
    let mut index = 0;
    let root = Node {
        r#type: generate_str("root"),
        children: Rc::new(RefCell::new(vec![])),
        content: None,
        index: 0,
        close_tag: None,
        tag: None,
        attrs: None,
    };

    let mut stack: NodeRef = vec![Rc::new(RefCell::new(root))];

    while index < file.len() {
        let item = file.get(index..index + 1).ok_or("start err")?;
        let token = lexer(item, index, file)?;
        let rc_token = Rc::new(RefCell::new(token));
        // println!("{:?}", rc_token.clone());
        index = rc_token.borrow().index as usize;
        // delete token.index
        if rc_token.borrow().r#type == String::from("EOF") {
            // return stack[0];
            // println!("{:?}", stack[0]);
            let serialized = serde_json::to_string(&stack[0]).unwrap();
            println!("serialized = {}", serialized);
            std::fs::write("json/res.json", serialized)?;
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
            if rc_token.borrow().r#type == String::from("node") {
                // 如果是标签节点，则放入栈中
                stack.push(rc_token.clone());
            }
        } else {
            // 闭合标签，栈顶标签出栈
            stack.pop().ok_or("stack pop err")?;
        }
    }
    Ok(())
}

pub fn lexer(
    item: &str,
    mut index: usize,
    file: &String,
) -> Result<Node, Box<dyn std::error::Error>> {
    fn is_end(file: &String, index: usize) -> bool {
        return file.len() <= index;
    }

    fn is_comment_end(hanlder: &String, index: usize) -> Result<bool, Box<dyn std::error::Error>> {
        let mut count = index;
        let target = String::from("-->");
        let mut sour = String::from("");
        while count < index + 3 {
            sour += hanlder.get(count..count + 1).ok_or("is_comment_end err")?;
            count += 1;
        }
        Ok(target == sour)
    }

    if item == String::from("<") {
        // <
        index += 1;
        if file.get(index..index + 1).ok_or("< err")? == String::from("/") {
            // </
            index += 1;
            let mut tag = String::from("");
            let mut cur = file.get(index..index + 1).ok_or("</ err")?;
            while cur != String::from(">") && !is_end(&file, index) {
                // </xx>
                tag += cur;
                index += 1;
                cur = file.get(index..index + 1).ok_or("</xx> err")?;
            }

            return Ok(Node {
                r#type: String::from("node"),
                tag: Some(tag),
                index: index as u32,
                close_tag: Some(true),
                content: None,
                children: Rc::new(RefCell::new(vec![])),
                attrs: None,
            });
        } else if file.get(index..index + 1).ok_or("comment ! err")? == String::from("!") {
            index += 1;
            let mut cur = file
                .get(index..index + 1)
                .ok_or("comment content start err")?;
            let mut count = 2;
            while count != 0 {
                // <!--
                if cur != String::from("-") {
                    // console.assert("fail")
                    println!("fail");
                }
                index += 1;
                cur = file
                    .get(index..index + 1)
                    .ok_or("comment content inner err")?;
                count -= 1;
            }
            // -->结束
            let mut content = String::from("");
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
                content += cur;
                index += 1;
                end = is_end(&file, index);
                if end {
                    return Ok(Node {
                        r#type: String::from("EOF"),
                        tag: None,
                        index: index as u32,
                        close_tag: None,
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
                content: Some(content),
                children: Rc::new(RefCell::new(vec![])),
                attrs: None,
            });
        } else {
            // <
            let mut tag = String::from("");
            let mut cur = file.get(index..index + 1).ok_or("< tag err")?;
            while cur != String::from(" ") && cur != String::from(">") && !is_end(&file, index) {
                tag += cur;
                index += 1;
                cur = file.get(index..index + 1).ok_or("after attrs err")?;
            }
            let mut attrs: Vec<Attr> = vec![];
            if cur == String::from(" ") {
                while file.get(index..index + 1).ok_or("inner attrs err")? == String::from(" ") {
                    index += 1;
                }
                let mut key = String::from("");
                let mut value = String::from("");
                cur = file.get(index..index + 1).ok_or("attrs start err")?;
                while cur != String::from(">") && !is_end(&file, index) {
                    if cur == String::from(" ") && cur != String::from(">") {
                        while file.get(index..index + 1).ok_or("attrs blank err")?
                            == String::from(" ")
                        {
                            index += 1;
                            cur = file.get(index..index + 1).ok_or("attrs blank inner err")?;
                        }
                    }
                    if cur != String::from("=") && cur != String::from(">") {
                        key += cur;
                    } else if cur == String::from("=") && cur != String::from(">") {
                        index += 1;
                        cur = file.get(index..index + 1).ok_or("attrs start value err")?;
                        while cur != String::from(" ")
                            && !is_end(&file, index)
                            && cur != String::from(">")
                        {
                            if cur == String::from('"') {
                                index += 1;
                                cur = file.get(index..index + 1).ok_or("quote start err")?;
                            } else {
                                value += cur;
                                index += 1;
                                cur = file.get(index..index + 1).ok_or("quote inner err")?;
                            }
                        }
                        attrs.push(Attr { key, value });
                        key = String::from("");
                        value = String::from("");
                        index -= 1;
                    } else if cur == String::from(">") {
                        break;
                    }
                    index += 1;
                    cur = file.get(index..index + 1).ok_or("attr end err")?;
                }
            }
            cur = file.get(index..index + 1).ok_or("blank start err")?;
            while cur != String::from(">") && !is_end(&file, index) {
                index += 1;
                cur = file.get(index..index + 1).ok_or("blank inner err")?;
            }
            index += 1; // >
            let temp = file.index(index);
            println!("{}", temp);
            if file.get(index..index + 1).ok_or("<xx></xx> err")? == String::from("<") {
                // <xx></xx>
                index += 1;
            }

            return Ok(Node {
                r#type: String::from("node"),
                tag: Some(tag),
                index: index as u32,
                close_tag: None,
                content: None,
                children: Rc::new(RefCell::new(vec![])),
                attrs: Some(attrs),
            });
        }
    } else if item == String::from(">") {
        // >
        index += 1;
        let mut content = String::from("");
        if is_end(&file, index) {
            return Ok(Node {
                r#type: String::from("EOF"),
                tag: None,
                index: index as u32,
                close_tag: None,
                content: None,
                children: Rc::new(RefCell::new(vec![])),
                attrs: None,
            });
        }
        let mut cur = file.get(index..index + 1).ok_or("> err")?;
        let mut end = is_end(&file, index);
        while cur != String::from("<") && !end {
            content += cur;
            index += 1;
            end = is_end(&file, index);
            if end {
                return Ok(Node {
                    r#type: String::from("EOF"),
                    tag: None,
                    index: index as u32,
                    close_tag: None,
                    content: None,
                    children: Rc::new(RefCell::new(vec![])),
                    attrs: None,
                });
            }
            cur = file.get(index..index + 1).ok_or("> content err")?;
        }

        return Ok(Node {
            r#type: String::from("text"),
            tag: None,
            index: index as u32,
            close_tag: None,
            content: Some(content),
            children: Rc::new(RefCell::new(vec![])),
            attrs: None,
        });
    } else {
        // content
        let mut content = String::from("");
        let mut cur = file.get(index..index + 1).ok_or("content start err")?;
        let mut end = false;
        while cur != String::from("<") && !end {
            content += cur;
            index += 1;
            end = is_end(&file, index);
            if end {
                return Ok(Node {
                    r#type: String::from("EOF"),
                    tag: None,
                    index: index as u32,
                    close_tag: None,
                    content: None,
                    children: Rc::new(RefCell::new(vec![])),
                    attrs: None,
                });
            }
            cur = file.get(index..index + 1).ok_or("content end err")?;
        }

        return Ok(Node {
            r#type: String::from("text"),
            tag: None,
            index: index as u32,
            close_tag: None,
            content: Some(content),
            children: Rc::new(RefCell::new(vec![])),
            attrs: None,
        });
    }
}
