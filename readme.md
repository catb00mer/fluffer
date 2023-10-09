# 🦊 Fluffer
Fluffer is a crate that aims to make writing Gemini apps fun
and easy.

## 🗼 Design
Similar to Axum, Fluffer routes are generic functions that
can return anything that implements the [`GemBytes`] trait.

There are some helpful implementations out of the box, so
please consult [`GemBytes`] and [`Fluff`] while you
experiment.

Also, this crate has a lot of examples for you to check
out. Including a dice roller app.

Here is a basic example of a Fluffer app.

``` rust
use fluffer::{App, Fluff};

#[tokio::main]
async fn main() {
    App::default()
        .route("/", |_| async {
            "# Welcome\n=> /u32 Should show a number\n=> /pic 🦊 Here's a cool picture!"
        })
        .route("/u32", |_| async { 777 })
        .route("/pic", |_| async { Fluff::File("picture.png".to_string()) })
        .run()
        .await;
}
```

### 💎 GemBytes
The [`GemBytes`] trait returns a Gemini byte
response, which is formatted like this:

``` text
<STATUS><SPACE><META>\r\n<CONTENT>
```

*Note: you must include the `<SPACE>` character, even if
`<META>` is blank.*

To implement [`GemBytes`] on a type is to decide which
Gemini response is appropriate for it.

For example: it is sensible to represent some mime-ambiguous
data as a successful Gemtext response so it can be read in a
client.

``` rust
use fluffer::{GemBytes, async_trait};

struct Profile {
    name: String,
    bio: String,
}

#[async_trait]
impl GemBytes for Profile {
    async fn gem_bytes(&self) -> Vec<u8> {
        format!("20 text/gemini\r\n# {},\n{}", self.name, self.bio).into_bytes()
    }
}
```

## 📜 Certificates

### Server
Fluffer looks for the files `./key.pem` (private) and `./cert.pem` (public) at
runtime. If they can't be located, a prompt appears to
generate a keypair interactively.

There's currently no way to define an alternate path to your
pem files.

## 🥴 Parameters and Input
Queries in Gemini aren't one-to-one with HTTP.

Gemini clients tend to consider the entire query line to be
a user's input. As such, they discard any queries you may
have included in a link.

In other words, `/?p=20` often becomes `/?user%20input`.

This is a problem for apps like search engines, which may
want to include filters and pagination in each request
alongside a user's search query.

To simplify the problem, Fluffer encourages you to use the
whole query as input, and [`matchit`]'s route parameters for
everything else.

#### Input
To get a user's input to a route, call [`Context::input`].
This returns the whole query line percent-decoded.

``` rust
App::default()
    .route("/" |ctx| async {
        ctx.input().unwrap_or("no input 😥".to_string())
    })
    .run()
    .await
    .unwrap()
```

#### Parameters
To access a parameter, first declare it in the route path
string.

``` rust
App::default()
    .route("/page=:number" |ctx| async {
        format!("{}", ctx.params.get("number").unwrap_or("no page number 💢"))
    })
    .run()
    .await
    .unwrap()
```

## 📚 Helpful Resources
* [Gemini spec](https://gemini.circumlunar.space/docs/specification.gmi)

## 📋 Todo
* [X] Async for route functions
* [X] Switch to openssl
* [X] Add peer certificate to context
* [X] Spawn threads
* [ ] User-defined data
* [ ] Titan support

