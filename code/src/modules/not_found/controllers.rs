use actix_web::{HttpResponse, http::header};

const NOT_FOUND: &'static str = r#"
<!DOCTYPE HTML>
<html lang="en">
    <head>
        <title>Not found</title>
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

            #image {
                display: block;
                width: 100%;
                max-width: 460px;
                margin: 0 0 20px 0;
                padding: 0;
                object-fit: contain;
                object-position: center;
                border-radius: 8px;
                box-shadow: 0 4px 16px rgba(0,0,0,.125);
            }

            #title {
                font-size: 32px;
                font-weight: 500;
                color: #374151;
                margin: 20px 0;
            }

            .link {
                color: #007aff;
                text-decoration: no-underline;
                padding: 2px 6px;
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
        <img id="image" src="https://user-images.githubusercontent.com/35027979/118144418-a38b9680-b436-11eb-9649-f54c8bbf5346.JPG" alt="Error cats" />
        <h1 id="title">Not found</h1>
        <a id="docs" class="link" href="/docs" title="Click to view documentation">Documentation</a>
    </body>
</html>
"#;

pub async fn not_found() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(header::ContentType::html())
        .body(NOT_FOUND)
}
