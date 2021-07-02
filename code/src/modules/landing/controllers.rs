use actix_web::{ get, HttpResponse, http::header };

const LANDING: &'static str = r#"
<!DOCTYPE HTML>
<html lang="en">
    <head>
        <title>API Challenge</title>
        <meta charset="utf8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <style>
            * {
                box-sizing: border-box;
            }

            html, body {
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
            }

            #page {
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                width: 100%;
                height: 100vh;
                margin: 0;
                padding: 0 20px;
            }

            #image-wrapper {
                position: relative;
                display: block;
                width: 144px;
                height: 144px;
                margin: 0 0 12px 0;
                background-color: #F3F4F6;
                border-radius: 16px;
                box-shadow: 0 8px 24px rgba(0,0,0,.175);
                overflow: hidden;
            }

            #image {
                position: absolute;
                display: block;
                width: 100%;
                object-fit: contain;
                object-position: center;
            }

            #title {
                font-size: 32px;
                font-weight: 500;
                color: #374151;
                margin: 24px 0 8px 0;
            }

            #section {
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: flex-start;
                max-width: 460px;
                width: 100%;
                margin: 12px 0;
            }

            .detail {
                font-size: 18px;
                color: #4B5563;
                margin: 8px 0;
                line-height: 1.375em;
            }

            .link {
                color: #007aff;
                text-decoration: no-underline;
                margin: 0 2px;
                padding: 2px 4px;
                text-decoration: none;
                border-radius: 4px;
                transition: background-color .2s ease-out;
            }

            .link:hover,
            .link:focus {
                background-color: rgba(0,123,255,.1);
            }

            #docs {
                font-size: 21px;
                padding: 12px 32px;
            }
        </style>
    </head>
    <body id="page">
        <div id="image-wrapper">
            <img 
                id="image" 
                src="https://sth5-dispenser.saltyaom.com/sucrose.jpg" 
                alt="sucrose.jpg" 
                title="sucrose.jpg"
            />
        </div>
        <h1 id="title">API Challenge</h1>
        <section id="section">
            <p class="detail">Welcome to Stupid Hack 5 API Challenge!</p>
            <p class="detail">This challenge is created because I'm lazy to create frontend for reserving ticket code, so you all going to do it yourself.</p>
            <p class="detail">
                After you got your reservation code, please head to
                <a class="link" href="https://eventpop.me" rel="norefferer noreopener">EventPop</a>
                and redeem the ticket.
            </p>
        </section>
        <a id="docs" class="link" href="/docs" title="Click to view documentation">Documentation</a>
    </body>
</html>
"#;

#[get("/")]
pub async fn landing() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(header::ContentType::html())
        .body(LANDING)
}
