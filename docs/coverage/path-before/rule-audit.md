# Rule coverage audit

- Sources: **970**
- Rule strings: **22220**
- Fully executable: **793 / 970 (81.8%)**
- Blocked sources: **177**

## Syntax tokens

| Token | Rule hits | Sources | % sources |
| --- | ---: | ---: | ---: |
| `{{ template }}` | 2318 | 961 | 99.1% |
| `## replace` | 3561 | 826 | 85.2% |
| `&& chain` | 639 | 435 | 44.8% |
| `exclude !n` | 344 | 299 | 30.8% |
| `@js:` | 661 | 286 | 29.5% |
| `url option ,{...}` | 362 | 277 | 28.6% |
| `|| alternative` | 751 | 265 | 27.3% |
| `<js>` | 404 | 226 | 23.3% |
| `java.* call` | 600 | 210 | 21.6% |
| `range .a:b` | 188 | 146 | 15.1% |
| `@@ force JSoup` | 178 | 102 | 10.5% |
| `@put:` | 104 | 89 | 9.2% |
| `@get:` | 252 | 79 | 8.1% |
| `JSONPath recursive ..` | 255 | 78 | 8.0% |
| `XPath` | 177 | 59 | 6.1% |
| `JSONPath filter ?()` | 76 | 43 | 4.4% |
| `%% cross-merge` | 8 | 8 | 0.8% |

## Execution errors

| Error | Rule count |
| --- | ---: |
| `default-mode rule is not supported: `$1` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 6 |
| `default-mode rule is not supported: `$2` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 6 |
| `default-mode rule is not supported: `$3` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `$4` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `$5,$7,$10` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `$6` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `$8` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `$9` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `%双女主` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `++https://readbook-service-freebook.cread.com/cx/itf/getvolume?bookId=803500510` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 2 |
| `default-mode rule is not supported: `+` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 9 |
| `default-mode rule is not supported: `-https://api.bilibili.com/x/article/view?id=34980384,{"js":"book?result:'https://www.bilibili.com/opus/937892609776418850?spm_id_from=333.1365.0.0'"}` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `. Gap_size-4s__F67Nf Gap_direction-x__RsHk8` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    WhiteSpace(
        " ",
    ),
)` | 2 |
| `default-mode rule is not supported: `.listmain a[href~=/[^/]+/\d+.htm]` is not a CSS selector: Unexpected error occurred. Please report this to the developer
DanglingCombinator` | 5 |
| `default-mode rule is not supported: `0` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `3-7+3*5*2*(2+(4+3*2*4*2-2-8+1)*2*2)-2*2+(1/2.5+1)*3` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `7号基地` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `[class=s6 wid6]` is not a CSS selector: Token "s6" was not expected` | 2 |
| `default-mode rule is not supported: `[property~ =category\|statuslupdate_time]` is not a CSS selector: Unexpected error occurred. Please report this to the developer
UnexpectedTokenInAttributeSelector(
    Delim(
        '~',
    ),
)` | 1 |
| `default-mode rule is not supported: `[property～=category\|status\|update_time]` is not a CSS selector: Token "\|" was not expected` | 1 |
| `default-mode rule is not supported: `[书籍状态] 连载{$.status}完结` is not a CSS selector: Token "<{" was not expected` | 1 |
| `default-mode rule is not supported: `a.` is not a CSS selector: Unexpected EOL` | 1 |
| `default-mode rule is not supported: `book.canUpdate=false` is not a CSS selector: Token "=" was not expected` | 1 |
| `default-mode rule is not supported: `cat[*]` is not a CSS selector: Unexpected EOL` | 2 |
| `default-mode rule is not supported: `categoryNames.*.className` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Delim(
        '*',
    ),
)` | 2 |
| `default-mode rule is not supported: `category_tag.*.*` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Delim(
        '*',
    ),
)` | 2 |
| `default-mode rule is not supported: `data.Content[0].Content` is not a CSS selector: Unexpected error occurred. Please report this to the developer
NoQualifiedNameInAttributeSelector(
    Number {
        has_sign: false,
        value: 0.0,
        int_value: Some(
            0,
        ),
    },
)` | 1 |
| `default-mode rule is not supported: `div.forum-content.mt-3,div[class =d_post_content j_d_post_content]` is not a CSS selector: Token "j_d_post_content" was not expected` | 1 |
| `default-mode rule is not supported: `div>div>div[class=card-ep mt-2], .book-ep a` is not a CSS selector: Token "card-ep" was not expected` | 1 |
| `default-mode rule is not supported: `href&lt;js>
var id = result.match(/_(\d+)/)[1];
var iid = parseInt(id/1000);
'http://www.biquge.info/files/article/image/'+iid+'/'+id+'/'+id+'s.jpg';
&lt;/js>` is not a CSS selector: Token "&" was not expected` | 1 |
| `default-mode rule is not supported: `href&lt;js>
var id = result.match(/_(\d+)/)[1];
var iid = parseInt(id/1000);
'http://www.loubiqu.com/files/article/image/'+iid+'/'+id+'/'+id+'s.jpg';
&lt;/js>` is not a CSS selector: Token "&" was not expected` | 1 |
| `default-mode rule is not supported: `http://api.jxgtzxc.com/` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `https://novel.snssdk.com/api/novel/book/directory/list/v1?book_id={$.book_id}` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `https://www.linovel.net/book/{$.id}.html` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 2 |
| `default-mode rule is not supported: `java.refreshBookUrl()` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Function(
        "refreshBookUrl",
    ),
)` | 1 |
| `default-mode rule is not supported: `java.refreshTocUrl()` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Function(
        "refreshTocUrl",
    ),
)` | 3 |
| `default-mode rule is not supported: `span.0:-text` is not a CSS selector: Token "0" was not expected` | 2 |
| `default-mode rule is not supported: `td.-1:0-2` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Number {
        has_sign: true,
        value: -1.0,
        int_value: Some(
            -1,
        ),
    },
)` | 1 |
| `default-mode rule is not supported: `title.replace(/【\d{6}】$/,'')` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Function(
        "replace",
    ),
)` | 1 |
| `default-mode rule is not supported: `{$.attributes.originalLanguage},{$.type},{$.attributes.tags..en}` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 2 |
| `default-mode rule is not supported: `{$.crazy_rating}分` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 6 |
| `default-mode rule is not supported: `{$.grade}分` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 12 |
| `default-mode rule is not supported: `{$.listencnt}播放` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 2 |
| `default-mode rule is not supported: `{$.score}分` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 3 |
| `default-mode rule is not supported: `{$.type_name},{$.catalog_name}` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 9 |
| `default-mode rule is not supported: `{baseUrl}` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `御兽之王 \| 24新晋白金作家轻泉流响 \| 起点霸榜 \| 快节奏爽文` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `穿进赛博游戏后逆袭成神
https://www.aqxsw666.com/txt-xx/nsxs/cycs/txt-240907.htm` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `连载中{$.status}已完结` is not a CSS selector: Token "<{" was not expected` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error calling function with 0 argument(s) while 1 where expected
    at <eval> (<input>:14:129)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:2147)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error calling function with 0 argument(s) while 1 where expected
    at <eval> (<input>:31:27)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:4850)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error calling function with 0 argument(s) while 1 where expected
    at <eval> (<input>:3:14)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:453)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': default-mode rule is not supported: `a[href~=[_-]\d+(/\|\.[a-z]+)?$]:has(i,img):not(:matches(\S),[href~=^javascript:\|^#])` is not a CSS selector: Token "\r" was not expected
    at <eval> (<input>:1:34)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:706)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': default-mode rule is not supported: `img[src~=^(data\|https?):\|^[^:]+/]` is not a CSS selector: Unexpected error occurred. Please report this to the developer
DanglingCombinator
    at <eval> (<input>:1:39)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:353)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'includes' of undefined
    at <eval> (<input>:3:12)
    at <anonymous> (eval_script:1:36)
    at <e
    at <eval> (<input>:2:28)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1040)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': json path is invalid: unexpected character at 14
    at <eval> (<input>:1:5)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:145)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': json rule is invalid: expected value at line 1 column 1
    at <eval> (<input>:1:4)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1314)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': json rule is invalid: expected value at line 1 column 1
    at <eval> (<input>:2:21)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:254)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:12371)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1692)
` | 10 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:5627)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:5629)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:5866)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:6236)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:7207)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cache is not defined
    at <eval> (<input>:6:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:8945)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Datas' of undefined
    at <eval> (<input>:9:15)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:642)
` | 8 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Datas' of undefined
    at <eval> (<input>:9:15)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:713)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Jsoup' of undefined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1003)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Jsoup' of undefined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:203)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Jsoup' of undefined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:394)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Jsoup' of undefined
    at <eval> (<input>:2:9)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:5367)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Jsoup' of undefined
    at <eval> (<input>:3:26)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:4391)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Symbol.iterator' of null
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:705)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'altTitles' of undefined
    at <eval> (<input>:1:37)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:593)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'altTitles' of undefined
    at <eval> (<input>:1:37)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:595)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'body' of undefined
    at <eval> (<input>:2:20)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:2574)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'data' of undefined
    at <eval> (<input>:1:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:267)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'description' of undefined
    at <eval> (<input>:1:37)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:497)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'forEach' of undefined
    at <eval> (<input>:6:3)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1308)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'length' of undefined
    at <eval> (<input>:5:23)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:459)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'length' of undefined
    at parseComments (<input>:2:22)
    at <eval> (<input>:10:89)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:606)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'map' of undefined
    at <eval> (<input>:4:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:300)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'map' of undefined
    at <eval> (<input>:5:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:455)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'maxPageId' of undefined
    at <eval> (<input>:4:24)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:247)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'split' of undefined
    at <eval> (<input>:2:19)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1484)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:13:23)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:834)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:14:11)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:809)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:14)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:106)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:15)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:150)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:16)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:85)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:18)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:299)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:18)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:374)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:19)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:150)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:45)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:247)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:56)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:130)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:7)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:123)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:8)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:74)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:2:14)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:398)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:2:19)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:547)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:2:20)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:456)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:2:23)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:278)
` | 9 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:2:42)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:129)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:2:45)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:240)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:3:21)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:174)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:3:21)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:422)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:3:31)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1473)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:4:13)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:348)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:5:17)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:293)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:5:32)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:267)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: creatRequest is not defined
    at <eval> (<input>:10:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:252)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: expecting ';'
    at <input>:2:5
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:428)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: formatTimeDynamic is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:82)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: formatTimeDynamic is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:95)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: getDynamicType is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:75)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: getSeachType is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:73)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: getSeachType is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:94)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:10:189)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1189)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:10:189)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1280)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:10:189)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1477)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:1:18)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:86)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:1:19)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:188)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:1:32)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:162)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:208:30)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:10147)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:104)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:217)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:231)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:30)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:219)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:42)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:181)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:43)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:153)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:45)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:223)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:3:29)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:363)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:3:36)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:328)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:3:47)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:345)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:4:12)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:517)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:4:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:236)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:4:35)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:543)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:5:30)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:937)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:6:141)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1416)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:8:23)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:506)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at tag (<input>:39:52)
    at <eval> (<input>:46:3)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:2429)
` | 10 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: replaceCover is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:89)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: sign is not defined
    at <eval> (<input>:4:142)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:327)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: sign is not defined
    at <eval> (<input>:5:135)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:340)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: sign is not defined
    at <eval> (<input>:6:142)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:420)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '=='
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:131)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:136)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:57)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:62)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:67)
` | 5 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:68)
` | 5 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:74)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:81)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:89)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:97)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: 'var'
    at <input>:44:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:7610)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: 'var'
    at eval_script:3:56
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '}'
    at <input>:5:12
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:146)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '}'
    at <input>:5:12
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:147)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:13:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:690)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:28:21
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:4262)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:3:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:188)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:3:11
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:233)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:3:12
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:234)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:3:9
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:674)
` | 4 |
| `json path is invalid: path must start with `$`` | 1 |
| `json path is invalid: unexpected character at 15` | 2 |
| `json path is invalid: unexpected character at 28` | 1 |
| `json rule is invalid: expected value at line 1 column 1` | 2 |
| `xpath rule is invalid: Unexpected error occurred. Please report this to the developer
DanglingCombinator` | 3 |

## Failed rule examples

| Error | Example paths |
| --- | --- |
| `default-mode rule is not supported: `$1` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[248].ruleToc.chapterUrl = $1<br>source[382].ruleSearch.bookUrl = $1<br>source[610].ruleToc.chapterUrl = $1 |
| `default-mode rule is not supported: `$2` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[248].ruleToc.chapterName = $2##[\(（【].*?[求更谢乐发订合补加].*?[】）\)]<br>source[382].ruleSearch.coverUrl = $2<br>source[610].ruleToc.chapterName = $2##[\(（【].*?[求更谢乐发订合补加].*?[】）\)] |
| `default-mode rule is not supported: `$3` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[382].ruleSearch.name = $3 |
| `default-mode rule is not supported: `$4` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[382].ruleSearch.author = $4 |
| `default-mode rule is not supported: `$5,$7,$10` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[382].ruleSearch.kind = $5,$7,$10 |
| `default-mode rule is not supported: `$6` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[382].ruleSearch.wordCount = $6 |
| `default-mode rule is not supported: `$8` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[382].ruleSearch.intro = $8 |
| `default-mode rule is not supported: `$9` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[382].ruleSearch.lastChapter = $9 |
| `default-mode rule is not supported: `%双女主` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[737].ruleSearch.checkKeyWord = %双女主 |
| `default-mode rule is not supported: `++https://readbook-service-freebook.cread.com/cx/itf/getvolume?bookId=803500510` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[10].ruleSearch.checkKeyWord = ++https://readbook-service-freebook.cread.com/cx/itf/getvolume?bookId=803500510<br>source[35].ruleSearch.checkKeyWord = ++https://readbook-service-freebook.cread.com/cx/itf/getvolume?bookId=803500510 |
| `default-mode rule is not supported: `+` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[124].ruleToc.chapterList = +@js:org.jsoup.Jsoup.parse(result).select(' #newlist a').toArray().sort().map(x=>({n:x.text(),u:x.attr('href')}))<br>source[248].ruleSearch.bookList = +@css:.bookbox<br>source[382].ruleToc.chapterList = +@css:.chapter a |
| `default-mode rule is not supported: `-https://api.bilibili.com/x/article/view?id=34980384,{"js":"book?result:'https://www.bilibili.com/opus/937892609776418850?spm_id_from=333.1365.0.0'"}` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | source[217].ruleSearch.checkKeyWord = --https://api.bilibili.com/x/article/view?id=34980384,{"js":"book?result:'https://www.bilibili.com/opus/937892609776418850?spm_id_from=333.1365.0.0'"} |
| `default-mode rule is not supported: `. Gap_size-4s__F67Nf Gap_direction-x__RsHk8` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    WhiteSpace(
        " ",
    ),
)` | source[281].ruleSearch.bookUrl = class. Gap_size-4s__F67Nf Gap_direction-x__RsHk8@tag.a@href<br>source[281].ruleSearch.name = class. Gap_size-4s__F67Nf Gap_direction-x__RsHk8@text |
| `default-mode rule is not supported: `.listmain a[href~=/[^/]+/\d+.htm]` is not a CSS selector: Unexpected error occurred. Please report this to the developer
DanglingCombinator` | source[199].ruleToc.chapterList = @css:.listmain a[href~=/[^/]+/\d+.htm]<br>source[523].ruleToc.chapterList = @css:.listmain a[href~=/[^/]+/\d+.htm]<br>source[585].ruleToc.chapterList = @css:.listmain a[href~=/[^/]+/\d+.htm] |
| `default-mode rule is not supported: `0` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[528].ruleContent.imageStyle = 0.0 |
| `default-mode rule is not supported: `3-7+3*5*2*(2+(4+3*2*4*2-2-8+1)*2*2)-2*2+(1/2.5+1)*3` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[128].ruleSearch.checkKeyWord = 3-7+3*5*2*(2+(4+3*2*4*2-2-8+1)*2*2)-2*2+(1/2.5+1)*3 |
| `default-mode rule is not supported: `7号基地` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[468].ruleSearch.checkKeyWord = 7号基地 |
| `default-mode rule is not supported: `[class=s6 wid6]` is not a CSS selector: Token "s6" was not expected` | source[885].ruleSearch.kind = [class=s6 wid6]@text<br>source[948].ruleSearch.kind = [class=s6 wid6]@text |
| `default-mode rule is not supported: `[property~ =category\|statuslupdate_time]` is not a CSS selector: Unexpected error occurred. Please report this to the developer
UnexpectedTokenInAttributeSelector(
    Delim(
        '~',
    ),
)` | source[311].ruleBookInfo.kind = [property~ =category\|statuslupdate_time]@content |
| `default-mode rule is not supported: `[property～=category\|status\|update_time]` is not a CSS selector: Token "\|" was not expected` | source[6].ruleBookInfo.kind = [property～=category\|status\|update_time]@content |
| `default-mode rule is not supported: `[书籍状态] 连载{$.status}完结` is not a CSS selector: Token "<{" was not expected` | source[518].ruleSearch.lastChapter = [书籍状态] 连载{$.status}完结
##连载1\|0完结 |
| `default-mode rule is not supported: `a.` is not a CSS selector: Unexpected EOL` | source[335].ruleSearch.lastChapter = class.author.-1@tag.a.@text |
| `default-mode rule is not supported: `book.canUpdate=false` is not a CSS selector: Token "=" was not expected` | source[85].ruleToc.preUpdateJs = book.canUpdate=false |
| `default-mode rule is not supported: `cat[*]` is not a CSS selector: Unexpected EOL` | source[33].ruleExplore.kind = .book-extra@text&&.book-tag@text&&cat[*]##^[^丨]+丨\s*\|\s.*<br>source[79].ruleExplore.kind = .book-extra@text&&.book-tag@text&&cat[*]##^[^丨]+丨\s*\|\s.* |
| `default-mode rule is not supported: `categoryNames.*.className` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Delim(
        '*',
    ),
)` | source[485].ruleBookInfo.kind = categoryNames.*.className<br>source[648].ruleBookInfo.kind = categoryNames.*.className |
| `default-mode rule is not supported: `category_tag.*.*` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Delim(
        '*',
    ),
)` | source[531].ruleBookInfo.kind = category_tag.*.*&&status&&unit_price<br>source[604].ruleBookInfo.kind = category_tag.*.*&&status&&unit_price |
| `default-mode rule is not supported: `data.Content[0].Content` is not a CSS selector: Unexpected error occurred. Please report this to the developer
NoQualifiedNameInAttributeSelector(
    Number {
        has_sign: false,
        value: 0.0,
        int_value: Some(
            0,
        ),
    },
)` | source[555].ruleContent.content = data.Content[0].Content |
| `default-mode rule is not supported: `div.forum-content.mt-3,div[class =d_post_content j_d_post_content]` is not a CSS selector: Token "j_d_post_content" was not expected` | source[26].ruleContent.content = @css:div.forum-content.mt-3,div[class =d_post_content j_d_post_content]@all
@js:
java.t2s(result) |
| `default-mode rule is not supported: `div>div>div[class=card-ep mt-2], .book-ep a` is not a CSS selector: Token "card-ep" was not expected` | source[26].ruleSearch.lastChapter = @css:div>div>div[class=card-ep mt-2], .book-ep a@text
@js:
java.t2s(result) |
| `default-mode rule is not supported: `href&lt;js>
var id = result.match(/_(\d+)/)[1];
var iid = parseInt(id/1000);
'http://www.biquge.info/files/article/image/'+iid+'/'+id+'/'+id+'s.jpg';
&lt;/js>` is not a CSS selector: Token "&" was not expected` | source[93].ruleExplore.coverUrl = class.s2@a@href&lt;js>
var id = result.match(/_(\d+)/)[1];
var iid = parseInt(id/1000);
'http://www.biquge.info/files/article/image/'+iid+'/'+id+'/'+id+'s.jpg';
&lt;/js> |
| `default-mode rule is not supported: `href&lt;js>
var id = result.match(/_(\d+)/)[1];
var iid = parseInt(id/1000);
'http://www.loubiqu.com/files/article/image/'+iid+'/'+id+'/'+id+'s.jpg';
&lt;/js>` is not a CSS selector: Token "&" was not expected` | source[93].ruleSearch.coverUrl = class.odd.0@tag.a@href&lt;js>
var id = result.match(/_(\d+)/)[1];
var iid = parseInt(id/1000);
'http://www.loubiqu.com/files/article/image/'+iid+'/'+id+'/'+id+'s.jpg';
&lt;/js> |
| `default-mode rule is not supported: `http://api.jxgtzxc.com/` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | source[56].ruleBookInfo.downloadUrls = http://api.jxgtzxc.com/ |
| `default-mode rule is not supported: `https://novel.snssdk.com/api/novel/book/directory/list/v1?book_id={$.book_id}` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | source[126].ruleSearch.lastChapter = https://novel.snssdk.com/api/novel/book/directory/list/v1?book_id={$.book_id}<js>
java.ajax(result)</js>$.data.book_info.last_chapter_title&&$.data.book_info.last_chapter_update_time##\n##·<js>
String(result).replace(/·\d+/,"·" + java.timeFormat(result.match(/·(\d+)/)[1]*1000))</js> |
| `default-mode rule is not supported: `https://www.linovel.net/book/{$.id}.html` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | source[33].ruleExplore.bookUrl = a.0@href\|\|https://www.linovel.net/book/{$.id}.html<br>source[79].ruleExplore.bookUrl = a.0@href\|\|https://www.linovel.net/book/{$.id}.html |
| `default-mode rule is not supported: `java.refreshBookUrl()` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Function(
        "refreshBookUrl",
    ),
)` | source[115].ruleBookInfo.tocUrl = java.refreshBookUrl() |
| `default-mode rule is not supported: `java.refreshTocUrl()` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Function(
        "refreshTocUrl",
    ),
)` | source[605].ruleToc.preUpdateJs = java.refreshTocUrl()<br>source[795].ruleToc.preUpdateJs = java.refreshTocUrl()<br>source[964].ruleToc.preUpdateJs = java.refreshTocUrl() |
| `default-mode rule is not supported: `span.0:-text` is not a CSS selector: Token "0" was not expected` | source[258].ruleExplore.kind = span.0:-text##\[\|\]<br>source[297].ruleExplore.kind = span.0:-text##\[\|\] |
| `default-mode rule is not supported: `td.-1:0-2` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Number {
        has_sign: true,
        value: -1.0,
        int_value: Some(
            -1,
        ),
    },
)` | source[435].ruleExplore.kind = td.-1:0-2@text |
| `default-mode rule is not supported: `title.replace(/【\d{6}】$/,'')` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Function(
        "replace",
    ),
)` | source[217].ruleToc.formatJs = title.replace(/【\d{6}】$/,'') |
| `default-mode rule is not supported: `{$.attributes.originalLanguage},{$.type},{$.attributes.tags..en}` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[81].ruleBookInfo.kind = {$.attributes.originalLanguage},{$.type},{$.attributes.tags..en}
@js:Array.from(result).join('\n').replace(/\n/,"");<br>source[81].ruleSearch.kind = {$.attributes.originalLanguage},{$.type},{$.attributes.tags..en}
@js:Array.from(result).join('\n').replace(/\n/,""); |
| `default-mode rule is not supported: `{$.crazy_rating}分` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[361].ruleExplore.kind = $.chapters_update_time&&$.c_class_name&&{$.crazy_rating}分<br>source[361].ruleSearch.kind = $.chapters_update_time&&$.c_class_name&&{$.crazy_rating}分<br>source[532].ruleSearch.kind = $.bookBClassificationName&&c_class_name&&{$.crazy_rating}分 |
| `default-mode rule is not supported: `{$.grade}分` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[10].ruleBookInfo.kind = {$.grade}分&&$.categoryName&&连载中{$.bookStatue}已完结
##连载中03\|01已完结<br>source[10].ruleExplore.kind = {$.grade}分&&$.categoryName&&连载中{$.bookStatue}已完结
##连载中03\|01已完结<br>source[10].ruleSearch.kind = {$.grade}分&&$.categoryName&&连载中{$.bookStatue}已完结
##连载中03\|01已完结 |
| `default-mode rule is not supported: `{$.listencnt}播放` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[121].ruleBookInfo.kind = {$.listencnt}播放&&$.tag<br>source[121].ruleSearch.kind = {$.listencnt}播放&&$.tag |
| `default-mode rule is not supported: `{$.score}分` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[18].ruleSearch.kind = {$.score}分&&$.category&&$.tags<br>source[1].ruleSearch.kind = $.category&&{$.score}分<br>source[549].ruleSearch.kind = {$.score}分&&$.category&&$.tags |
| `default-mode rule is not supported: `{$.type_name},{$.catalog_name}` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[108].ruleSearch.kind = {$.type_name},{$.catalog_name}##\{.*?\}<br>source[356].ruleSearch.kind = {$.type_name},{$.catalog_name}##\{.*?\}<br>source[728].ruleSearch.kind = {$.type_name},{$.catalog_name}##\{.*?\} |
| `default-mode rule is not supported: `{baseUrl}` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[5].ruleToc.chapterUrl = {baseUrl} |
| `default-mode rule is not supported: `御兽之王 \| 24新晋白金作家轻泉流响 \| 起点霸榜 \| 快节奏爽文` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | source[235].ruleSearch.checkKeyWord = 御兽之王 \| 24新晋白金作家轻泉流响 \| 起点霸榜 \| 快节奏爽文 |
| `default-mode rule is not supported: `穿进赛博游戏后逆袭成神
https://www.aqxsw666.com/txt-xx/nsxs/cycs/txt-240907.htm` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | source[154].ruleSearch.checkKeyWord = 穿进赛博游戏后逆袭成神
https://www.aqxsw666.com/txt-xx/nsxs/cycs/txt-240907.htm |
| `default-mode rule is not supported: `连载中{$.status}已完结` is not a CSS selector: Token "<{" was not expected` | source[314].ruleBookInfo.kind = 连载中{$.status}已完结&&category_name&&sub_category_name&&update_time
##连载中50\|30已完结\| \d.*<br>source[314].ruleExplore.kind = 连载中{$.status}已完结&&category_name&&sub_category_name
##连载中50\|30已完结<br>source[314].ruleSearch.kind = 连载中{$.status}已完结&&category_name&&sub_category_name
##连载中50\|30已完结 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error calling function with 0 argument(s) while 1 where expected
    at <eval> (<input>:14:129)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:2147)
` | source[126].ruleToc.chapterList = <js>
  let defaultVariable = {
    "volume": {
      "status": false,
      "mode": 1,
      "spacer": " \| ",
      "emptyVolumeName": {
        "action": false,
        "replace": "第一卷：默认"
      },
      "onlyOneShow": false
    }
  }
  let variable = (book.getVariable("custom") != "" && book.getVariable("custom") != null) ? book.getVariable("custom") : source.getVariable();
  if(variable != "" && variable != null){
    try{
      variable = JSON.parse(variable);
    } catch(err) {
      java.log(err);
      variable = defaultVariable;
    }
  } else {
    variable = defaultVariable;
  }
  let data = JSON.parse(result).data;
  let list = data.chapterListWithVolume;
  let volume = data.volumeNameList;
  let chapters = [];
  let volume_names = [];
  for(let i in list){
    for(let x in list[i]){
      let c = list[i][x];
      if(variable.volume.status == true && variable.volume.mode == 1 && volume_names.indexOf(c.volume_name) == -1){
        volume_names.push(c.volume_name);
        if(volume.length != 1 \|\| variable.volume.onlyOneShow){
          chapters.push({
            title: ((c.volume_name == "" && variable.volume.emptyVolumeName.action == "replace" && variable.volume.emptyVolumeName.replace) ? variable.volume.emptyVolumeName.replace : c.volume_name),
            isVolume: true
          });
        }
      }
      if(variable.volume.status == true && variable.volume.mode == 2){
        if(c.volume_name == "" && variable.volume.emptyVolumeName.action == "replace" && variable.volume.emptyVolumeName.replace){
          c.title = (c.volume_name == "" ? variable.volume.emptyVolumeName.replace : c.volume_name) + (variable.volume.spacer \|\| " \| ") + c.title;
        } else if(volume.length != 1 \|\| variable.volume.onlyOneShow){
          c.title = c.volume_name + (variable.volume.spacer \|\| " \| ") + c.title;
        }
      }
      c.url  = source.bookSourceUrl +  `/content?item_id=${c.itemId}`;
      chapters.push(c);
    }
  }
  chapters
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error calling function with 0 argument(s) while 1 where expected
    at <eval> (<input>:31:27)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:4850)
` | source[23].ruleContent.content = <js>
let srcall = src;
if(/\.mp4/.test(baseUrl)){
	java.startBrowser(baseUrl,"视频");
	result = "请点击章节链接观看视频\n视频链接："+baseUrl
	}else{
let returnContent = "";		
let returnimages = [];
let content = String(java.getString("$..content"));
let pics = JSON.parse(src)?.data?.plan?.images??[];
let posts = JSON.parse(src)?.response?.posts??[];
posts.forEach(x=>{
	   let returnContents = x.post?.returnContent??[];
	   returnContents.forEach(y=>{
	   		   	returnContent += "\n🏷 "+y.planTypeName+"\n"+y.content+"\n";
	   		   	y.images?returnimages.concat(y.images):null;
	   	})
	});
	
if(pics){
	pics=pics.map(x=>`<img src="${x.raw}">\n`).join("");
	}
let postid = java.getString("$..post.id");
let blogid = java.getString("$..post.blogId");
let f = 0;
try{
	book.getVariable();
	f = 1;
	}catch(e){	}
let dpcontent = "";
if(/段评\|1/.test(source.getVariable()) && f===1){
    java.log("段评")
if(!/myReturnGift/.test(baseUrl)){
	let api = `https://api.lofter.com/comment/pCommentCounts.json?postId=${postid}&blogId=${blogid}`;
  let pidall = JSON.parse(java.ajax(api)).data.list;
  java.put("pidall",JSON.stringify(pidall));
  java.put("postid",String(postid));
  java.put("blogid",String(blogid));
	java.setContent(content);
	let c = "";
	content = java.getElements("@@tag.p").toArray().map((x,i)=>{
		var pid = String(x.attr("id"));
		var originalText = x.html(); 
   var matchedItem = pidall.find(function(item) {
       
       if(/^p_i\d+$/.test(item.pid)){
           pid = "p_i"+i;
       }
       
        return item.pid === pid && item.count!==0;
    });
    if (matchedItem && String(x.text()).trim()!=="") {
        var comment_total = matchedItem.count;                 
        var pid = matchedItem.pid;                                 
        var imageOrContent = getImage(comment_total, postid, blogid, pid);
        return originalText+imageOrContent;      
    }else{
    	 return originalText
    	}
		}).join("\n");
        if(returnContent.trim()!==""){
            content = content+returnContent
        }
	}else if(/myReturnGift/.test(baseUrl) && String(content).trim()!==""){
   pidall = JSON.parse(java.get("pidall"));  
   content = content.split("\n").map((x,i)=>{
			   originalText = String(x);
			   matchedItem = pidall.find(function(item,index) {			    
			   
           return (item.pid.match(/r_.*?_i(\d+)/)?item.pid.match(/r_.*?_i(\d+)/)[1]:item.pid) == i;			
    });
			
			if (matchedItem) {
         comment_total = matchedItem.count;                 
         pid = matchedItem.pid;                               
         imageOrContent = getImage(comment_total, java.get("postid"), java.get("blogid"), pid);
        return originalText.replace(/\s+$/g,'')+imageOrContent;      
    }else if(originalText.trim()!==""){
    	   return originalText
    	}
			}).join("\n");
}

java.setContent(srcall);
}else{    
    if(returnContent.trim()!==""){    	
            content = content.match(/<p[^>]*>[\s\S]*?<\/p>/g).join("")+returnContent
     }
}

imgs = eval(String(java.getString("$..photoLinks")));
video =/video_down_url\\":\\"(.*?)\\"/.test(result)? "视频链接："+String(result).match(/video_down_url\\":\\"(.*?)\\"/)[1]:"";
img = "";
imgs?imgs.forEach(x=>{
	x?img += "<img src=\""+x.orign.replace(/%7C.*/g,'')+"\">\n":""
	}):"";
g=	(/myReturnGift/.test(baseUrl)&&(/[\u4e00-\u9fa5]/.test(content) \|\| pics))?"🏷 "+java.getString("$..planType.name")+" "+java.getString("$.data.plan.title")+"\n"+(String(java.getString("$..promotion"))?"【"+java.getString("$..promotion")+"】\n":""):"";


result = g + content + pics+ "\n"+img+"\n"+video;

if(video && (chapter.index == book.durChapterIndex)){
 	java.startBrowser(video.replace(/视频链接：/,''),"内容");
 	result = "❗️刷新本章节播放视频❗️"+result;
 			}
}
if(result =="\n\n")result = String(java.getString("$..msg")).replace(/success/g,'');
result = result.replace(/<img[^>]*?src[^>]*?prompt_list_risk[^>]*?>/g,'').replace(/<a[^>]*?store-vip\/verify-phone"[^>]*?>/,'当前账号存在风险，需验证。\n（❗️要看彩蛋请完成验证后刷新正文）\n1、点击登录。\n2、点击获取账号风险验证码\n3、填入验证码\n4、点击立即验证\n5、提示成功即可刷新正文获取内容');
result
</js>
##tbc\.\|没有赠礼记录\|【成为我的高级粉丝，解锁我的回礼与海量权益】##<br> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error calling function with 0 argument(s) while 1 where expected
    at <eval> (<input>:3:14)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:453)
` | source[290].ruleSearch.bookList = <js>
var sk=java.get("sk");
var su=source.getKey();
var v=source.getVariable();
var so=/s.php/.test(baseUrl);
if(so){
	var bu=baseUrl.match(/^(.+)\/s.php/)[1];
if(bu!=su&&bu!=v){
	source.setVariable(bu);
	java.toast("地址变动，重新搜索加载！")
	result=java.ajax(baseUrl+`,{
		"body": "s=${String(sk)}",
		"method": "POST"}`);
		}
	}else{result=result}
	result;
</js>
.bd li |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': default-mode rule is not supported: `a[href~=[_-]\d+(/\|\.[a-z]+)?$]:has(i,img):not(:matches(\S),[href~=^javascript:\|^#])` is not a CSS selector: Token "\r" was not expected
    at <eval> (<input>:1:34)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:706)
` | source[289].ruleContent.nextContentUrl = @js:if((r=java.getStringList((nx=java.get("next"))+"a:matches(第二[頁页]\|下[一\\s]*[頁页]):not([href~=^javascript:\|^#])@href\|\|a:matches(下[一\\s]*[篇章回节節话話]):not([href~=^javascript:\|^#])@href\|\|a[href~=[_-]\\d+(/\|\\.[a-z]+)?$]:has(i,img):not(:matches(\\S),[href~=^javascript:\|^#])@href")).size()){
if(~String(book.tocUrl+(nextChapterUrl\|\|'')).indexOf(r=String(r.get(r.size()-1)))\|\|~r.indexOf(nextChapterUrl\|\|String(book.tocUrl).replace(/.+(?=_\d+\/$)/,'')))r=null
}else r=null;
if(r){if(nx!='')chapter.putVariable("next",(n=r.replace(/\d+(?=[^\d]*$)/,it=>+it+1))==r?'':'a[href="'+n+'"]@href\|\|');
r+java.get("动")} |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': default-mode rule is not supported: `img[src~=^(data\|https?):\|^[^:]+/]` is not a CSS selector: Unexpected error occurred. Please report this to the developer
DanglingCombinator
    at <eval> (<input>:1:39)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:353)
` | source[289].ruleBookInfo.coverUrl = @js:(s=java.getStringList('img[alt*='+book.name+']@src\|\|meta[property$=image]@content\|\|img[src~=(cover\|file\|article)[^a-z]\|/\\d+[/_-]\\d+(s?\\.\|$)]@src\|\|img[data-src~=\\S]@data-src\|\|img[src*=/img]@src\|\|img[src~=^(data\|https?):\|^[^:]+/]@src')).size()?/^data:/.test(s=s.get(0))?java.base64Encode(s):s:null |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'includes' of undefined
    at <eval> (<input>:3:12)
    at <anonymous> (eval_script:1:36)
    at <e
    at <eval> (<input>:2:28)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1040)
` | source[183].ruleContent.content = <js>
var step1 = java.base64Decode('PGpzPgp2YXIgYT1zb3VyY2UuYm9va1NvdXJjZUNvbW1lbnQ7CnZhciBiPWphdmEuYmFzZTY0RGVjb2RlKCdMeTh5TURJMUxqRXdMallnSU9Xa25PYVlqdWVwdWdvdkwrYTZrT2VrdnVXTXV1KzhtbWgwZEhCek9pOHZlV052Ynk1dVpYUT0nKTsKaWYgKGEuaW5jbHVkZXMoYikpIHsKY29uPWphdmEuYmFzZTY0RGVjb2RlKCdQR3B6UGdwaFBXcGhkbUV1WjJWMFUzUnlhVzVuS0NjdVkyaGhjSFJsY2kxcGJXRm5aVUJrWVhSaExXOXlhV2RwYm1Gc0p5azdDbUV1YzNCc2FYUW9JbHh1SWlrdWJXRndLR2s5UG1BOGFXMW5JSE55WXowaUpIdHBmU0krWUNrdWFtOXBiaWdpWEc0aUtRbzhMMnB6UGc9PScpOwpyZXN1bHQ9amF2YS5nZXRTdHJpbmcoY29uKTsKfSBlbHNlIHsKZXJyb3I9amF2YS5iYXNlNjREZWNvZGUoJzRwcWc3N2lQSU9hamdPYTFpK1dJc09TNXB1YTZrT2lpcSthQnR1YUVqK2V2b2VhVXVRcmlyWkFnNksrMzVZbU41YjZBNUwyYzZJQ0Y1TGk3NmFHMTVwdTA1cGF3NUxtbTVycVFLT1dGcyttWHJlV0hnT1dNbHVhWXZ1ZWt1dW1UdnVhT3BTbnZ2Sm9LYUhSMGNITTZMeTk1WTI5dkxtNWxkQXJpclpBZzVwQ2M1N1NpNTVTbzVvaTM1WkNONzd5YTVhU2M1cGlPNTZtNicpOwpyZXN1bHQ9amF2YS50b2FzdCgnXG4nK2Vycm9yKTsKcmVzdWx0PSdcbiZscm07XG4nK2Vycm9yOwp9CnJlc3VsdAo8L2pzPg==');
var step2 = java.getString(step1);
result = step2
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': json path is invalid: unexpected character at 14
    at <eval> (<input>:1:5)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:145)
` | source[708].ruleContent.content = @js:
java.getStringList("$.data.page[*]image").toArray().map
(a=>'<img src="'+a+'">').join("\n") |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': json rule is invalid: expected value at line 1 column 1
    at <eval> (<input>:1:4)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1314)
` | source[217].ruleExplore.bookUrl = https://api.bilibili.com/x/web-interface/view?aid={{$.id\|\|$.aid\|\|$.archives[0].aid\|\|$.modules..aid\|\|$.mid\|\|$.opus_id\|\|$.history.oid}}&cid={{$.cid}}@put:{"cid":"$.cid"}
@js:
if (S("$.rpid")) {
    result = `https://api.bilibili.com/x/v2/reply/reply?oid=${S("$.oid")}&type=${baseUrl.match(/type=(\d+)/)[1]}&root=${S("$.rpid")}&ps=100&pn=1&web_location=333.788`;
   
} else if (S("$.meta.series_id")) {
    result = `https://api.bilibili.com/x/series/archives?mid=${baseUrl.match(/mid=(\d+)/)[1]}&series_id=${S("$.meta.series_id")}&only_normal=true&sort=desc&ps=1000&pn=1&web_location=333.1387`;
} else if (/feed\/all.*?type=article/.test(baseUrl)) {
    result = getApi("article",S("$..rid_str"),S("$..id_str"))
}else if(/opus\/feed/.test(baseUrl)){
	
	result = getApi("opus",S("$.opus_id"),S("$.opus_id"))
	} else if(/article\/up\/lists/.test(baseUrl)){
		result = "https://api.bilibili.com/x/article/list/web/articles?id="+S("$.id")
	} else if(/\/pgc\//.test(baseUrl)){
		    result = getApi("media_bangumi",S("$.season_id\|\|$.id"))
		}else if(/x\/relation\|xlive/.test(baseUrl)){
         result = getApi("user",S("$.mid\|\|$.uid"))
	}else if(/feed\/space/.test(baseUrl)){
		 result = getDynamicUrl(S("$.type"),src);
		}else {
			if(/aid=&/.test(result)){
				result = result.replace(/aid=&/,'aid='+java.get('aid')+'&');
				}
    result = result
} |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': json rule is invalid: expected value at line 1 column 1
    at <eval> (<input>:2:21)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:254)
` | source[126].ruleBookInfo.intro = &nbsp;&nbsp;🔖 更新时间：{{java.timeFormat(java.getString('$..last_publish_time')*1000)}}
{{'\n&lrm;\n'}}
✏️ 开坑：{{$.create_time##T\|\+.*## }}
🔗 源站：{{$..source}}{{'\n&lrm;\n'}}
📖 阅读：{{$..sub_info}}{{'\n&lrm;\n'}}
🏷️ 简介：{{$..abstract}}
{{'\n&lrm;\n'}}📍{{$..copyright_info##，如有任何疑问，请通过“我的-意见反馈”告知我们##。}}
<js>
read_count=java.getString('$..read_count')
if(read_count>10000)read_count=(Number(read_count)/10000).toFixed(2)+'万'
result.replace('{count}',read_count).replace(/.+：\n&lrm;\n/,'')
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:12371)
` | source[310].ruleContent.content = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.java.io,
    Packages.java.util,
    Packages.java.security.spec,
    Packages.java.security,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security.interfaces,
    Packages.java.security.spec
);
with(javaImport){
	function encrypt(str){
                var bArr = String(str).getBytes("UTF-8");
		var pKCS8EncodedKeySpec =PKCS8EncodedKeySpec([48,-126,4,-65,2,1,0,48,13,6,9,42,-122,72,-122,-9,13,1,1,1,5,0,4,-126,4,-87,48,-126,4,-91,2,1,0,2,-126,1,1,0,-118,121,-93,-123,61,113,-87,-20,-33,52,-70,-125,-13,67,-61,92,-49,-41,-14,60,-49,31,-68,-88,-7,-38,-52,-62,-31,-76,-123,90,-28,-70,-23,-106,93,-93,-5,-57,22,62,51,45,-88,80,-38,-55,75,-3,-73,-96,65,-15,-110,11,-121,-91,105,-91,-11,-62,-111,96,17,-68,-82,113,-116,-82,-108,51,-100,31,61,14,-26,-5,-48,14,-36,-110,-90,113,46,-31,-67,-64,95,98,95,-123,2,-83,16,-105,74,-42,-100,3,-125,53,126,-72,-72,-76,79,108,1,-71,-82,-51,0,60,-94,84,-65,73,-44,-88,-107,-81,44,3,40,-33,-87,114,88,21,-53,111,-84,-88,-83,112,41,101,-118,50,22,-74,29,47,-51,-106,-89,-26,55,44,66,118,-7,-45,-72,78,-125,-37,73,72,-80,-11,38,-82,65,54,103,9,43,-106,-88,-81,29,50,56,-127,104,122,56,-38,-93,-76,-12,-99,-17,-87,97,-128,21,118,-78,-13,-67,15,-74,-116,71,63,122,5,-59,120,93,90,-104,116,21,-13,108,10,-50,-103,-33,-21,125,113,86,-73,46,-84,-92,106,-86,8,23,7,95,-65,80,83,6,-85,53,85,-85,43,-73,41,-84,-10,76,106,-36,119,99,-125,121,-92,-122,-94,-73,-15,125,115,120,-63,2,3,1,0,1,2,-126,1,1,0,-121,-55,-103,-48,-55,28,-47,125,102,-81,37,10,-55,28,52,-47,-87,58,95,-34,-61,88,-94,-66,-84,43,-93,72,-94,-35,75,59,-125,57,-54,94,-39,-70,56,-27,-45,-24,-16,116,-96,45,-111,45,125,103,-15,-115,-93,-68,-121,-14,-24,116,10,-14,99,-39,4,-121,73,61,85,110,33,126,-5,-14,-45,-16,74,6,119,-8,-117,-32,86,-23,51,111,-97,-126,91,120,-19,-49,-32,76,-28,-1,-30,90,9,88,3,42,-22,-102,37,-122,108,16,-36,36,-44,88,73,-111,-38,-34,-102,108,50,27,-22,-3,-39,-78,41,-99,123,-110,63,108,85,22,-119,93,-85,-98,-114,70,-31,-120,-122,10,-92,-31,87,117,-119,-49,25,1,20,-42,-61,35,96,-41,-46,-50,-114,-31,-36,-21,95,-70,113,110,62,-28,62,76,36,-57,81,-21,100,36,124,-74,112,-52,-73,109,-117,37,-3,-40,-111,-102,-13,62,118,-93,-119,-118,-33,-82,20,53,64,-57,24,63,-113,-126,-61,-69,90,-72,-56,120,80,-95,-14,124,-2,74,80,53,22,3,57,61,113,-117,-5,-54,61,-1,-105,-96,-68,7,-98,-49,95,-61,-16,51,-65,82,66,-125,-66,-92,115,75,-116,32,14,68,-51,124,-126,32,-63,52,84,117,2,-127,-127,0,-15,-52,16,-109,-8,94,1,-65,14,-119,72,18,115,86,17,19,-68,-19,24,86,12,5,-97,55,-16,-75,84,110,-126,23,-33,-56,120,17,-32,-127,-106,-54,-27,54,20,40,92,70,-18,94,31,-79,-42,69,74,18,85,17,-64,-80,60,4,-102,23,36,-52,-112,124,11,-30,-15,5,-92,125,51,-42,26,109,27,-19,93,10,49,-48,-106,91,-38,12,-45,73,-44,-26,37,114,21,105,36,-38,96,39,-5,-8,-4,-70,-11,106,-107,69,68,14,-43,-123,-127,6,127,65,-25,-49,75,110,99,-50,-32,-33,-54,-26,117,125,-18,24,-89,107,2,-127,-127,0,-110,-101,-24,103,-95,2,-73,-114,-109,-97,107,126,-32,75,-124,-39,-92,-98,-127,-33,2,-65,49,-80,24,118,94,4,-117,122,-70,-41,-42,59,-90,-53,-81,-101,-57,75,-99,-67,-87,-67,-35,40,-66,53,-45,95,79,-10,-120,-111,71,-12,-110,76,-74,82,-79,83,-84,3,43,111,-44,-109,4,61,-22,3,99,113,113,-121,-15,-108,44,-32,-28,46,-83,-115,-99,-86,108,-111,3,-11,-43,121,67,-73,-68,99,107,55,-119,-77,20,-68,-77,98,-113,-21,39,-128,-21,119,113,57,-99,-30,-65,-128,-76,-100,40,8,39,-72,18,9,-17,-99,-89,-125,2,-127,-128,108,110,19,93,23,-70,-88,83,-46,35,-13,-30,-6,63,-75,70,-63,-87,29,9,-79,56,112,46,-8,-51,-120,0,74,108,-124,88,-12,-89,39,-93,85,72,-59,66,-36,5,65,100,57,-114,-111,-18,0,-27,111,-109,10,-4,-4,8,-53,-47,80,124,98,111,45,-73,-62,-24,-47,38,-77,-99,-59,-70,20,125,-85,81,101,48,-90,40,32,-43,45,-46,36,-119,-18,100,10,-108,-65,79,56,76,-119,100,68,-43,98,24,64,-25,-69,-22,-92,-37,118,26,-7,66,61,-99,3,99,-19,50,-94,-91,106,40,81,103,-55,118,96,104,67,-29,2,-127,-127,0,-121,39,-59,-77,-85,34,119,23,-80,-115,-38,42,-120,25,-10,-86,49,-15,-110,102,-122,0,-66,-116,-55,-80,109,114,33,39,-114,-110,37,-60,-82,58,-66,116,-116,-32,-17,-43,-122,99,43,60,65,70,27,-53,-107,75,0,-111,118,85,72,126,1,-30,-17,-24,-29,-3,-76,16,-113,86,-51,37,74,-45,-66,-36,57,62,-118,-3,-1,-11,127,70,108,-26,-51,-1,-21,-64,48,119,116,74,43,-100,121,-58,-23,115,-76,-76,-20,28,29,-1,114,15,-26,70,26,76,-19,-117,-95,59,5,50,96,-50,72,-75,99,-16,116,104,-58,-122,127,-125,2,-127,-127,0,-105,-116,-61,-50,-11,-71,-17,-45,114,88,67,113,51,37,-77,53,82,-22,36,70,-19,34,93,22,-103,-36,28,-47,-113,120,-57,4,-95,11,-82,-62,-127,13,115,93,104,-110,-44,39,42,-125,64,-111,53,-14,82,73,69,0,-66,37,-56,-115,3,-103,5,-69,55,75,-113,7,-18,-32,97,-56,28,85,-48,72,27,-115,119,63,1,41,115,-27,-73,24,-63,57,-72,107,-9,39,13,120,-75,-42,33,9,-39,-93,72,-122,66,33,-36,-1,-18,-102,11,-49,28,-61,-120,6,102,-37,-106,-6,5,-111,107,-76,49,78,-40,19,53,106,-37,-125]);
		var signature = Signature.getInstance("SHA256WithRSA");
		signature.initSign(KeyFactory.getInstance("RSA").generatePrivate(pKCS8EncodedKeySpec));
		signature.update(bArr);
		return signature.sign();        
    }
	function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }

}
	var bookId = java.get("bookId");
	var str = baseUrl;
	var chapterId = str.match(/\d+/).toString();


	function getTransferRespond(Urltype){
		
		var myDate = new Date();
		var timestamp = myDate.getFullYear() + 
		"0" + (myDate.getMonth()+1) + 
		"0" + myDate.getDate() + 
		"0" + myDate.getDay() + 
		myDate.getMinutes() + 
		myDate.getSeconds();
		
		
		var url;
		var body;
		var body;
		if(Urltype == 1)
		{
	
			var url = "https://xgmf.zuanqianyi.com/glory/free/1152?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp;

			var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"autoPay\":\"1\",\"chapterId\":\""+chapterId+"\",\"keepShowAd\":1,\"confirmPay\":\"1\",\"bookId\":\""+bookId+"\"}8cfaaedef7a7e24a716732ff5428958f"))

			var body =
			"{\"autoPay\":\"1\",\"chapterId\":\""+chapterId+"\",\"keepShowAd\":1,\"confirmPay\":\"1\",\"bookId\":\""+bookId+"\"}";

			var header = {};

			header["uid"]="1706970565";
			header["pname"]="com.dzmf.zmfxsdq";
			header["sign"]=sign;
			header["signType"]="2";
			header["Content-Type"]="application/json; charset=UTF-8";
			//header["Content-Length"]=93;
			header["Host"]="xgmf.zuanqianyi.com";
		
		}else
		{
			var url = "https://xgmf.zuanqianyi.com/glory/free/1153?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp;

			var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"autoPay\":\"1\",\"keepShowAd\":1,\"chapterIds\":[\""+chapterId+"\"],\"bookId\":\""+bookId+"\"}8cfaaedef7a7e24a716732ff5428958f"))

			var body =
			"{\"autoPay\":\"1\",\"keepShowAd\":1,\"chapterIds\":[\""+chapterId+"\"],\"bookId\":\""+bookId+"\"}";
			
			var header = {};

			header["uid"]="1706970565";
			header["pname"]="com.dzmf.zmfxsdq";
			header["sign"]=sign;
			header["signType"]="2";
			header["Content-Type"]="application/json; charset=UTF-8";
			//header["Content-Length"]=79;
			header["Host"]="xgmf.zuanqianyi.com";		
		}
		
		var respond= java.post(url,body,header);
		return respond;
	}
	
	var UrltypeNum = 1;
	
	var respond = getTransferRespond(UrltypeNum);

	var transferHtml=respond.body();

	var is = String(transferHtml).search(/不合法/);

	while(is > 0){
		if(UrltypeNum == 1){UrltypeNum = 2}else{UrltypeNum = 1}
		var respond = getTransferRespond(UrltypeNum);
		var transferHtml=respond.body();
		var is = String(transferHtml).search(/不合法/);
	}
	//java.log(transferHtml);
	var transferJson=JSON.parse(transferHtml);
	var chapterUrl =transferJson.data.chapterInfo[0].cdnUrls[0];
	chapterHtml = java.ajax(chapterUrl);
	//java.log(chapterUrl);
	//java.log(java.ajax(chapterUrl));
</js><br>source[500].ruleContent.content = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.java.io,
    Packages.java.util,
    Packages.java.security.spec,
    Packages.java.security,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security.interfaces,
    Packages.java.security.spec
);
with(javaImport){
	function encrypt(str){
                var bArr = String(str).getBytes("UTF-8");
		var pKCS8EncodedKeySpec =PKCS8EncodedKeySpec([48,-126,4,-65,2,1,0,48,13,6,9,42,-122,72,-122,-9,13,1,1,1,5,0,4,-126,4,-87,48,-126,4,-91,2,1,0,2,-126,1,1,0,-118,121,-93,-123,61,113,-87,-20,-33,52,-70,-125,-13,67,-61,92,-49,-41,-14,60,-49,31,-68,-88,-7,-38,-52,-62,-31,-76,-123,90,-28,-70,-23,-106,93,-93,-5,-57,22,62,51,45,-88,80,-38,-55,75,-3,-73,-96,65,-15,-110,11,-121,-91,105,-91,-11,-62,-111,96,17,-68,-82,113,-116,-82,-108,51,-100,31,61,14,-26,-5,-48,14,-36,-110,-90,113,46,-31,-67,-64,95,98,95,-123,2,-83,16,-105,74,-42,-100,3,-125,53,126,-72,-72,-76,79,108,1,-71,-82,-51,0,60,-94,84,-65,73,-44,-88,-107,-81,44,3,40,-33,-87,114,88,21,-53,111,-84,-88,-83,112,41,101,-118,50,22,-74,29,47,-51,-106,-89,-26,55,44,66,118,-7,-45,-72,78,-125,-37,73,72,-80,-11,38,-82,65,54,103,9,43,-106,-88,-81,29,50,56,-127,104,122,56,-38,-93,-76,-12,-99,-17,-87,97,-128,21,118,-78,-13,-67,15,-74,-116,71,63,122,5,-59,120,93,90,-104,116,21,-13,108,10,-50,-103,-33,-21,125,113,86,-73,46,-84,-92,106,-86,8,23,7,95,-65,80,83,6,-85,53,85,-85,43,-73,41,-84,-10,76,106,-36,119,99,-125,121,-92,-122,-94,-73,-15,125,115,120,-63,2,3,1,0,1,2,-126,1,1,0,-121,-55,-103,-48,-55,28,-47,125,102,-81,37,10,-55,28,52,-47,-87,58,95,-34,-61,88,-94,-66,-84,43,-93,72,-94,-35,75,59,-125,57,-54,94,-39,-70,56,-27,-45,-24,-16,116,-96,45,-111,45,125,103,-15,-115,-93,-68,-121,-14,-24,116,10,-14,99,-39,4,-121,73,61,85,110,33,126,-5,-14,-45,-16,74,6,119,-8,-117,-32,86,-23,51,111,-97,-126,91,120,-19,-49,-32,76,-28,-1,-30,90,9,88,3,42,-22,-102,37,-122,108,16,-36,36,-44,88,73,-111,-38,-34,-102,108,50,27,-22,-3,-39,-78,41,-99,123,-110,63,108,85,22,-119,93,-85,-98,-114,70,-31,-120,-122,10,-92,-31,87,117,-119,-49,25,1,20,-42,-61,35,96,-41,-46,-50,-114,-31,-36,-21,95,-70,113,110,62,-28,62,76,36,-57,81,-21,100,36,124,-74,112,-52,-73,109,-117,37,-3,-40,-111,-102,-13,62,118,-93,-119,-118,-33,-82,20,53,64,-57,24,63,-113,-126,-61,-69,90,-72,-56,120,80,-95,-14,124,-2,74,80,53,22,3,57,61,113,-117,-5,-54,61,-1,-105,-96,-68,7,-98,-49,95,-61,-16,51,-65,82,66,-125,-66,-92,115,75,-116,32,14,68,-51,124,-126,32,-63,52,84,117,2,-127,-127,0,-15,-52,16,-109,-8,94,1,-65,14,-119,72,18,115,86,17,19,-68,-19,24,86,12,5,-97,55,-16,-75,84,110,-126,23,-33,-56,120,17,-32,-127,-106,-54,-27,54,20,40,92,70,-18,94,31,-79,-42,69,74,18,85,17,-64,-80,60,4,-102,23,36,-52,-112,124,11,-30,-15,5,-92,125,51,-42,26,109,27,-19,93,10,49,-48,-106,91,-38,12,-45,73,-44,-26,37,114,21,105,36,-38,96,39,-5,-8,-4,-70,-11,106,-107,69,68,14,-43,-123,-127,6,127,65,-25,-49,75,110,99,-50,-32,-33,-54,-26,117,125,-18,24,-89,107,2,-127,-127,0,-110,-101,-24,103,-95,2,-73,-114,-109,-97,107,126,-32,75,-124,-39,-92,-98,-127,-33,2,-65,49,-80,24,118,94,4,-117,122,-70,-41,-42,59,-90,-53,-81,-101,-57,75,-99,-67,-87,-67,-35,40,-66,53,-45,95,79,-10,-120,-111,71,-12,-110,76,-74,82,-79,83,-84,3,43,111,-44,-109,4,61,-22,3,99,113,113,-121,-15,-108,44,-32,-28,46,-83,-115,-99,-86,108,-111,3,-11,-43,121,67,-73,-68,99,107,55,-119,-77,20,-68,-77,98,-113,-21,39,-128,-21,119,113,57,-99,-30,-65,-128,-76,-100,40,8,39,-72,18,9,-17,-99,-89,-125,2,-127,-128,108,110,19,93,23,-70,-88,83,-46,35,-13,-30,-6,63,-75,70,-63,-87,29,9,-79,56,112,46,-8,-51,-120,0,74,108,-124,88,-12,-89,39,-93,85,72,-59,66,-36,5,65,100,57,-114,-111,-18,0,-27,111,-109,10,-4,-4,8,-53,-47,80,124,98,111,45,-73,-62,-24,-47,38,-77,-99,-59,-70,20,125,-85,81,101,48,-90,40,32,-43,45,-46,36,-119,-18,100,10,-108,-65,79,56,76,-119,100,68,-43,98,24,64,-25,-69,-22,-92,-37,118,26,-7,66,61,-99,3,99,-19,50,-94,-91,106,40,81,103,-55,118,96,104,67,-29,2,-127,-127,0,-121,39,-59,-77,-85,34,119,23,-80,-115,-38,42,-120,25,-10,-86,49,-15,-110,102,-122,0,-66,-116,-55,-80,109,114,33,39,-114,-110,37,-60,-82,58,-66,116,-116,-32,-17,-43,-122,99,43,60,65,70,27,-53,-107,75,0,-111,118,85,72,126,1,-30,-17,-24,-29,-3,-76,16,-113,86,-51,37,74,-45,-66,-36,57,62,-118,-3,-1,-11,127,70,108,-26,-51,-1,-21,-64,48,119,116,74,43,-100,121,-58,-23,115,-76,-76,-20,28,29,-1,114,15,-26,70,26,76,-19,-117,-95,59,5,50,96,-50,72,-75,99,-16,116,104,-58,-122,127,-125,2,-127,-127,0,-105,-116,-61,-50,-11,-71,-17,-45,114,88,67,113,51,37,-77,53,82,-22,36,70,-19,34,93,22,-103,-36,28,-47,-113,120,-57,4,-95,11,-82,-62,-127,13,115,93,104,-110,-44,39,42,-125,64,-111,53,-14,82,73,69,0,-66,37,-56,-115,3,-103,5,-69,55,75,-113,7,-18,-32,97,-56,28,85,-48,72,27,-115,119,63,1,41,115,-27,-73,24,-63,57,-72,107,-9,39,13,120,-75,-42,33,9,-39,-93,72,-122,66,33,-36,-1,-18,-102,11,-49,28,-61,-120,6,102,-37,-106,-6,5,-111,107,-76,49,78,-40,19,53,106,-37,-125]);
		var signature = Signature.getInstance("SHA256WithRSA");
		signature.initSign(KeyFactory.getInstance("RSA").generatePrivate(pKCS8EncodedKeySpec));
		signature.update(bArr);
		return signature.sign();        
    }
	function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }

}
	var bookId = java.get("bookId");
	var str = baseUrl;
	var chapterId = str.match(/\d+/).toString();


	function getTransferRespond(Urltype){
		
		var myDate = new Date();
		var timestamp = myDate.getFullYear() + 
		"0" + (myDate.getMonth()+1) + 
		"0" + myDate.getDate() + 
		"0" + myDate.getDay() + 
		myDate.getMinutes() + 
		myDate.getSeconds();
		
		
		var url;
		var body;
		var body;
		if(Urltype == 1)
		{
	
			var url = "https://xgmf.zuanqianyi.com/glory/free/1152?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp;

			var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"autoPay\":\"1\",\"chapterId\":\""+chapterId+"\",\"keepShowAd\":1,\"confirmPay\":\"1\",\"bookId\":\""+bookId+"\"}8cfaaedef7a7e24a716732ff5428958f"))

			var body =
			"{\"autoPay\":\"1\",\"chapterId\":\""+chapterId+"\",\"keepShowAd\":1,\"confirmPay\":\"1\",\"bookId\":\""+bookId+"\"}";

			var header = {};

			header["uid"]="1706970565";
			header["pname"]="com.dzmf.zmfxsdq";
			header["sign"]=sign;
			header["signType"]="2";
			header["Content-Type"]="application/json; charset=UTF-8";
			//header["Content-Length"]=93;
			header["Host"]="xgmf.zuanqianyi.com";
		
		}else
		{
			var url = "https://xgmf.zuanqianyi.com/glory/free/1153?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp;

			var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"autoPay\":\"1\",\"keepShowAd\":1,\"chapterIds\":[\""+chapterId+"\"],\"bookId\":\""+bookId+"\"}8cfaaedef7a7e24a716732ff5428958f"))

			var body =
			"{\"autoPay\":\"1\",\"keepShowAd\":1,\"chapterIds\":[\""+chapterId+"\"],\"bookId\":\""+bookId+"\"}";
			
			var header = {};

			header["uid"]="1706970565";
			header["pname"]="com.dzmf.zmfxsdq";
			header["sign"]=sign;
			header["signType"]="2";
			header["Content-Type"]="application/json; charset=UTF-8";
			//header["Content-Length"]=79;
			header["Host"]="xgmf.zuanqianyi.com";		
		}
		
		var respond= java.post(url,body,header);
		return respond;
	}
	
	var UrltypeNum = 1;
	
	var respond = getTransferRespond(UrltypeNum);

	var transferHtml=respond.body();

	var is = String(transferHtml).search(/不合法/);

	while(is > 0){
		if(UrltypeNum == 1){UrltypeNum = 2}else{UrltypeNum = 1}
		var respond = getTransferRespond(UrltypeNum);
		var transferHtml=respond.body();
		var is = String(transferHtml).search(/不合法/);
	}
	//java.log(transferHtml);
	var transferJson=JSON.parse(transferHtml);
	var chapterUrl =transferJson.data.chapterInfo[0].cdnUrls[0];
	chapterHtml = java.ajax(chapterUrl);
	//java.log(chapterUrl);
	//java.log(java.ajax(chapterUrl));
</js><br>source[656].ruleContent.content = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.java.io,
    Packages.java.util,
    Packages.java.security.spec,
    Packages.java.security,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security.interfaces,
    Packages.java.security.spec
);
with(javaImport){
	function encrypt(str){
                var bArr = String(str).getBytes("UTF-8");
		var pKCS8EncodedKeySpec =PKCS8EncodedKeySpec([48,-126,4,-65,2,1,0,48,13,6,9,42,-122,72,-122,-9,13,1,1,1,5,0,4,-126,4,-87,48,-126,4,-91,2,1,0,2,-126,1,1,0,-118,121,-93,-123,61,113,-87,-20,-33,52,-70,-125,-13,67,-61,92,-49,-41,-14,60,-49,31,-68,-88,-7,-38,-52,-62,-31,-76,-123,90,-28,-70,-23,-106,93,-93,-5,-57,22,62,51,45,-88,80,-38,-55,75,-3,-73,-96,65,-15,-110,11,-121,-91,105,-91,-11,-62,-111,96,17,-68,-82,113,-116,-82,-108,51,-100,31,61,14,-26,-5,-48,14,-36,-110,-90,113,46,-31,-67,-64,95,98,95,-123,2,-83,16,-105,74,-42,-100,3,-125,53,126,-72,-72,-76,79,108,1,-71,-82,-51,0,60,-94,84,-65,73,-44,-88,-107,-81,44,3,40,-33,-87,114,88,21,-53,111,-84,-88,-83,112,41,101,-118,50,22,-74,29,47,-51,-106,-89,-26,55,44,66,118,-7,-45,-72,78,-125,-37,73,72,-80,-11,38,-82,65,54,103,9,43,-106,-88,-81,29,50,56,-127,104,122,56,-38,-93,-76,-12,-99,-17,-87,97,-128,21,118,-78,-13,-67,15,-74,-116,71,63,122,5,-59,120,93,90,-104,116,21,-13,108,10,-50,-103,-33,-21,125,113,86,-73,46,-84,-92,106,-86,8,23,7,95,-65,80,83,6,-85,53,85,-85,43,-73,41,-84,-10,76,106,-36,119,99,-125,121,-92,-122,-94,-73,-15,125,115,120,-63,2,3,1,0,1,2,-126,1,1,0,-121,-55,-103,-48,-55,28,-47,125,102,-81,37,10,-55,28,52,-47,-87,58,95,-34,-61,88,-94,-66,-84,43,-93,72,-94,-35,75,59,-125,57,-54,94,-39,-70,56,-27,-45,-24,-16,116,-96,45,-111,45,125,103,-15,-115,-93,-68,-121,-14,-24,116,10,-14,99,-39,4,-121,73,61,85,110,33,126,-5,-14,-45,-16,74,6,119,-8,-117,-32,86,-23,51,111,-97,-126,91,120,-19,-49,-32,76,-28,-1,-30,90,9,88,3,42,-22,-102,37,-122,108,16,-36,36,-44,88,73,-111,-38,-34,-102,108,50,27,-22,-3,-39,-78,41,-99,123,-110,63,108,85,22,-119,93,-85,-98,-114,70,-31,-120,-122,10,-92,-31,87,117,-119,-49,25,1,20,-42,-61,35,96,-41,-46,-50,-114,-31,-36,-21,95,-70,113,110,62,-28,62,76,36,-57,81,-21,100,36,124,-74,112,-52,-73,109,-117,37,-3,-40,-111,-102,-13,62,118,-93,-119,-118,-33,-82,20,53,64,-57,24,63,-113,-126,-61,-69,90,-72,-56,120,80,-95,-14,124,-2,74,80,53,22,3,57,61,113,-117,-5,-54,61,-1,-105,-96,-68,7,-98,-49,95,-61,-16,51,-65,82,66,-125,-66,-92,115,75,-116,32,14,68,-51,124,-126,32,-63,52,84,117,2,-127,-127,0,-15,-52,16,-109,-8,94,1,-65,14,-119,72,18,115,86,17,19,-68,-19,24,86,12,5,-97,55,-16,-75,84,110,-126,23,-33,-56,120,17,-32,-127,-106,-54,-27,54,20,40,92,70,-18,94,31,-79,-42,69,74,18,85,17,-64,-80,60,4,-102,23,36,-52,-112,124,11,-30,-15,5,-92,125,51,-42,26,109,27,-19,93,10,49,-48,-106,91,-38,12,-45,73,-44,-26,37,114,21,105,36,-38,96,39,-5,-8,-4,-70,-11,106,-107,69,68,14,-43,-123,-127,6,127,65,-25,-49,75,110,99,-50,-32,-33,-54,-26,117,125,-18,24,-89,107,2,-127,-127,0,-110,-101,-24,103,-95,2,-73,-114,-109,-97,107,126,-32,75,-124,-39,-92,-98,-127,-33,2,-65,49,-80,24,118,94,4,-117,122,-70,-41,-42,59,-90,-53,-81,-101,-57,75,-99,-67,-87,-67,-35,40,-66,53,-45,95,79,-10,-120,-111,71,-12,-110,76,-74,82,-79,83,-84,3,43,111,-44,-109,4,61,-22,3,99,113,113,-121,-15,-108,44,-32,-28,46,-83,-115,-99,-86,108,-111,3,-11,-43,121,67,-73,-68,99,107,55,-119,-77,20,-68,-77,98,-113,-21,39,-128,-21,119,113,57,-99,-30,-65,-128,-76,-100,40,8,39,-72,18,9,-17,-99,-89,-125,2,-127,-128,108,110,19,93,23,-70,-88,83,-46,35,-13,-30,-6,63,-75,70,-63,-87,29,9,-79,56,112,46,-8,-51,-120,0,74,108,-124,88,-12,-89,39,-93,85,72,-59,66,-36,5,65,100,57,-114,-111,-18,0,-27,111,-109,10,-4,-4,8,-53,-47,80,124,98,111,45,-73,-62,-24,-47,38,-77,-99,-59,-70,20,125,-85,81,101,48,-90,40,32,-43,45,-46,36,-119,-18,100,10,-108,-65,79,56,76,-119,100,68,-43,98,24,64,-25,-69,-22,-92,-37,118,26,-7,66,61,-99,3,99,-19,50,-94,-91,106,40,81,103,-55,118,96,104,67,-29,2,-127,-127,0,-121,39,-59,-77,-85,34,119,23,-80,-115,-38,42,-120,25,-10,-86,49,-15,-110,102,-122,0,-66,-116,-55,-80,109,114,33,39,-114,-110,37,-60,-82,58,-66,116,-116,-32,-17,-43,-122,99,43,60,65,70,27,-53,-107,75,0,-111,118,85,72,126,1,-30,-17,-24,-29,-3,-76,16,-113,86,-51,37,74,-45,-66,-36,57,62,-118,-3,-1,-11,127,70,108,-26,-51,-1,-21,-64,48,119,116,74,43,-100,121,-58,-23,115,-76,-76,-20,28,29,-1,114,15,-26,70,26,76,-19,-117,-95,59,5,50,96,-50,72,-75,99,-16,116,104,-58,-122,127,-125,2,-127,-127,0,-105,-116,-61,-50,-11,-71,-17,-45,114,88,67,113,51,37,-77,53,82,-22,36,70,-19,34,93,22,-103,-36,28,-47,-113,120,-57,4,-95,11,-82,-62,-127,13,115,93,104,-110,-44,39,42,-125,64,-111,53,-14,82,73,69,0,-66,37,-56,-115,3,-103,5,-69,55,75,-113,7,-18,-32,97,-56,28,85,-48,72,27,-115,119,63,1,41,115,-27,-73,24,-63,57,-72,107,-9,39,13,120,-75,-42,33,9,-39,-93,72,-122,66,33,-36,-1,-18,-102,11,-49,28,-61,-120,6,102,-37,-106,-6,5,-111,107,-76,49,78,-40,19,53,106,-37,-125]);
		var signature = Signature.getInstance("SHA256WithRSA");
		signature.initSign(KeyFactory.getInstance("RSA").generatePrivate(pKCS8EncodedKeySpec));
		signature.update(bArr);
		return signature.sign();        
    }
	function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }

}
	var bookId = java.get("bookId");
	var str = baseUrl;
	var chapterId = str.match(/\d+/).toString();


	function getTransferRespond(Urltype){
		
		var myDate = new Date();
		var timestamp = myDate.getFullYear() + 
		"0" + (myDate.getMonth()+1) + 
		"0" + myDate.getDate() + 
		"0" + myDate.getDay() + 
		myDate.getMinutes() + 
		myDate.getSeconds();
		
		
		var url;
		var body;
		var body;
		if(Urltype == 1)
		{
	
			var url = "https://xgmf.zuanqianyi.com/glory/free/1152?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp;

			var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"autoPay\":\"1\",\"chapterId\":\""+chapterId+"\",\"keepShowAd\":1,\"confirmPay\":\"1\",\"bookId\":\""+bookId+"\"}8cfaaedef7a7e24a716732ff5428958f"))

			var body =
			"{\"autoPay\":\"1\",\"chapterId\":\""+chapterId+"\",\"keepShowAd\":1,\"confirmPay\":\"1\",\"bookId\":\""+bookId+"\"}";

			var header = {};

			header["uid"]="1706970565";
			header["pname"]="com.dzmf.zmfxsdq";
			header["sign"]=sign;
			header["signType"]="2";
			header["Content-Type"]="application/json; charset=UTF-8";
			//header["Content-Length"]=93;
			header["Host"]="xgmf.zuanqianyi.com";
		
		}else
		{
			var url = "https://xgmf.zuanqianyi.com/glory/free/1153?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp;

			var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"autoPay\":\"1\",\"keepShowAd\":1,\"chapterIds\":[\""+chapterId+"\"],\"bookId\":\""+bookId+"\"}8cfaaedef7a7e24a716732ff5428958f"))

			var body =
			"{\"autoPay\":\"1\",\"keepShowAd\":1,\"chapterIds\":[\""+chapterId+"\"],\"bookId\":\""+bookId+"\"}";
			
			var header = {};

			header["uid"]="1706970565";
			header["pname"]="com.dzmf.zmfxsdq";
			header["sign"]=sign;
			header["signType"]="2";
			header["Content-Type"]="application/json; charset=UTF-8";
			//header["Content-Length"]=79;
			header["Host"]="xgmf.zuanqianyi.com";		
		}
		
		var respond= java.post(url,body,header);
		return respond;
	}
	
	var UrltypeNum = 1;
	
	var respond = getTransferRespond(UrltypeNum);

	var transferHtml=respond.body();

	var is = String(transferHtml).search(/不合法/);

	while(is > 0){
		if(UrltypeNum == 1){UrltypeNum = 2}else{UrltypeNum = 1}
		var respond = getTransferRespond(UrltypeNum);
		var transferHtml=respond.body();
		var is = String(transferHtml).search(/不合法/);
	}
	//java.log(transferHtml);
	var transferJson=JSON.parse(transferHtml);
	var chapterUrl =transferJson.data.chapterInfo[0].cdnUrls[0];
	chapterHtml = java.ajax(chapterUrl);
	//java.log(chapterUrl);
	//java.log(java.ajax(chapterUrl));
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1692)
` | source[0].ruleContent.content = @js:
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto.spec,
    Packages.javax.crypto,
    Packages.java.util
);
with(javaImport) {
    function decode(content) {
        var ivEncData = Base64.getDecoder().decode(String(content));
        var key = SecretKeySpec(String("242ccb8230d709e1").getBytes(), "AES");
        var iv = IvParameterSpec(Arrays.copyOfRange(ivEncData, 0, 16));
        var chipher = Cipher.getInstance("AES/CBC/PKCS5Padding");
        chipher.init(2, key, iv);
        return String(chipher.doFinal(Arrays.copyOfRange(ivEncData, 16, ivEncData.length)));
    }
}


sign_key='d3dGiJc651gSQ8w1'

params={'id':String(java.get('bid')),'chapterId':String(baseUrl.split("/").pop())}

var urlEncode = function (param, key, encode) {  
  if(param==null) return '';  
  var paramStr = '';  
  var t = typeof (param);  
  if (t == 'string' \|\| t == 'number' \|\| t == 'boolean') {  
    paramStr += '&' + key + '=' + ((encode==null\|\|encode) ? encodeURIComponent(param) : param);  
  } else {  
    for (var i in param) {  
      var k = key == null ? i : key + (param instanceof Array ? '[' + i + ']' : '.' + i);  
      paramStr += urlEncode(param[i], k, encode);  
    }
  }
  return paramStr;
};

paramSign=String(java.md5Encode(Object.keys(params).sort().reduce((pre,n)=>pre+n+'='+params[n],'')+sign_key))
params['sign']=paramSign
url="https://api-ks.wtzw.com/api/v1/chapter/content?"+urlEncode(params)
decode(JSON.parse(java.ajax(url+','+java.get("headers"))).data.content)

<br>source[162].ruleContent.content = @js:
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto.spec,
    Packages.javax.crypto,
    Packages.java.util
);
with(javaImport) {
    function decode(content) {
        var ivEncData = Base64.getDecoder().decode(String(content));
        var key = SecretKeySpec(String("242ccb8230d709e1").getBytes(), "AES");
        var iv = IvParameterSpec(Arrays.copyOfRange(ivEncData, 0, 16));
        var chipher = Cipher.getInstance("AES/CBC/PKCS5Padding");
        chipher.init(2, key, iv);
        return String(chipher.doFinal(Arrays.copyOfRange(ivEncData, 16, ivEncData.length)));
    }
}


sign_key='d3dGiJc651gSQ8w1'

params={'id':String(java.get('bid')),'chapterId':String(baseUrl.split("/").pop())}

var urlEncode = function (param, key, encode) {  
  if(param==null) return '';  
  var paramStr = '';  
  var t = typeof (param);  
  if (t == 'string' \|\| t == 'number' \|\| t == 'boolean') {  
    paramStr += '&' + key + '=' + ((encode==null\|\|encode) ? encodeURIComponent(param) : param);  
  } else {  
    for (var i in param) {  
      var k = key == null ? i : key + (param instanceof Array ? '[' + i + ']' : '.' + i);  
      paramStr += urlEncode(param[i], k, encode);  
    }
  }
  return paramStr;
};

paramSign=String(java.md5Encode(Object.keys(params).sort().reduce((pre,n)=>pre+n+'='+params[n],'')+sign_key))
params['sign']=paramSign
url="https://api-ks.wtzw.com/api/v1/chapter/content?"+urlEncode(params)
decode(JSON.parse(java.ajax(url+','+java.get("headers"))).data.content)

<br>source[4].ruleContent.content = @js:
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto.spec,
    Packages.javax.crypto,
    Packages.java.util
);
with(javaImport) {
    function decode(content) {
        var ivEncData = Base64.getDecoder().decode(String(content));
        var key = SecretKeySpec(String("242ccb8230d709e1").getBytes(), "AES");
        var iv = IvParameterSpec(Arrays.copyOfRange(ivEncData, 0, 16));
        var chipher = Cipher.getInstance("AES/CBC/PKCS5Padding");
        chipher.init(2, key, iv);
        return String(chipher.doFinal(Arrays.copyOfRange(ivEncData, 16, ivEncData.length)));
    }
}


sign_key='d3dGiJc651gSQ8w1'

params={'id':String(java.get('bid')),'chapterId':String(baseUrl.split("/").pop())}

var urlEncode = function (param, key, encode) {  
  if(param==null) return '';  
  var paramStr = '';  
  var t = typeof (param);  
  if (t == 'string' \|\| t == 'number' \|\| t == 'boolean') {  
    paramStr += '&' + key + '=' + ((encode==null\|\|encode) ? encodeURIComponent(param) : param);  
  } else {  
    for (var i in param) {  
      var k = key == null ? i : key + (param instanceof Array ? '[' + i + ']' : '.' + i);  
      paramStr += urlEncode(param[i], k, encode);  
    }
  }
  return paramStr;
};

paramSign=String(java.md5Encode(Object.keys(params).sort().reduce((pre,n)=>pre+n+'='+params[n],'')+sign_key))
params['sign']=paramSign
url="https://api-ks.wtzw.com/api/v1/chapter/content?"+urlEncode(params)
decode(JSON.parse(java.ajax(url+','+java.get("headers"))).data.content)

 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:5627)
` | source[500].ruleExplore.bookUrl = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security,
    Packages.java.security.interfaces,
    Packages.java.security.spec,
    Packages.java.io,
    Packages.java.util
);
with(javaImport){
    function encrypt(str){
        var bArr = String(str).getBytes("UTF-8");
        var mac = Mac.getInstance("HmacSHA256");
        mac .init(SecretKeySpec(String("wj3imab73kwceuf51lf01ORHe2cmo8X0YrZwF4p2uv3WEfmqxrT2oIBwRFRNErXW20UKal15ZTDdxPKU43puZFqcuXkvrQmadhp1wn6YPEDO4WRgInp8NNNQo4uNsWF1CELOzx7yPOS4pQbhTRWB4qRm0a4ENIegN0SH7K2STbsyaCuWh7m7s3rTpb5dK3CcDdsT35vo0xNbPZI2dJmKoIeKk9p3YhBLNWp9WoZqn9Qihpvn4nvgJzVSacgy2MTo").getBytes("UTF-8"),"HmacSHA256"));
        return mac.doFinal(bArr)
    }
  function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }
}

var bookId= java.getString("bookId");

var myDate = new Date();
var timestamp = myDate.getFullYear() + 
"0" + (myDate.getMonth()+1) + 
"0" + myDate.getDate() + 
"0" + myDate.getDay() + 
myDate.getMinutes() + 
myDate.getSeconds();

var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"bookId\":\""+bookId+"\"}8cfaaedef7a7e24a716732ff5428958f"))

url = "https://xgmf.zuanqianyi.com/glory/free/1110?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp + ",";
post={
  "method": "POST",
  "headers": {
    "pname": "com.dzmf.zmfxsdq",
    "sign": sign,
    "signType": "1"
  },
  "body":'{"bookId":"'+bookId+'"}'
}
url+JSON.stringify(post)
</js><br>source[656].ruleExplore.bookUrl = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security,
    Packages.java.security.interfaces,
    Packages.java.security.spec,
    Packages.java.io,
    Packages.java.util
);
with(javaImport){
    function encrypt(str){
        var bArr = String(str).getBytes("UTF-8");
        var mac = Mac.getInstance("HmacSHA256");
        mac .init(SecretKeySpec(String("wj3imab73kwceuf51lf01ORHe2cmo8X0YrZwF4p2uv3WEfmqxrT2oIBwRFRNErXW20UKal15ZTDdxPKU43puZFqcuXkvrQmadhp1wn6YPEDO4WRgInp8NNNQo4uNsWF1CELOzx7yPOS4pQbhTRWB4qRm0a4ENIegN0SH7K2STbsyaCuWh7m7s3rTpb5dK3CcDdsT35vo0xNbPZI2dJmKoIeKk9p3YhBLNWp9WoZqn9Qihpvn4nvgJzVSacgy2MTo").getBytes("UTF-8"),"HmacSHA256"));
        return mac.doFinal(bArr)
    }
  function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }
}

var bookId= java.getString("bookId");

var myDate = new Date();
var timestamp = myDate.getFullYear() + 
"0" + (myDate.getMonth()+1) + 
"0" + myDate.getDate() + 
"0" + myDate.getDay() + 
myDate.getMinutes() + 
myDate.getSeconds();

var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"bookId\":\""+bookId+"\"}8cfaaedef7a7e24a716732ff5428958f"))

url = "https://xgmf.zuanqianyi.com/glory/free/1110?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp + ",";
post={
  "method": "POST",
  "headers": {
    "pname": "com.dzmf.zmfxsdq",
    "sign": sign,
    "signType": "1"
  },
  "body":'{"bookId":"'+bookId+'"}'
}
url+JSON.stringify(post)
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:5629)
` | source[310].ruleSearch.bookUrl = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security,
    Packages.java.security.interfaces,
    Packages.java.security.spec,
    Packages.java.io,
    Packages.java.util
);
with(javaImport){
    function encrypt(str){
        var bArr = String(str).getBytes("UTF-8");
        var mac = Mac.getInstance("HmacSHA256");
        mac .init(SecretKeySpec(String("wj3imab73kwceuf51lf01ORHe2cmo8X0YrZwF4p2uv3WEfmqxrT2oIBwRFRNErXW20UKal15ZTDdxPKU43puZFqcuXkvrQmadhp1wn6YPEDO4WRgInp8NNNQo4uNsWF1CELOzx7yPOS4pQbhTRWB4qRm0a4ENIegN0SH7K2STbsyaCuWh7m7s3rTpb5dK3CcDdsT35vo0xNbPZI2dJmKoIeKk9p3YhBLNWp9WoZqn9Qihpvn4nvgJzVSacgy2MTo").getBytes("UTF-8"),"HmacSHA256"));
        return mac.doFinal(bArr)
    }
  function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }

}

var bookId= java.getString("bookId");

var myDate = new Date();
var timestamp = myDate.getFullYear() + 
"0" + (myDate.getMonth()+1) + 
"0" + myDate.getDate() + 
"0" + myDate.getDay() + 
myDate.getMinutes() + 
myDate.getSeconds();

var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"bookId\":\""+bookId+"\"}8cfaaedef7a7e24a716732ff5428958f"))

url = "https://xgmf.zuanqianyi.com/glory/free/1110?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp + ",";
post={
  "method": "POST",
  "headers": {
    "pname": "com.dzmf.zmfxsdq",
    "sign": sign,
    "signType": "1"
  },
  "body":'{"bookId":"'+bookId+'"}'
}
url+JSON.stringify(post)
</js><br>source[500].ruleSearch.bookUrl = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security,
    Packages.java.security.interfaces,
    Packages.java.security.spec,
    Packages.java.io,
    Packages.java.util
);
with(javaImport){
    function encrypt(str){
        var bArr = String(str).getBytes("UTF-8");
        var mac = Mac.getInstance("HmacSHA256");
        mac .init(SecretKeySpec(String("wj3imab73kwceuf51lf01ORHe2cmo8X0YrZwF4p2uv3WEfmqxrT2oIBwRFRNErXW20UKal15ZTDdxPKU43puZFqcuXkvrQmadhp1wn6YPEDO4WRgInp8NNNQo4uNsWF1CELOzx7yPOS4pQbhTRWB4qRm0a4ENIegN0SH7K2STbsyaCuWh7m7s3rTpb5dK3CcDdsT35vo0xNbPZI2dJmKoIeKk9p3YhBLNWp9WoZqn9Qihpvn4nvgJzVSacgy2MTo").getBytes("UTF-8"),"HmacSHA256"));
        return mac.doFinal(bArr)
    }
  function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }

}

var bookId= java.getString("bookId");

var myDate = new Date();
var timestamp = myDate.getFullYear() + 
"0" + (myDate.getMonth()+1) + 
"0" + myDate.getDate() + 
"0" + myDate.getDay() + 
myDate.getMinutes() + 
myDate.getSeconds();

var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"bookId\":\""+bookId+"\"}8cfaaedef7a7e24a716732ff5428958f"))

url = "https://xgmf.zuanqianyi.com/glory/free/1110?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp + ",";
post={
  "method": "POST",
  "headers": {
    "pname": "com.dzmf.zmfxsdq",
    "sign": sign,
    "signType": "1"
  },
  "body":'{"bookId":"'+bookId+'"}'
}
url+JSON.stringify(post)
</js><br>source[656].ruleSearch.bookUrl = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security,
    Packages.java.security.interfaces,
    Packages.java.security.spec,
    Packages.java.io,
    Packages.java.util
);
with(javaImport){
    function encrypt(str){
        var bArr = String(str).getBytes("UTF-8");
        var mac = Mac.getInstance("HmacSHA256");
        mac .init(SecretKeySpec(String("wj3imab73kwceuf51lf01ORHe2cmo8X0YrZwF4p2uv3WEfmqxrT2oIBwRFRNErXW20UKal15ZTDdxPKU43puZFqcuXkvrQmadhp1wn6YPEDO4WRgInp8NNNQo4uNsWF1CELOzx7yPOS4pQbhTRWB4qRm0a4ENIegN0SH7K2STbsyaCuWh7m7s3rTpb5dK3CcDdsT35vo0xNbPZI2dJmKoIeKk9p3YhBLNWp9WoZqn9Qihpvn4nvgJzVSacgy2MTo").getBytes("UTF-8"),"HmacSHA256"));
        return mac.doFinal(bArr)
    }
  function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }

}

var bookId= java.getString("bookId");

var myDate = new Date();
var timestamp = myDate.getFullYear() + 
"0" + (myDate.getMonth()+1) + 
"0" + myDate.getDate() + 
"0" + myDate.getDay() + 
myDate.getMinutes() + 
myDate.getSeconds();

var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"bookId\":\""+bookId+"\"}8cfaaedef7a7e24a716732ff5428958f"))

url = "https://xgmf.zuanqianyi.com/glory/free/1110?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp + ",";
post={
  "method": "POST",
  "headers": {
    "pname": "com.dzmf.zmfxsdq",
    "sign": sign,
    "signType": "1"
  },
  "body":'{"bookId":"'+bookId+'"}'
}
url+JSON.stringify(post)
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:5866)
` | source[310].ruleBookInfo.tocUrl = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security,
    Packages.java.security.interfaces,
    Packages.java.security.spec,
    Packages.java.io,
    Packages.java.util
);
with(javaImport){
    function encrypt(str){
        var bArr = String(str).getBytes("UTF-8");
        var mac = Mac.getInstance("HmacSHA256");
        mac .init(SecretKeySpec(String("wj3imab73kwceuf51lf01ORHe2cmo8X0YrZwF4p2uv3WEfmqxrT2oIBwRFRNErXW20UKal15ZTDdxPKU43puZFqcuXkvrQmadhp1wn6YPEDO4WRgInp8NNNQo4uNsWF1CELOzx7yPOS4pQbhTRWB4qRm0a4ENIegN0SH7K2STbsyaCuWh7m7s3rTpb5dK3CcDdsT35vo0xNbPZI2dJmKoIeKk9p3YhBLNWp9WoZqn9Qihpvn4nvgJzVSacgy2MTo").getBytes("UTF-8"),"HmacSHA256"));
        return mac.doFinal(bArr)
    }
  function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }

}

var bookId= java.getString("data.book.bookId");

var myDate = new Date();
var timestamp = myDate.getFullYear() + 
"0" + (myDate.getMonth()+1) + 
"0" + myDate.getDate() + 
"0" + myDate.getDay() + 
myDate.getMinutes() + 
myDate.getSeconds();



var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"chapterId\":\"\",\"needBlockList\":\"1\",\"chapterOffset\":\"666666\",\"bookId\":\"" + bookId + "\",\"chapterEndId\":\"\"}8cfaaedef7a7e24a716732ff5428958f"))

url = "https://xgmf.zuanqianyi.com/glory/free/111?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp + ",";
post={
  "method": "POST",
  "headers": {
    "pname": "com.dzmf.zmfxsdq",
    "sign": sign,
    "signType": "1"
  },
  "body":'{"chapterId":"","needBlockList":"1","chapterOffset":"666666","bookId":"'+bookId+'","chapterEndId":""}'
}
url+JSON.stringify(post)
</js><br>source[500].ruleBookInfo.tocUrl = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security,
    Packages.java.security.interfaces,
    Packages.java.security.spec,
    Packages.java.io,
    Packages.java.util
);
with(javaImport){
    function encrypt(str){
        var bArr = String(str).getBytes("UTF-8");
        var mac = Mac.getInstance("HmacSHA256");
        mac .init(SecretKeySpec(String("wj3imab73kwceuf51lf01ORHe2cmo8X0YrZwF4p2uv3WEfmqxrT2oIBwRFRNErXW20UKal15ZTDdxPKU43puZFqcuXkvrQmadhp1wn6YPEDO4WRgInp8NNNQo4uNsWF1CELOzx7yPOS4pQbhTRWB4qRm0a4ENIegN0SH7K2STbsyaCuWh7m7s3rTpb5dK3CcDdsT35vo0xNbPZI2dJmKoIeKk9p3YhBLNWp9WoZqn9Qihpvn4nvgJzVSacgy2MTo").getBytes("UTF-8"),"HmacSHA256"));
        return mac.doFinal(bArr)
    }
  function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }

}

var bookId= java.getString("data.book.bookId");

var myDate = new Date();
var timestamp = myDate.getFullYear() + 
"0" + (myDate.getMonth()+1) + 
"0" + myDate.getDate() + 
"0" + myDate.getDay() + 
myDate.getMinutes() + 
myDate.getSeconds();



var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"chapterId\":\"\",\"needBlockList\":\"1\",\"chapterOffset\":\"666666\",\"bookId\":\"" + bookId + "\",\"chapterEndId\":\"\"}8cfaaedef7a7e24a716732ff5428958f"))

url = "https://xgmf.zuanqianyi.com/glory/free/111?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp + ",";
post={
  "method": "POST",
  "headers": {
    "pname": "com.dzmf.zmfxsdq",
    "sign": sign,
    "signType": "1"
  },
  "body":'{"chapterId":"","needBlockList":"1","chapterOffset":"666666","bookId":"'+bookId+'","chapterEndId":""}'
}
url+JSON.stringify(post)
</js><br>source[656].ruleBookInfo.tocUrl = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security,
    Packages.java.security.interfaces,
    Packages.java.security.spec,
    Packages.java.io,
    Packages.java.util
);
with(javaImport){
    function encrypt(str){
        var bArr = String(str).getBytes("UTF-8");
        var mac = Mac.getInstance("HmacSHA256");
        mac .init(SecretKeySpec(String("wj3imab73kwceuf51lf01ORHe2cmo8X0YrZwF4p2uv3WEfmqxrT2oIBwRFRNErXW20UKal15ZTDdxPKU43puZFqcuXkvrQmadhp1wn6YPEDO4WRgInp8NNNQo4uNsWF1CELOzx7yPOS4pQbhTRWB4qRm0a4ENIegN0SH7K2STbsyaCuWh7m7s3rTpb5dK3CcDdsT35vo0xNbPZI2dJmKoIeKk9p3YhBLNWp9WoZqn9Qihpvn4nvgJzVSacgy2MTo").getBytes("UTF-8"),"HmacSHA256"));
        return mac.doFinal(bArr)
    }
  function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }

}

var bookId= java.getString("data.book.bookId");

var myDate = new Date();
var timestamp = myDate.getFullYear() + 
"0" + (myDate.getMonth()+1) + 
"0" + myDate.getDate() + 
"0" + myDate.getDay() + 
myDate.getMinutes() + 
myDate.getSeconds();



var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"chapterId\":\"\",\"needBlockList\":\"1\",\"chapterOffset\":\"666666\",\"bookId\":\"" + bookId + "\",\"chapterEndId\":\"\"}8cfaaedef7a7e24a716732ff5428958f"))

url = "https://xgmf.zuanqianyi.com/glory/free/111?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp + ",";
post={
  "method": "POST",
  "headers": {
    "pname": "com.dzmf.zmfxsdq",
    "sign": sign,
    "signType": "1"
  },
  "body":'{"chapterId":"","needBlockList":"1","chapterOffset":"666666","bookId":"'+bookId+'","chapterEndId":""}'
}
url+JSON.stringify(post)
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:6236)
` | source[500].ruleExplore.bookList = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security,
    Packages.java.security.interfaces,
    Packages.java.security.spec,
    Packages.java.io,
    Packages.java.util
);
with(javaImport){
    function encrypt(str){
        var bArr = String(str).getBytes("UTF-8");
        var mac = Mac.getInstance("HmacSHA256");
        mac .init(SecretKeySpec(String("wj3imab73kwceuf51lf01ORHe2cmo8X0YrZwF4p2uv3WEfmqxrT2oIBwRFRNErXW20UKal15ZTDdxPKU43puZFqcuXkvrQmadhp1wn6YPEDO4WRgInp8NNNQo4uNsWF1CELOzx7yPOS4pQbhTRWB4qRm0a4ENIegN0SH7K2STbsyaCuWh7m7s3rTpb5dK3CcDdsT35vo0xNbPZI2dJmKoIeKk9p3YhBLNWp9WoZqn9Qihpvn4nvgJzVSacgy2MTo").getBytes("UTF-8"),"HmacSHA256"));
        return mac.doFinal(bArr)
    }
  function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }

}

var myDate = new Date();
var timestamp = myDate.getFullYear() + 
"0" + (myDate.getMonth()+1) + 
"0" + myDate.getDate() + 
"0" + myDate.getDay() + 
myDate.getMinutes() + 
myDate.getSeconds();

page = java.get("page")
cid = baseUrl.match(/cid%3D(\d+)/)[1];
tid = /tid=/.test(baseUrl)?baseUrl.match(/tid=(\d+)/)[1]:"";


function GetURL(cid,tid){

	var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"flag\":\"1\",\"size\":\"15\",\"index\":\""+page+"\",\"sort\":\"1\",\"tid\":\""+tid+"\",\"status\":\"\",\"cid\":\""+cid+"\"}8cfaaedef7a7e24a716732ff5428958f"))

	url = "https://xgmf.zuanqianyi.com/glory/free/1166?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp + ",";
	post={
	  "method": "POST",
	  "headers": {
		"uid": "1706970565",
		"pname": "com.dzmf.zmfxsdq",
		"sign": sign,
		"signType": "1",
		"Content-Type": "application/json; charset=UTF-8",
		"Host": "xgmf.zuanqianyi.com",
	  },
	  "body":JSON.stringify({"flag":"1","size":"15","index":String(page),"sort":"1","tid":String(tid),"status":"","cid":String(cid)})
	}
	return url+JSON.stringify(post);
}
url = GetURL(cid,tid);
java.ajax(url)
</js>
$.data.bookList<br>source[656].ruleExplore.bookList = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.javax.crypto,
    Packages.javax.crypto.spec,
    Packages.java.security,
    Packages.java.security.interfaces,
    Packages.java.security.spec,
    Packages.java.io,
    Packages.java.util
);
with(javaImport){
    function encrypt(str){
        var bArr = String(str).getBytes("UTF-8");
        var mac = Mac.getInstance("HmacSHA256");
        mac .init(SecretKeySpec(String("wj3imab73kwceuf51lf01ORHe2cmo8X0YrZwF4p2uv3WEfmqxrT2oIBwRFRNErXW20UKal15ZTDdxPKU43puZFqcuXkvrQmadhp1wn6YPEDO4WRgInp8NNNQo4uNsWF1CELOzx7yPOS4pQbhTRWB4qRm0a4ENIegN0SH7K2STbsyaCuWh7m7s3rTpb5dK3CcDdsT35vo0xNbPZI2dJmKoIeKk9p3YhBLNWp9WoZqn9Qihpvn4nvgJzVSacgy2MTo").getBytes("UTF-8"),"HmacSHA256"));
        return mac.doFinal(bArr)
    }
  function encode(bArr) {
        if (bArr === null) {
            return null;
        }
        var length = bArr.length * 8;
        if (length === 0) {
            return "";
        }
        var i10 = length % 24;
        var i11 = parseInt(length / 24);
        var cArr = new Array((i10 !== 0 ? i11 + 1 : i11) * 4);
        var i12 = 0;
        var i13 = 0;
        var i14 = 0;
        while (i12 < i11) {
                var i15 = i13 + 1;
                var b10 = bArr[i13];
                var i16 = i15 + 1;
                var b11 = bArr[i15];
                var i17 = i16 + 1;
                var b12 = bArr[i16];
                var b13 = (b11 & 15)&255;
                var b14 = (b10 & 3)&255;
                var i18 = b10 & -128;
                var i19 = b10 >> 2;
                if (i18 !== 0) {
                    i19 ^= 192;
                }
                var b15 = i19 & 255;
                var i20 = b11 & -128;
                var i21 = b11 >> 4;
                if (i20 !== 0) {
                    i21 ^= 240;
                }
                var b16 = (i21 & 255);
                var i22 = (b12 & -128) === 0 ? b12 >> 6 : (b12 >> 6) ^ 252;
                var i23 = i14 + 1;
                var cArr2 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
                cArr[i14] = cArr2[b15];
                var i24 = i23 + 1;
                cArr[i23] = cArr2[(b14 << 4) \| b16];
                var i25 = i24 + 1;
                cArr[i24] = cArr2[(b13 << 2) \| (i22&255)];
                cArr[i25] = cArr2[b12 & 63];
                i12++;
                i14 = i25 + 1;
                i13 = i17;
				}
        if (i10 === 8) {
            var b17 = bArr[i13];
            var b18 = b17 & 3;
            var i26 = b17 & -128;
            var i27 = b17 >> 2;
            if (i26 !== 0) {
                i27 ^= 192;
            }
            var i28 = i14 + 1;
            var cArr3 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr3[i27];
            var i29 = i28 + 1;
            cArr[i28] = cArr3[b18 << 4];
            cArr[i29] = '=';
            cArr[i29 + 1] = '=';
		}else if (i10 === 16) {
            var b19 = bArr[i13];
            var b20 = bArr[i13 + 1];
            var b21 = (b20 & 15)&255;
            var b22 = (b19 & 3)&255;
            var i30 = b19 & -128;
            var i31 = b19 >> 2;
            if (i30 !== 0) {
                i31 ^= 192;
            }
            var b23 = i31&255;
            var i32 = b20 & -128;
            var i33 = b20 >> 4;
            if (i32 !== 0) {
                i33 ^= 240;
            }
            var i34 = i14 + 1;
            var cArr4 = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
            cArr[i14] = cArr4[b23];
            var i35 = i34 + 1;
            cArr[i34] = cArr4[(i33&255) \| (b22 << 4)];
            cArr[i35] = cArr4[b21 << 2];
            cArr[i35 + 1] = '=';
        }
        return cArr.join('');
    }

}

var myDate = new Date();
var timestamp = myDate.getFullYear() + 
"0" + (myDate.getMonth()+1) + 
"0" + myDate.getDate() + 
"0" + myDate.getDay() + 
myDate.getMinutes() + 
myDate.getSeconds();

page = java.get("page")
cid = baseUrl.match(/cid%3D(\d+)/)[1];
tid = /tid=/.test(baseUrl)?baseUrl.match(/tid=(\d+)/)[1]:"";


function GetURL(cid,tid){

	var sign = encode(encrypt("appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp="+ timestamp +"{\"flag\":\"1\",\"size\":\"15\",\"index\":\""+page+"\",\"sort\":\"1\",\"tid\":\""+tid+"\",\"status\":\"\",\"cid\":\""+cid+"\"}8cfaaedef7a7e24a716732ff5428958f"))

	url = "https://xgmf.zuanqianyi.com/glory/free/1166?appId=100398183&country=CN&lang=zh_CN&ver=10009248&appVer=1.0.9.248&timestamp=" + timestamp + ",";
	post={
	  "method": "POST",
	  "headers": {
		"uid": "1706970565",
		"pname": "com.dzmf.zmfxsdq",
		"sign": sign,
		"signType": "1",
		"Content-Type": "application/json; charset=UTF-8",
		"Host": "xgmf.zuanqianyi.com",
	  },
	  "body":JSON.stringify({"flag":"1","size":"15","index":String(page),"sort":"1","tid":String(tid),"status":"","cid":String(cid)})
	}
	return url+JSON.stringify(post);
}
url = GetURL(cid,tid);
java.ajax(url)
</js>
$.data.bookList |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: JavaImporter is not defined
    at <eval> (<input>:2:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:7207)
` | source[518].ruleContent.content = <js>
var javaImport = new JavaImporter();
javaImport.importPackage(
    Packages.java.lang,
    Packages.java.security,
    Packages.java.security.spec,
    Packages.java.io,
    Packages.java.util
);
with(javaImport){
    function encode(str){
    	//该方法由💖尐哖💖提供
jmkey=[48,-126,4,-65,2,1,0,48,13,6,9,42,-122,72,-122,-9,13,1,1,1,5,0,4,-126,4,-87,48,-126,4,-91,2,1,0,2,-126,1,1,0,-118,121,-93,-123,61,113,-87,-20,-33,52,-70,-125,-13,67,-61,92,-49,-41,-14,60,-49,31,-68,-88,-7,-38,-52,-62,-31,-76,-123,90,-28,-70,-23,-106,93,-93,-5,-57,22,62,51,45,-88,80,-38,-55,75,-3,-73,-96,65,-15,-110,11,-121,-91,105,-91,-11,-62,-111,96,17,-68,-82,113,-116,-82,-108,51,-100,31,61,14,-26,-5,-48,14,-36,-110,-90,113,46,-31,-67,-64,95,98,95,-123,2,-83,16,-105,74,-42,-100,3,-125,53,126,-72,-72,-76,79,108,1,-71,-82,-51,0,60,-94,84,-65,73,-44,-88,-107,-81,44,3,40,-33,-87,114,88,21,-53,111,-84,-88,-83,112,41,101,-118,50,22,-74,29,47,-51,-106,-89,-26,55,44,66,118,-7,-45,-72,78,-125,-37,73,72,-80,-11,38,-82,65,54,103,9,43,-106,-88,-81,29,50,56,-127,104,122,56,-38,-93,-76,-12,-99,-17,-87,97,-128,21,118,-78,-13,-67,15,-74,-116,71,63,122,5,-59,120,93,90,-104,116,21,-13,108,10,-50,-103,-33,-21,125,113,86,-73,46,-84,-92,106,-86,8,23,7,95,-65,80,83,6,-85,53,85,-85,43,-73,41,-84,-10,76,106,-36,119,99,-125,121,-92,-122,-94,-73,-15,125,115,120,-63,2,3,1,0,1,2,-126,1,1,0,-121,-55,-103,-48,-55,28,-47,125,102,-81,37,10,-55,28,52,-47,-87,58,95,-34,-61,88,-94,-66,-84,43,-93,72,-94,-35,75,59,-125,57,-54,94,-39,-70,56,-27,-45,-24,-16,116,-96,45,-111,45,125,103,-15,-115,-93,-68,-121,-14,-24,116,10,-14,99,-39,4,-121,73,61,85,110,33,126,-5,-14,-45,-16,74,6,119,-8,-117,-32,86,-23,51,111,-97,-126,91,120,-19,-49,-32,76,-28,-1,-30,90,9,88,3,42,-22,-102,37,-122,108,16,-36,36,-44,88,73,-111,-38,-34,-102,108,50,27,-22,-3,-39,-78,41,-99,123,-110,63,108,85,22,-119,93,-85,-98,-114,70,-31,-120,-122,10,-92,-31,87,117,-119,-49,25,1,20,-42,-61,35,96,-41,-46,-50,-114,-31,-36,-21,95,-70,113,110,62,-28,62,76,36,-57,81,-21,100,36,124,-74,112,-52,-73,109,-117,37,-3,-40,-111,-102,-13,62,118,-93,-119,-118,-33,-82,20,53,64,-57,24,63,-113,-126,-61,-69,90,-72,-56,120,80,-95,-14,124,-2,74,80,53,22,3,57,61,113,-117,-5,-54,61,-1,-105,-96,-68,7,-98,-49,95,-61,-16,51,-65,82,66,-125,-66,-92,115,75,-116,32,14,68,-51,124,-126,32,-63,52,84,117,2,-127,-127,0,-15,-52,16,-109,-8,94,1,-65,14,-119,72,18,115,86,17,19,-68,-19,24,86,12,5,-97,55,-16,-75,84,110,-126,23,-33,-56,120,17,-32,-127,-106,-54,-27,54,20,40,92,70,-18,94,31,-79,-42,69,74,18,85,17,-64,-80,60,4,-102,23,36,-52,-112,124,11,-30,-15,5,-92,125,51,-42,26,109,27,-19,93,10,49,-48,-106,91,-38,12,-45,73,-44,-26,37,114,21,105,36,-38,96,39,-5,-8,-4,-70,-11,106,-107,69,68,14,-43,-123,-127,6,127,65,-25,-49,75,110,99,-50,-32,-33,-54,-26,117,125,-18,24,-89,107,2,-127,-127,0,-110,-101,-24,103,-95,2,-73,-114,-109,-97,107,126,-32,75,-124,-39,-92,-98,-127,-33,2,-65,49,-80,24,118,94,4,-117,122,-70,-41,-42,59,-90,-53,-81,-101,-57,75,-99,-67,-87,-67,-35,40,-66,53,-45,95,79,-10,-120,-111,71,-12,-110,76,-74,82,-79,83,-84,3,43,111,-44,-109,4,61,-22,3,99,113,113,-121,-15,-108,44,-32,-28,46,-83,-115,-99,-86,108,-111,3,-11,-43,121,67,-73,-68,99,107,55,-119,-77,20,-68,-77,98,-113,-21,39,-128,-21,119,113,57,-99,-30,-65,-128,-76,-100,40,8,39,-72,18,9,-17,-99,-89,-125,2,-127,-128,108,110,19,93,23,-70,-88,83,-46,35,-13,-30,-6,63,-75,70,-63,-87,29,9,-79,56,112,46,-8,-51,-120,0,74,108,-124,88,-12,-89,39,-93,85,72,-59,66,-36,5,65,100,57,-114,-111,-18,0,-27,111,-109,10,-4,-4,8,-53,-47,80,124,98,111,45,-73,-62,-24,-47,38,-77,-99,-59,-70,20,125,-85,81,101,48,-90,40,32,-43,45,-46,36,-119,-18,100,10,-108,-65,79,56,76,-119,100,68,-43,98,24,64,-25,-69,-22,-92,-37,118,26,-7,66,61,-99,3,99,-19,50,-94,-91,106,40,81,103,-55,118,96,104,67,-29,2,-127,-127,0,-121,39,-59,-77,-85,34,119,23,-80,-115,-38,42,-120,25,-10,-86,49,-15,-110,102,-122,0,-66,-116,-55,-80,109,114,33,39,-114,-110,37,-60,-82,58,-66,116,-116,-32,-17,-43,-122,99,43,60,65,70,27,-53,-107,75,0,-111,118,85,72,126,1,-30,-17,-24,-29,-3,-76,16,-113,86,-51,37,74,-45,-66,-36,57,62,-118,-3,-1,-11,127,70,108,-26,-51,-1,-21,-64,48,119,116,74,43,-100,121,-58,-23,115,-76,-76,-20,28,29,-1,114,15,-26,70,26,76,-19,-117,-95,59,5,50,96,-50,72,-75,99,-16,116,104,-58,-122,127,-125,2,-127,-127,0,-105,-116,-61,-50,-11,-71,-17,-45,114,88,67,113,51,37,-77,53,82,-22,36,70,-19,34,93,22,-103,-36,28,-47,-113,120,-57,4,-95,11,-82,-62,-127,13,115,93,104,-110,-44,39,42,-125,64,-111,53,-14,82,73,69,0,-66,37,-56,-115,3,-103,5,-69,55,75,-113,7,-18,-32,97,-56,28,85,-48,72,27,-115,119,63,1,41,115,-27,-73,24,-63,57,-72,107,-9,39,13,120,-75,-42,33,9,-39,-93,72,-122,66,33,-36,-1,-18,-102,11,-49,28,-61,-120,6,102,-37,-106,-6,5,-111,107,-76,49,78,-40,19,53,106,-37,-125];
key=KeyFactory.getInstance("RSA").generatePrivate(new PKCS8EncodedKeySpec(jmkey));
        var bArr=String(str).getBytes("UTF-8");
        signature = Signature.getInstance("SHA256WithRSA");
        signature.initSign(key);
        signature.update(bArr);
       return encodes(signature.sign());
       }
       function encodes(bArr){
       	var bArr2=String("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/").getBytes("UTF_8");
       bArr3 = [];
        length = bArr.length - (bArr.length % 3);
        i = 0;
        i2 = 0;
        while (i < length) {     	
            i3 = i + 1;
            b2 = bArr[i];
            i4 = i3 + 1;
            b3 = bArr[i3];
            i = i4 + 1;
            b4 = bArr[i4];
            i5 = i2 + 1;
            bArr3[i2] = bArr2[(b2 & 255) >> 2];
            i6 = i5 + 1;
            bArr3[i5] = bArr2[((b2 & 3) << 4) \| ((b3 & 255) >> 4)];
            i7 = i6 + 1;
            bArr3[i6] = bArr2[((b3 & 15) << 2) \| ((b4 & 255) >> 6)];
            i2 = i7 + 1;
            bArr3[i7] = bArr2[b4 & 63];
        }
        length2 = bArr.length - length;
        if (length2 == 1) {
            b5 = bArr[i];
            i8 = i2 + 1;
            bArr3[i2] = bArr2[(b5 & 255) >> 2];
            i9 = i8 + 1;
            bArr3[i8] = bArr2[(b5 & 3) << 4];
            b6 = 61;
            bArr3[i9] = b6;
            bArr3[i9 + 1] = b6;
        } else if (length2 == 2) {
            i10 = i + 1;
            b7 = bArr[i];
            b8 = bArr[i10];
            i11 = i2 + 1;
            bArr3[i2] = bArr2[(b7 & 255) >> 2];
            i12 = i11 + 1;
            bArr3[i11] = bArr2[((b7 & 3) << 4) \| ((b8 & 255) >> 4)];
            bArr3[i12] = bArr2[(b8 & 15) << 2];
            bArr3[i12 + 1] = 61;
        }
        return new String(bArr3,"UTF_8");
     }
}

$ = JSON.parse(baseUrl.replace(/^.+?,/,''));

sign = encode("a"+$.body+"8cfaaedef7a7e24a716732ff5428958f");

$.headers.sign = sign;

chapter.url = baseUrl = baseUrl.replace(/,.+/, ','+JSON.stringify($));

result = java.ajax(baseUrl);
java.setContent(result, baseUrl);
result
</js>
$..cdnUrls[0]
@js:
'\n' + java.ajax(result); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cache is not defined
    at <eval> (<input>:6:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:8945)
` | source[217].ruleToc.chapterList = <js>
let b = String(book.bookUrl);
let bmid = S("$.data.mid\|\|$.data.list.mid\|\|$.data.item.basic.uid\|\|$.data.item.modules.module_author.mid\|\|$.data.owner.mid\|\|$.data.root.mid");

book.putVariable("mid", bmid);
cache.put("mid",bmid);

caid = String(cache.get("aid"));	cache.put("aid",S("$.result.main_section.episodes[0].aid"));

book.putVariable("id",S("$.data.item.basic.comment_id_str"));
getMode()
if (/web-interface\/view\|series\|season_id=/.test(b)) {
    if (/^🎧/.test(Map("模式"))) {
        book.type = 32;
    } else if (/^🎥/.test(Map("模式"))) {
        book.type = 4;
    }
} else if (/reply\/reply\|article/.test(b)) {
    book.type = 8;
}

if(/acc\/info/.test(b)){
	book.type = 4;
	}


if (/web-interface\/view/.test(baseUrl)) {
    var p = JSON.parse(result).data;
    book.putVariable("mid", p.owner.mid);
    var d = [];
    var aid = p.aid;
    
    if (p.pages.length == 1) {
        cid = p.pages[0].cid;
        d.push({
            title: p.title,
            url: `data:bilibili;base64,${java.base64Encode(aid+"&"+p.cid)},{"type":"c"}`,
            pubdate: p.pubdate,
            desc: formatTimeDynamic(p.duration)
        });
    } else if (p.pages.length > 1) {
        p.pages.forEach(item => {
            cid = item.cid;
            d.push({
                title: item.part===""?"无名":item.part,
                desc: formatTimeDynamic(item.duration),
                url: `data:bilibili;base64,${java.base64Encode(aid+"&"+item.cid)},{"type":"c"}`,
                pubdate: item.pubdate,

            })
        })
    }
    if (p.ugc_season) {
        d.unshift({
            title: "当前视频",
            vol: 1
        });

        let s = p.ugc_season.sections;
                    if(/倒序/.test(M("合集"))){
                          s= s.reverse();
                    }
                    d.push({
                        title:"视频合集",
                        vol:1
                    });
                      
                    
        s.forEach(el=> {
            if(s.length>1){
            d.push({
                        title: "🎥 "+ el.title+"【"+el.episodes.length+"个视频】",
                        url:`data:bilibili;base64,${java.base64Encode("🎥"+el.title)},{"type":"el"}`
                    });
               }
                    ep = el.episodes;
                    if(/倒序/.test(M("合集"))){
                          ep = ep.reverse();
                    }
               ep.forEach((x,i) => {
                     if(x.bvid === p.bvid){
                        d[0].title = d[0].title+ "  "+(i+1)+"/" +el.episodes.length+"【"+el.title+"】"+x.title
                    }
                if (x.pages.length == 1) {
                    d.push({
                        title:(i+1)+"🏷 "+ x.title+" ",
                        url: `data:bilibili;base64,${java.base64Encode(x.aid + "&" + (x.pages[0] ? x.pages[0].cid : x.cid))},{"type":""}`,
                        pubdate: x.arc.pubdate,
                        desc: formatTimeDynamic(x.arc.duration)
                    });
                }

                if (x.pages.length > 1) {
                     d.push({
                        title: (i+1)+" 📖 "+ x.title+"【"+x.pages.length+"P】",
                        url:`data:bilibili;base64,${java.base64Encode("🎥"+x.title)},{"type":"el"}`
                    });
                    x.pages.forEach((y,j)=> {
                        d.push({
                            title: (i+1)+"-" +(j+1)+"🏷 "+y.part,
                            url: `data:bilibili;base64,${java.base64Encode(x.aid + "&" + y.cid)},{"type":""}`,
                            pubdate: y?.arc?.pubdate ?? x?.arc?.pubdate ??"",
                            desc: formatTimeDynamic(y?.arc.duration ?? y.duration) + " "+x.title
                        });
                    });
                }
            });
        });
    }
    result = JSON.stringify(d)
} else if (/reply\/reply/.test(baseUrl)) {
    if(JSON.parse(src)?.data?.root?.mid){
        book.putVariable("mid", JSON.parse(src)?.data?.root?.mid);
    }
} else if (/x\/article\/view/.test(baseUrl)) {
    book.putVariable("mid", S("$.data.author.mid"));
    result = JSON.stringify([{
        "title": S("$.data.title"),
        "url": getApi("article",S("$.data.id"),S("$.data.dyn_id_str"))
    }])
} else if (/acc\/info/.test(baseUrl)) {
    let name = S("$.data.name");
    let mid = S("$.data.mid");
    let d = [];
    let roomid = S("$.data.live_room.roomid");
    
    let liveStatus = S("$.data.live_room.liveStatus");
    if (liveStatus == "1") {
        liveStatus = "🔴";
        java.toast("【" + name + "】正在直播");
        java.log("【" + name + "】正在直播");
    } else if (liveStatus == "0") {
        liveStatus = "⚪️"
    }
    if (S("$.data.live_room.roundStatus") == "1") {
        liveStatus = "🟢"
    }
    let title = S("$.data.live_room.title");
    
    if (/🔴\|🟢/.test(liveStatus)) {
        result = java.ajax("https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id="+roomid+"&protocol=0,1&format=0,1,2&codec=0,1&qn=30000");
        let data = JSON.parse(result).data;
        let stream = data.playurl_info?.playurl?.stream;
        
        if(stream){
            let codec = stream[0].format[0].codec[0];
            let base_url = codec.base_url;
            let accept_qn = codec.accept_qn;
            accept_qn.forEach(code=>{
                d.push({
                    title:getVideoQuality(code),
                    vol:true
                });
               
                    codec.url_info.forEach((u,i)=>{
                         if(codec.current_qn==code){
                             let host = u.host;
                             let extra = u.extra;
                             d.push({
                                 title: (liveStatus+getVideoQuality(code)+(i+1) + "、【" + title + "】"  + " " + name + "的直播间").replace(/【】/, ""),
                                 url: `data:liveurl;base64,${java.base64Encode(host+base_url+extra+"❌"+roomid)},{type:"live"}`
                             })
                         }else{
                             d.push({
                                 title: (liveStatus+getVideoQuality(code)+(i+1) + "、【" + title + "】"  + " " + name + "的直播间").replace(/【】/, ""),
                                 url: `data:liveurl;base64,${java.base64Encode(i+"&"+code+"❌"+roomid)},{type:"live"}`
                             })
                         }
                  })
              })
             
          }else if(data.playurl_info == null){
              d.push({
                  title: (liveStatus+"直播已结束【" + title + "】"  + " " + name + "的直播间").replace(/【】/, ""),
                  url: `data:liveurl;base64,${java.base64Encode("❌"+roomid)},{type:"live"}`
        })
          }
      }else {
        d.push({
            title: (liveStatus+"【" + title + "】" + " " + name + "的直播间").replace(/【】/, ""),
            url: `data:liveurl;base64,${java.base64Encode("❌"+roomid)},{type:"live"}`
        })
    }
    
   
   /*
  //  直接获取视频流
    if (/中/.test(liveStatus)) {
        let liveApi = "https://api.live.bilibili.com/room/v1/Room/playUrl?qn=30000&cid=" + roomid;
        let data = java.ajax(liveApi);
        let durl = JSON.parse(data).data.durl;
        durl.forEach((x,i) => {
        	java.log(durl.length-1-i)
            d.push({
                title: "线路" + x.order,
                vol: true
            }, {
                title: (x.order + "、【" + title + "】" + liveStatus + " " + name + "的直播间").replace(/【】/, ""),
                url: `data:liveurl;base64,${java.base64Encode(durl[durl.length-1-i].url)},{type:"live"}`
            })
        })
    } else {
        d.push({
            title: ("【" + title + "】" + liveStatus + " " + name + "的直播间").replace(/【】/, ""),
            url: `data:liveurl;base64,${java.base64Encode(roomid+"&"+mid)},{type:"live"}`
        })
    }
    */
   
    result = JSON.stringify(d)
} else if (/data:dynamic_type\|opus\/detail\|v1\/detail/.test(baseUrl)) {
    
    result = JSON.stringify([{
        title:String(book.name),
        url:baseUrl
    }])
}
result

</js>
$.[*]&&$.data.root&&$.data.replies[*]&&$.data.archives[*]&&$.data.articles[*]\|\|$.data.items[*]\|\|$.result.main_section.episodes[*]&&$.result.section[*].episodes[*] |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Datas' of undefined
    at <eval> (<input>:9:15)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:642)
` | source[356].ruleSearch.bookList = <js>
key=java.get('key');
page=java.get('page');

//创建两个空数组
json=[];json2=[];

//判定json里是否有列表
if(JSON.parse(result).info.Datas){
json=JSON.parse(result).info.Datas;}

//加载单曲搜索页面并转为json
json1=JSON.parse(java.ajax('https://www.missevan.com/sound/getsearch?s='+key+'&type=3&page_size=10&p='+page));

//判定json里是否有列表
if(json1.info.Datas){
json2=json1.info.Datas
}

//剧集搜索列表与单曲搜索列表拼接
list=json.concat(json2);

result=JSON.stringify(list)
</js>
$.[*]<br>source[728].ruleSearch.bookList = <js>
key=java.get('key');
page=java.get('page');

//创建两个空数组
json=[];json2=[];

//判定json里是否有列表
if(JSON.parse(result).info.Datas){
json=JSON.parse(result).info.Datas;}

//加载作者搜索页面并转为json
json1=JSON.parse(java.ajax('https://www.missevan.com/sound/getsearch?s='+key+'&type=3&page_size=10&p='+page));

//判定json里是否有列表
if(json1.info.Datas){
json2=json1.info.Datas
}

//书名搜索列表与作者搜索列表拼接
list=json.concat(json2);

result=JSON.stringify(list)
</js>
$.[*]<br>source[730].ruleSearch.bookList = <js>
key=java.get('key');
page=java.get('page');

//创建两个空数组
json=[];json2=[];

//判定json里是否有列表
if(JSON.parse(result).info.Datas){
json=JSON.parse(result).info.Datas;}

//加载单曲搜索页面并转为json
json1=JSON.parse(java.ajax('https://www.missevan.com/sound/getsearch?s='+key+'&type=3&page_size=10&p='+page));

//判定json里是否有列表
if(json1.info.Datas){
json2=json1.info.Datas
}

//剧集搜索列表与单曲搜索列表拼接
list=json.concat(json2);

result=JSON.stringify(list)
</js>
$.[*] |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Datas' of undefined
    at <eval> (<input>:9:15)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:713)
` | source[108].ruleSearch.bookList = <js>
key=java.get('key');
page=java.get('page');

//������������
json=[];json2=[];

//�ж�json���Ƿ����б�
if(JSON.parse(result).info.Datas){
json=JSON.parse(result).info.Datas;}

//���ص�������ҳ�沢תΪjson
json1=JSON.parse(java.ajax('https://www.missevan.com/sound/getsearch?s='+key+'&type=3&page_size=10&p='+page));

//�ж�json���Ƿ����б�
if(json1.info.Datas){
json2=json1.info.Datas
}

//�缯�����б��뵥�������б�ƴ��
list=json.concat(json2);

result=JSON.stringify(list)
</js>
$.[*] |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Jsoup' of undefined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1003)
` | source[189].ruleBookInfo.init = <js> 
var J = org.jsoup.Jsoup.parse(result);
var o = selector => String(J.select(selector).text()) ;
var og = selector => String(J.select('[property="og:' + selector + '"]').attr('content'));
var book = {
	name: og('novel:book_name').replace(/(全文\|小说\|免费阅读\|最新章节).*\|[(（].*[）)]/g, ''),
	author: og('novel:author'),
	kind: og('novel:category') + ',' + og('novel:status').replace(/中\|已/, ''),
	latest: og('novel:latest_chapter_name').replace(/^(正文\|VIP章节\|章节目录\|最新章节)?(\s+\|_)\|[\(（【].*[求更谢乐发推].*/g, '') + ' • ' + og('novel:update_time').replace(/(T\|\s).*/, ' ').replace(/\//g, '-'),
	intro: '　　🔸最近更新：' + og('novel:update_time').replace(/(T\|\s).*/, ' ').replace(/\//g, '-') + '\n' + og('description').replace(/.*(观看小说\|简介)[:：]\|分享书籍.*\|各位书友.*/g, '').replace(/\s+/g, '\n'),
	cover: og('image'),
url:og('novel:read_url'),
};
book;
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Jsoup' of undefined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:203)
` | source[597].ruleBookInfo.init = <js>
var act=org.jsoup.Jsoup.parse(result).select(".downButton").attr("href");
var url = "http://www.xiaxs.la"+act;
java.put("url",url);
java.ajax(url)</js>
<br>source[611].ruleBookInfo.init = <js>
var act=org.jsoup.Jsoup.parse(result).select(".downButton").attr("href");
var url = "http://www.xiaxs.la"+act;
java.put("url",url);
java.ajax(url)</js>
 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Jsoup' of undefined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:394)
` | source[608].ruleContent.content = @js:
doc=org.jsoup.Jsoup.parse(result.replace(/\(*读万卷 www.duwanjuan.com\)*\s*/g,''));
doc.select("h1").remove();
doc.select(".tishi").remove();
doc.select("tbody").remove();
result=doc.select("#acontent").html();
if(String(result).match(/content_sign/)){
result='由于版权问题不能显示'
}else{result=result} |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Jsoup' of undefined
    at <eval> (<input>:2:9)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:5367)
` | source[289].ruleToc.chapterList = @js:if(java.get("单")==''){
src=org.jsoup.Jsoup.parse(src);

if((result=java.get("录"))==""){if(页=(result=src.select('a[href~=\\S]:matches(下[一\\s]*[页頁]\|下[一二三四五六七八九十百千万〇零0-9]{2,}章):not([href~=^#\|javascript:])')).size())result=result.first().attr('href')
}else{网=String(result).split("🌕");
for(i=3,页=+网[0],result=网[1]+2+网[3];i<=页;i++)result+='\n'+网[1]+i+网[3];
result=String(result)}

if(页)java.put("页",/,/.test(book.tocUrl)?result.split('\n').join(',{"webView":true}\n')+',{"webView":true}':result);

嗅=()=>String(src).match(/[\[(](["'])<[a-z]+[ >][^\[\]()]+<\/a>(?:[^\[\],()]*<\/[a-z]+>)?\1[\])]/);
转=it=>it.replace(/\\[Uu]([0-9a-zA-Z]{4})/g,(_,it)=>String.fromCharCode(parseInt('0x'+it)));
兜=()=>src.select(':matchesOwn(^$\|[0-9〇一二三四五六七八九十])>a:matches(\\S):not(:has(*>*>:not(span)),[href~=(?i)passport\|\\.aspx$\|\\.php$\|^https://[^/]+(/\|index\\.[a-z]+)?$\|(^\|[^/])[?/].*((book\|[^a-z])(info\|case\|page\|reg\|Game\|Play)[^a-z]\|buy[^a-z]\|SystemInfo\|hot\|sort\|desc\|asc\|top\|coins\|nutrition\|review\|ticket\|update\|(app\|author\|xiazai\|down)(?!=))\|/list\\d*([/_-][^/_-]+/?)?$\|\\d+_\\d+_\\d+\|target=iframe\|https%],:matches((?i)^((点击\|软件\|应用\|安装\|客户\|移动\|手机\|电脑\|安卓\|苹果\|下载\|阅读\|pc\|ap[kp]\|ipa\|plx\|deb\|exe\|zip\|rar\|txt\|epub)[\\s.端版]*(?=$\|点击\|软件\|应用\|安装\|客户\|移动\|手机\|电脑\|安卓\|苹果\|下载\|阅读\|pc\|ap[kp]\|ipa\|plx\|deb\|exe\|zip\|rar\|txt\|epub)\|[A-Za-z0-9\\u4e00-\\u9fa5]?返回.*(简介\|书页\|目录)[A-Za-z0-9\\u4e00-\\u9fa5]?\|[<>-]+\|\\d+-\\d+章\|书页\|目录\|简介\|[上下首尾]([一\\s]*[页頁]\|[\\s\\d零〇一二三四五六七八九十百]*章)\|[↑\\[]?[倒正反逆顺順]序[↓\\]]?)$))');

if((zt=java.get("嗅"))!=''){
if(zt>0&&(嗅=嗅())){
src=嗅[0];
if(zt==2)src=转(src);
src=org.jsoup.Jsoup.parse(src)}
src.select(java.get("除")).remove();
if(java.get("兜")==1)src=兜();
src=src.select(java.get("查"))

if(!页){for(首=String(java.get("首")).split('\n'),ss=src.size(),i=0;i<ss;i++)if(首[i]!=src.get(i)){
if(i>1)src.subList(1,i).clear();break;}}

}else{
book.putVariable("除",除="meta,link,a:has(img),"+((zt=java.get("全")!=1)?"a[href$="+String(book.tocUrl).replace(/,\{"webView":true\}\|^.{8}[^/]*/g,'')+"],":"")+(基=String(java.get("基")),基==''?'':"a[href$="+基.replace(/^.{8}[^/]*/,'')+"],")+"a[href~=javascript:\|#\|[a-z]+[A-Z][a-z]+Id[=_-]\|[^/][/?&]sub[A-Z]\|action=list],a:matches(^$\|最新章节$\|^[^\\u4e00-\\u9fa5A-Z0-9]*(正文\|.{0,2}书架\|(免费\|在线\|开始\|立即\|全文\|从头\|点击\|正文)+[试阅]读\|[^\\s\\d外内楔前后卷篇章]*(更新调整\|[两一二三四五六七八九十]+连更\|作者[:：给要有]\|双倍月票\|感言\|推书\|推[a-z0-9A-Z_\\u4e00-\\u9fa5-]+书\|[求个请投点下张](月?票\|收藏\|订阅\|推荐)\|(感谢\|作者)[^\\s]*(读者\|书友\|大家\|各位)\|[书点]评[^\\s]*活动\|[没有空]更新\|没时间更新\|请个?假\|关于本书\|关于更新\|打赏名单\|起点活动)[^\\s]*)[^\\u4e00-\\u9fa5A-Z0-9]*$)");

book.putVariable("嗅",(基=嗅())?(src=org.jsoup.Jsoup.parse((zt=/\\[Uu]([0-9a-zA-Z]{4})/.test(基=基[0]))?转(基):基),zt)?2:1:0);
src.select(除).remove();

if(zt=java.get("全")!=1){
找=ll=null;
$=it=>(qc=ll,查=找,ll=src.select(找=it)).size()>14&&(查=it,src=re=ll);

if(!(((ck=java.get("ck"))!=""&&$("[href~="+ck+"(?!index(/\|\.[a-z]+)?$)[^.?/_-][^&/_-]*/?$\|/[vV][iI][pP][_-]?([Rr]ead\|[Cc]hapter\|action=article)\|([A-Za-z]\\d+\|\\d[A-Za-z]+\|[A-Z][a-z]+\|[a-z][A-Z]+){3,}[^/?&]*$]"))\|\|$("[href~=^[a-z0-9]+(/\|\\.[^./]+)?$]"))){
if(qc&&qc.size()>ll.size())找=查,ll=qc;

$=it=>(re=src.select(it)).size()&&(查=it,src=re);

if(!($("[data-cid]")\|\|$("[href~=(?i)(^\|[/_-])(chapter\|read)+([_-]?id)?/[^/_-]+[/_-][^/_-]+]")\|\|$("[href~=(?i)(^\|[&?/_-](book\|novel\|comic\|manhua\|mh?)?)(chapter\|read)+([_-]?id)?[?/=]]")\|\|$("[href~=(?i)[&?/_-]cid[&?/_=-]]")\|\|$("[data-href]"))){
src=兜();
book.putVariable("兜",1);

ba=(ba=String(java.get("ba"))).match(/(\?(?:[^=]+=)+)(.+)$/)\|\|ba.match(/(?:[^/_-][/_-]([^/._-]+))?[/_-]([^/._-]+)(?:\/\|\/index[^/]*\|\.[^/.]+)?$/);

if(xi=(id=ba[2]).match(/\?[^=]+=([^&]+)/)\|\|id.match(/^[^\d%]*(\d{2,}\|[1-9])$/)\|\|ba[1]&&ba[1].match(/^[^\d%]*(\d{2,})$/))id=xi[1];

$("[href~=([^\\d]\|^)"+id+"[/_&-][^\\d]*\\d+]:not([href~="+id+"[^\\d]*$]),[href~=/[vV][iI][pP]\|([A-Za-z]\\d+\|\\d[a-zA-Z]+\|[A-Z][a-z]+\|[a-z][A-Z]+){3,}[^/?]*$\|([^\\d]\|^)"+id+"[/_&-][^\\d]*"+id+"(/\|\\.[^.]+\|&.+)?$]")\|\|$("[title]")\|\|$("[href~=/view/\\d+\\.[a-zA-Z]+$]")}
if(re.size()<ll.size())查=找,src=ll}}

if(!(zt&&re.size()))src=src.select('a'),查='a';
if(查)book.putVariable("查",查);
if(页)java.put("首",src)}

src}else if((list=java.get("单"))!=1){

if((网=String(list).split("🌕")).length>1){
for(i=+网[2],x=+网[0],j=2,list="<a href='"+baseUrl+"'>正文1</a>";i<=x;i++,j++)list+="<a href='"+网[1]+i+网[3]+"'>正文"+j+"</a>"}

org.jsoup.Jsoup.parse(list).select('a')

}else org.jsoup.Jsoup.parse("<a href='"+baseUrl+"'>正文</a>").select('a')<br>source[295].ruleToc.chapterList = @js:if(java.get("单")==''){
src=org.jsoup.Jsoup.parse(src);

if((result=java.get("录"))==""){if(页=(result=src.select('a[href~=\\S]:matches(下[一\\s]*[页頁]\|下[一二三四五六七八九十百千万〇零0-9]{2,}章):not([href~=^#\|javascript:])')).size())result=result.first().attr('href')
}else{网=String(result).split("🌕");
for(i=3,页=+网[0],result=网[1]+2+网[3];i<=页;i++)result+='\n'+网[1]+i+网[3];
result=String(result)}

if(页)java.put("页",/,/.test(book.tocUrl)?result.split('\n').join(',{"webView":true}\n')+',{"webView":true}':result);

嗅=()=>String(src).match(/[\[(](["'])<[a-z]+[ >][^\[\]()]+<\/a>(?:[^\[\],()]*<\/[a-z]+>)?\1[\])]/);
转=it=>it.replace(/\\[Uu]([0-9a-zA-Z]{4})/g,(_,it)=>String.fromCharCode(parseInt('0x'+it)));
兜=()=>src.select(':matchesOwn(^$\|[0-9〇一二三四五六七八九十])>a:matches(\\S):not(:has(*>*>:not(span)),[href~=(?i)passport\|\\.aspx$\|\\.php$\|^https://[^/]+(/\|index\\.[a-z]+)?$\|(^\|[^/])[?/].*((book\|[^a-z])(info\|case\|page\|reg\|Game\|Play)[^a-z]\|buy[^a-z]\|SystemInfo\|hot\|sort\|desc\|asc\|top\|coins\|nutrition\|review\|ticket\|update\|(app\|author\|xiazai\|down)(?!=))\|/list\\d*([/_-][^/_-]+/?)?$\|\\d+_\\d+_\\d+\|target=iframe\|https%],:matches((?i)^((点击\|软件\|应用\|安装\|客户\|移动\|手机\|电脑\|安卓\|苹果\|下载\|阅读\|pc\|ap[kp]\|ipa\|plx\|deb\|exe\|zip\|rar\|txt\|epub)[\\s.端版]*(?=$\|点击\|软件\|应用\|安装\|客户\|移动\|手机\|电脑\|安卓\|苹果\|下载\|阅读\|pc\|ap[kp]\|ipa\|plx\|deb\|exe\|zip\|rar\|txt\|epub)\|[A-Za-z0-9\\u4e00-\\u9fa5]?返回.*(简介\|书页\|目录)[A-Za-z0-9\\u4e00-\\u9fa5]?\|[<>-]+\|\\d+-\\d+章\|书页\|目录\|简介\|[上下首尾]([一\\s]*[页頁]\|[\\s\\d零〇一二三四五六七八九十百]*章)\|[↑\\[]?[倒正反逆顺順]序[↓\\]]?)$))');

if((zt=java.get("嗅"))!=''){
if(zt>0&&(嗅=嗅())){
src=嗅[0];
if(zt==2)src=转(src);
src=org.jsoup.Jsoup.parse(src)}
src.select(java.get("除")).remove();
if(java.get("兜")==1)src=兜();
src=src.select(java.get("查"))

if(!页){for(首=String(java.get("首")).split('\n'),ss=src.size(),i=0;i<ss;i++)if(首[i]!=src.get(i)){
if(i>1)src.subList(1,i).clear();break;}}

}else{
book.putVariable("除",除="meta,link,a:has(img),"+((zt=java.get("全")!=1)?"a[href$="+String(book.tocUrl).replace(/,\{"webView":true\}\|^.{8}[^/]*/g,'')+"],":"")+(基=String(java.get("基")),基==''?'':"a[href$="+基.replace(/^.{8}[^/]*/,'')+"],")+"a[href~=javascript:\|#\|[a-z]+[A-Z][a-z]+Id[=_-]\|[^/][/?&]sub[A-Z]\|action=list],a:matches(^$\|最新章节$\|^[^\\u4e00-\\u9fa5A-Z0-9]*(正文\|.{0,2}书架\|(免费\|在线\|开始\|立即\|全文\|从头\|点击\|正文)+[试阅]读\|[^\\s\\d外内楔前后卷篇章]*(更新调整\|[两一二三四五六七八九十]+连更\|作者[:：给要有]\|双倍月票\|感言\|推书\|推[a-z0-9A-Z_\\u4e00-\\u9fa5-]+书\|[求个请投点下张](月?票\|收藏\|订阅\|推荐)\|(感谢\|作者)[^\\s]*(读者\|书友\|大家\|各位)\|[书点]评[^\\s]*活动\|[没有空]更新\|没时间更新\|请个?假\|关于本书\|关于更新\|打赏名单\|起点活动)[^\\s]*)[^\\u4e00-\\u9fa5A-Z0-9]*$)");

book.putVariable("嗅",(基=嗅())?(src=org.jsoup.Jsoup.parse((zt=/\\[Uu]([0-9a-zA-Z]{4})/.test(基=基[0]))?转(基):基),zt)?2:1:0);
src.select(除).remove();

if(zt=java.get("全")!=1){
找=ll=null;
$=it=>(qc=ll,查=找,ll=src.select(找=it)).size()>14&&(查=it,src=re=ll);

if(!(((ck=java.get("ck"))!=""&&$("[href~="+ck+"(?!index(/\|\.[a-z]+)?$)[^.?/_-][^&/_-]*/?$\|/[vV][iI][pP][_-]?([Rr]ead\|[Cc]hapter\|action=article)\|([A-Za-z]\\d+\|\\d[A-Za-z]+\|[A-Z][a-z]+\|[a-z][A-Z]+){3,}[^/?&]*$]"))\|\|$("[href~=^[a-z0-9]+(/\|\\.[^./]+)?$]"))){
if(qc&&qc.size()>ll.size())找=查,ll=qc;

$=it=>(re=src.select(it)).size()&&(查=it,src=re);

if(!($("[data-cid]")\|\|$("[href~=(?i)(^\|[/_-])(chapter\|read)+([_-]?id)?/[^/_-]+[/_-][^/_-]+]")\|\|$("[href~=(?i)(^\|[&?/_-](book\|novel\|comic\|manhua\|mh?)?)(chapter\|read)+([_-]?id)?[?/=]]")\|\|$("[href~=(?i)[&?/_-]cid[&?/_=-]]")\|\|$("[data-href]"))){
src=兜();
book.putVariable("兜",1);

ba=(ba=String(java.get("ba"))).match(/(\?(?:[^=]+=)+)(.+)$/)\|\|ba.match(/(?:[^/_-][/_-]([^/._-]+))?[/_-]([^/._-]+)(?:\/\|\/index[^/]*\|\.[^/.]+)?$/);

if(xi=(id=ba[2]).match(/\?[^=]+=([^&]+)/)\|\|id.match(/^[^\d%]*(\d{2,}\|[1-9])$/)\|\|ba[1]&&ba[1].match(/^[^\d%]*(\d{2,})$/))id=xi[1];

$("[href~=([^\\d]\|^)"+id+"[/_&-][^\\d]*\\d+]:not([href~="+id+"[^\\d]*$]),[href~=/[vV][iI][pP]\|([A-Za-z]\\d+\|\\d[a-zA-Z]+\|[A-Z][a-z]+\|[a-z][A-Z]+){3,}[^/?]*$\|([^\\d]\|^)"+id+"[/_&-][^\\d]*"+id+"(/\|\\.[^.]+\|&.+)?$]")\|\|$("[title]")\|\|$("[href~=/view/\\d+\\.[a-zA-Z]+$]")}
if(re.size()<ll.size())查=找,src=ll}}

if(!(zt&&re.size()))src=src.select('a'),查='a';
if(查)book.putVariable("查",查);
if(页)java.put("首",src)}

src}else if((list=java.get("单"))!=1){

if((网=String(list).split("🌕")).length>1){
for(i=+网[2],x=+网[0],j=2,list="<a href='"+baseUrl+"'>正文1</a>";i<=x;i++,j++)list+="<a href='"+网[1]+i+网[3]+"'>正文"+j+"</a>"}

org.jsoup.Jsoup.parse(list).select('a')

}else org.jsoup.Jsoup.parse("<a href='"+baseUrl+"'>正文</a>").select('a') |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Jsoup' of undefined
    at <eval> (<input>:3:26)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:4391)
` | source[289].ruleBookInfo.tocUrl = @js:if(java.get("单")==''){
if(java.get("录")==java.get("目")){
r=org.jsoup.Jsoup.parse(result);

script=r.select(":matchesOwn(^$)>a[href^=javascript:]:matches(全文\|章[節节]\|目[錄录]):not(:matches(最新))");re=false;

if(!script.size()){
ba=(bas=baseUrl.replace(/\/$\|\.[a-zA-Z]+$/,'')).match(/(http....[^/?]+)(?:([?/])(.*))?$/);b=ba[1];v=ba[3];

r.select("a[href~=[^a-z]page[^a-z]]:not(:matches([反正顺順逆倒]序\|[全正]文\|更多\|全部\|所有\|章[節节]\|作品\|目[錄录]\|列表)),a:not([href~=^(?!//)[^#:]+$\|"+b.split(':')[1]+"]),:matchesOwn(\\S)>a,a[href~=javascript:\|#],a:matches(^\\S{1,4}$):not(:matches([反正顺順逆倒]序\|[阅閱][讀读]\|查看\|展[开開]\|进入\|[全正]文\|更多\|完整\|全部\|所有\|目[錄录]\|列表\|章[節节]):not(:contains(分类)))").remove();

y=r.select("a[href~=(?i)catalog\|contents\|chapters\|mulu\|(^\|[^a-z])ml\|showchapter\|(^\|chapter[/_-]?\|"+v.replace(/(.+)((\/\d\d)\d+)$/,"$1$3$2")+"[/_-])(more\|list\|all)\|sort[/=_-]asc]");

if(!y.size()){b3='';

if(ba[2]=="?"){
b3='[href~=^[^?]+$],'
}else if(ba[3]){
b3=ba[3].replace(/[*$\|?+\\\^\[\](){}]/g,'\\$0');

b3="[href~="+b3+"(\\.[^.]+\|/\\d+(\\.[^.]+\|/)?)?$],[href~=[/?]"+b3.replace(/[&/_-][^/_-]+$/,'')+".*$]:not([href~=[/?]"+b3.replace(/[/_-]/g,'[/_-]')+"]),"}

y=r.select("a:not("+b3+"[href~=(?i)(^\|[^/])[?/].*((book\|[^a-z])(info\|case)[^a-z]\|(cid\|buy)[^a-z]\|desc\|coins\|nutrition\|review\|ticket\|update\|(app\|author\|xiazai\|down)(?!=))\|/chapter\|/index/\|/d/],:matches((?i)^$\|[0-9零〇一二三四五六七八九十百千万、，：；？！。…‘’“”（）()]\|[票榜:：.]\|推荐\|排行\|等级\|说明\|收藏\|书评\|简介\|分[类卷]\|简介\|作者\|手机\|软件\|应用\|安装\|客户\|移动\|pc\|电脑\|安卓\|苹果\|下载\|最新\|ap[kp]\|ipa\|plx\|deb\|exe\|zip\|rar\|txt\|epub))")}

if(y.size()){
ys=y.select("a[href~=(?i)catalog\|contents\|list\|chapter\|mulu\|(^\|[^a-z])ml\|more\|read\|all]:matches([反正顺順逆倒]序\|全文\|章[節节]\|目[錄录]):not(:matches(阅读)),a[href~=(?i)catalog\|contents\|chapter\|(^\|[^a-z])ml\|mulu\|read]:matches(更多\|列表),a[href~=(?i)catalog\|contents\|list\|chapter\|mulu\|(^\|[^a-z])ml\|more\|all]:matches(^$),a:matches(^[^\\u4e00-\\u9fa50-9]*([反正顺順逆倒]序\|全文(免[費费])?[阅閱][讀读]\|(点击\|查看\|展[开開]\|进入\|返?回到?)*([全正]文\|(更多\|完整\|全部\|所有)?(章[節节]\|(作品)?目[錄录])+)+(列表)?(\\s*(查看\|展开)?更多)?)[^\\u4e00-\\u9fa50-9]*$)");zt=false;

if(!ys.size()){
ba=bas.match(/(\?(?:[^=]+=)+)(.+)$/)\|\|bas.match(/(?:[^/_-][/_-]([^/._-]+))?[/_-]([^/._-]+)(?:\/index[^/]*)?$/);

if(xi=(id=ba[2]).match(/\?[^=]+=([^&]+)/)\|\|id.match(/^[^\d%]*(\d{2,}\|[1-9])$/))id=xi[1];
if(ba[1])if(xi=ba[1].match(/^[^\d%]*(\d{2,})$/))id=/^\d$/.test(id)\|\|!xi[1].indexOf(id)?xi[1]:"("+id+"\|"+xi[1]+")";

ys=y.select("[href~=(?i)^((.*//[^/]+/)?[^=.]*[^\\d=.])?"+id+"([?_-][a-z=_-]*0\|\\.[^.]+\|[/?]([^\\d]*\|[^/\\d]*/?\|(list\|more\|all)([=_-][a-z]*)?\\d+[^\\d]*)?)?(&.+)?$]:not(:contains("+(bs=String(book.name)[0])+"))");

if(!ys.size()){zt=true;
ys=y.select("[href~=(?i)^((.*//[^/]+/)?[^=.]*[^\\d=.])?(\\d+/"+id+"[_-]\\d+[^\\d]*\|"+id+"[_-][a-z_-]*1[^\\d]*)$]:not(:matches(阅读\|"+bs+"))")}}

if(ys.size()){
if((re=ys.select("[href~=(/\|^)[^.]+$]")).size())ys=re;
re=String(ys.first().attr("href"));

for(x=1;x<ys.size();x++)if((xs=String(ys.get(x).attr("href"))).length>re.length)re=xs;

if(/(?:[2-9]\d*\|1\d+)[^\d]*$/.test(re)){
if(!zt&&(q=re.match(/^(.*[^/])?[&?/].*(?:catalog(ue)?\|contents\|(?:show)?chapters?\|mulu\|ml\|more\|all\|list\|page)(?:[=_-][a-z]*)?\d+[^\d]*$/i))&&v==(q[1]\|\|'').replace(/^http....[^/?]+/,''))zt=true;
if(zt)re=re.replace(/\d+(?=[^\d]*$)/,"☯1")}

}}}

re=String(!re?baseUrl:(java.put("基",baseUrl),/^\/[^/]/.test(re)?b+re:/^https?:/.test(re)?re:/^\/\//.test(re)?b.split('//')[0]+re:String(baseUrl).replace(/[^/]*$/,'')+re))}else re=baseUrl;

r=re.replace(/(?:[☯?&/_-][^\d?/&_-]*[01])+[^\d]*$/,"");
java.put("ba",r);
re=re.replace("☯","")}else re=baseUrl;

java.get("跳")==1\|\|re==baseUrl&&/,/.test(book.bookUrl)?re+',{"webView":true}':re<br>source[295].ruleBookInfo.tocUrl = @js:if(java.get("单")==''){
if(java.get("录")==java.get("目")){
r=org.jsoup.Jsoup.parse(result);

script=r.select(":matchesOwn(^$)>a[href^=javascript:]:matches(全文\|章[節节]\|目[錄录]):not(:matches(最新))");re=false;

if(!script.size()){
ba=(bas=baseUrl.replace(/\/$\|\.[a-zA-Z]+$/,'')).match(/(http....[^/?]+)(?:([?/])(.*))?$/);b=ba[1];v=ba[3];

r.select("a[href~=[^a-z]page[^a-z]]:not(:matches([反正顺順逆倒]序\|[全正]文\|更多\|全部\|所有\|章[節节]\|作品\|目[錄录]\|列表)),a:not([href~=^(?!//)[^#:]+$\|"+b.split(':')[1]+"]),:matchesOwn(\\S)>a,a[href~=javascript:\|#],a:matches(^\\S{1,4}$):not(:matches([反正顺順逆倒]序\|[阅閱][讀读]\|查看\|展[开開]\|进入\|[全正]文\|更多\|完整\|全部\|所有\|目[錄录]\|列表\|章[節节]):not(:contains(分类)))").remove();

y=r.select("a[href~=(?i)catalog\|contents\|chapters\|mulu\|(^\|[^a-z])ml\|showchapter\|(^\|chapter[/_-]?\|"+v.replace(/(.+)((\/\d\d)\d+)$/,"$1$3$2")+"[/_-])(more\|list\|all)\|sort[/=_-]asc]");

if(!y.size()){b3='';

if(ba[2]=="?"){
b3='[href~=^[^?]+$],'
}else if(ba[3]){
b3=ba[3].replace(/[*$\|?+\\\^\[\](){}]/g,'\\$0');

b3="[href~="+b3+"(\\.[^.]+\|/\\d+(\\.[^.]+\|/)?)?$],[href~=[/?]"+b3.replace(/[&/_-][^/_-]+$/,'')+".*$]:not([href~=[/?]"+b3.replace(/[/_-]/g,'[/_-]')+"]),"}

y=r.select("a:not("+b3+"[href~=(?i)(^\|[^/])[?/].*((book\|[^a-z])(info\|case)[^a-z]\|(cid\|buy)[^a-z]\|desc\|coins\|nutrition\|review\|ticket\|update\|(app\|author\|xiazai\|down)(?!=))\|/chapter\|/index/\|/d/],:matches((?i)^$\|[0-9零〇一二三四五六七八九十百千万、，：；？！。…‘’“”（）()]\|[票榜:：.]\|推荐\|排行\|等级\|说明\|收藏\|书评\|简介\|分[类卷]\|简介\|作者\|手机\|软件\|应用\|安装\|客户\|移动\|pc\|电脑\|安卓\|苹果\|下载\|最新\|ap[kp]\|ipa\|plx\|deb\|exe\|zip\|rar\|txt\|epub))")}

if(y.size()){
ys=y.select("a[href~=(?i)catalog\|contents\|list\|chapter\|mulu\|(^\|[^a-z])ml\|more\|read\|all]:matches([反正顺順逆倒]序\|全文\|章[節节]\|目[錄录]):not(:matches(阅读)),a[href~=(?i)catalog\|contents\|chapter\|(^\|[^a-z])ml\|mulu\|read]:matches(更多\|列表),a[href~=(?i)catalog\|contents\|list\|chapter\|mulu\|(^\|[^a-z])ml\|more\|all]:matches(^$),a:matches(^[^\\u4e00-\\u9fa50-9]*([反正顺順逆倒]序\|全文(免[費费])?[阅閱][讀读]\|(点击\|查看\|展[开開]\|进入\|返?回到?)*([全正]文\|(更多\|完整\|全部\|所有)?(章[節节]\|(作品)?目[錄录])+)+(列表)?(\\s*(查看\|展开)?更多)?)[^\\u4e00-\\u9fa50-9]*$)");zt=false;

if(!ys.size()){
ba=bas.match(/(\?(?:[^=]+=)+)(.+)$/)\|\|bas.match(/(?:[^/_-][/_-]([^/._-]+))?[/_-]([^/._-]+)(?:\/index[^/]*)?$/);

if(xi=(id=ba[2]).match(/\?[^=]+=([^&]+)/)\|\|id.match(/^[^\d%]*(\d{2,}\|[1-9])$/))id=xi[1];
if(ba[1])if(xi=ba[1].match(/^[^\d%]*(\d{2,})$/))id=/^\d$/.test(id)\|\|!xi[1].indexOf(id)?xi[1]:"("+id+"\|"+xi[1]+")";

ys=y.select("[href~=(?i)^((.*//[^/]+/)?[^=.]*[^\\d=.])?"+id+"([?_-][a-z=_-]*0\|\\.[^.]+\|[/?]([^\\d]*\|[^/\\d]*/?\|(list\|more\|all)([=_-][a-z]*)?\\d+[^\\d]*)?)?(&.+)?$]:not(:contains("+(bs=String(book.name)[0])+"))");

if(!ys.size()){zt=true;
ys=y.select("[href~=(?i)^((.*//[^/]+/)?[^=.]*[^\\d=.])?(\\d+/"+id+"[_-]\\d+[^\\d]*\|"+id+"[_-][a-z_-]*1[^\\d]*)$]:not(:matches(阅读\|"+bs+"))")}}

if(ys.size()){
if((re=ys.select("[href~=(/\|^)[^.]+$]")).size())ys=re;
re=String(ys.first().attr("href"));

for(x=1;x<ys.size();x++)if((xs=String(ys.get(x).attr("href"))).length>re.length)re=xs;

if(/(?:[2-9]\d*\|1\d+)[^\d]*$/.test(re)){
if(!zt&&(q=re.match(/^(.*[^/])?[&?/].*(?:catalog(ue)?\|contents\|(?:show)?chapters?\|mulu\|ml\|more\|all\|list\|page)(?:[=_-][a-z]*)?\d+[^\d]*$/i))&&v==(q[1]\|\|'').replace(/^http....[^/?]+/,''))zt=true;
if(zt)re=re.replace(/\d+(?=[^\d]*$)/,"☯1")}

}}}

re=String(!re?baseUrl:(java.put("基",baseUrl),/^\/[^/]/.test(re)?b+re:/^https?:/.test(re)?re:/^\/\//.test(re)?b.split('//')[0]+re:String(baseUrl).replace(/[^/]*$/,'')+re))}else re=baseUrl;

r=re.replace(/(?:[☯?&/_-][^\d?/&_-]*[01])+[^\d]*$/,"");
java.put("ba",r);
re=re.replace("☯","")}else re=baseUrl;

java.get("跳")==1\|\|re==baseUrl&&/,/.test(book.bookUrl)?re+',{"webView":true}':re |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Symbol.iterator' of null
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:705)
` | source[3].ruleContent.content = <js>
let [, aid, cid] = baseUrl.match(/\/books\/(\d+)\/(\d+)\.html/);
let url = `https://www.deqixs.co/scripts/chapter.js.php?aid=${aid}&cid=${cid}&referrer=${baseUrl}`;
let tokenHtml = java.ajax(url);
eval(String(tokenHtml));

let params = {
  aid,
  cid,
  token: chapterToken,
  timestamp,
  nonce
};
let paramStr =	Object.entries(params)
    .map(x=>x.join("=")).join("&");
let headers = {
	 "Referer": baseUrl,
	 "Accept":	"text/plain, */*; q=0.01",
  "X-Requested-With": "XMLHttpRequest"
}
url = "https://www.deqixs.co/modules/article/ajax2.php?"+paramStr+","+JSON.stringify({
	 headers
})

java.ajax(url)
</js>
$..content\|\|$.message |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'altTitles' of undefined
    at <eval> (<input>:1:37)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:593)
` | source[81].ruleBookInfo.name = @js:
const list = JSON.parse(JSON.stringify(src));
const altTitles = list.attributes.altTitles;
const originalLang = list.attributes.originalLanguage;
const znEntry = altTitles.find(item => 'zn' in item);
if (znEntry) {
  result = znEntry.zn;
} else {
  const zhHkEntry = altTitles.find(item => 'zh-hk' in item);
  if (zhHkEntry) {
    result = zhHkEntry['zh-hk'];
  } else {
    const originalEntry = altTitles.find(item => originalLang in item);
    result = originalEntry?.[originalLang] \|\| list.attributes.title.en;
  }
}
result.split('\n')[0]; |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'altTitles' of undefined
    at <eval> (<input>:1:37)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:595)
` | source[81].ruleSearch.name = @js:
const list = JSON.parse(JSON.stringify(src));
const altTitles = list.attributes.altTitles;
const originalLang = list.attributes.originalLanguage;
const zhEntry = altTitles.find(item => 'zh' in item);
if (zhEntry) {
  result = zhEntry.zh;
} else {
  const zhHkEntry = altTitles.find(item => 'zh-hk' in item);
  if (zhHkEntry) {
    result = zhHkEntry['zh-hk'];
  } else {
    const originalEntry = altTitles.find(item => originalLang in item);
    result = originalEntry?.[originalLang] \|\| list.attributes.title.en;
  }
}
result.split("\n")[0]; |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'body' of undefined
    at <eval> (<input>:2:20)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:2574)
` | source[411].ruleContent.content = <js>
$ = JSON.parse(result);
body = $.chapter.body;
if(body!=null){
	

//解密方式一（阅读内置方法）：
jmkey="MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAjYFYoMbA0uW8by6+YIghxxsvibS9YW4yKVSulykAzZZwZ/+dNTkZ4inY7Pj08aksm6RCGKS6+WfvVQo/EdkcS5p2LY2/76qVzapyHsyQf/Pud6ATPKnwxNt/DaqjL35Z9K0NI/RF9x732RdIEOTKXppfRdzCa/1Ctm/5ZFilY8UmZsppkjDd3XkuPr3n3wVC8WFvqmdJ1N55prRlnaRaO+mIOXo3OsOzIxE5EdcE0TLT9OFZ3Wlbi3E0iI0v/ZsrWoL57YvLwo7BsARp7BansDCx8NZg6ObGQN/tNrE/nKqQTXeJjnFWXdLfhI7xivPPphkj5fNpiufIsIUEd7eXBwIDAQAB";
data = java.base64DecodeToByteArray(body);
key = java.base64DecodeToByteArray(jmkey);

content = java.createAsymmetricCrypto("RSA").setPublicKey(key).decryptStr(data)
	
/*	
//解密方式二（构造函数）：
var javaImport = new JavaImporter();
javaImport.importPackage(
	   Packages.java.lang,
     Packages.java.security,
     Packages.java.security.interfaces,
     Packages.java.security.spec,
     Packages.android.util,
     Packages.java.io,      
     Packages.javax.crypto,
     Packages.javax.crypto.spec
);
with(javaImport){
 function Decode(str){
jmkey="MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAjYFYoMbA0uW8by6+YIghxxsvibS9YW4yKVSulykAzZZwZ/+dNTkZ4inY7Pj08aksm6RCGKS6+WfvVQo/EdkcS5p2LY2/76qVzapyHsyQf/Pud6ATPKnwxNt/DaqjL35Z9K0NI/RF9x732RdIEOTKXppfRdzCa/1Ctm/5ZFilY8UmZsppkjDd3XkuPr3n3wVC8WFvqmdJ1N55prRlnaRaO+mIOXo3OsOzIxE5EdcE0TLT9OFZ3Wlbi3E0iI0v/ZsrWoL57YvLwo7BsARp7BansDCx8NZg6ObGQN/tNrE/nKqQTXeJjnFWXdLfhI7xivPPphkj5fNpiufIsIUEd7eXBwIDAQAB";
 	publicKey = 	KeyFactory.getInstance("RSA").generatePublic(new X509EncodedKeySpec(Base64.decode(jmkey, 0)));

       instance = Cipher.getInstance("RSA");
        instance.init(2, publicKey);
        decode = Base64.decode(str,0);
        blockSize = instance.getBlockSize();
        byteArrayOutputStream = new ByteArrayOutputStream(64);
         i = 0;
        while (true) {
             i2 = i * blockSize;
            if (decode.length - i2 > 0) {
                byteArrayOutputStream.write(instance.doFinal(decode, i2, blockSize));
                i++;
            } else {
                decode = byteArrayOutputStream.toByteArray();
                byteArrayOutputStream.close();
                return new String(decode);
            }
        }
    }
}
content = Decode(body);

*/

}
else{ msg = $.msg; }
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'data' of undefined
    at <eval> (<input>:1:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:267)
` | source[81].ruleContent.content = @js:
var data = JSON.parse(src);
var base_url = data.baseUrl;
var imglist = data.chapter.data;
var hash = data.chapter.hash;
result = imglist.map( x => `<img src="${base_url}/data/${hash}/${x}">`);
result.join("\n"); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'description' of undefined
    at <eval> (<input>:1:37)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:497)
` | source[81].ruleBookInfo.intro = @js:
const list = JSON.parse(JSON.stringify(src));
const description = list.attributes.description;
const originalLang = list.attributes.originalLanguage;
let result;
if (description['zh-hk']) {
  result = description['zh-hk'];
} else if (description['en']) {
  result = description['en'];
} else if (description[originalLang]) {
  result = description[originalLang];
} else {
  result = list.attributes.title.en;
}
result = result.split('\n')[0];<br>source[81].ruleSearch.intro = @js:
const list = JSON.parse(JSON.stringify(src));
const description = list.attributes.description;
const originalLang = list.attributes.originalLanguage;
let result;
if (description['zh-hk']) {
  result = description['zh-hk'];
} else if (description['en']) {
  result = description['en'];
} else if (description[originalLang]) {
  result = description[originalLang];
} else {
  result = list.attributes.title.en;
}
result = result.split('\n')[0]; |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'forEach' of undefined
    at <eval> (<input>:6:3)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1308)
` | source[434].ruleToc.chapterList = <js>
  //showjname设置章节名显示卷名true or false
  let obj = {showjname: false}
  let $ = JSON.parse(String(result))
  let array = []
  $.volumes.forEach((booklet) => {
  	  java.put('jname',booklet.name)
    array.push({ 
    	    name:'◆◇'+String(booklet.name)+'◇◆',
    	    voltype:true
    	 })
    booklet.chapters.forEach((chapter) => {
href='http://api.ieasou.com/api/bookapp/chargeChapter.m?a=1&autoBuy=0&cid=eef_easou_book&version=002&os=android&udid=1c5b2618a57a0848e2510649dc1e03896f462284&appverion=1122&ch=blf1298_12337_001&session_id=-nEvkqSq_9ZyORN5OoVOOzJ&dzh=1&scp=0&appid=10001&utype=0&rtype=3&pushid=f97b7f81a269472c07708277b7c40b4f&ptype=5&gender=0&userInitPay=3&birt=1658424532472&instime=1658424529924&instId=1658424529924&chType=3&bidType=0&recSw=1&appType=0&gid='+java.get('gid')+'&nid='+chapter.nid+'&sort='+chapter.sort+'&gsort=0&sgsort=0&sequence=4&chapter_name='+chapter.chapter_name
      array.push({
        name: !java.get('jname')?chapter.chapter_name:((obj.showjname?'※'+java.get('jname')+'※ ':'').padStart(3,''))+chapter.chapter_name,
        url: href,
        time:"本章字数:"+String(chapter.wordCount)+"字",
        voltype:false
      })
    })
  })
  array
</js><br>source[439].ruleToc.chapterList = <js>
  //showjname设置章节名显示卷名true or false
  let obj = {showjname: false}
  let $ = JSON.parse(String(result))
  let array = []
  $.volumes.forEach((booklet) => {
  	  java.put('jname',booklet.name)
    array.push({ 
    	    name:'◆◇'+String(booklet.name)+'◇◆',
    	    voltype:true
    	 })
    booklet.chapters.forEach((chapter) => {
href='http://api.ieasou.com/api/bookapp/chargeChapter.m?a=1&autoBuy=0&cid=eef_easou_book&version=002&os=android&udid=1c5b2618a57a0848e2510649dc1e03896f462284&appverion=1122&ch=blf1298_12337_001&session_id=-nEvkqSq_9ZyORN5OoVOOzJ&dzh=1&scp=0&appid=10001&utype=0&rtype=3&pushid=f97b7f81a269472c07708277b7c40b4f&ptype=5&gender=0&userInitPay=3&birt=1658424532472&instime=1658424529924&instId=1658424529924&chType=3&bidType=0&recSw=1&appType=0&gid='+java.get('gid')+'&nid='+chapter.nid+'&sort='+chapter.sort+'&gsort=0&sgsort=0&sequence=4&chapter_name='+chapter.chapter_name
      array.push({
        name: !java.get('jname')?chapter.chapter_name:((obj.showjname?'※'+java.get('jname')+'※ ':'').padStart(3,''))+chapter.chapter_name,
        url: href,
        time:"本章字数:"+String(chapter.wordCount)+"字",
        voltype:false
      })
    })
  })
  array
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'length' of undefined
    at <eval> (<input>:5:23)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:459)
` | source[310].ruleToc.chapterList = <js>
var obj = JSON.parse(result); 
var chapterNameList= obj.data.chapterNameList;
var chapterIdList= obj.data.chapterIdList;
var bookId = obj.data.bookId;
var length = parseInt(obj.data.chapterNameList.length);
var list = [];
var ret;
for(var i = 0; i < length; i++)
{ 	 list.push({"chapterName":String(chapterNameList[i]), "chapterId":String(chapterIdList[i])});
}
java.put("bookId",bookId);

list
</js>
<br>source[500].ruleToc.chapterList = <js>
var obj = JSON.parse(result); 
var chapterNameList= obj.data.chapterNameList;
var chapterIdList= obj.data.chapterIdList;
var bookId = obj.data.bookId;
var length = parseInt(obj.data.chapterNameList.length);
var list = [];
var ret;
for(var i = 0; i < length; i++)
{ 	 list.push({"chapterName":String(chapterNameList[i]), "chapterId":String(chapterIdList[i])});
}
java.put("bookId",bookId);

list
</js>
<br>source[656].ruleToc.chapterList = <js>
var obj = JSON.parse(result); 
var chapterNameList= obj.data.chapterNameList;
var chapterIdList= obj.data.chapterIdList;
var bookId = obj.data.bookId;
var length = parseInt(obj.data.chapterNameList.length);
var list = [];
var ret;
for(var i = 0; i < length; i++)
{ 	 list.push({"chapterName":String(chapterNameList[i]), "chapterId":String(chapterIdList[i])});
}
java.put("bookId",bookId);

list
</js>
 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'length' of undefined
    at parseComments (<input>:2:22)
    at <eval> (<input>:10:89)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:606)
` | source[294].ruleContent.content = <js>
function parseComments(n){
  for(var c=[],e=0;e<n.length;e++){
    var o=n[e];
    c.push("书友：".concat(o.user.nickname,"\n时间：").concat(o.create_time,"\n评分：").concat("♥".repeat(o.score/2),"\n评语：").concat(o.content.replace(/([。；！？—…]”?)([\u4e00-\u9fa5][^。！？—…]{9})/g,"$1\n$2")))
  }
  return c.join("\n————\n");
}
var data = JSON.parse(result);
data.data.total === 0;
data.data.total === 0 ? '还没有人对这本书发表评价哦！<br>': parseComments(data.data.data);
</js><br>source[746].ruleContent.content = <js>
function parseComments(n){
  for(var c=[],e=0;e<n.length;e++){
    var o=n[e];
    c.push("书友：".concat(o.user.nickname,"\n时间：").concat(o.create_time,"\n评分：").concat("♥".repeat(o.score/2),"\n评语：").concat(o.content.replace(/([。；！？—…]”?)([\u4e00-\u9fa5][^。！？—…]{9})/g,"$1\n$2")))
  }
  return c.join("\n————\n");
}
var data = JSON.parse(result);
data.data.total === 0;
data.data.total === 0 ? '还没有人对这本书发表评价哦！<br>': parseComments(data.data.data);
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'map' of undefined
    at <eval> (<input>:4:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:300)
` | source[18].ruleToc.chapterList = @js:
$ = JSON.parse(result).data;
		bid = $.book_id;
$.chapters.map($=>{
		$.url = `http://119.45.176.116:5006/chapterContent,{"body":{"book_id":${bid},"chapterIdList":"${$.id},"},"method":"POST"}`
		return $;
	});<br>source[549].ruleToc.chapterList = @js:
$ = JSON.parse(result).data;
		bid = $.book_id;
$.chapters.map($=>{
		$.url = `http://119.45.176.116:5006/chapterContent,{"body":{"book_id":${bid},"chapterIdList":"${$.id},"},"method":"POST"}`
		return $;
	}); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'map' of undefined
    at <eval> (<input>:5:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:455)
` | source[518].ruleToc.chapterList = @js:

url = `https://xgmf.zuanqianyi.com/glory/free/1152?a,`;

$ = JSON.parse(result).data;
$.chapterNameList.map((text,i)=>{

body = JSON.stringify({
	chapterId: $.chapterIdList[i],
	bookId: $.bookId
});

href = url+JSON.stringify({
  "body": body,
  "headers": {
    "signtype": "2"
  },
  "method": "POST"
});
		return {text:text,href:href}
	}); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'maxPageId' of undefined
    at <eval> (<input>:4:24)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:247)
` | source[729].ruleToc.nextTocUrl = @js:
baseUrl = String(baseUrl).split('1/200')[0];
list = [];
page=JSON.parse(String(result)).tracks.maxPageId
for(var i = 2; i < 10; ++i){
list.push(String(baseUrl +i+"/200"));
}
list<br>source[733].ruleToc.nextTocUrl = @js:
baseUrl = String(baseUrl).split('1/200')[0];
list = [];
page=JSON.parse(String(result)).tracks.maxPageId
for(var i = 2; i < 10; ++i){
list.push(String(baseUrl +i+"/200"));
}
list<br>source[739].ruleToc.nextTocUrl = @js:
baseUrl = String(baseUrl).split('1/200')[0];
list = [];
page=JSON.parse(String(result)).tracks.maxPageId
for(var i = 2; i < 10; ++i){
list.push(String(baseUrl +i+"/200"));
}
list |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'split' of undefined
    at <eval> (<input>:2:19)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1484)
` | source[110].ruleContent.content = <js>
var arr = baseUrl.split('/');
var songtype = arr[3];
var songid = arr[4].split('.')[0];
/*
var data = java.ajax("http://service.5sing.kugou.com/song/getSongUrl?version=6.6.70&songid=" + songid + "&songtype=" + songtype);
data = JSON.parse(data).data
data.squrl \|\| data.hqurl \|\| data.lqurl;
*/
	
let params = {
  appid: 3146,
  clienttime: Math.ceil(Date.now() / 1000),
  clientver: 610850,
  dfid: "-",
  from: "com.sing.client.player",
  mid: 114514,
  songfields: "ID,SN,SK,SW,SS,ST,SI,CT,M,S,ZQ,WO,ZC,HY,YG,CK,D,RQ,DD,E,R,RC,SG,C,CS,LV,LG,SY,UID,PT,SCSR,SC,KM5",
  songid: songid,
  songtype: songtype,
  token: "",
  userfields: "ID,NN,I,YCRQ,FCRQ",
  uuid: "-"
}

let signstr = ""
let keys = Object.keys(params).sort()
for (let i = 0; i < keys.length; i++) {
  signstr += `${keys[i]}=${params[keys[i]]}`
}
let signature = java.md5Encode("UqgPMZpjgRZQ7s8JAuUIP5DQdo5O5NB" + signstr + "UqgPMZpjgRZQ7s8JAuUIP5DQdo5O5NB")

let querys = []
for (let i = 0; i < keys.length; i++) {
  querys.push(`${keys[i]}=${params[keys[i]]}`.replace(/\,/g, "%2c"))
}
let querystr = querys.join("&") + "&signature=" + signature
let request = java.ajax("https://5sapi.kugou.com/song/getSongUrl?" + querystr)
// java.log(request)
let data = JSON.parse(request).data
data.squrl \|\| data.squrl_backup \|\| data.hqurl \|\| data.hqurl_backup \|\| data.lqurl \|\| data.lqurl_backup
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:13:23)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:834)
` | source[45].ruleContent.content = <js>
s="2c6689f91ee4d4e87d798397d47310ebbe1dad79ixdzs";
for (i = 0, h = ""; i < 8; i++) {
	x="abcdefghijklmnopqrstuvwxyz0123456789";
	r = parseInt(Math.random() * (36 - 0 +1) + 0, 10);
	h += x.charAt(r);
}
a=String(Math.round(new Date()));
sha = s + h + a + h;
sha1 = java.digestHex(sha,'SHA-1');

b = baseUrl.match(/"chapterId":(\d+),"bookId":"(\d+)"/)
u=source.getKey().match(/([^\#]+)\#/)[1];
url = u + "/chapter/content,";
post=`{
  "method": "POST",
  "body":'{"chapterId": ${b[1]},"bookId":"${b[2]}"}',
  "headers":{"checkSumDTO":'{"appid":"ixdzs","checksum":"${sha1}","curtime":"${a}","nonce":"${h}"}'}
}`

result = java.ajax(url+post);
</js>$.data.chapter.chapterContent |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:14:11)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:809)
` | source[63].ruleContent.content = <js>
s="2c6689f91ee4d4e87d798397d47310ebbe1dad79ixdzs";
for (i = 0, h = ""; i < 8; i++) {
	x="abcdefghijklmnopqrstuvwxyz0123456789";
	r = parseInt(Math.random() * (36 - 0 +1) + 0, 10);
	h += x.charAt(r);
}
a=String(Math.round(new Date()));
sha = s + h + a + h;
sha1 = java.digestHex(sha,'SHA-1');

b = baseUrl.match(/"chapterId":(\d+),"bookId":"(\d+)"/)
u=source.getKey();
url = u + "/chapter/content,";
post=`{
  "method": "POST",
  "body":'{"chapterId": ${b[1]},"bookId":"${b[2]}"}',
  "headers":{"checkSumDTO":'{"appid":"ixdzs","checksum":"${sha1}","curtime":"${a}","nonce":"${h}"}'}
}`

result = java.ajax(url+post);
</js>$.data.chapter.chapterContent |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:14)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:106)
` | source[53].ruleToc.chapterList = <js>
result=result.match(/Html = "(.*?)"/)[1]
unescape(result)
</js>li a |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:15)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:150)
` | source[12].ruleToc.chapterList = <js>
String(result).match(/<div class="clearfix"><\/div>([\s\S]*?)<div class="clearfix"><\/div>/)[1];
</js>
@@h3,li |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:16)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:85)
` | source[880].ruleBookInfo.tocUrl = /ajaxService?action=chapterlist&articleno={{String(baseUrl).match(/\d+(?=\.)/)[0]}}&index=0&size=99999&sort=1<br>source[928].ruleBookInfo.tocUrl = /ajaxService?action=chapterlist&articleno={{String(baseUrl).match(/\d+(?=\.)/)[0]}}&index=0&size=99999&sort=1<br>source[933].ruleBookInfo.tocUrl = /ajaxService?action=chapterlist&articleno={{String(baseUrl).match(/\d+(?=\.)/)[0]}}&index=0&size=99999&sort=1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:18)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:299)
` | source[694].ruleContent.content = @js:var json = result.match(/window.comicInfo=(.*?),window.hideguide/)[1];
var json = eval('json='+json)
var image_list = [];
json.current_chapter.chapter_img_list.map(item=>{
    image_list.push('<img src="'+item+'">')
})
image_list.join("\n");<br>source[710].ruleContent.content = @js:var json = result.match(/window.comicInfo=(.*?),window.hideguide/)[1];
var json = eval('json='+json)
var image_list = [];
json.current_chapter.chapter_img_list.map(item=>{
    image_list.push('<img src="'+item+'">')
})
image_list.join("\n"); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:18)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:374)
` | source[213].ruleContent.content = @js:
var pUrl = result.match(/"url_next":"([^"]*)"/)
pUrl = pUrl[1].replace(/\\/g, "");
if (cache.get(book.name+title)){
    try{
        java.startBrowserAwait(pUrl, title);
    }catch(err){
       "其实...1+1=2!"
    }
}else{
    cache.put(book.name+title,true, 31536000);
}
"请刷新查看内容"; |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:19)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:150)
` | source[321].ruleSearch.lastChapter = <js>
url=String(result).match(/="newWebView\('([^']+)'/)[1];url='https://m.suixkan.com'+url
java.ajax(url)</js>
class.chapter-entrance@text<br>source[66].ruleSearch.lastChapter = <js>
url=String(result).match(/="newWebView\('([^']+)'/)[1];url='https://m.suixkan.com'+url
java.ajax(url)</js>
class.chapter-entrance@text |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:45)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:247)
` | source[78].ruleContent.content = <js>
result=String(src).trim().replace(/\s+/g,'').match(/(let\|var\|const)+urls=(\[.*?\])/g)[0].replace(/(let\|var\|const)+urls=/g,"")

result=JSON.parse(result).map(n=>`<img src="${n}" />`).join("")
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:56)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:130)
` | source[740].ruleContent.content = @js:'http://music.163.com/song/media/outer/url?id='+baseUrl.match(/id=(\d+)/)[1]+'.mp3'<br>source[741].ruleContent.content = @js:'http://music.163.com/song/media/outer/url?id='+baseUrl.match(/id=(\d+)/)[1]+'.mp3'<br>source[742].ruleContent.content = @js:'http://music.163.com/song/media/outer/url?id='+baseUrl.match(/id=(\d+)/)[1]+'.mp3' |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:7)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:123)
` | source[131].ruleToc.chapterList = <js>result.match(/flip.setData\('imgData',( [\s\S]+?,\{\}\]\})\)\;/)[1];
</js>$.data[*]<br>source[133].ruleToc.chapterList = <js>result.match(/flip.setData\('imgData',( [\s\S]+?,\{\}\]\})\)\;/)[1];
</js>$.data[*]<br>source[747].ruleToc.chapterList = <js>result.match(/flip.setData\('imgData',( [\s\S]+?,\{\}\]\})\)\;/)[1];
</js>$.data[*] |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:8)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:74)
` | source[157].ruleBookInfo.tocUrl = {{java.put("url",baseUrl);
	"https://ixdzs.tw/novel/clist/"}},{
  "body": "bid={{baseUrl.match(/(\d+).$/)[1]}}",
  "method": "POST"
} |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:2:14)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:398)
` | source[303].ruleToc.chapterList = <js>
n=result.match(/第1页 \/ 共(\d+) 页/)[1];
var list=[];
var n = parseInt(n);
ul=java.getString('ul a.0@href');
ui=baseUrl.match(/(.*\/\/[^\/]+)\//)[1];

for(var i=1;i<=n;i++)
{
	url=ui+String(ul).replace(/page=1/,"page="+i);
	me='第'+i+'页';
	list.push({"text":me,"href":url});
}
list
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:2:19)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:547)
` | source[117].ruleToc.chapterList = @js:
var bid = baseUrl.match(/read\/(\d+)/)[1];
var resp = java.post(source.getKey()+"/novel/clist/","bid="+bid,{});
var json = JSON.parse(resp.body()).data;
var page = 0;
var n = "";
for(var i=json.length - 1; i >= 0; i--){
	if(json[i].ctype === "1"){
		n = json[i].title;
		json.splice(i, 1);
		continue;
		}
	page = json[i]["ordernum"];
	json[i]["url"] = source.getKey() + "/read/" + bid + "/p" + page + ".html";
	json[i]["n"] = n;
	}
json; |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:2:20)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:456)
` | source[698].ruleContent.content = @js:
comicId=result.match(/var comicId = (\d+)/)[1];
chapterId=result.match(/var chapterId=(\d+)/)[1];
nv=result.match(/var nv = "(.*?)"/)[1];
url='https://mm.sfacg.com/ajax/Common.ashx?op=getPics&cid='+comicId+'&chapId='+chapterId+'&serial=ZP&path='+nv
//java.ajax(url)
result=JSON.parse(java.ajax(url)).data
result.map(x=>'<img src=\"'+x+'\">').join("\n") |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:2:23)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:278)
` | source[108].ruleSearch.bookUrl = <js>
id=String(result).match(/id=(\d+)/)[1];
if(!String(result).match(/catalog_name/)){
result='https://www.missevan.com/sound/getsound?soundid='+id
}else{result='https://www.missevan.com/dramaapi/getdrama?drama_id='+id}
</js><br>source[356].ruleSearch.bookUrl = <js>
id=String(result).match(/id=(\d+)/)[1];
if(!String(result).match(/catalog_name/)){
result='https://www.missevan.com/sound/getsound?soundid='+id
}else{result='https://www.missevan.com/dramaapi/getdrama?drama_id='+id}
</js><br>source[728].ruleSearch.bookUrl = <js>
id=String(result).match(/id=(\d+)/)[1];
if(!String(result).match(/catalog_name/)){
result='https://www.missevan.com/sound/getsound?soundid='+id
}else{result='https://www.missevan.com/dramaapi/getdrama?drama_id='+id}
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:2:42)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:129)
` | source[226].ruleBookInfo.tocUrl = @js:
var id = baseUrl.match(/(\d+)/);
"https://www.tadu.com/book/catalogue/" + id[1] |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:2:45)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:240)
` | source[131].ruleSearch.bookList = @js:list=[{title:decodeURIComponent(baseUrl.match(/word=(.*)/)[1]),url:baseUrl,img:"https://ss2.bdstatic.com/70cFvnSh_Q1YnxGkpoWK1HF6hhy/it/u=1105349167,1082680177&fm=11&gp=0.jpg"}]<br>source[133].ruleSearch.bookList = @js:list=[{title:decodeURIComponent(baseUrl.match(/word=(.*)/)[1]),url:baseUrl,img:"https://ss2.bdstatic.com/70cFvnSh_Q1YnxGkpoWK1HF6hhy/it/u=1105349167,1082680177&fm=11&gp=0.jpg"}]<br>source[747].ruleSearch.bookList = @js:list=[{title:decodeURIComponent(baseUrl.match(/word=(.*)/)[1]),url:baseUrl,img:"https://ss2.bdstatic.com/70cFvnSh_Q1YnxGkpoWK1HF6hhy/it/u=1105349167,1082680177&fm=11&gp=0.jpg"}] |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:3:21)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:174)
` | source[186].ruleToc.chapterName = html@js:if(result.match(/isvip/)){
result="🔒"+result.match(/>([^<]+)<\/a>/)[1];
}else{result=result.match(/>([^<]+)<\/a>/)[1];} |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:3:21)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:422)
` | source[702].ruleContent.content = @js:
host='http://mhpic.zymkcdn.com/comic/';
original=result.match(/dr_original:"([^"]+)"/)[1];
end=result.match(/end_var:(\d+)/)[1];
//画质
high='.jpg-zymk.high.webp';
low='.jpg-zymk.low.webp';
middle='.jpg-zymk.middle.webp';

html='';
for(i=1;i<=end;i++){
html+='<img src="'+host+original+i+high+'">\n'
}
html<br>source[712].ruleContent.content = @js:
host='http://mhpic.zymkcdn.com/comic/';
original=result.match(/dr_original:"([^"]+)"/)[1];
end=result.match(/end_var:(\d+)/)[1];
//画质
high='.jpg-zymk.high.webp';
low='.jpg-zymk.low.webp';
middle='.jpg-zymk.middle.webp';

html='';
for(i=1;i<=end;i++){
html+='<img src="'+host+original+i+high+'">\n'
}
html |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:3:31)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1473)
` | source[27].ruleContent.content = <js>
b=baseUrl.match(/gid=(\d+)&nid=(\d+)&sort=(\d+)&chapter_name=(.*$)/);
t = String(Math.round(new Date()));
gid=b[1];
nid=b[2];
s = b[3];
c = b[4];
snk=java.md5Encode("a=1&ac=999&appType=0&appid=10001&appverion=508500&autoBuy=0&bidType=0&birt=1706674841000&ch=blf1298_10928_001&chType=6&chapter_name="+c+"&cid=eef_easou_book&dzh=1&gender=1&gid="+gid+"&gsort=0&instId="+t+"&instime="+t+"&nid="+nid+"&os=android&pr=-1.0&ptype=5&pushid=7b4aaf1210a5bdbac3cea26d5030a419&recSw=1&rtype=2&scp=0&sequence=1&session_id=153F4EEE16F56A43FD63ZD21B866413ED9BE044EFB876A115C62DBED82EE4C824D&sgsort=0&sort="+s+"&tm=0&udid=3d3ec742930b635fc4c61f0575dbc4d2939edbe2&userInitPay=3&utype=0&vm=5.8.5&key=EaSoU0517+PuBlIsHkEy-JRKKOWTUNZCNTWY-");
snk = snk.toUpperCase();

url=`https://api.ieasou.com/api/bookapp/chargeChapter.m?a=1&autoBuy=0&cid=eef_easou_book&vm=5.8.5&os=android&udid=3d3ec742930b635fc4c61f0575dbc4d2939edbe2&appverion=508500&ch=blf1298_10928_001&session_id=153F4EEE16F56A43FD63ZD21B866413ED9BE044EFB876A115C62DBED82EE4C824D&dzh=1&scp=0&appid=10001&utype=0&rtype=2&pushid=7b4aaf1210a5bdbac3cea26d5030a419&ptype=5&gender=1&userInitPay=3&birt=1706674841000&instime=${t}&instId=${t}&chType=6&bidType=0&recSw=1&appType=0&pr=-1.0&tm=0&ac=999&gid=${gid}&nid=${nid}&sort=${s}&gsort=0&sgsort=0&sequence=1&chapter_name=${c}&snk=${snk}`;
java.ajax(url)
</js>$.content<js>
function a(str) {
	str = String(str);
	length = str.length / 2;
	bArr = [];
	for (i = 0; i < length; i++) {
		i2 = i * 2;
		bArr[i] =  intToByte(parseInt(str.substring(i2, i2 + 2), 16));
  }
  return bArr;
}
function intToByte(i) {
	var b = i & 0xFF;var c = 0;
	if (b >= 128) {c = b % 128;c = -1 * (128 - c);}
	else {c = b;}
	return c;
}

 function byteToHexString(arr) {
    return arr.reduce((accu, item) => {
        let a =(item&0xff).toString(16)
        if (a.length ==1) a = "0" + a
        return accu += a
    }, "")
  }
  function hextoBase64(t) {
	function parse(t) {
		t = String(t)
		for (var e = t.length, r = [], i = 0; i < e; i += 2){
		r[i >>> 3] \|= parseInt(t.substr(i, 2), 16) << 24 - i % 8 * 4;
		}
		return r;
	}
	function stringify(t) {
		var e = t
		, r = 4 * t.length
		, i = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
		for (var n = [], o = 0; o < r; o += 3){
		    for (var s = (e[o >>> 2] >>> 24 - o % 4 * 8 & 255) << 16 \| (e[o + 1 >>> 2] >>> 24 - (o + 1) % 4 * 8 & 255) << 8 \| e[o + 2 >>> 2] >>> 24 - (o + 2) % 4 * 8 & 255, a = 0; a < 4 && o + .75 * a < r; a++){
			n.push(i.charAt(s >>> 6 * (3 - a) & 63));}
		}
		var c = i.charAt(64);
		if (c){
		   for (; n.length % 4; ){
			      n.push(c);}
		}
		return n.join("")
	}
	return stringify(parse(t));
}

sf = "DES/CBC/PKCS5Padding";
d=hextoBase64(byteToHexString(a(result)));
key = iv = "EaSoUcNt";
result=java.aesBase64DecodeToString(d,key,sf,iv);
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:4:13)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:348)
` | source[695].ruleContent.content = @js:header={"Referer":baseUrl};
headers={"headers":JSON.stringify(header)};
eval(result.match(/eval(.*?)\{\}\)\)/)[0])
var image_list = []
newImgs.map(item=>{
    image_list.push('<img src="'+item+','+JSON.stringify(headers)+'">')
})
image_list.join("\n")<br>source[697].ruleContent.content = @js:header={"Referer":baseUrl};
headers={"headers":JSON.stringify(header)};
eval(result.match(/eval(.*?)\{\}\)\)/)[0])
var image_list = []
newImgs.map(item=>{
    image_list.push('<img src="'+item+','+JSON.stringify(headers)+'">')
})
image_list.join("\n") |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:5:17)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:293)
` | source[343].ruleToc.chapterUrl = @js:
headers = "";
link_Urlpath = "/api/v1/chapter/content?";
link_Params = {
		"id": baseUrl.match(/id=(\d+)/)[1],
		"chapterId":"{{$.id}}"
	};
eval(String(source.bookSourceComment)); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:5:32)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:267)
` | source[718].ruleContent.content = <js>
var url="";
if(/.*\.m3u8.*/.test(baseUrl)){url=baseUrl}
else{
jm=java.htmlFormat(result).match(/.*(\{.*\}).*/)[1];
url=baseUrl.match(/(.*)\/share.*/)[1]+jm.match(/.*url":"(.*)"\}.*/)[1];
}
url
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: creatRequest is not defined
    at <eval> (<input>:10:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:252)
` | source[34].ruleToc.chapterUrl = @js:
let bid = "{{$.bid}}"
,cid = "{{$.cid}}"
,t = Date.now().toString()
params = {
	bid:bid,
	cid:cid,
	t:t
}
creatRequest(
	"https://qcbook.taoyuewenhua.net/tf/chapter_content",
	params
) |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: expecting ';'
    at <input>:2:5
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:428)
` | source[289].ruleContent.content = @js:dt=lr='';c=1;动=java.get("动");
if(动!=''&&!~baseUrl.indexOf(",")){
result=String(java.ajax(baseUrl+动))
}else if(java.get("静")==动)c=dt=2;

r=org.jsoup.Jsoup.parse(r1=result.replace(/&nbsp;/g,' '));
查=i=java.get("序");

if(java.get("文")==1){for(;c;c--){
d=["img[data-src],img[src~=[^a-z]cid[^a-z]]"
,"img:not([src~=(?i)^$\|^javascript:\|\\.gif\|\\.png\|[^a-z](cover\|css\|ic(on)?\|load(ing\|ed)?)[^a-z]])"
,"img[src~=(?i)\\.png]:not([src~=(?i)[^a-z](cover\|css\|ic(on)?\|load(ing\|ed)?)[^a-z]])"
,'img[src~=(?i)\\.gif]:not([src~=(?i)[^a-z](cover\|css\|ic(on)?\|load(ing\|ed)?)[^a-z]])'];

if(!(查!=""&&(查!=-1&&((lr=r.select(d[查])).size(),true)))){
$=it=>(lr=r.select(it)).size();
if($(d[i=0])\|\|$(d[i=1])\|\|$(d[i=2])\|\|false)break;

if(c==2){dt=1;
r=org.jsoup.Jsoup.parse(java.ajax(baseUrl+',{"webView":true}'))
}else i=$(d[3])?3:-1}}
if(i==0)lr=String(lr).replace(/(?:src=['"][^'"]+['"] +)?data-/g,'');
if(查=="")book.putVariable("序",i);

}else{
sc=java.get("文")==2?
'[style~=(?i)text-align:center\|(^\| \|;)color: *(rgb.(?!255[ ,]+255[ ,]+255)[\\d, ]*2\\d\\d\|#(?=[a-f\\d]{3}([^a-f\\d]\|$))(?!fff)[a-f\\d]*[d-f]\|#(?=[a-f\\d]{4})(?!ffffff)([\\da-f]{2})*[d-f][\\da-f]\|green\|red\|blue\|yellow\|purple\|pink\|brown)],script,noscript,style,header,footer,[class~=^foot\|^head],[id~=^foot\|^head],:has(>a):not(:has(p:matchesOwn(\\S),br)),a>*,:has(a):not(:matchesOwn([\\S\\s]{50,}),:has(:matchesOwn([\\S\\s]{50,}))),:matchesOwn([\\s\\S]{50})>:not(br,a,:matchesOwn([\\s\\S]{50})),:not(br,p,a,:matches([\\s\\S]{200}),:has(p,br,div:matchesOwn(，\|。)+div:matchesOwn(，\|。)),:has(p,br,div:matchesOwn(，\|。)+div:matchesOwn(，\|。)) :matchesOwn(\\S):not(:not(p,div,span:has(br))))'
:
'[style~=(?i)text-align:center\|(^\| \|;)color: *(rgb.(?!255[ ,]+255[ ,]+255)[\\d, ]*2\\d\\d\|#(?=[a-f\\d]{3}([^a-f\\d]\|$))(?!fff)[a-f\\d]*[d-f]\|#(?=[a-f\\d]{4})(?!ffffff)([\\da-f]{2})*[d-f][\\da-f]\|green\|red\|blue\|yellow\|purple\|pink\|brown)],script,noscript,style,header,footer,[class~=^foot\|^head],[id~=^foot\|^head],:has(>a):not(:has(p:matchesOwn(\\S),br,img:not([src~=(?i)^$\|^javascript:\|[^a-z](css\|ic(on)?\|load(ing\|ed)?)[^a-z]\|/\\d+s\\.jpg]))),img[src~=(?i)^$\|^javascript:\|[^a-z](css\|ic(on)?\|load(ing\|ed)?)[^a-z]\|/\\d+s\\.jpg],a:not(:matches(^$)>img)>*,:has(a):not(img,:matchesOwn([\\S\\s]{50,}),:has(img,:matchesOwn([\\S\\s]{50,}))),:matchesOwn([\\s\\S]{50})>:not(img,br,a,:has(img),:matchesOwn([\\s\\S]{50})),:not(img,br,p,a,:matches([\\s\\S]{200}),:has(p,br,img,div:matchesOwn(，\|。)+div:matchesOwn(，\|。)),:has(img,p,br,div:matchesOwn(，\|。)+div:matchesOwn(，\|。)) :matchesOwn(\\S):not(:not(p,div,span:has(br))))';

d=[":matchesOwn(\\S):has(br):has(:matchesOwn(\\S):has(br))"
,":matchesOwn(\\S):has(br)"
,":has(>:matchesOwn(\\S):not(:has(*))+:matchesOwn(\\S):not(:has(*)))"
,":has(>:has(>p:only-child:matchesOwn(\\S):not(:has(*)))+:has(>p:only-child:matchesOwn(\\S):not(:has(*))))"
,"img"
,":matchesOwn(\\S)"];

try{for(查=i!=''?i:java.get("元");c;c--){

if(!(c>1&&(String(r.text()).length<400\|\|r.select(':matchesOwn(内容未加载完成\|关闭(阅读\|小说)模式)').size()))){
r.select(sc).remove();

if(!(查!=""&&(lr=r.select(i?d[i==6?5:i]:查)).size()))for(i=0;i<6&&(lr=r.select(d[i]),i==4&&c==1?!lr.size():String(lr.text()).length<200);i++);
if(c<2\|\|i<6)break;}

r=org.jsoup.Jsoup.parse(r2=String(java.ajax(baseUrl+',{"webView":true}')).replace(/(<[a-z]+)&nbsp;/g,'$1 '));
dt=r1.length==r2.length?2:1}

for(c=lr.first(),v=1;v<lr.size();v++)if(lr.get(v).parents().contains(c)){
lr.remove(v);
v--}else c=lr.get(v);

lr=String((c=lr.size()==2&&i<4)?String(lr.first().text()).length>String(lr.get(1).text()).length?lr.first():lr.get(1):(c=lr.size()==1)?lr.first():lr);

if(查==""){
if(c&&(查=lr.match(/<([a-z]+) ([^>]+)>/))&&(查[2]=查[2].match(/(?:id\|class\|style)=(?:"[^"]+"\|'[^']+')\|[^= ]+(?=="[^"]+"\|'[^']+')/g))){
book.putVariable("元",查[1]+'['+查[2].join('][')+']')
}else book.putVariable("序",i)}

lr=lr.replace(/<([a-z]+)[^>]*"-\d+"[^>]*>[^<]+<\/\1>\|[^<>]*<a[^<]+<\/a>[^<]*\|&lt[; ]?\/?[a-z]+(?= \|\/?&gt)(?:[ a-z=-]+\|"[^"]+"\|'[^']+')*\/?&gt[; ]?\|[☯📑⚙️🌕︴]/g,"").replace(/\s+(?:\s\|(?:(?:(?:n?b)?s)?p)?;)/g,"　　");

if(java.get("原")!=1)lr=(!lr.indexOf("　　")?lr.replace(/>(?!　　\|\s*(?:(?:(?:n?b)?s)?p)?;)\s*(?=[^\s<>])/g,">︴"):lr)
.replace(/((?:[〖【『「（《〈〔［\[(][^〖【『「（《〈〔［\[()］〕〉》）」』】〗\]]*[)］〕〉》）」』】〗\]]\s*)*(?:第?\s*[一二三四五六七八九十百千万〇零0-9]+\s*[章节回話话：:.．,，、]*\s*)?{{
n=(t=title.match(/\S+$/)[0].replace(/[*$\|?+\\\^\[\](){}/]/g,".?")).replace(/^(正文[^\u4e00-\u9fa5A-Za-z]*\|第?[一二三四五六七八九十百千万〇零0-9]+[章节回話话\s：:.．,，、]*)+/,""),n!=t&&/\S/.test(n)?"(?:第?\\s*[一二三四五六七八九十百千万〇零0-9]+\\s*[章节回話话：:.．,，、]*\\s*"+n+"\|"+t+")":t
}}(?:\s*[〖【『「（《〈〔［\[(][^〖【『「（《〈〔［\[()］〕〉》）」』】〗\]]*[)］〕〉》）」』】〗\]])*)/g,"⚙️$1⚙️")+"📑"

}catch(e){}}
if(dt)book.putVariable(dt==1?"动":"静",',{"webView":true}');lr<br>source[295].ruleContent.content = @js:dt=lr='';c=1;动=java.get("动");
if(动!=''&&!~baseUrl.indexOf(",")){
result=String(java.ajax(baseUrl+动))
}else if(java.get("静")==动)c=dt=2;

r=org.jsoup.Jsoup.parse(r1=result.replace(/&nbsp;/g,' '));
查=i=java.get("序");

if(java.get("文")==1){for(;c;c--){
d=["img[data-src],img[src~=[^a-z]cid[^a-z]]"
,"img:not([src~=(?i)^$\|^javascript:\|\\.gif\|\\.png\|[^a-z](cover\|css\|ic(on)?\|load(ing\|ed)?)[^a-z]])"
,"img[src~=(?i)\\.png]:not([src~=(?i)[^a-z](cover\|css\|ic(on)?\|load(ing\|ed)?)[^a-z]])"
,'img[src~=(?i)\\.gif]:not([src~=(?i)[^a-z](cover\|css\|ic(on)?\|load(ing\|ed)?)[^a-z]])'];

if(!(查!=""&&(查!=-1&&((lr=r.select(d[查])).size(),true)))){
$=it=>(lr=r.select(it)).size();
if($(d[i=0])\|\|$(d[i=1])\|\|$(d[i=2])\|\|false)break;

if(c==2){dt=1;
r=org.jsoup.Jsoup.parse(java.ajax(baseUrl+',{"webView":true}'))
}else i=$(d[3])?3:-1}}
if(i==0)lr=String(lr).replace(/(?:src=['"][^'"]+['"] +)?data-/g,'');
if(查=="")book.putVariable("序",i);

}else{
sc=java.get("文")==2?
'[style~=(?i)text-align:center\|(^\| \|;)color: *(rgb.(?!255[ ,]+255[ ,]+255)[\\d, ]*2\\d\\d\|#(?=[a-f\\d]{3}([^a-f\\d]\|$))(?!fff)[a-f\\d]*[d-f]\|#(?=[a-f\\d]{4})(?!ffffff)([\\da-f]{2})*[d-f][\\da-f]\|green\|red\|blue\|yellow\|purple\|pink\|brown)],script,noscript,style,header,footer,[class~=^foot\|^head],[id~=^foot\|^head],:has(>a):not(:has(p:matchesOwn(\\S),br)),a>*,:has(a):not(:matchesOwn([\\S\\s]{50,}),:has(:matchesOwn([\\S\\s]{50,}))),:matchesOwn([\\s\\S]{50})>:not(br,a,:matchesOwn([\\s\\S]{50})),:not(br,p,a,:matches([\\s\\S]{200}),:has(p,br,div:matchesOwn(，\|。)+div:matchesOwn(，\|。)),:has(p,br,div:matchesOwn(，\|。)+div:matchesOwn(，\|。)) :matchesOwn(\\S):not(:not(p,div,span:has(br))))'
:
'[style~=(?i)text-align:center\|(^\| \|;)color: *(rgb.(?!255[ ,]+255[ ,]+255)[\\d, ]*2\\d\\d\|#(?=[a-f\\d]{3}([^a-f\\d]\|$))(?!fff)[a-f\\d]*[d-f]\|#(?=[a-f\\d]{4})(?!ffffff)([\\da-f]{2})*[d-f][\\da-f]\|green\|red\|blue\|yellow\|purple\|pink\|brown)],script,noscript,style,header,footer,[class~=^foot\|^head],[id~=^foot\|^head],:has(>a):not(:has(p:matchesOwn(\\S),br,img:not([src~=(?i)^$\|^javascript:\|[^a-z](css\|ic(on)?\|load(ing\|ed)?)[^a-z]\|/\\d+s\\.jpg]))),img[src~=(?i)^$\|^javascript:\|[^a-z](css\|ic(on)?\|load(ing\|ed)?)[^a-z]\|/\\d+s\\.jpg],a:not(:matches(^$)>img)>*,:has(a):not(img,:matchesOwn([\\S\\s]{50,}),:has(img,:matchesOwn([\\S\\s]{50,}))),:matchesOwn([\\s\\S]{50})>:not(img,br,a,:has(img),:matchesOwn([\\s\\S]{50})),:not(img,br,p,a,:matches([\\s\\S]{200}),:has(p,br,img,div:matchesOwn(，\|。)+div:matchesOwn(，\|。)),:has(img,p,br,div:matchesOwn(，\|。)+div:matchesOwn(，\|。)) :matchesOwn(\\S):not(:not(p,div,span:has(br))))';

d=[":matchesOwn(\\S):has(br):has(:matchesOwn(\\S):has(br))"
,":matchesOwn(\\S):has(br)"
,":has(>:matchesOwn(\\S):not(:has(*))+:matchesOwn(\\S):not(:has(*)))"
,":has(>:has(>p:only-child:matchesOwn(\\S):not(:has(*)))+:has(>p:only-child:matchesOwn(\\S):not(:has(*))))"
,"img"
,":matchesOwn(\\S)"];

try{for(查=i!=''?i:java.get("元");c;c--){

if(!(c>1&&(String(r.text()).length<400\|\|r.select(':matchesOwn(内容未加载完成\|关闭(阅读\|小说)模式)').size()))){
r.select(sc).remove();

if(!(查!=""&&(lr=r.select(i?d[i==6?5:i]:查)).size()))for(i=0;i<6&&(lr=r.select(d[i]),i==4&&c==1?!lr.size():String(lr.text()).length<200);i++);
if(c<2\|\|i<6)break;}

r=org.jsoup.Jsoup.parse(r2=String(java.ajax(baseUrl+',{"webView":true}')).replace(/(<[a-z]+)&nbsp;/g,'$1 '));
dt=r1.length==r2.length?2:1}

for(c=lr.first(),v=1;v<lr.size();v++)if(lr.get(v).parents().contains(c)){
lr.remove(v);
v--}else c=lr.get(v);

lr=String((c=lr.size()==2&&i<4)?String(lr.first().text()).length>String(lr.get(1).text()).length?lr.first():lr.get(1):(c=lr.size()==1)?lr.first():lr);

if(查==""){
if(c&&(查=lr.match(/<([a-z]+) ([^>]+)>/))&&(查[2]=查[2].match(/(?:id\|class\|style)=(?:"[^"]+"\|'[^']+')\|[^= ]+(?=="[^"]+"\|'[^']+')/g))){
book.putVariable("元",查[1]+'['+查[2].join('][')+']')
}else book.putVariable("序",i)}

lr=lr.replace(/<([a-z]+)[^>]*"-\d+"[^>]*>[^<]+<\/\1>\|[^<>]*<a[^<]+<\/a>[^<]*\|&lt[; ]?\/?[a-z]+(?= \|\/?&gt)(?:[ a-z=-]+\|"[^"]+"\|'[^']+')*\/?&gt[; ]?\|[☯📑⚙️🌕︴]/g,"").replace(/\s+(?:\s\|(?:(?:(?:n?b)?s)?p)?;)/g,"　　");

if(java.get("原")!=1)lr=(!lr.indexOf("　　")?lr.replace(/>(?!　　\|\s*(?:(?:(?:n?b)?s)?p)?;)\s*(?=[^\s<>])/g,">︴"):lr)
.replace(/((?:[〖【『「（《〈〔［\[(][^〖【『「（《〈〔［\[()］〕〉》）」』】〗\]]*[)］〕〉》）」』】〗\]]\s*)*(?:第?\s*[一二三四五六七八九十百千万〇零0-9]+\s*[章节回話话：:.．,，、]*\s*)?{{
n=(t=title.match(/\S+$/)[0].replace(/[*$\|?+\\\^\[\](){}/]/g,".?")).replace(/^(正文[^\u4e00-\u9fa5A-Za-z]*\|第?[一二三四五六七八九十百千万〇零0-9]+[章节回話话\s：:.．,，、]*)+/,""),n!=t&&/\S/.test(n)?"(?:第?\\s*[一二三四五六七八九十百千万〇零0-9]+\\s*[章节回話话：:.．,，、]*\\s*"+n+"\|"+t+")":t
}}(?:\s*[〖【『「（《〈〔［\[(][^〖【『「（《〈〔［\[()］〕〉》）」』】〗\]]*[)］〕〉》）」』】〗\]])*)/g,"⚙️$1⚙️")+"📑"

}catch(e){}}
if(dt)book.putVariable(dt==1?"动":"静",',{"webView":true}');lr<br>source[382].ruleContent.content = @js:dt=lr='';c=1;动=java.get("动");
if(动!=''&&!~baseUrl.indexOf(",")){
result=String(java.ajax(baseUrl+动))
}else if(java.get("静")==动)c=dt=2;

r=org.jsoup.Jsoup.parse(r1=result.replace(/&nbsp;/g,' '));
查=i=java.get("序");

if(java.get("文")==1){for(;c;c--){
d=["img[data-src],img[src~=[^a-z]cid[^a-z]]"
,"img:not([src~=(?i)^$\|^javascript:\|\\.gif\|\\.png\|[^a-z](cover\|css\|ic(on)?\|load(ing\|ed)?)[^a-z]])"
,"img[src~=(?i)\\.png]:not([src~=(?i)[^a-z](cover\|css\|ic(on)?\|load(ing\|ed)?)[^a-z]])"
,'img[src~=(?i)\\.gif]:not([src~=(?i)[^a-z](cover\|css\|ic(on)?\|load(ing\|ed)?)[^a-z]])'];

if(!(查!=""&&(查!=-1&&((lr=r.select(d[查])).size(),true)))){
$=it=>(lr=r.select(it)).size();
if($(d[i=0])\|\|$(d[i=1])\|\|$(d[i=2])\|\|false)break;

if(c==2){dt=1;
r=org.jsoup.Jsoup.parse(java.ajax(baseUrl+',{"webView":true}'))
}else i=$(d[3])?3:-1}}
if(i==0)lr=String(lr).replace(/(?:src=['"][^'"]+['"] +)?data-/g,'');
if(查=="")book.putVariable("序",i);

}else{
sc=java.get("文")==2?
'[style~=(?i)text-align:center\|(^\| \|;)color: *(rgb.(?!255[ ,]+255[ ,]+255)[\\d, ]*2\\d\\d\|#(?=[a-f\\d]{3}([^a-f\\d]\|$))(?!fff)[a-f\\d]*[d-f]\|#(?=[a-f\\d]{4})(?!ffffff)([\\da-f]{2})*[d-f][\\da-f]\|green\|red\|blue\|yellow\|purple\|pink\|brown)],script,noscript,style,header,footer,[class~=^foot\|^head],[id~=^foot\|^head],:has(>a):not(:has(p:matchesOwn(\\S),br)),a>*,:has(a):not(:matchesOwn([\\S\\s]{50,}),:has(:matchesOwn([\\S\\s]{50,}))),:matchesOwn([\\s\\S]{50})>:not(br,a,:matchesOwn([\\s\\S]{50})),:not(br,p,a,:matches([\\s\\S]{200}),:has(p,br,div:matchesOwn(，\|。)+div:matchesOwn(，\|。)),:has(p,br,div:matchesOwn(，\|。)+div:matchesOwn(，\|。)) :matchesOwn(\\S):not(:not(p,div,span:has(br))))'
:
'[style~=(?i)text-align:center\|(^\| \|;)color: *(rgb.(?!255[ ,]+255[ ,]+255)[\\d, ]*2\\d\\d\|#(?=[a-f\\d]{3}([^a-f\\d]\|$))(?!fff)[a-f\\d]*[d-f]\|#(?=[a-f\\d]{4})(?!ffffff)([\\da-f]{2})*[d-f][\\da-f]\|green\|red\|blue\|yellow\|purple\|pink\|brown)],script,noscript,style,header,footer,[class~=^foot\|^head],[id~=^foot\|^head],:has(>a):not(:has(p:matchesOwn(\\S),br,img:not([src~=(?i)^$\|^javascript:\|[^a-z](css\|ic(on)?\|load(ing\|ed)?)[^a-z]\|/\\d+s\\.jpg]))),img[src~=(?i)^$\|^javascript:\|[^a-z](css\|ic(on)?\|load(ing\|ed)?)[^a-z]\|/\\d+s\\.jpg],a:not(:matches(^$)>img)>*,:has(a):not(img,:matchesOwn([\\S\\s]{50,}),:has(img,:matchesOwn([\\S\\s]{50,}))),:matchesOwn([\\s\\S]{50})>:not(img,br,a,:has(img),:matchesOwn([\\s\\S]{50})),:not(img,br,p,a,:matches([\\s\\S]{200}),:has(p,br,img,div:matchesOwn(，\|。)+div:matchesOwn(，\|。)),:has(img,p,br,div:matchesOwn(，\|。)+div:matchesOwn(，\|。)) :matchesOwn(\\S):not(:not(p,div,span:has(br))))';

d=[":matchesOwn(\\S):has(br):has(:matchesOwn(\\S):has(br))"
,":matchesOwn(\\S):has(br)"
,":has(>:matchesOwn(\\S):not(:has(*))+:matchesOwn(\\S):not(:has(*)))"
,":has(>:has(>p:only-child:matchesOwn(\\S):not(:has(*)))+:has(>p:only-child:matchesOwn(\\S):not(:has(*))))"
,"img"
,":matchesOwn(\\S)"];

try{for(查=i!=''?i:java.get("元");c;c--){

if(!(c>1&&(String(r.text()).length<400\|\|r.select(':matchesOwn(内容未加载完成\|关闭(阅读\|小说)模式)').size()))){
r.select(sc).remove();

if(!(查!=""&&(lr=r.select(i?d[i==6?5:i]:查)).size()))for(i=0;i<6&&(lr=r.select(d[i]),i==4&&c==1?!lr.size():String(lr.text()).length<200);i++);
if(c<2\|\|i<6)break;}

r=org.jsoup.Jsoup.parse(r2=String(java.ajax(baseUrl+',{"webView":true}')).replace(/(<[a-z]+)&nbsp;/g,'$1 '));
dt=r1.length==r2.length?2:1}

for(c=lr.first(),v=1;v<lr.size();v++)if(lr.get(v).parents().contains(c)){
lr.remove(v);
v--}else c=lr.get(v);

lr=String((c=lr.size()==2&&i<4)?String(lr.first().text()).length>String(lr.get(1).text()).length?lr.first():lr.get(1):(c=lr.size()==1)?lr.first():lr);

if(查==""){
if(c&&(查=lr.match(/<([a-z]+) ([^>]+)>/))&&(查[2]=查[2].match(/(?:id\|class\|style)=(?:"[^"]+"\|'[^']+')\|[^= ]+(?=="[^"]+"\|'[^']+')/g))){
book.putVariable("元",查[1]+'['+查[2].join('][')+']')
}else book.putVariable("序",i)}

lr=lr.replace(/<([a-z]+)[^>]*"-\d+"[^>]*>[^<]+<\/\1>\|[^<>]*<a[^<]+<\/a>[^<]*\|&lt[; ]?\/?[a-z]+(?= \|\/?&gt)(?:[ a-z=-]+\|"[^"]+"\|'[^']+')*\/?&gt[; ]?\|[☯📑⚙️🌕︴]/g,"").replace(/\s+(?:\s\|(?:(?:(?:n?b)?s)?p)?;)/g,"　　");

if(java.get("原")!=1)lr=(!lr.indexOf("　　")?lr.replace(/>(?!　　\|\s*(?:(?:(?:n?b)?s)?p)?;)\s*(?=[^\s<>])/g,">︴"):lr)
.replace(/((?:[〖【『「（《〈〔［\[(][^〖【『「（《〈〔［\[()］〕〉》）」』】〗\]]*[)］〕〉》）」』】〗\]]\s*)*(?:第?\s*[一二三四五六七八九十百千万〇零0-9]+\s*[章节回話话：:.．,，、]*\s*)?{{
n=(t=title.match(/\S+$/)[0].replace(/[*$\|?+\\\^\[\](){}/]/g,".?")).replace(/^(正文[^\u4e00-\u9fa5A-Za-z]*\|第?[一二三四五六七八九十百千万〇零0-9]+[章节回話话\s：:.．,，、]*)+/,""),n!=t&&/\S/.test(n)?"(?:第?\\s*[一二三四五六七八九十百千万〇零0-9]+\\s*[章节回話话：:.．,，、]*\\s*"+n+"\|"+t+")":t
}}(?:\s*[〖【『「（《〈〔［\[(][^〖【『「（《〈〔［\[()］〕〉》）」』】〗\]]*[)］〕〉》）」』】〗\]])*)/g,"⚙️$1⚙️")+"📑"

}catch(e){}}
if(dt)book.putVariable(dt==1?"动":"静",',{"webView":true}');lr |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: formatTimeDynamic is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:82)
` | source[217].ruleBookInfo.intro = &emsp;&emsp;
🕑 时长：{{formatTimeDynamic(S("$.duration"))}}
📤 发布：{{S("$.pubtime\|\|$.meta.ptime\|\|$.pubdate\|\|$.list.publish_time")?java.timeFormat(Number(S("$.pubtime\|\|$.meta.ptime\|\|$.pubdate\|\|$.list.publish_time")+"000")):""}}
⌨️ 更新：{{S("$.list.update_time\|\|$.update_time")?java.timeFormat(Number(S("$.list.update_time\|\|$.update_time")+"000")):""}}
—————————————
👍🏻 点赞：{{formatCount(S("$.stat.like\|\|$.stats.like"))}}
⭐️ 收藏：{{formatCount(S("$.stat.favorite\|\|$.stats.view"))}}
🪙 投币：{{formatCount(S("$.stat.coin\|\|$.stats.coin"))}}
—————————————
👁 观看：{{formatCount(S("$.stat.view\|\|$.stats.view"))}}
📖 阅读：{{formatCount(S("$.stats.view\|\|$.list.read"))}}
🗯 弹幕：{{formatCount(S("$.stat.danmaku"))}}
💬 评论：{{formatCount(S("$.stat.reply\|\|$.stats.reply"))}}
🔗 分享：{{formatCount(S("$.stat.share\|\|$.stats.share"))}}
—————————————
{{$.desc}}
{{$.summary\|\|$.list.summary\|\|$.sign\|\|$.evaluate}}
{{S("$.seasons[*].season_title")?"<br>&lrm;<br>同系列剧集：":""}}
{{S("$.seasons[*].season_title")?S("$.seasons[*].season_title").split("\n").map((x,i)=>{let b =S("$.seasons[*].new_ep.index_show").split("\n");return x+"【"+b[i]+"】"}).join("\n"):""}}
{{content = "\n&lrm;\n"+S("$.root.content.message");pic = S("$.root.content.pictures");pic?content+"[图片]":content}}
{{if(S("$.replies")){let r = java.getElements("$.replies[*]");let x="";r.forEach(y=>{x+="👤【"+y.member.uname+"】 "+y.reply_control.time_desc+"\n"+y.content.message+(y.content.pictures?"[图片]":"")+"\n&lrm;\n";});"\n&lrm;\n————回复————\n"+x};}}
<js>##(📤 发布\|💬 评论\|🗯 弹幕\|👁 观看\|🪙 投币\|⭐️ 收藏\|👍🏻 点赞\|🔗 分享\|🕑 时长\|📖 阅读\|⌨️ 更新)：\n\|🕑 时长：0:00</js>
<js>##(—————————————\n){2,}</js>
<js>if(/&emsp;&emsp;\s+&lrm;/.test(result)){result = "&emsp;&emsp;"+book.intro}result</js>
##null |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: formatTimeDynamic is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:95)
` | source[217].ruleExplore.kind = {{$.area_name&&$.area_v2_name&&$.area_v2_parent_name}},{{$.vip.label.text}},{{$.badge}},🕑 {{formatTimeDynamic(java.getString("$.duration"))}}
👍🏻 {{formatCount(java.getString("$.stat.like\|\|$.cnt_info.thumb_up"))}}
▶️ {{formatCount(java.getString("$.stat.view\|\|$.cnt_info.play"))}}
🗯 {{formatCount(java.getString("$.stat.danmaku\|\|$.cnt_info.danmaku"))}}
##🕑 0:00\|(?:👍🏻\|▶️)\s+(?!\d)\|🗯$ |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: getDynamicType is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:75)
` | source[217].ruleExplore.name = {{getDynamicType(S("$.type"))}}{{$.modules..major..summary.text\|\|$.season_title\|\|$.meta.name\|\|$.title\|\|$.content.message\|\|$.modules..major..title\|\|$.modules..desc.text\|\|$.modules..major..desc\|\|$.uname\|\|$.part\|\|$.content\|\|$.name\|\|$.modules..major.live_rcmd.content##(.{0,20})##$1###}}{{S("$.id_str")?"【"+S("$.id_str").replace(/^\d{13}/,'')+"】":""}}
<js>
if(/relation/.test(baseUrl)){
	result = "👤"+result
}else if(/,"live_play_info/.test(result)){
	result = "📺"+ result.match(/"title":"([^"]+)"/)[1]
	}
	result
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: getSeachType is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:73)
` | source[217].ruleBookInfo.kind = {{if(S("$.publish.is_finish")=="0"){"连载"};if(S("$.publish.is_finish")=="1"){"完结"};}},{{$.areas[*].name}},{{$.type_name}},{{S("$.rating.score")?S("$.rating.score")+"分":""}},{{$.styles}},{{getSeachType(S("$.type"))}},{{$.payment.tip}},{{$.vip.label.text}},{{$.sex##保密}},{{$.official.title}},{{$.tname_v2&&$.tname}},合集共{{$.ugc_season.ep_count}}个视频,所属合集：{{$.ugc_season.title\|\|$.list.name}}##所属合集：$\|合集共个视频 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: getSeachType is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:94)
` | source[217].ruleSearch.kind = {{S("$.type")=="live_room"?"📺直播间":""}}
,{{S("$.type")=="article"?"📖专栏":""}},{{S("$.type")=="live_user"?"📺主播":""}}
,{{S("$.type")=="bili_user"?"👤用户":""}}
{{S("$.fans")?"粉丝："+formatCount(S("$.fans")):""}}
,{{S("$.live_status")=="true"?"已开播":(S("$.live_status")=="false"?"未开播":"")}}
{{$.official_verify.desc},}{{$.category_name}}
,{{$.cate_name}},{{getSeachType(S("$.media_type\|\|$.season_type"))}},{{$.tag\|\|$.tas}},{{$.category_name}}
,{{S("$.media_score.score")?S("$.media_score.score")+"分":""}},{{$.styles}},{{$.areas}},
🕑 {{S("$.duration")}},👍🏻 {{formatCount(S("$.like"))}},👁 {{formatCount(S("$.play\|\|$.view"))}},🗯 {{formatCount(S("$.danmaku\|\|$.reply"))}}
##🕑 ,\|👍🏻 ,\|👁 ,\|🗯$\|</*em.*?>\|,0分 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:10:189)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1189)
` | source[122].ruleBookInfo.tocUrl = @js:
time=Math.round(new Date()/1000);

uth=
java.HMacHex("b313789b-2d2a-41ce-8982-26af5271fe7c&"+time,"HmacMD5","snY%169j");

sign=
java.HMacHex("/userCenter/getChapterListbookId={{$.id}}&chapterId=0&isAll=1&platId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=1&uth="+uth,"HmacSHA256","snY%169j");

_p=java.desEncodeToBase64String("utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=1&uth="+uth+"&sign="+sign,"snY%169j","DES/ECB/PKCS5Padding","");

let option={
"method": "POST",
"body":"tmp="};

"http://dl.reader.yueyouxs.com/userCenter/getChapterList?bookId={{$.id}}&chapterId=0&isAll=1&platId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&_p="+java.encodeURI(_p)+","+JSON.stringify(option)<br>source[372].ruleBookInfo.tocUrl = @js:
time=Math.round(new Date()/1000);

uth=
java.HMacHex("b313789b-2d2a-41ce-8982-26af5271fe7c&"+time,"HmacMD5","snY%169j");

sign=
java.HMacHex("/userCenter/getChapterListbookId={{$.id}}&chapterId=0&isAll=1&platId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=1&uth="+uth,"HmacSHA256","snY%169j");

_p=java.desEncodeToBase64String("utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=1&uth="+uth+"&sign="+sign,"snY%169j","DES/ECB/PKCS5Padding","");

let option={
"method": "POST",
"body":"tmp="};

"http://dl.reader.yueyouxs.com/userCenter/getChapterList?bookId={{$.id}}&chapterId=0&isAll=1&platId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&_p="+java.encodeURI(_p)+","+JSON.stringify(option)<br>source[581].ruleBookInfo.tocUrl = @js:
time=Math.round(new Date()/1000);

uth=
java.HMacHex("b313789b-2d2a-41ce-8982-26af5271fe7c&"+time,"HmacMD5","snY%169j");

sign=
java.HMacHex("/userCenter/getChapterListbookId={{$.id}}&chapterId=0&isAll=1&platId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=1&uth="+uth,"HmacSHA256","snY%169j");

_p=java.desEncodeToBase64String("utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=1&uth="+uth+"&sign="+sign,"snY%169j","DES/ECB/PKCS5Padding","");

let option={
"method": "POST",
"body":"tmp="};

"http://dl.reader.yueyouxs.com/userCenter/getChapterList?bookId={{$.id}}&chapterId=0&isAll=1&platId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&_p="+java.encodeURI(_p)+","+JSON.stringify(option) |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:10:189)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1280)
` | source[122].ruleSearch.bookUrl = @js:
time=Math.round(new Date()/1000);

uth=
java.HMacHex("b313789b-2d2a-41ce-8982-26af5271fe7c&"+time,"HmacMD5","snY%169j");

sign=
java.HMacHex("/goway/goread/app/book/detailplatId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=2&uth="+uth+"&bookId={{$.id}}&trace=33_33-10-1x0_40_40-4-11x{{$.id}}%3Ftype%3Dbook%26sortValue%3D%26pos%3D2","HmacSHA256","snY%169j");

_p=java.desEncodeToBase64String("utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=2&uth="+uth+"&sign="+sign,"snY%169j","DES/ECB/PKCS5Padding","");

let option={
"method": "POST",
"body":"trace=33_33-10-1x0_40_40-4-11x{{$.id}}%253Ftype%253Dbook%2526sortValue%253D%2526pos%253D2&bookId={{$.id}}"};

"/goway/goread/app/book/detail?platId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&_p="+java.encodeURI(_p)+","+JSON.stringify(option)
<br>source[372].ruleSearch.bookUrl = @js:
time=Math.round(new Date()/1000);

uth=
java.HMacHex("b313789b-2d2a-41ce-8982-26af5271fe7c&"+time,"HmacMD5","snY%169j");

sign=
java.HMacHex("/goway/goread/app/book/detailplatId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=2&uth="+uth+"&bookId={{$.id}}&trace=33_33-10-1x0_40_40-4-11x{{$.id}}%3Ftype%3Dbook%26sortValue%3D%26pos%3D2","HmacSHA256","snY%169j");

_p=java.desEncodeToBase64String("utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=2&uth="+uth+"&sign="+sign,"snY%169j","DES/ECB/PKCS5Padding","");

let option={
"method": "POST",
"body":"trace=33_33-10-1x0_40_40-4-11x{{$.id}}%253Ftype%253Dbook%2526sortValue%253D%2526pos%253D2&bookId={{$.id}}"};

"/goway/goread/app/book/detail?platId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&_p="+java.encodeURI(_p)+","+JSON.stringify(option)
<br>source[581].ruleSearch.bookUrl = @js:
time=Math.round(new Date()/1000);

uth=
java.HMacHex("b313789b-2d2a-41ce-8982-26af5271fe7c&"+time,"HmacMD5","snY%169j");

sign=
java.HMacHex("/goway/goread/app/book/detailplatId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=2&uth="+uth+"&bookId={{$.id}}&trace=33_33-10-1x0_40_40-4-11x{{$.id}}%3Ftype%3Dbook%26sortValue%3D%26pos%3D2","HmacSHA256","snY%169j");

_p=java.desEncodeToBase64String("utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=2&uth="+uth+"&sign="+sign,"snY%169j","DES/ECB/PKCS5Padding","");

let option={
"method": "POST",
"body":"trace=33_33-10-1x0_40_40-4-11x{{$.id}}%253Ftype%253Dbook%2526sortValue%253D%2526pos%253D2&bookId={{$.id}}"};

"/goway/goread/app/book/detail?platId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&_p="+java.encodeURI(_p)+","+JSON.stringify(option)
 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:10:189)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1477)
` | source[122].ruleContent.content = @js:
time=Math.round(new Date()/1000);

uth=
java.HMacHex("b313789b-2d2a-41ce-8982-26af5271fe7c&"+time,"HmacMD5","snY%169j");

sign=
java.HMacHex("/userCenter/v310/downloadChapterplatId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=2&uth="+uth+"&autoBuy=false&bookId="+java.get("id")+"&chapterId="+java.get("chapterId")+"&feeState=1&inBuyView=false&isOssed=1&ossSwitch=1&readCount=2&useSrvAutoBuy=1","HmacSHA256","snY%169j");

_p=java.desEncodeToBase64String("utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=2&uth="+uth+"&sign="+sign,"snY%169j","DES/ECB/PKCS5Padding","");

let option={
"method": "POST",
"body":"readCount=2&inBuyView=false&isOssed=1&bookId={{java.get("id")}}&autoBuy=false&feeState=1&useSrvAutoBuy=1&chapterId={{java.get("chapterId")}}&ossSwitch=1"};

url="https://dl.reader.yueyouxs.com/userCenter/v310/downloadChapter?platId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&_p="+java.encodeURI(_p)+","+JSON.stringify(option);

java.ajax(JSON.parse(java.ajax(url)).data.contentUrl)<br>source[372].ruleContent.content = @js:
time=Math.round(new Date()/1000);

uth=
java.HMacHex("b313789b-2d2a-41ce-8982-26af5271fe7c&"+time,"HmacMD5","snY%169j");

sign=
java.HMacHex("/userCenter/v310/downloadChapterplatId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=2&uth="+uth+"&autoBuy=false&bookId="+java.get("id")+"&chapterId="+java.get("chapterId")+"&feeState=1&inBuyView=false&isOssed=1&ossSwitch=1&readCount=2&useSrvAutoBuy=1","HmacSHA256","snY%169j");

_p=java.desEncodeToBase64String("utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=2&uth="+uth+"&sign="+sign,"snY%169j","DES/ECB/PKCS5Padding","");

let option={
"method": "POST",
"body":"readCount=2&inBuyView=false&isOssed=1&bookId={{java.get("id")}}&autoBuy=false&feeState=1&useSrvAutoBuy=1&chapterId={{java.get("chapterId")}}&ossSwitch=1"};

url="https://dl.reader.yueyouxs.com/userCenter/v310/downloadChapter?platId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&_p="+java.encodeURI(_p)+","+JSON.stringify(option);

java.ajax(JSON.parse(java.ajax(url)).data.contentUrl)<br>source[581].ruleContent.content = @js:
time=Math.round(new Date()/1000);

uth=
java.HMacHex("b313789b-2d2a-41ce-8982-26af5271fe7c&"+time,"HmacMD5","snY%169j");

sign=
java.HMacHex("/userCenter/v310/downloadChapterplatId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=2&uth="+uth+"&autoBuy=false&bookId="+java.get("id")+"&chapterId="+java.get("chapterId")+"&feeState=1&inBuyView=false&isOssed=1&ossSwitch=1&readCount=2&useSrvAutoBuy=1","HmacSHA256","snY%169j");

_p=java.desEncodeToBase64String("utId=996ff65deb7fbfc9f9bdcefb6c62f4ce&deviceId=183860518871512&userId=y76498329&sex=boy&wx=0&tmpToken="+java.get("tmpToken")+"&st=2&uth="+uth+"&sign="+sign,"snY%169j","DES/ECB/PKCS5Padding","");

let option={
"method": "POST",
"body":"readCount=2&inBuyView=false&isOssed=1&bookId={{java.get("id")}}&autoBuy=false&feeState=1&useSrvAutoBuy=1&chapterId={{java.get("chapterId")}}&ossSwitch=1"};

url="https://dl.reader.yueyouxs.com/userCenter/v310/downloadChapter?platId=2&appId=com.yueyou.adreader&channelId=pc&appVersion=3.6.4&srcChannelId=pc&time="+time+"&_s=C66A42978B576873D32F0EACEBBF044D&_p="+java.encodeURI(_p)+","+JSON.stringify(option);

java.ajax(JSON.parse(java.ajax(url)).data.contentUrl) |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:1:18)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:86)
` | source[455].ruleBookInfo.intro = <js>
java.ajax(baseUrl.replace(/index/,'js'))
</js>
tag.p!0:1@html |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:1:19)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:188)
` | source[33].ruleToc.chapterName = @js:
var title = result.parentNode().parentNode().parentNode().parentNode().parentNode();
result = title.select('h2').text() +'：'+ result.text()<br>source[79].ruleToc.chapterName = @js:
var title = result.parentNode().parentNode().parentNode().parentNode().parentNode();
result = title.select('h2').text() +'：'+ result.text() |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:1:32)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:162)
` | source[519].ruleContent.content = <js>
java.getElements('@@id.txt@dd').toArray().sort((a,b)=>a.attr('data-id')-b.attr('data-id')).map(x=>x.html()).join('')</js>##.本章未完，请点击下一页继续阅读.\|{{String(title).replace(/ /,'')}} |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:208:30)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:10147)
` | source[217].ruleContent.content = @js:
if (/data:reply/.test(baseUrl)) {
    let j = String(java.hexDecodeToString(result));
    result = parseContent(j);
} else if (/x\/article/.test(baseUrl)) {
    let json = JSON.parse(src);
    book.author = json.data.author.name;
    if (S("$.data.type") == "3") {
        let data = JSON.parse(S("$.data.content"));
        result = `${processDelta(data.ops)}`
    } else if (S("$.data.type") == "0") {
        result = S("$.data.content");
        result = processHtml(result);
    } else if (S("$.data.type") == "2") {
        result = S("$.data.content")
    }else if(S("$.data.type") == "4"){
     	  if(S("$.data.opus")){
     	  	let r = "";
     	  	let prs = JSON.parse(src).data.opus.content.paragraphs;
     	  	prs.forEach(x=>{
     	  		    switch(x.para_type){
     	  		    	    case 1:
     	  		    	         r += x.text.nodes?getNodes(x.text.nodes)+"\n":"\n"
     	  		    	         break
     	  		    	    case 2:
                                   r += getNodes(x.pic)+"\n"
     	  		   	         break
     	  		    	}   	  		
     	  		});
                   result = r
     	  	}else{
     	  		result = S("$.data.content");
       	imgs = JSON.parse(src).data.image_urls;
       	result += imgs.map(z=>`<img src="${z}">`).join("")
     	  		}
       	
    	}
    if (JSON.parse(src).data.list) {
        let articlelist = JSON.parse(src).data.list;
        let name = articlelist.name;
        if (!/article\/list/.test(book.bookUrl)) {
            result = `${this.getClickImg("link",articlelist.id,null,null,"articlelist")}所属文集：${name}\n` + result;
        }
    }
    if (json.data?.top_video_info?.bvid) {
        result = `${this.getClickImg("video",json.data.top_video_info.bvid,json.data.top_video_info.pic,1)}` + result;
        let title = `${this.getClickImg("video",json.data.top_video_info.bvid)}${json.data.top_video_info.title}`;
        result = title + "\n" + result
    }
} else if (/opus\/detail/.test(baseUrl)) {
    let modules = JSON.parse(result)?.data?.item?.modules;
    let html = "";
    modules ? modules.forEach(module => {
        switch (module.module_type) {
            case "MODULE_TYPE_TOP":
                let pics = module?.module_top?.display?.album?.pics;
                if (pics) {
                    pics.forEach(pic => {
                        html += `<img src="${pic.url}">\n`
                    });
                }
                break;

            case "MODULE_TYPE_CONTENT":
                let paragraphs = module?.module_content?.paragraphs;
                paragraphs ? paragraphs.forEach(paragraph => {
                    let type = paragraph.para_type;
                    switch (type) {
                        case 1:
                        case 4:
                            let nodes = paragraph?.text?.nodes;
                            html += getNodes(nodes)
                            break;

                        case 2:
                            let pics = paragraph?.pic?.pics;
                            pics.forEach(pic => {
                                html += `<img src="${pic.url}">\n`
                            });
                            break;

                        case 3:
                            let pic = paragraph?.line?.pic?.url;
                            html += `<img src="${pic}">\n`
                            break;

                            //列表
                        case 5:
                            let items = paragraph?.list?.items;
                            items.forEach(item => {
                                html += item.order + getNodes(item.nodes) + "\n"
                            });
                            break;

                            //卡片 
                        case 6:
                            let card = paragraph.link_card.card;
                            if (card.type == "LINK_CARD_TYPE_LIVE") {
                                html += getMajor(card)
                            } else {
                                html += getLinkCard(card)
                            }
                            break;

                        case 7:
                            html += JSON.stringify(paragraph)
                            break;
                    }
                }) : null
                break
        }
    }) : null

    result = html;

} else if (/v1\/detail/.test(baseUrl)) {
    let html = "";
    let item = JSON.parse(src).data.item;

    let module_dynamic = item.modules.module_dynamic;
    let module_author = item.modules.module_author;

    html += "🕥 " + module_author.pub_time + " 发布\n";

    rich_text_nodes = module_dynamic?.desc?.rich_text_nodes;
    rich_text_nodes ? rich_text_nodes.forEach(node => {
        let nodes = [{}];
        nodes[0].type = "TEXT_NODE_TYPE_RICH";
        nodes[0].rich = node;
        html += getNodes(nodes);
    }) : null;

    if (module_dynamic.major) {
        html += getMajor(module_dynamic.major)
    }
    if (module_dynamic.additional) {
        html += getLinkCard(module_dynamic.additional)
    }

    let orig = item?.orig?.modules?.module_dynamic;
    if (orig) {
        module_author = item?.orig?.modules?.module_author;
        let mid = module_author?.mid;
        name = module_author.name
        html += `\n—转发自：${this.getClickImg("user",mid)}${name}—\n`
        rich_text_nodes = orig?.desc?.rich_text_nodes;
        rich_text_nodes ? rich_text_nodes.forEach(node => {
            let nodes = [{}];
            nodes[0].type = "TEXT_NODE_TYPE_RICH";
            nodes[0].rich = node;
            html += getNodes(nodes);
        }) : null;

        if (orig.major) {
            html += getMajor(orig.major)
        }
        if (orig.additional) {
            html += getLinkCard(orig.additional)
        }
    }
    result = html
} else if (/data:live/.test(baseUrl)) {
    result = String(java.hexDecodeToString(result));
    let c = result.split("❌");
    if (/\d+&\d+❌/.test(result)) {
        let code = c[0].split("&");
        let data = java.ajax("https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id=" + c[1] + "&protocol=0,1&format=0,1,2&codec=0,1&qn=" + code[1]);
        data = JSON.parse(data).data;
        let stream = data.playurl_info?.playurl?.stream;
        let codec = stream[0].format[0].codec[0];
        let base_url = codec.base_url;
        let host = codec.url_info[code[0]].host;
        let extra = codec.url_info[code[0]].extra;
        result = host + base_url + extra;
    } else {
        result = c[0];
    }
    if (!result) {
        result = 1;
        java.toast("当前直播可能未开播，尝试刷新详情页更新状态")
    }
} else {
    let aid, cid;
    aid = java.hexDecodeToString(result);
    
    if (!/🎥/.test(aid)) {
        if (/series_id/.test(baseUrl)) {
            let data = java.ajax(getApi("video", aid));
            p = JSON.parse(data).data;
            if (p.pages.length > 1) {
                java.longToast("当前视频分 " + p.pages.length + " P\n可在详情页点击书名或长按书名查看更多分P视频")
            }
            book.author = p.owner.name
            cid = p.cid;
        } else {
            let l = java.hexDecodeToString(result).split("&");
            aid = l[0];
            cid = l[1];
        }

        //dash视频
        url = `http://api.bilibili.com/x/player/playurl?avid=${aid}&cid=${cid}&qn=116&fnver=0&fnval=16&fourk=1`;

        if (/season_id=/.test(book.bookUrl)) {
            url = `https://api.bilibili.com/pgc/player/web/playurl?avid=${aid}&cid=${cid}&qn=116&fnver=0&fnval=16&fourk=1`;
        }

        let data = java.ajax(url);
        let json = JSON.parse(data);
        book.putVariable("aid", aid);
        cache.put("aid", aid, 300)

        if (json.data) {
            json = json.data
        } else if (json.result) {
            json = json.result
        }

        dm = String(java.ajax('https://comment.bilibili.com/' + cid + '.xml'));
        java.put("dm", modifyDanmaku(dm, 15));
        if (/DASH/i.test(json.type) \|\| !json.type) {
            if (json.dash.video[0].id < 64) {
                //切换mp4线路
                url = `http://api.bilibili.com/x/player/playurl?avid=${aid}&cid=${cid}&qn=116&fnver=0&fnval=1&fourk=1`;
                data = java.ajax(url);
                result = JSON.parse(data).data.durl[0].url;
            } else {
                if (/🎧/.test(M("模式"))) {
                    java.put("dm", danmakuToLRC(dm));
                    //result = JSON.parse(data).data.durl[0].url;
                    result = json.dash.audio[1].baseUrl;
                } else {
                    result = generateMPD(json.dash);
                
                    //result = JSON.parse(data).data.durl[0].url
                }
            }
        } else if (/mp4/i.test(json.type)) {
            result = json.durl[0].url
        }
    }else{
        
result = `<MPD xmlns="urn:mpeg:dash:schema:mpd:2011"
     type="static"
     mediaPresentationDuration="PT0.5S"
     minBufferTime="PT0.5S">
  <Period id="1" start="PT1S">
    <AdaptationSet contentType="video" mimeType="video/mp4">
      <Representation bandwidth="1000000">
        <BaseURL></BaseURL>
        <SegmentBase indexRangeExact="true">
          <Initialization sourceURL=""/>
        </SegmentBase>
      </Representation>
    </AdaptationSet>
  </Period>
</MPD>
`;
        
    }
}

result |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:104)
` | source[9].ruleContent.content = @js:
eval(String(source.bookSourceComment));
run("content"); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:217)
` | source[9].ruleBookInfo.init = @js:
eval(String(source.bookSourceComment));
run("data");
$$.cover = $$.cover.replace(_reg, host);
$$.tocUrl = eurl('/novel/'+$$.novelId+'/chapters');
JSON.stringify($$); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:231)
` | source[9].ruleSearch.bookList = @js:
eval(String(source.bookSourceComment));
run("data").map(($$,i)=>{
$$.cover = $$.cover.replace(_reg, host);
$$.bookUrl = eurl('/novel/'+$$.novelId);
return JSON.stringify($$);
}); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:30)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:219)
` | source[692].ruleBookInfo.init = <js>
result=String(java.getString("$.data")).replace(/arsadata/,"");
java.aesBase64DecodeToString(result,"4548ded8c9e02690","AES/CBC/PKCS5Padding","1992360ee9bc4f8f");
</js><br>source[705].ruleBookInfo.init = <js>
result=String(java.getString("$.data")).replace(/arsadata/,"");
java.aesBase64DecodeToString(result,"4548ded8c9e02690","AES/CBC/PKCS5Padding","1992360ee9bc4f8f");
</js><br>source[714].ruleBookInfo.init = <js>
result=String(java.getString("$.data")).replace(/arsadata/,"");
java.aesBase64DecodeToString(result,"4548ded8c9e02690","AES/CBC/PKCS5Padding","1992360ee9bc4f8f");
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:42)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:181)
` | source[497].ruleContent.content = @js:
list=java.getElements("@@id.txt@dd").toArray().sort((a,b)=>a.attr("data-id")-b.attr("data-id"));
list.join('\n') |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:43)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:153)
` | source[460].ruleBookInfo.downloadUrls = <js>a=String(java.getElements(".btn@a.1").attr("href"));
java.ajax('http://www.yunxs.info'+a);
</js>.down a@href<br>source[505].ruleBookInfo.downloadUrls = <js>a=String(java.getElements(".btn@a.1").attr("href"));
java.ajax('http://www.yunxs.info'+a);
</js>.down a@href<br>source[660].ruleBookInfo.downloadUrls = <js>a=String(java.getElements(".btn@a.1").attr("href"));
java.ajax('http://www.yunxs.info'+a);
</js>.down a@href |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:45)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:223)
` | source[745].ruleBookInfo.downloadUrls = @js:
var url = java.getString("class.down-button@href")
var a = java.ajax("https://www.ypppt.com" + url)
url = String(a).match(/href\=\"(.*?)\"\>下载地址1/)
url[1] |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:3:29)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:363)
` | source[117].ruleContent.content = @js:
var token = src.match(/token\s*=\s*"([^"]+)"/)?.[1];
var tourl = baseUrl + "?challenge=" + encodeURIComponent(token);
var sectionHtml = java.ajax(tourl).match(/<section>\s*((?:<p>.*?<\/p>\s*)+)(.*?)\s*<\/section>/i)[1];
var text = sectionHtml.replace(new RegExp('<\\/?p>', 'g'), '\n').trim();
text; |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:3:36)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:328)
` | source[692].ruleContent.content = <js>
result=String(java.getString("$.data")).replace(/arsadata/,"");
u=java.aesBase64DecodeToString(result,"4548ded8c9e02690","AES/CBC/PKCS5Padding","1992360ee9bc4f8f");
img=u.match(/\[(.*)\]/)[1].split(",").map(x=>'\n<img src='+x+'>').join("\n")
</js><br>source[705].ruleContent.content = <js>
result=String(java.getString("$.data")).replace(/arsadata/,"");
u=java.aesBase64DecodeToString(result,"4548ded8c9e02690","AES/CBC/PKCS5Padding","1992360ee9bc4f8f");
img=u.match(/\[(.*)\]/)[1].split(",").map(x=>'\n<img src='+x+'>').join("\n")
</js><br>source[714].ruleContent.content = <js>
result=String(java.getString("$.data")).replace(/arsadata/,"");
u=java.aesBase64DecodeToString(result,"4548ded8c9e02690","AES/CBC/PKCS5Padding","1992360ee9bc4f8f");
img=u.match(/\[(.*)\]/)[1].split(",").map(x=>'\n<img src='+x+'>').join("\n")
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:3:47)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:345)
` | source[497].ruleToc.chapterList = @js:
//获取目录，根据div的data-id排序
list=java.getElements("@@id.listsss@div").toArray().sort((a,b)=>a.attr("data-id")-b.attr("data-id"));

//创建数组
l=[];


for(i in list){
a=list[i].select("a").toArray();
l=l.concat(a)
}

l.map(x=>x) |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:4:12)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:517)
` | source[9].ruleToc.chapterList = @js:
eval(String(source.bookSourceComment));

List = run("data").list;

Cipher = java.createSymmetricCrypto("AES/CBC/PKCS5Padding", aesKey, "0123456789abcdef");

List.map($$=>{
	path = Cipher.decryptStr($$.path);
try{
	path = String(path).split(_reg)[2];
}catch(err){}
	return {
		name: $$.chapterName,
		path: eurl(path),
		info: `章节字数：${$$.wordNum}　更新时间：${$$.updatedAt}`
		}
}); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:4:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:236)
` | source[154].ruleBookInfo.downloadUrls = <js>
url = "http://www.aiqu127.com"+java.getString("@tag.center.3@a@href");
java.log(url)
html = java.ajax(url);

java.getStringList("@text.下载地址@href",html,true)
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:4:35)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:543)
` | source[455].ruleContent.content = <js>
result = java.getString("@@class.hycolor.-1@html");
if(result == ""){
     html = java.ajax(baseUrl.replace(/index/,'js'));
     java.setContent(html);
     result = "❗️刷新正文跳转添加书籍页面❗️\n"+java.getString("@@tag.p@html"); java.startBrowser("https://www.coolapk.com/link?url="+java.encodeURI("legado://import/addToBookshelf?src="+baseUrl+",{origin:'https://www.wenxue88.com/'}"),"添加书籍《"+chapter.title+"》");
	}
result
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:5:30)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:937)
` | source[81].ruleToc.chapterList = @js:
var lists_ = [];
var seenChapters = new Set();
var volumeCounter = {};

JSON.parse(src).data.forEach(chapter => {
    const { chapter: chap, updatedAt, translatedLanguage } = chapter.attributes;
    const url = chapter.id;
    
    if (!seenChapters.has(chap)) {
        seenChapters.add(chap);
        lists_.push({
            title: `第${chap}章 分译`, 
            url : "",
            volume: 1
        });
        volumeCounter[chap] = 0;
    }
    const isChapter = true;
    if (isChapter) {
        volumeCounter[chap] += 1;
    }

    lists_.push({
        title: `第${chap}章 （译文${volumeCounter[chap]}）`,
        url: `https://api.mangadex.org/at-home/server/${url}?forcePort443=false`,
        info: `${formatDate(updatedAt)} \| ${getLanguage(translatedLanguage)}`,
        volume: 0
    });
});
//java.log(JSON.stringify(lists_))
lists_; |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:6:141)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1416)
` | source[23].ruleBookInfo.intro = @js:
a='{{$.response.collection.description}}';
b='{{$.response.blogInfo.blogNickName}}';
c=java.getString('$.response.posts[0].post.digest\|\|$.response.collection.description\|\|$.response.blogInfo.selfIntro');

book.putCustomVariable("h:"+String(java.getString("$..homePageUrl")).replace(/\n.*/g,'')+"\n"+"p:"+String(java.getString("$..blogPageUrl")).replace(/\n.*/g,''));

result ="&emsp;&emsp;"+c;

cname="{{$.response.posts[0].post.postCollection.name}}";

cdes=java.getString('$.response.posts[0].post.postCollection.description');
ccount="{{$.response.posts[0].post.postCollection.postCount}}";
if(cname \|\| a){
result=result+"\n&lrm;\n🈴所属合集："+cname+"\n📜合集介绍："+cdes+"\n"+"🔢合集章节总数："+ccount
+"\n🔍搜索合集：#"+cname+"<关闭精确搜索>";

result = String(result).replace(/📜合集介绍：\n/,'').replace(/🈴所属合集：\s+🔢合集章节总数：\s+🔍搜索合集：#<关闭精确搜索>/g,'')
}

result += `<br>&lrm;<br>--复制下面的文字，可将作者添加发现或者订阅--<br>&lrm;<br>${book.author}::http://api.lofter.com/v2.0/blogHomePage.api?product=lofter-android-7.4.4,{"method":"POST","body":"supportposttypes=1%2C2%2C3%2C4%2C5%2C6&blogdomain=${String(java.getString("$..blogInfo.blogName")).match(/(.*)\n*/)[1]}.lofter.com&offset={\{(page-1)*18}}&method=getPostLists&postdigestnew=1&returnData=1&limit=18&checkpwd=1&needgetpoststat=1"}` |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:8:23)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:506)
` | source[149].ruleBookInfo.downloadUrls = <js>
if(baseUrl.indexOf('read')>-1){
	url = book.origin + '/' +java.getString("@@.readcontent@a@href")
	html = java.ajax(url)
	result = html.match(/var downloadurl = "([\s\S]+?)";/)[1]
}else{
	url = baseUrl.replace(/.*-(\d+).*/, "https://www.527txt.com/txt-xx/softdownfree.asp?softid=$1&ckm=mianfei")
	html = java.ajax(url)
	java.setContent(html)
	result = java.getString("@@text.第一下载地址(首选)@href")
}
</js> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at tag (<input>:39:52)
    at <eval> (<input>:46:3)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:2429)
` | source[0].ruleExplore.bookList = <js>
gender=baseUrl.match(/gender=(\d+)/)?baseUrl.match(/gender=(\d+)/)[1]:""
category_id=baseUrl.match(/category_id=(\d+)/)?baseUrl.match(/category_id=(\d+)/)[1]:""
need_filters=baseUrl.match(/need_filters=(\d+)/)?baseUrl.match(/need_filters=(\d+)/)[1]:""
page=baseUrl.match(/page=(\d+)/)?baseUrl.match(/page=(\d+)/)[1]:""
need_category=baseUrl.match(/need_category=(\d+)/)?baseUrl.match(/need_category=(\d+)/)[1]:""
tag_id=baseUrl.match(/tag_id=(\d+)/)?baseUrl.match(/tag_id=(\d+)/)[1]:""
sign_key='d3dGiJc651gSQ8w1'
headers={'app-version':'51110','platform':'android','reg':'0','AUTHORIZATION':'','application-id':'com.****.reader','net-env':'1','channel':'unknown','qm-params':''}
headers['sign']=String(java.md5Encode(Object.keys(headers).sort().reduce((pre,n)=>pre+n+'='+headers[n],'')+sign_key))


var urlEncode = function (param, key, encode) {
  if(param==null) return '';
  var paramStr = '';
  var t = typeof (param);
  if (t == 'string' \|\| t == 'number' \|\| t == 'boolean') {
    paramStr += '&' + key + '=' + ((encode==null\|\|encode) ? encodeURIComponent(param) : param);
  } else {
    for (var i in param) {
      var k = key == null ? i : key + (param instanceof Array ? '[' + i + ']' : '.' + i);
      paramStr += urlEncode(param[i], k, encode);
    }
  }
  return paramStr;
};

var category = function () {
  params={'gender':gender,'category_id':category_id,'need_filters':need_filters,'page':page,'need_category':need_category}
  params['sign']=String(java.md5Encode(Object.keys(params).sort().reduce((pre,n)=>pre+n+'='+params[n],'')+sign_key))
  url="https://api-bc.wtzw.com/api/v4/category/get-list?"+urlEncode(params)
  return java.ajax(url+','+java.put("headers",JSON.stringify({"headers":headers})))
};

var tag = function () {
  params={'gender':gender,'need_filters':need_filters,'page':page,'tag_id':tag_id}
  params['sign']=String(java.md5Encode(Object.keys(params).sort().reduce((pre,n)=>pre+n+'='+params[n],'')+sign_key))
  url="https://api-bc.wtzw.com/api/v4/tag/index?"+urlEncode(params)
  return java.ajax(url+','+java.put("headers",JSON.stringify({"headers":headers})))
};


if(baseUrl.match(/category/)){
  category()
}else {
  tag()
}
</js>
$.data.books<br>source[162].ruleExplore.bookList = <js>
gender=baseUrl.match(/gender=(\d+)/)?baseUrl.match(/gender=(\d+)/)[1]:""
category_id=baseUrl.match(/category_id=(\d+)/)?baseUrl.match(/category_id=(\d+)/)[1]:""
need_filters=baseUrl.match(/need_filters=(\d+)/)?baseUrl.match(/need_filters=(\d+)/)[1]:""
page=baseUrl.match(/page=(\d+)/)?baseUrl.match(/page=(\d+)/)[1]:""
need_category=baseUrl.match(/need_category=(\d+)/)?baseUrl.match(/need_category=(\d+)/)[1]:""
tag_id=baseUrl.match(/tag_id=(\d+)/)?baseUrl.match(/tag_id=(\d+)/)[1]:""
sign_key='d3dGiJc651gSQ8w1'
headers={'app-version':'51110','platform':'android','reg':'0','AUTHORIZATION':'','application-id':'com.****.reader','net-env':'1','channel':'unknown','qm-params':''}
headers['sign']=String(java.md5Encode(Object.keys(headers).sort().reduce((pre,n)=>pre+n+'='+headers[n],'')+sign_key))


var urlEncode = function (param, key, encode) {
  if(param==null) return '';
  var paramStr = '';
  var t = typeof (param);
  if (t == 'string' \|\| t == 'number' \|\| t == 'boolean') {
    paramStr += '&' + key + '=' + ((encode==null\|\|encode) ? encodeURIComponent(param) : param);
  } else {
    for (var i in param) {
      var k = key == null ? i : key + (param instanceof Array ? '[' + i + ']' : '.' + i);
      paramStr += urlEncode(param[i], k, encode);
    }
  }
  return paramStr;
};

var category = function () {
  params={'gender':gender,'category_id':category_id,'need_filters':need_filters,'page':page,'need_category':need_category}
  params['sign']=String(java.md5Encode(Object.keys(params).sort().reduce((pre,n)=>pre+n+'='+params[n],'')+sign_key))
  url="https://api-bc.wtzw.com/api/v4/category/get-list?"+urlEncode(params)
  return java.ajax(url+','+java.put("headers",JSON.stringify({"headers":headers})))
};

var tag = function () {
  params={'gender':gender,'need_filters':need_filters,'page':page,'tag_id':tag_id}
  params['sign']=String(java.md5Encode(Object.keys(params).sort().reduce((pre,n)=>pre+n+'='+params[n],'')+sign_key))
  url="https://api-bc.wtzw.com/api/v4/tag/index?"+urlEncode(params)
  return java.ajax(url+','+java.put("headers",JSON.stringify({"headers":headers})))
};


if(baseUrl.match(/category/)){
  category()
}else {
  tag()
}
</js>
$.data.books<br>source[4].ruleExplore.bookList = <js>
gender=baseUrl.match(/gender=(\d+)/)?baseUrl.match(/gender=(\d+)/)[1]:""
category_id=baseUrl.match(/category_id=(\d+)/)?baseUrl.match(/category_id=(\d+)/)[1]:""
need_filters=baseUrl.match(/need_filters=(\d+)/)?baseUrl.match(/need_filters=(\d+)/)[1]:""
page=baseUrl.match(/page=(\d+)/)?baseUrl.match(/page=(\d+)/)[1]:""
need_category=baseUrl.match(/need_category=(\d+)/)?baseUrl.match(/need_category=(\d+)/)[1]:""
tag_id=baseUrl.match(/tag_id=(\d+)/)?baseUrl.match(/tag_id=(\d+)/)[1]:""
sign_key='d3dGiJc651gSQ8w1'
headers={'app-version':'51110','platform':'android','reg':'0','AUTHORIZATION':'','application-id':'com.****.reader','net-env':'1','channel':'unknown','qm-params':''}
headers['sign']=String(java.md5Encode(Object.keys(headers).sort().reduce((pre,n)=>pre+n+'='+headers[n],'')+sign_key))


var urlEncode = function (param, key, encode) {
  if(param==null) return '';
  var paramStr = '';
  var t = typeof (param);
  if (t == 'string' \|\| t == 'number' \|\| t == 'boolean') {
    paramStr += '&' + key + '=' + ((encode==null\|\|encode) ? encodeURIComponent(param) : param);
  } else {
    for (var i in param) {
      var k = key == null ? i : key + (param instanceof Array ? '[' + i + ']' : '.' + i);
      paramStr += urlEncode(param[i], k, encode);
    }
  }
  return paramStr;
};

var category = function () {
  params={'gender':gender,'category_id':category_id,'need_filters':need_filters,'page':page,'need_category':need_category}
  params['sign']=String(java.md5Encode(Object.keys(params).sort().reduce((pre,n)=>pre+n+'='+params[n],'')+sign_key))
  url="https://api-bc.wtzw.com/api/v4/category/get-list?"+urlEncode(params)
  return java.ajax(url+','+java.put("headers",JSON.stringify({"headers":headers})))
};

var tag = function () {
  params={'gender':gender,'need_filters':need_filters,'page':page,'tag_id':tag_id}
  params['sign']=String(java.md5Encode(Object.keys(params).sort().reduce((pre,n)=>pre+n+'='+params[n],'')+sign_key))
  url="https://api-bc.wtzw.com/api/v4/tag/index?"+urlEncode(params)
  return java.ajax(url+','+java.put("headers",JSON.stringify({"headers":headers})))
};


if(baseUrl.match(/category/)){
  category()
}else {
  tag()
}
</js>
$.data.books |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: replaceCover is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:89)
` | source[126].ruleExplore.coverUrl = @js:replaceCover(java.getString("thumb_url")) |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: sign is not defined
    at <eval> (<input>:4:142)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:327)
` | source[683].ruleBookInfo.tocUrl = @js:
Params = "bid="+java.put('bid', '{{$.bid}}');
eval(String(source.bookSourceComment));
url ="https://fapi.fanxxs.com/v8/book/chapter.api?vs=7.1.2&uuid="+uuid+"&_t="+time+"&app=FXBook&pbv=v1.0.28&channel=fx0001&os=Android&sign="+sign+"," + JSON.stringify(option); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: sign is not defined
    at <eval> (<input>:5:135)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:340)
` | source[683].ruleExplore.bookUrl = @js:
Params =  "bid="+java.getString('$.bid');
eval(String(source.bookSourceComment));

url ="https://fapi.fanxxs.com/2/v8/book3/detail.api?uuid="+uuid+"&nsc=0&timestamp="+time+"&channel=fx0001&bid="+Params+"&nci=1&sign="+sign+"&app=FXBook," + JSON.stringify(option);<br>source[683].ruleSearch.bookUrl = @js:
Params =  "bid="+java.getString('$.bid');
eval(String(source.bookSourceComment));

url ="https://fapi.fanxxs.com/2/v8/book3/detail.api?uuid="+uuid+"&nsc=0&timestamp="+time+"&channel=fx0001&bid="+Params+"&nci=1&sign="+sign+"&app=FXBook," + JSON.stringify(option); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: sign is not defined
    at <eval> (<input>:6:142)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:420)
` | source[683].ruleToc.chapterUrl = @js:
bid = java.get('bid');
cid = java.getString('$.id');
Params =  "cid="+java.getString('$.bid');
eval(String(source.bookSourceComment));
url ="https://fapi.fanxxs.com/v8/book/content.api?timestamp="+time+"&gid=1&bid="+bid+"&channel=fx0001&chapterid="+cid+"&uuid="+uuid+"&sign="+sign+"&app=FXBook," + JSON.stringify(option); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '=='
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:131)
` | source[729].ruleExplore.wordCount = @js:if({{$.isFinished}}==1){if({{$.isPaid}}){'已完结💰'}else{'已完结'}}else{if({{$.isPaid}}){'连载💰'}else{'连载'}}<br>source[733].ruleExplore.wordCount = @js:if({{$.isFinished}}==1){if({{$.isPaid}}){'已完结💰'}else{'已完结'}}else{if({{$.isPaid}}){'连载💰'}else{'连载'}}<br>source[739].ruleExplore.wordCount = @js:if({{$.isFinished}}==1){if({{$.isPaid}}){'已完结💰'}else{'已完结'}}else{if({{$.isPaid}}){'连载💰'}else{'连载'}} |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:136)
` | source[132].ruleBookInfo.intro = &nbsp;&nbsp;
📕书名：{{@css:body > div.container > div.content > div.book.pt10 > div.bookinfo > h1.booktitle@text}}
✏️作者：{{@css:body > div.container > div.content > div.book.pt10 > div.bookinfo > p.booktag > a.red@text}}{{'\n'+'​'}}
🏷相关标签：{{@css:body > div.container > div.content > div.book.pt10 > div.bookinfo > p.booktag > span.red@text}} \| 字数：{{@@.booktag@.blue.0@text}} \| 阅读量：{{@@class.booktag@.blue.1@text##阅读量：}}
💮最新章节：{{@css:body > div.container > div.content > div.book.pt10 > div.bookinfo > p > a.bookchapter@text}}
⏳更新时间：{{@@class.booktime@text##更新时间：}}
📜简       介：{{@@class.bookintro@text}} |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:57)
` | source[145].ruleExplore.bookUrl = {{getUrl(source.key)}}{{@.s2 a@href}},{"webView":true} |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:62)
` | source[33].ruleBookInfo.intro = 🔖  标签：
{{@.book-cats@text}}
🏷️  简介：
{{@.about-text@html}}##(^\|[。！？]+[”」）】]?)##$1<br><br>source[79].ruleBookInfo.intro = 🏷️   {{@.book-cats@text}}{{'\n'+'​'}}
{{@.about-text@html}}##(^\|[。！？]+[”」）】]?)##$1<br> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:67)
` | source[241].ruleBookInfo.intro = 🏷️   {{@class.tagList.0@text}}{{'\n'+'​'}}
{{@#novel_intro@html}}##展开收起\|飞卢小说(.\|\n)*<br>source[904].ruleBookInfo.intro = 🔖   标签：{{@class.tagList.0@text}}{{'\n'+'​'}}
{{@#novel_intro@html}}##展开收起\|飞卢小说(.\|\n)*<br>source[906].ruleBookInfo.intro = 🔖   标签：{{@class.tagList.0@text}}{{'\n'+'​'}}
{{@#novel_intro@html}}##展开收起\|飞卢小说(.\|\n)* |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:68)
` | source[843].ruleBookInfo.intro = 🔖  {{@.w_txt li.4@textNodes}}{{'\n'+'​'}}
{{@.pWorkInformation@html}}##(^\|[。！？]+[”」）】]?)##$1<br><br>source[862].ruleBookInfo.intro = 🔖  {{@.w_txt li.4@textNodes}}{{'\n'+'​'}}
{{@.pWorkInformation@html}}##(^\|[。！？]+[”」）】]?)##$1<br><br>source[907].ruleBookInfo.intro = 🔖  {{@.w_txt li.4@textNodes}}{{'\n'+'​'}}
{{@.pWorkInformation@html}}##(^\|[。！？]+[”」）】]?)##$1<br> |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:74)
` | source[117].ruleBookInfo.intro = 爱下电子书
书名:《{{@class.n-text[0]@tag.h1@text}}》
作者:{{@class.bauthor[0]@text}}
{{@class.trend@text}}
简介:{{@id.intro@text## 　　##
}} |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:81)
` | source[26].ruleSearch.kind = {{@css:.column:has(.icon-star-s)@text}},关注：{{@css:.column:has(.icon-eye)@text}},喜欢：{{@css:.column:has(.icon-heart)@text}},羽毛：{{@css:.column:has(.icon-feather)@text}},评论：{{@css:.column:has(.icon-message-square)@text}}##(\([^()]+)##星$1人
@js:
!!result.split(",")[0] ? result : java.getString("class.book-update@text") |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:89)
` | source[26].ruleBookInfo.kind = 关注：{{@css:.book-detail label:has(.icon-eye)@text}},喜欢：{{@css:.book-detail label:has(.icon-heart)@text}}
{{@@class.list-unstyled mb-2 book-detail@tag.li.-2@text##.+: }}
{{@@class.list-unstyled mb-2 book-detail@tag.li.-1@text## .+}} |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '@'
    at <input>:1:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:97)
` | source[161].ruleContent.content = {{@css:.text-content1 .c-en@text\|\|.text-content1@text}} |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: 'var'
    at <input>:44:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:7610)
` | source[289].ruleBookInfo.name = @js:
j=String(java.get("custom")).match(/^ *((?:[录原单动静直全跳逆字图]\|\[[^\[\]]+\]\|\d+#[^#]+#)+)/);

key=String((u=(baseUrl=String(book.bookUrl).replace(/,{[^{}]+}$/,'')).match(/^(.+)\?((?:[录原单动静直全跳逆字图]\|\[[^\[\]]+\]\|\d+#[^#]+#)+)$/))&&(baseUrl=u[1])&&j?j[1]+u[2]:u?u[2]:j?j[1]:'');

$=it=>(fn=r.select(it)).size()&&(fn=fn.first());

r=org.jsoup.Jsoup.parse(result);
r.select("script,noscript,style,head>:not(meta,title),footer,[class~=^foot],[id~=^foot],a:has(>:last-child:matchesOwn(^分类$)),[value]").remove();

m=String(r).replace(/(?:&nbsp;)+/g," ");

书=false;
n=($('[property$=book_name]')&&(书=String(fn.attr('content')).replace(/^[\s「『【〖（(\[]+/,""))
\|\|(function(){
if($('title')&&(书=String(fn.text()).replace(/^(简介页\|详情页\|目录页\|正版\|全本\|免费阅读\|[\s。.,_/\|「『【〖（(\[\])）〗】』」─—-]+)+/,""))){
for(x=0,c=r.select("h1,h2,h3,strong").eachText();x<c.size();x++)if(
(y=c.get(x))!=''&&(u=书.indexOf(y),~u&&u<4))return String(y)}return 书}())\|\|"请自行修改书名")
.replace(/(?!^)[^\u4e00-\u9fa5a-zA-Z0-9《》]*(?:笔趣阁\|思路客\|燃文\|小说\|漫画\|手机)?(?:[.\|,_/\s「『【〖（(\[\])）〗】』」。─—-]\|(?:人工\|机器\|电脑)?校正\|精校\|完[整结]\|加料\|番外\|未删节\|简介\|全[文本集]\|下载\|(?:小说\|漫画\|大全\|正版(?:小说\|漫画)?\|免费\|免费小说\|免费漫画\|免费全[文本]\|在线\|最[新快]\|全部\|手机\|电脑)(?:全[文本集部]\|大全\|免费\|在线\|阅读\|下载\|章节\|小说\|更新\|漫画\|\.)\|([^a-zA-Z0-9])(?:azw\|mobi\|epub\|txt)(?![a-zA-Z0-9])\|(?:最全)?(?:章节\|目录\|列表){2,}\|更新章节最快\|无广告\|(?::顶点)?无弹窗\|无防盗\|小说网\|手打全文\|[纯全](?:手打\|文字)\|\s*by\s*(?=[\u4e00-\u9fa5]))[\S\s]*/i,"$1");

if(n[0]=="《"&&n[n.length-1]=="》")n=n.slice(1,-1);

if($('[property$=author]')){
x=String(fn.attr("content")).replace(/^作\s*[者家][\s:：]*\|(?!^)[/／｜\|，,\s][^⚙️]*$/,"")
}else{
x=m.match(/>\s*([^>]+?)(?:\s*<\/[a-z]+>\s*\|\s+)著\s*<\|[\s\[\];?!,.()、，；？！。…─（）［］〖〗【】>《》](?:小说\|漫画)?作\s*者(?![^>]+->)(?:[:：\s〖【（《［\[\(]\|<[^it\/][^>]*>\|<\/[^>]+>)+([^\s<">,，/／｜\|\)\]］》）】〗]+)/);
x=x?x[1]?x[1]:x[2]:$('#author,.author')?String(fn.text()).replace(/(?!^)[/／｜\|，,\s][^⚙️]*$/,""):""}
java.put("x",x);

c=(fn=r.select("meta[property~=category$]")).size()?String(fn.attr("content"))
.replace(/(?!^)\s*[，,./／｜\|]\s*/,","):(fn=m.match(/(?:[\s\[\];?!,.()、，；？！。…─（）［］〖〗【】》]\|<[^a/][^>]*>\|<\/[^>]+>)(?:[分大]\s*类\|类\s*[型别])(?:[:：\s]\|<[^>]+>)+([^\s<."/／｜\|>]+)/))&&fn[1];
if(c)java.put("v",c);

c=(fn=r.select("meta[property~=status$]")).size()?fn.attr("content"):(fn=m.match(/(?:[\s\[\];?!,.()、，；？！。…─（）［］〖〗【】》]\|<[^a/][^>]*>\|<\/[^>]+>)状\s*态(?:[:：\s]\|<[^>]+>)+([^\s<."/／｜\|>]+)/))&&fn[1];
if(c)java.put("s",c);

c=$('meta[property~=latest_chapter_name$]')?fn.attr("content"):(fn=m.match(/>(?:\s*[更最]\s*[新近])+(?:\s*章\s*节)?(?:[:：\s\[]\|<[^>]+>)+(?!\s*(?:-\|&gt;)\s*<\|[:：\s0-9T年月日时分秒*-]{5,}<\|[^:：]+[^章\s]\s*[:：]\s*<\|更新(?:时间)?[:：])([^<"/／｜\|\]>]+)/))&&fn[1];
if(c)java.put("z",c);

正=true;
if(key.length){
if(~key.indexOf("全"))java.put("全",1);
if((
u=key.match(/[^\[\]]+(?=\])/),
c=~key.indexOf("录"),
y=~key.indexOf("单"),
baseUrl=u?u[0]:baseUrl,
(c\|\|u)&&(baseUrl=c\|\|y?(baseUrl=String((c=baseUrl.match(/(.+[^\d])(\d+)([^\d]*)$/))[1]+1+c[3]),
c=c[2]+'🌕'+c[1]+'🌕'+2+'🌕'+c[3],
baseUrl):baseUrl)
)\|\|~key.indexOf("直")
)java.put("目",1),正=1;
if(~key.indexOf("录"))java.put("录",c),正=1;
if(y\|\|~key.indexOf("#")){
if(!u){
if((网=key.match(/(\d+)#([^#]+)/))
&&(尾=网[1],网=网[2].match(/^(.*[^\d])([12])([^/?\d]*)$/))
\|\|(尾=r.select('a[href~=\\d[^/?\\d]*$]:matches(^(尾\|末\|最后一)[頁页篇章回节節话話]$)')).size()
&&(网=r.select('a:matches(^2$)')).size()
&&(网=String(网.first().attr('href')).match(/^(.*[^\d])([12])([^/?\d]*)$/))
&&(尾=String(尾.first().attr('href')).match(/\d+(?=[^/?\d]*$)/)[0])){
c=尾+'🌕'+网[1]+'🌕'+网[2]+'🌕'+网[3]
}else if((c=r.select('a:matches(^(\\d+\|…+\|\\.+)$)')).size()){
if((网=String(c).split(/<a[^>]+>[^\d<]+<\/a>/)).length==2){
for(c=网[0],尾=网[1],x=+网[1].match(/>([^<]+)/)[1],网=网[0].match(/href="([^"]*[^\d])(\d+)([^/?\d"]*)"[^>]*>([^<]+)<[^<]+$/),j=+网[4],i=+网[2]-j;j<x;j++)c+="<a href='"+网[1]+(j+i)+网[3]+"'>"+j+"</a>";
c+=尾}
}else c=1}
java.put("单",c),正=false}
if(~key.indexOf("跳"))java.put("跳",1);
if(~key.indexOf("逆"))book.setReverseToc(true);
else book.setReverseToc(false);
if((u=~key.indexOf("图"))\|\|~key.indexOf("原"))java.put("原",1);
if(~key.indexOf("动"))java.put("动",',{"webView":true}');
else if(~key.indexOf("静"))java.put("静",1);
if(u\|\|~key.indexOf("字"))java.put("文",u?1:2)
}else book.setReverseToc(false);

c=(fn=r.select("meta[property$=description][content~=\\S]")).size()?fn.get(fn.size()-1).attr("content"):(r.select(':matchesOwn([\\u4e00-\\u9fa5]{2,})>:not(br),:not(body,br,:matchesOwn([\\s\\S]{50}),:has(body,:matchesOwn([\\s\\S]{50})))').remove(),r.select(":matchesOwn(\\S)").text());

java.put("g",c=String(c).replace(/[\snbsp;]*(?:&nbsp;\|\s){2,}\|\s*([？！。]+[”」』\]\}\)）｝】〗〕〉]?)\s*/g,"$1　　").replace(/(?=　　)/g,"\n"));

if(c.length&&(c=c.match(/(.{0,2})《([^《》]+)》(.{0,3})/))&&c[3]!="作品集"&&!c[1].match(/新[书作]/)&&(c=c[2],书?~String(书).indexOf(c)&&!~c.indexOf(n):true))n=c;

if(正==1)m=java.ajax(baseUrl);
java.setContent(m,baseUrl);

if(正){
zl=java.getStringList("[property$=latest_chapter_url]@content\|\|:matches(^最新章节)>a:only-child:not([href~=^$\|#\|javascript:])@href\|\|a:matches(^正文\\s*[\\d第一二三四五六七八九十〇零百千]\|^[【《]?("+n+")?[\\s》】（\\u0028:：＿_－-]*(第[\\s0〇零]*[一1]\\s*[\\u4e00-\\u9fa5]\|([\\u4e00-\\u9fa5]{2}阅读[（\\u0028:：＿_－-]?)?(0*1([）\\u0029.、:：_-]\|$)\|[〇零]*一([）\\u0029\\s.、:：_-]\|$)))):not([href~=(^\|[^/])/[vV][iI][pP]\|([A-Za-z]\\d+\|\\d[A-Za-z]+\|[A-Z][a-z]+\|[a-z][A-Z]+){3,}[^/?&_-]*$\|^$\|#\|javascript:\|"+(bas=baseUrl.replace(/\/+$\|\.[a-zA-Z]+$/,'')).match(/[^?/]+$/)[0].replace(/([*$\|?+\\\^\[\](){}])/g,'\\$1')+"(?:[/_-]1)?(?:\\/\|\\.[a-zA-Z]+)?$])@href\|\|a:matches(^[^\\u4e00-\\u9fa5]*(免费\|在线\|开始\|立即\|全文\|正文\|从头)+[试阅]读[^\\u4e00-\\u9fa5]*$):not([href~=^$\|#\|javascript:])@href\|\|a:matches(^0*1[^\\d]):not([href~=(^\|[^/])/[vV][iI][pP]\|([A-Za-z]\\d+\|\\d[A-Za-z]+\|[A-Z][a-z]+\|[a-z][A-Z]+){3,}[^/?&_-]*$\|^$\|#\|javascript:])@href");

if(zl.size()&&(ck=String(zl.get(0)).match(/^(.*\/\/[^/]+)?([/?]?[^/].+[?&/_-])[^&/_-]+\/?$/)))(jd=ck[1])&&(h=baseUrl.lastIndexOf('/',baseUrl.indexOf(jd.match(/(?:\.[^.]+){2,}$\|[^./]+\.[^.]+$/)[0])))>8&&(
q=baseUrl.indexOf(':'),
(bas=java.get(baseUrl=baseUrl.slice(0,q+2)+baseUrl.slice(h),{})).statusCode()==200&&java.setContent(bas.body(),baseUrl)
),java.put("ck",ck[2])}
n |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: 'var'
    at eval_script:3:56
` | source[711].ruleContent.content = @js:
header={"Referer":baseUrl};
headers={"headers":JSON.stringify(header)};
imgl=eval(result.match(/(eval\(.+?\}\)\))/)[1]);

host = "http://images.720rs.com";
image=cInfo.fs;
//piclist.map(u=>"<img src=\""+server+u+','+JSON.stringify(headers)+"\">").join("\n")

html='';
function get7ImageUrl(image) {
    if (image.match(/^(\/Man?)/i)) {
        return host + image
    } else if (image.match(/^(http?)/i)) {
        return image
    }
}
for(i in image){
url=get7ImageUrl(image[i]);
html+='<img src="'+url+','+JSON.stringify(headers)+'">\n'
}
html |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '}'
    at <input>:5:12
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:146)
` | source[518].ruleBookInfo.tocUrl = @js:
uid = 111;
body = {
	bookId: {{$..bookId}}
}
eval(String(source.bookSourceComment)); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: unexpected token in expression: '}'
    at <input>:5:12
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:147)
` | source[518].ruleSearch.bookUrl = @js:
uid = 1110;
body = {
	bookId: {{$.bookId}}
}
eval(String(source.bookSourceComment)); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:13:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:690)
` | source[137].ruleBookInfo.tocUrl = @js:
$ = _ => java.getElement(_);
var bookid=$("#bookDetails").attr("data-bookid");
var cid=$("#bookDetails").attr("data-classid");
var atb=$("#bookDetails").attr("data-atb");
var title=$("#bookDetails").attr("data-title");
title = encodeURIComponent(title);
dz = "asc" \|\| "desc";
id = bookid
var loadlist=1
pagenum=0
pageIndex = 0,
pageCount = 0;

url = `/e/extend/bookpage/pages.php?classid=${cid}&id=${id}&atb=${atb}&pageNum=${pageIndex}&dz=${dz}&pageCount=${pageCount}&totalPage=${pagenum}&loadlist=${loadlist}&title=${title}`; |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:28:21
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:4262)
` | source[128].ruleSearch.intro = @js:
key = String(java.get('key'));
// str 即为input输入的数据
function sliceCountStr(str) {
   let arr = []; // 记录分割计算表达式
   for (let i = 0; i < str.length; i++) {
      let item = str.charAt(i);
      let num = item;
      if (/[\d\|\.]/.test(item)) {
          let j = i + 1;
          for (; j < str.length - 1; j++) {
            let otherItem = str.charAt(j);
            if (!/[\d\|\.]/.test(otherItem)) {
              break;
            }
          }
          num = str.slice(i, j);
          i = j - 1;
          num = +num;
      }
      arr.push(num);
   }
   return arr;
}
// arr 即为上一步处理好的数组
function countHandle(arr) {
      let charArr = [],
        numArr = [];
      for (let i = 0; i < arr.length; i++) {
        if (typeof arr[i] == 'number') {
          numArr.push(arr[i]);
        } else {
          if (charArr.length) {
            // 步骤1
            // 如果当前的运算符的优先级比栈顶的优先级低或相等，就说明需要把前面的值全部计算好
            // 存储运算符的栈要一直出栈，直到栈为空或当前的字符的优先级比栈顶的优先级高
            while (this.isPop(arr[i], charArr[charArr.length - 1])) {
              let t2 = numArr.pop();
              let t1 = numArr.pop();
              let char = charArr.pop();
              this.handleCalculation(numArr, t1, t2, char);
            }
            // 当前运算符为右括号
            if (arr[i] == ')') {
              // 取栈顶运算符
              let st = charArr[charArr.length - 1];
              // 步骤2
              // 遇到右括号也要一直出栈，直到遇到左括号
              while (st != '(') {
                let t1, t2;
                let char = charArr.pop();
                if (char != '(') {
                  t2 = numArr.pop();
                  t1 = numArr.pop();
                  this.handleCalculation(numArr, t1, t2, char);
                }
                st = char;
              }
            }
            // 运算符不为右括号
            if (arr[i] != ')') {
              charArr.push(arr[i]);
            }
          } else {
            // 步骤3
            // 运算符栈为空，直接入栈
            charArr.push(arr[i]);
          }
        }
      }
      // 步骤4
      // 最后运算符栈如果还有字符，要一直出栈直到为空
      while (charArr.length) {
        let t2 = numArr.pop();
        let t1 = numArr.pop();
        let char = charArr.pop();
        this.handleCalculation(numArr, t1, t2, char);
      }
   return numArr[0];
}
// 基本加减乘除运算处理
    function handleCalculation(numArr, num1, num2, char) {
      if (char == '+') {
        numArr.push(num1 + num2);
      } else if (char == '-') {
        numArr.push(num1 - num2);
      } else if (char == '*') {
        numArr.push(num1 * num2);
      } else if (char == '/') {
        numArr.push(num1 / num2);
      }
    }
    // 判断运算符的优先级，是否出栈进行计算
    function isPop(char1, char2) {
      // 运算符栈为空
      if (!char2) {
        return false;
      }
      // 运算符优先级相同
      if ((char1 == '+' \|\| char1 == '-') && (char2 == '+' \|\| char2 == '-')) {
        return true;
      }
      // 前者运算符优先级比后者低
      if ((char1 == '+' \|\| char1 == '-') && (char2 == '*' \|\| char2 == '/')) {
        return true;
      }
      // 运算符优先级相同
      if ((char1 == '*' \|\| char1 == '/') && (char2 == '*' \|\| char2 == '/')) {
        return true;
      }
      // 前者运算符优先级比后者高
      if ((char1 == '*' \|\| char1 == '/') && (char2 == '+' \|\| char2 == '-')) {
        return false;
    }
}
try{
iresult=countHandle(sliceCountStr(key))
if(!iresult) throw('error')
result=`答案是${iresult}`
}catch{
api=`http://api.qingyunke.com/api.php?key=free&appid=0&msg=${key}`
result=JSON.parse(java.ajax(api)).content
}
java.put('result',result)
result |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:3:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:188)
` | source[34].ruleSearch.bookUrl = @js:
let t = Date.now().toString(),
    params = { 
    	bid: "{{$.sourceId}}", 
    	t: t
    	};
creatRequest("/tf/book", params); |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:3:11
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:233)
` | source[260].ruleContent.content = @js:
var txt = [],
dd = java.getElement("#txt dd");
for(i = 0; i < dd.length-1; i++){
	data = dd.select(`[data-id=${i}]`)
	txt.push(data)
	}
txt.join("\n") |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:3:12
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:234)
` | source[260].ruleToc.chapterList = <js>
var list = [],
d=java.getElement(".list > div");
for(i = 0; i < d.length-1; i++){
	data = d.select(`[data-id=${i}]`)
	list.push(data)
	}
list.join("\n")
</js>
li a |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:3:9
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:674)
` | source[10].ruleToc.chapterList = @js:
let v = [],
		list = [];
JSON.parse(result).list.map($=>{

//分卷判定
		V = $.name;
		if(v[v.length-1]!=V&&!/^\s*$/.test(V)){
				v.push(V)
				list.push({
						name: '📖['+V+']📖',
						volume: true
					})
			}

$.bookChapters.map($=>{
		return list.push({
				name: $.name,
				url: `https://readbook-service-freebook.cread.com/cx/itf/chapterRead?bookId=${$.bookid}&chapterId=${$.id}`,
				info: `章节字数：${$.wordCount}　更新时间：${$.updateDate}`
			});
	});
});
v.length<2?list.filter($=>!$.volume):list<br>source[35].ruleToc.chapterList = @js:
let v = [],
		list = [];
JSON.parse(result).list.map($=>{

//分卷判定
		V = $.name;
		if(v[v.length-1]!=V&&!/^\s*$/.test(V)){
				v.push(V)
				list.push({
						name: '📖['+V+']📖',
						volume: true
					})
			}

$.bookChapters.map($=>{
		return list.push({
				name: $.name,
				url: `https://readbook-service-freebook.cread.com/cx/itf/chapterRead?bookId=${$.bookid}&chapterId=${$.id}`,
				info: `章节字数：${$.wordCount}　更新时间：${$.updateDate}`
			});
	});
});
v.length<2?list.filter($=>!$.volume):list<br>source[484].ruleToc.chapterList = @js:
let v = [],
		list = [];
JSON.parse(result).list.map($=>{

//分卷判定
		V = $.name;
		if(v[v.length-1]!=V&&!/^\s*$/.test(V)){
				v.push(V)
				list.push({
						name: '📖['+V+']📖',
						volume: true
					})
			}

$.bookChapters.map($=>{
		return list.push({
				name: $.name,
				url: `https://readbook-service-freebook.cread.com/cx/itf/chapterRead?bookId=${$.bookid}&chapterId=${$.id}`,
				info: `章节字数：${$.wordCount}　更新时间：${$.updateDate}`
			});
	});
});
v.length<2?list.filter($=>!$.volume):list |
| `json path is invalid: path must start with `$`` | source[698].ruleSearch.bookUrl = @json:/b/{$.FolderName} |
| `json path is invalid: unexpected character at 15` | source[725].ruleBookInfo.lastChapter = $.data.list[-1]title<br>source[738].ruleBookInfo.lastChapter = $.data.list[-1]title |
| `json path is invalid: unexpected character at 28` | source[83].ruleContent.content = $.data.info.images.images[*]url
<js>
//java.log(result);
let newResult = result.split("\n").map(x => "https://f40-1-4.g-mh.online" + x);
let headers = JSON.stringify({"headers":{"Referer":baseUrl}});
newResult.map(x => `<img src="${x},${headers}">`).join("\n");
</js> |
| `json rule is invalid: expected value at line 1 column 1` | source[752].ruleToc.updateTime = {{$.content_length}}字 💰{{$.min_price}} 目录总数：{{$.post_count}}
<js>
time="{{$.publish_time}}"?java.timeFormat("{{$.publish_time}}000"):""
result=time+" "+result
</js>
##💰0\.00\| 字 💰\| 目录总数：$<br>source[753].ruleToc.updateTime = {{$.content_length}}字 💰{{$.min_price}} 目录总数：{{$.post_count}}
<js>
time="{{$.publish_time}}"?java.timeFormat("{{$.publish_time}}000"):""
result=time+" "+result
</js>
##💰0\.00\| 字 💰\| 目录总数：$ |
| `xpath rule is invalid: Unexpected error occurred. Please report this to the developer
DanglingCombinator` | source[176].ruleToc.isVip = //删掉这行字，vip章节会显示🔓
$.isFree<br>source[22].ruleToc.isVip = //删掉这行字，vip章节会显示🔓
$.isFree<br>source[374].ruleExplore.name = /a//h4/text() |

## Execution errors by category

| Category | Rule count |
| --- | ---: |
| js runtime | 186 |
| css compatibility | 111 |
| unsupported JVM access | 24 |
| other | 6 |
| path parser | 3 |

## java.* methods

| Method | Sources |
| --- | ---: |
| `java.ajax` | 100 |
| `java.getString` | 69 |
| `java.put` | 62 |
| `java.get` | 58 |
| `java.timeFormat` | 40 |
| `java.log` | 28 |
| `java.md5Encode` | 23 |
| `java.toast` | 21 |
| `java.lang` | 20 |
| `java.getElements` | 17 |
| `java.toNumChapter` | 16 |
| `java.util` | 16 |
| `java.post` | 14 |
| `java.base64Decode` | 13 |
| `java.startBrowser` | 8 |
| `java.connect` | 7 |
| `java.encodeURI` | 7 |
| `java.getElement` | 7 |
| `java.aesBase64DecodeToString` | 6 |
| `java.base64Encode` | 6 |
| `java.createSymmetricCrypto` | 6 |
| `java.randomUUID` | 6 |
| `java.setContent` | 6 |
| `java.timeFormatUTC` | 6 |
| `java.io` | 5 |
| `java.security` | 5 |
| `java.getStringList` | 4 |
| `java.longToast` | 4 |
| `java.startBrowserAwait` | 4 |
| `java.webView` | 4 |
| `java.HMacHex` | 3 |
| `java.desEncodeToBase64String` | 3 |
| `java.digestHex` | 3 |
| `java.hexDecodeToString` | 3 |
| `java.refreshTocUrl` | 3 |
| `java.androidId` | 2 |
| `java.openUrl` | 2 |
| `java.t2s` | 2 |
| `java.ajaxAll` | 1 |
| `java.base64DecodeToByteArray` | 1 |
| `java.createAsymmetricCrypto` | 1 |
| `java.getCookie` | 1 |
| `java.getReadBookConfigMap` | 1 |
| `java.getStrResponse` | 1 |
| `java.getThemeConfigMap` | 1 |
| `java.getVerificationCode` | 1 |
| `java.getWebViewUA` | 1 |
| `java.head` | 1 |
| `java.hexEncodeToString` | 1 |
| `java.htmlFormat` | 1 |
| `java.importScript` | 1 |
| `java.initUrl` | 1 |
| `java.readBookConfig` | 1 |
| `java.refreshBookUrl` | 1 |
| `java.refreshExplore` | 1 |
| `java.searchBook` | 1 |
| `java.showBrowser` | 1 |
| `java.upLoginData` | 1 |

## Blocked sources by category

| Category | Blocked sources |
| --- | ---: |
| js runtime | 123 |
| css compatibility | 72 |
| unsupported JVM access | 14 |
| other | 6 |
| path parser | 3 |
