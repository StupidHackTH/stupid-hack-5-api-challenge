use actix_web::{HttpResponse, http::header};

const NOT_FOUND: &'static str = r#"
<!DOCTYPE HTML>
<html lang="en">
    <head>
        <title>Not found</title>
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
                padding: 0 20px;
            }

            #image {
                display: block;
                width: 100%;
                max-width: 460px;
                margin: 0 0 20px 0;
                padding: 0;
                object-fit: contain;
                object-cover: center;
                border-radius: 8px;
                box-shadow: 0 4px 16px rgba(0,0,0,.125);
            }

            #title {
                font-size: 32px;
                font-weight: 500;
                color: #374151;
                margin: 20px 0;
            }

            #docs {
                font-size: 18px;
                color: #007aff;
                margin: 0;
                text-decoration: none;
            }
        </style>
    </head>
    <body id="page">
        <img id="image" src="https://user-images.githubusercontent.com/35027979/118144418-a38b9680-b436-11eb-9649-f54c8bbf5346.JPG" alt="Error cats" />
        <h1 id="title">Not found</h1>
        <a id="docs" href="/">Documentation</a>
    </body>
</html>
"#;

pub async fn not_found() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(header::ContentType::html())
        .body(NOT_FOUND)
}
