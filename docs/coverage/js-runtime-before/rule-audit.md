# Rule coverage audit

- Sources: **970**
- Rule strings: **22220**
- Fully executable: **678 / 970 (69.9%)**
- Blocked sources: **292**

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
| `default-mode rule is not supported: `.rows[*]` is not a CSS selector: Unexpected EOL` | 1 |
| `default-mode rule is not supported: `0` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `3-7+3*5*2*(2+(4+3*2*4*2-2-8+1)*2*2)-2*2+(1/2.5+1)*3` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `7号基地` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `<br>{$.introduction}` is not a CSS selector: Unexpected error occurred. Please report this to the developer
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
| `default-mode rule is not supported: `book_tag_list[*].title` is not a CSS selector: Unexpected EOL` | 8 |
| `default-mode rule is not supported: `cat[*]` is not a CSS selector: Unexpected EOL` | 2 |
| `default-mode rule is not supported: `categoryNames.*.className` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    Delim(
        '*',
    ),
)` | 2 |
| `default-mode rule is not supported: `categoryNames.[*].className` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ClassNeedsIdent(
    SquareBracketBlock,
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
| `default-mode rule is not supported: `data.list[*]` is not a CSS selector: Unexpected EOL` | 3 |
| `default-mode rule is not supported: `data.state[*]` is not a CSS selector: Unexpected EOL` | 1 |
| `default-mode rule is not supported: `data[*]` is not a CSS selector: Unexpected EOL` | 7 |
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
| `default-mode rule is not supported: `http://api.ieasou.com/api/bookapp/bookSummary.m?gid=&nid=&sort=1&size=1000&session_id=-nEGYU48h9ZiOcED3oVOOzJ&returnType=100&gsort=1&cid=eef_easou_book&version=002&os=android&udid=5f0523be952d7c668bc069ad38d9470ae15fc6c5&appverion=1122&ch=blf1298_12337_001&session_id=-nEGYU48h9ZiOcED3oVOOzJ&dzh=1&scp=0&appid=10001&utype=0&rtype=3&pushid=9f8b02370838a641dc4ddff894b98848&ptype=5&gender=0&userInitPay=3&birt=1660848272929&instime=1660848269464&instId=1660848269464&chType=3&bidType=0&recSw=1&appType=0` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 2 |
| `default-mode rule is not supported: `http://api.jxgtzxc.com/` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `http://image.mhxk.com/mh/{$.comic_id}.jpg-600x800.jpg.webp` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 2 |
| `default-mode rule is not supported: `http://m.yueyouxs.com/b/{$.wapBookId}.html` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 6 |
| `default-mode rule is not supported: `http://mobile.ximalaya.com/fmobile-album/album/track/ts-1576706285548?ac=4G&albumId=&device=android&isAsc=true&isQueryInvitationBrand=true&isVideoAsc=true&pageSize=200&source=3&supportWebp=true&pageId=1` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 9 |
| `default-mode rule is not supported: `http://mobile.ximalaya.com/fmobile-album/album/track/ts-1576706285548?ac=4G&albumId={$.id}&device=android&isAsc=true&isQueryInvitationBrand=true&isVideoAsc=true&pageSize=200&source=3&supportWebp=true&pageId=1` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 9 |
| `default-mode rule is not supported: `http://novel.tingroom.com/wap/` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `http://sma.yueyouxs.com/b/{$.wapBookId}.html` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 4 |
| `default-mode rule is not supported: `http://touchlife.cootekservice.com/doReader/enter_bookinfo_index?_token=e72ca407-9caa-475d-a829-46e15d3d4834&bookId={$.bookId}` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 4 |
| `default-mode rule is not supported: `http://touchlife.cootekservice.com/doReader/get_content_by_chapterId?_token=e72ca407-9caa-475d-a829-46e15d3d4834&bookId={$.bookId}&chapterId={$.chapterId}` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 2 |
| `default-mode rule is not supported: `https://api.mangadex.org/manga/{$.id}/feed?limit=96&includes[]=scanlation_group&includes[]=user&order[volume]=asc&order[chapter]=asc&offset=0&contentRating[]=safe&contentRating[]=suggestive&contentRating[]=erotica&contentRating[]=pornographic` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `https://api.mangadex.org/manga/{$.id}?includes[]=artist&includes[]=author&includes[]=cover_art` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `https://book.midukanshu.com/book/chapter/segment/master/{$.bookId}_{$.chapterId}.txt?md5={$.content_md5}` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `https://comic.mkzhan.com/chapter/?comic_id={$.data.comic_id}` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `https://comic.mkzhan.com/comic/info/?comic_id={$.comic_id}` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `https://image.yqmh.com/mh/{$.comic_id}.jpg-300x400.webp` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `https://m.iyouman.com/api/getcomicinfo_body/?comic_id={$.comic_id}&productname=aym&platformname=wap` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 2 |
| `default-mode rule is not supported: `https://m.kanman.com/api/getchapterlist?product_id=1&productname=kmh&platformname=wap&comic_id=` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 6 |
| `default-mode rule is not supported: `https://m.manhuatai.com/{$.comic_newid}/` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 2 |
| `default-mode rule is not supported: `https://m.suixkan.com/b/{$.wapBookId}.html` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `https://mangadex.org/covers/{$.id}/{$..fileName}.256.jpg` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 2 |
| `default-mode rule is not supported: `https://novel.snssdk.com/api/novel/book/directory/list/v1?book_id={$.book_id}` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `https://pre-api.tuishujun.com/api/listBookScoreByBook?book_id={$.book_id}&type=all&page=1&pageSize=10&sort_field=create_time&sort_value=desc` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 2 |
| `default-mode rule is not supported: `https://www.linovel.net/book/{$.id}.html` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 2 |
| `default-mode rule is not supported: `https://xapi.xinmanhua.net/chapters/{$.id}/qualitywebs` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 2 |
| `default-mode rule is not supported: `https://xapi.xinmanhua.net/worksinfos/{$.attributes.wid}?include=chapters.digests` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 2 |
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
| `default-mode rule is not supported: `result.classificationInfoBooks[*]` is not a CSS selector: Unexpected EOL` | 2 |
| `default-mode rule is not supported: `span.0:-text` is not a CSS selector: Token "0" was not expected` | 2 |
| `default-mode rule is not supported: `src\|class.red` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ExpectedNamespace(
    "src",
)` | 7 |
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
| `default-mode rule is not supported: `{$.book.totalWordSize}字` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `{$.book_words_num}字` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 2 |
| `default-mode rule is not supported: `{$.crazy_rating}分` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 6 |
| `default-mode rule is not supported: `{$.grade}分` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 12 |
| `default-mode rule is not supported: `{$.listencnt}播放` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 2 |
| `default-mode rule is not supported: `{$.score}分` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 3 |
| `default-mode rule is not supported: `{$.total}首歌` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `{$.type_name},{$.catalog_name}` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 9 |
| `default-mode rule is not supported: `{baseUrl}` is not a CSS selector: Unexpected error occurred. Please report this to the developer
EmptySelector` | 1 |
| `default-mode rule is not supported: `共{$.trackCount}集` is not a CSS selector: Token "<{" was not expected` | 1 |
| `default-mode rule is not supported: `御兽之王 \| 24新晋白金作家轻泉流响 \| 起点霸榜 \| 快节奏爽文` is not a CSS selector: Unexpected error occurred. Please report this to the developer
ExplicitNamespaceUnexpectedToken(
    WhiteSpace(
        " ",
    ),
)` | 1 |
| `default-mode rule is not supported: `播放数{$.play}  共{$.tracks}集` is not a CSS selector: Token "<{" was not expected` | 3 |
| `default-mode rule is not supported: `收听{$.playCount}  共{$.trackCount}章` is not a CSS selector: Token "<{" was not expected` | 3 |
| `default-mode rule is not supported: `最后更新：{$.updated_at}` is not a CSS selector: Token "<{" was not expected` | 1 |
| `default-mode rule is not supported: `穿进赛博游戏后逆袭成神
https://www.aqxsw666.com/txt-xx/nsxs/cycs/txt-240907.htm` is not a CSS selector: Expected identity for pseudoelement, got "/" instead` | 1 |
| `default-mode rule is not supported: `第{$.chapterNum}章` is not a CSS selector: Token "<{" was not expected` | 1 |
| `default-mode rule is not supported: `连载中{$.status}已完结` is not a CSS selector: Token "<{" was not expected` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error calling function with 0 argument(s) while 1 where expected
    at <eval> (<input>:3:14)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:453)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'includes' of undefined
    at <eval> (<input>:3:12)
    at <anonymous> (eval_script:1:36)
    at <e
    at <eval> (<input>:2:28)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1040)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': json rule is invalid: expected value at line 1 column 1
    at <eval> (<input>:1:11)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:419)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: Error converting from js 'Rule' into type 'String': json rule is invalid: expected value at line 1 column 1
    at <eval> (<input>:1:4)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1314)
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
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'Symbol.iterator' of null
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:705)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property 'body' of undefined
    at <eval> (<input>:2:20)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:2574)
` | 1 |
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
    at <eval> (<input>:1:16)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:86)
` | 4 |
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
    at <eval> (eval_script:1:72)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:8)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:74)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:8)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:76)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: cannot read property of null
    at <eval> (<input>:1:8)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:79)
` | 4 |
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
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: getUrl is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:64)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: i is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:809)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: i is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:834)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: i is not defined
    at <eval> (<input>:2:12)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:189)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: i is not defined
    at <eval> (<input>:4:12)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:425)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: i is not defined
    at <eval> (<input>:5:14)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:541)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid keyword: with
    at <input>:10:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:7207)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid keyword: with
    at <input>:13:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:5627)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid keyword: with
    at <input>:13:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:5629)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid keyword: with
    at <input>:13:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:5866)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid keyword: with
    at <input>:13:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:6236)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid keyword: with
    at <input>:14:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:12371)
` | 3 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid keyword: with
    at <input>:9:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1692)
` | 10 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid redefinition of a variable
    at <input>:14:26
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:2147)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid redefinition of a variable
    at <input>:2:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:300)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid redefinition of a variable
    at <input>:40:5
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1484)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid redefinition of a variable
    at <input>:4:13
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:272)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid redefinition of lexical identifier
    at <input>:142:21
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:10147)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid redefinition of lexical identifier
    at <input>:17:15
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:4262)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid redefinition of lexical identifier
    at <input>:21:34
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:4850)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: invalid redefinition of lexical identifier
    at <input>:61:11
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:8945)
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
    at <eval> (<input>:1:10)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:63)
` | 1 |
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
    at <eval> (<input>:1:42)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:145)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: not a function
    at <eval> (<input>:2:30)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:219)
` | 6 |
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
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: nx is not defined
    at <eval> (<input>:1:34)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:706)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: org is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:1003)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: org is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:203)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: org is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:394)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: org is not defined
    at <eval> (<input>:2:9)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:5367)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: org is not defined
    at <eval> (<input>:3:26)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:4391)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: replaceCover is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:89)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: run is not defined
    at <eval> (<input>:2:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:104)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: run is not defined
    at <eval> (<input>:2:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:217)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: run is not defined
    at <eval> (<input>:2:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:231)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: run is not defined
    at <eval> (<input>:2:13)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:517)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: s is not defined
    at <eval> (<input>:1:39)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:353)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: src is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:363)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: src is not defined
    at <eval> (<input>:1:1)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:58)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: src is not defined
    at <eval> (<input>:1:14)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:247)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: src is not defined
    at <eval> (<input>:1:18)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:143)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: src is not defined
    at <eval> (<input>:1:22)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:267)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: src is not defined
    at <eval> (<input>:1:39)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:503)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: src is not defined
    at <eval> (<input>:1:39)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:605)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: src is not defined
    at <eval> (<input>:1:39)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:607)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: src is not defined
    at <eval> (<input>:2:18)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:466)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: src is not defined
    at <eval> (<input>:2:20)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:327)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: src is not defined
    at <eval> (<input>:2:37)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:274)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: src is not defined
    at <eval> (<input>:5:12)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:943)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: t is not defined
    at <eval> (<input>:1:15)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:125)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: t is not defined
    at <eval> (<input>:1:15)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:136)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: time is not defined
    at <eval> (<input>:6:63)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:420)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: title is not defined
    at <eval> (<input>:2:21)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:300)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: title is not defined
    at <eval> (<input>:4:4)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:154)
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
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: uuid is not defined
    at <eval> (<input>:4:67)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:327)
` | 1 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: uuid is not defined
    at <eval> (<input>:5:60)
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:340)
` | 2 |
| `default-mode rule is not supported: source error: JavaScript 执行失败: Error: variable name expected
    at <input>:13:1
    at <anonymous> (eval_script:1:36)
    at <eval> (eval_script:1:690)
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
| `default-mode rule is not supported: template is missing `}}`` | 167 |
| `json path is invalid: path must start with `$`` | 1 |
| `json path is invalid: unexpected character at 15` | 2 |
| `json path is invalid: unexpected character at 28` | 1 |
| `json rule is invalid: expected value at line 1 column 1` | 2 |
| `xpath rule is invalid: Token "?" was not expected` | 13 |
| `xpath rule is invalid: Unexpected error occurred. Please report this to the developer
DanglingCombinator` | 5 |
| `xpath rule is invalid: Unexpected error occurred. Please report this to the developer
NoQualifiedNameInAttributeSelector(
    Function(
        "position",
    ),
)` | 4 |
| `xpath rule is invalid: empty selector` | 2 |

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
| other | 223 |
| template delimiter | 97 |
| path parser | 21 |
