# SIMPLE HTML PARSER USE Rust 使用Rust写的简单HTML解析器

## 代码仅供参考，初次写的比较垃圾

## 其他语言

- [JavaScript](https://github.com/liulinboyi/simple-htmlparser)

```
<div id="out">
    <div class="inner" style="color: red;">haha</div>
</div>
```

```
{
    "type": "root",
    "content": null,
    "index": 0,
    "tag": null,
    "children": [
        {
            "type": "node",
            "content": null,
            "index": 14,
            "tag": "div",
            "children": [
                {
                    "type": "text",
                    "content": "\r\n    ",
                    "index": 20,
                    "tag": null,
                    "children": [],
                    "close_tag": null,
                    "attrs": null
                },
                {
                    "type": "node",
                    "content": null,
                    "index": 59,
                    "tag": "div",
                    "children": [
                        {
                            "type": "text",
                            "content": "haha",
                            "index": 63,
                            "tag": null,
                            "children": [],
                            "close_tag": null,
                            "attrs": null
                        }
                    ],
                    "close_tag": null,
                    "attrs": [
                        {
                            "key": "class",
                            "value": "inner"
                        },
                        {
                            "key": "style",
                            "value": "color:"
                        }
                    ]
                },
                {
                    "type": "text",
                    "content": "\r\n",
                    "index": 71,
                    "tag": null,
                    "children": [],
                    "close_tag": null,
                    "attrs": null
                }
            ],
            "close_tag": null,
            "attrs": [
                {
                    "key": "id",
                    "value": "out"
                }
            ]
        }
    ],
    "close_tag": null,
    "attrs": null
}
```