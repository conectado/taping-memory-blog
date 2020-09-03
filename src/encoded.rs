use brotli;
use brotli::CompressorReader;
use http::header::ACCEPT_ENCODING;
use libflate::{deflate, gzip};
use rocket::http::hyper::header::{qitem, AcceptEncoding, ContentEncoding, Encoding};
use rocket::http::ContentType;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::Responder;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Responder)]
pub struct EncodedContent {
    pub inner: Vec<u8>,
    pub content_type: ContentType,
    pub encoding: ContentEncoding,
}

fn negotiate_encoding(
    accept_encoding: Option<AcceptEncoding>,
    encoders: Vec<Encoding>,
) -> ContentEncoding {
    let mut encoding = ContentEncoding(vec![Encoding::Identity]);
    if let Some(accept_encoding) = accept_encoding {
        for enc in encoders {
            if accept_encoding.iter().any(|e| e.item == enc) {
                encoding = ContentEncoding(vec![enc]);
            }
        }
    }

    encoding
}

fn encode(mut content: impl std::io::Read, encoding: &ContentEncoding) -> Vec<u8> {
    let ContentEncoding(encodings) = encoding;
    let encoding = &encodings[0];

    match encoding {
        Encoding::EncodingExt(val) if val.to_string().to_lowercase().trim() == "br" => {
            let mut reader = CompressorReader::new(&mut content, 4096, 9, 16);
            let mut encoded_data = Vec::new();
            let _ = io::copy(&mut reader, &mut encoded_data);
            let _ = reader.read_to_end(&mut encoded_data);
            encoded_data
        }
        Encoding::Gzip => {
            let mut encoder = gzip::Encoder::new(Vec::new()).unwrap();
            io::copy(&mut content, &mut encoder).unwrap();
            encoder.finish().into_result().unwrap()
        }
        Encoding::Deflate => {
            let mut encoder = deflate::Encoder::new(Vec::new());
            io::copy(&mut content, &mut encoder).unwrap();
            encoder.finish().into_result().unwrap()
        }
        _ => content.bytes().map(|res| res.unwrap()).collect(),
    }
}

impl EncodedContent {
    pub fn new(
        content: impl std::io::Read,
        accept_encoding: Option<AcceptEncoding>,
        content_type: ContentType,
    ) -> EncodedContent {
        let encoding_brotli = Encoding::EncodingExt("br".to_string());
        let encoders = vec![Encoding::Deflate, Encoding::Gzip, encoding_brotli];

        let encoding = negotiate_encoding(accept_encoding, encoders);
        let inner = encode(content, &encoding);

        EncodedContent {
            inner,
            content_type,
            encoding,
        }
    }
}

pub struct AcceptEncodingHeader {
    pub accept_encoding: Option<AcceptEncoding>,
}

impl<'a, 'r> FromRequest<'a, 'r> for AcceptEncodingHeader {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        Outcome::Success(AcceptEncodingHeader {
            accept_encoding: request
                .headers()
                .get_one(ACCEPT_ENCODING.as_str())
                .map(|h| {
                    h.split(',')
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|h| Encoding::from_str(h.to_lowercase().trim()).unwrap())
                        .map(|h| qitem(h))
                        .collect::<Vec<_>>()
                })
                .map(|res| AcceptEncoding(res)),
        })
    }
}
