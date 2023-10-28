# 🦊 Fluffer
Fluffer is an *experimental* crate that aims to make writing
Gemini apps fun and easy.

## 🗼 Design
Similar to Axum, Fluffer routes are generic functions that
can return anything that implements the [`GemBytes`] trait.

There are some helpful implementations out of the box, so
please consult [`GemBytes`] and [`Fluff`] while you
experiment.

Also, this crate has a lot of examples for you to check out.
Including a dice roller app.

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
The [`GemBytes`] trait returns a Gemini byte response, which
is formatted like this:

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
Fluffer looks for the files `./key.pem` (private) and
`./cert.pem` (public) at runtime. If they can't be located,
a prompt appears to generate a key pair interactively.

There's currently no way to define an alternate path to your
pem files.

### Client identity
Gemini uses client certificates to facilitate identities.

[`Client`] exposes functions with the `ident_` prefix,
which correspond to common identity practices in Gemini.

* [`Client::ident_get`] gets the client's certificate.
* [`Client::ident_verify`] returns true if the current
  client's certificate matches one you pass.
* [`Client::ident_name`] returns the first entry in the
  certificate's `subject_name` field. This can be used to
  provide temporary usernames, or just to say hello.
* [`Client::ident_expired`] returns true if there's no
  certificate, or if the client's certificate is
  invalid/expired.

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
To get a user's input to a route, call [`Client::input`].
This returns the whole query line percent-decoded.

``` rust
App::default()
    .route("/" |c| async {
        c.input().unwrap_or("no input 😥".to_string())
    })
    .run()
    .await
    .unwrap()
```

#### Parameters
To access a parameter, *you **must** declare it first* in
the path string. Referencing an undefined parameter causes
the connection's thread to panic.

``` rust
App::default()
    .route("/page=:number" |c| async {
        format!("{}", c.parameter("number").unwrap_or("no page number 💢"))
    })
    .run()
    .await
    .unwrap()
```

If you're unfamiliar with matchit patterns, here's a couple
of examples:

- `"/owo/:a/:b"` defines parameters `a` and `b`, e.g: `/owo/thisisa/thisisb`
- `"/page=:n/filter=:f` defines the parameter `n`, e.g: `/page=20/filter=date`.

***Things to keep in mind:***

- Every parameter **must** be included in your url for the
  route to be found.
- Be careful where you define your parameters. It's possible
  to consume requests intended for a different route.
- It's more flexible to represent complex expressions as a
  single parameter, which you parse manually inside the
  route function.

## 📚 Helpful Resources
* [Gemini spec](https://gemini.circumlunar.space/docs/specification.gmi)

## 📋 Todo
* [X] Async for route functions
* [X] Switch to openssl
* [X] Add peer certificate to client
* [X] Spawn threads
* [ ] App data
* [ ] Titan support

