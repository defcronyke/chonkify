use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::env;

async fn index(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />

    <title>chonkify.me</title>

    <!-- Global site tag (gtag.js) - Google Analytics -->
    <script async src="https://www.googletagmanager.com/gtag/js?id=G-Q39PHFXCGE"></script>
    <script>
      window.dataLayer = window.dataLayer || [];
      function gtag(){dataLayer.push(arguments);}
      gtag('js', new Date());
    
      gtag('config', 'G-Q39PHFXCGE');
    </script>
</head>

<body style="background-color: rgba(128, 200, 200, 0.7);">
    <h2><a href="https://chonkify.me" style="text-decoration: none; color: black;">chonkify.me</a></h2>

    <p>This is to chonkify yourself only, or cat or lizard / turtle / snake.
    You aren't allowed to chonkify someone else.. and especially not dogs!!</p>

    <p>agree? <input type="checkbox" name="terms" id="terms" style="width: 1.6em; height: 1.6em;" required /><br />
    <br />
    chonkiness (1 - 99): <input inputmode="numeric" pattern="[0-9]*" type="text" name="chonk" id="chonk" value="74" style="padding: 0.5em; width: 4em;" required />
    <input type="file" name="me-in" id="me-in" />
    </p>

    <div id="me-area" style="width: auto; height: 600px;">
        &nbsp;
    </div>

    <br /><br />

    Part of <a href="https://eternalvoid.net" style="color: black;">The Eternal Void Network</a>.

<script>
(function() {
    var meIn = document.getElementById('me-in');

    if (!meIn) {
        console.log('error: no input element');
        return;
    }

    var meArea = document.getElementById('me-area');
    if (!meArea) {
        console.log('error: no area element');
        return;
    }

    meIn.addEventListener('change', function(_e) {
        var terms = document.getElementById('terms');
        if (!terms) {
            console.log('error: no terms element');
            return;
        }

        if (!terms.checked) {
            var termsErr = 'error: you must agree to the terms to use this';
            console.log(termsErr);
            window.alert(termsErr);

            return;
        }

        var files = meIn.files;
        if (!files || files.length < 1) {
            console.log('error: no files');
            return;
        }

        var file = files[0];
        if (!file) {
            console.log('error: no file');
        }

        var chonk = document.getElementById('chonk');
        if (!chonk) {
            console.log('error: no chonk element');
            return;
        }

        var inverseChonk = parseInt(chonk.value);

        if (inverseChonk < 1 || inverseChonk > 99) {
            var chonkErr = 'error: chonkiness must be a number in the range: 1 - 99';
            console.log(chonkErr);
            window.alert(chonkErr);
            return;
        }

        var chonkiness = 100 - inverseChonk;

        meArea.style.background = 'url(' + URL.createObjectURL(file) + ')';
        meArea.style.backgroundRepeat = 'no-repeat';
        meArea.style.backgroundSize = '100% ' + chonkiness + '%';
    });
})();
</script>
</body>
</html>
"#
        .into(),
    ))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut port: u16 = 8080;
    match env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => {
                    port = n;
                }
                Err(_e) => {}
            };
        }
        Err(_e) => {}
    };
    let addr = ([0, 0, 0, 0], port).into();

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(index)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
