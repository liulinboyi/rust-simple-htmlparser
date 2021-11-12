use serde_derive::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

extern crate serde;
extern crate serde_json;

type Ref = Rc<RefCell<Vec<Rc<RefCell<Node>>>>>;
type NodeRef = Vec<Rc<RefCell<Node>>>;

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
    let node_string = String::from("node");
    let eof_string = String::from("EOF");

    while index < file.len() {
        let item = file.get(index..index + 1).ok_or("start err")?;
        let token = lexer(&item[0], index, &file, &stack)?;
        let rc_token = Rc::new(RefCell::new(token));
        // println!("{:?}", rc_token.clone());
        index = rc_token.borrow().index as usize;
        if rc_token.borrow().r#type == eof_string {
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
            if rc_token.borrow().r#type == node_string
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
    let null_string = String::from("");
    let left_slash = String::from("/");
    let left_angle_bracket = String::from("<");
    let right_angle_bracket = String::from(">");
    let node_string = String::from("node");
    let doctype = String::from("DOCTYPE");
    let blank_string = String::from(" ");
    let eof_string = String::from("EOF");
    let equal_sring = String::from("=");
    let text_string = String::from("text");

    fn is_end(file: &Vec<String>, index: usize) -> bool {
        return file.len() - 1 <= index;
    }

    // 使用闭包，捕获上下文中变量
    let is_comment_end =
        |hanlder: &Vec<String>, index: usize| -> Result<bool, Box<dyn std::error::Error>> {
            let mut count = index;
            let target = String::from("-->");
            let mut sour = null_string.clone();
            while count < index + 3 {
                sour += &hanlder.get(count..count + 1).ok_or("is_comment_end err")?[0];
                count += 1;
            }
            Ok(target == sour)
        };

    if *item == left_angle_bracket {
        // <
        index += 1;
        if file.get(index..index + 1).ok_or("< err")?[0] == left_slash {
            // </
            index += 1;
            let mut tag = null_string;
            let mut cur = file.get(index..index + 1).ok_or("</ err")?;
            while cur[0] != right_angle_bracket && !is_end(&file, index) {
                // </xx>
                tag += &cur[0];
                index += 1;
                cur = file.get(index..index + 1).ok_or("</xx> err")?;
            }

            return Ok(Node {
                r#type: node_string,
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
                .get(index..index + doctype.len())
                .ok_or("DOCTYPE err")?;
            let temp_doc = String::from(doc.join(""));
            // println!("{:?}", doc);
            // println!("{:?}", temp_doc);
            if temp_doc == doctype && !is_end(&file, index) {
                index += doctype.len(); // 跳过DOCTYPE
                cur = file
                    .get(index..index + 1)
                    .ok_or("DOCTYPE content inner err")?;
                while cur[0] == blank_string {
                    index += 1;
                    cur = file
                        .get(index..index + 1)
                        .ok_or("DOCTYPE content inner err")?;
                }
                cur = file
                    .get(index..index + 1)
                    .ok_or("DOCTYPE content inner err")?;
                let mut content = null_string;
                while cur[0] != right_angle_bracket {
                    content += &cur[0];
                    index += 1;
                    cur = file
                        .get(index..index + 1)
                        .ok_or("DOCTYPE content inner err")?;
                }
                return Ok(Node {
                    r#type: doctype,
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
            let mut content = null_string.clone();
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
                        r#type: eof_string,
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
            let mut tag = null_string.clone();
            let mut cur = file.get(index..index + 1).ok_or("< tag err")?;
            while cur[0] != blank_string && cur[0] != right_angle_bracket && !is_end(&file, index) {
                tag += &cur[0];
                index += 1;
                cur = file.get(index..index + 1).ok_or("after attrs err")?;
            }
            let mut attrs: Vec<Attr> = vec![];
            if cur[0] == blank_string {
                // 删除空格
                while file.get(index..index + 1).ok_or("inner attrs err")?[0] == blank_string {
                    index += 1;
                }
                let mut key = null_string.clone();
                let mut value = null_string.clone();
                cur = file.get(index..index + 1).ok_or("attrs start err")?;
                while cur[0] != right_angle_bracket && !is_end(&file, index) {
                    // 删除空格
                    if cur[0] == blank_string && cur[0] != right_angle_bracket {
                        while file.get(index..index + 1).ok_or("attrs blank err")?[0]
                            == blank_string
                        {
                            index += 1;
                            cur = file.get(index..index + 1).ok_or("attrs blank inner err")?;
                        }
                    }
                    if cur[0] != equal_sring
                        && cur[0] != right_angle_bracket
                        && cur[0] != left_slash
                    {
                        key += &cur[0];
                    } else if cur[0] == equal_sring && cur[0] != right_angle_bracket {
                        index += 1;
                        cur = file.get(index..index + 1).ok_or("attrs start value err")?;
                        while cur[0] != blank_string
                            && !is_end(&file, index)
                            && cur[0] != right_angle_bracket
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
                        key = null_string.clone();
                        value = null_string.clone();
                        index -= 1;
                    } else if cur[0] == left_slash {
                        index += 1;
                        cur = file.get(index..index + 1).ok_or("/ inner err")?;
                        if cur[0] == right_angle_bracket {
                            // <xx />
                            return Ok(Node {
                                r#type: node_string,
                                tag: Some(tag),
                                index: index as u32,
                                close_tag: None,
                                self_close: Some(true),
                                content: None,
                                children: Rc::new(RefCell::new(vec![])),
                                attrs: Some(attrs),
                            });
                        }
                    } else if cur[0] == right_angle_bracket {
                        // 这里没加1
                        break;
                    }
                    index += 1;
                    cur = file.get(index..index + 1).ok_or("attr end err")?;
                }
            }
            cur = file.get(index..index + 1).ok_or("blank start err")?;
            while cur[0] != right_angle_bracket && !is_end(&file, index) {
                index += 1;
                cur = file.get(index..index + 1).ok_or("blank inner err")?;
            }
            index += 1; // >
            if file.get(index..index + 1).ok_or("<xx></xx> err")?[0] == left_angle_bracket {
                // <xx></xx>
                index += 1;
            }

            return Ok(Node {
                r#type: node_string,
                tag: Some(tag),
                index: index as u32,
                close_tag: None,
                self_close: Some(false),
                content: None,
                children: Rc::new(RefCell::new(vec![])),
                attrs: Some(attrs),
            });
        }
    } else if *item == right_angle_bracket {
        // >
        index += 1;
        let mut content = null_string;
        if is_end(&file, index) {
            return Ok(Node {
                r#type: eof_string,
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
        while (cur[0] != left_angle_bracket || is_script) && !end {
            content += &cur[0];
            index += 1;
            end = is_end(&file, index);
            if end {
                return Ok(Node {
                    r#type: eof_string,
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
            r#type: text_string,
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
        let mut content = null_string;
        let mut cur = file.get(index..index + 1).ok_or("content start err")?;
        let mut end = false;
        while cur[0] != left_angle_bracket && !end {
            content += &cur[0];
            index += 1;
            end = is_end(&file, index);
            if end {
                return Ok(Node {
                    r#type: eof_string,
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
            r#type: text_string,
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
