#[derive (Debug)]
pub struct UrlFrontier<'a> {
    _urls_to_prioritize: Vec<Option<Url<'a>>>,
}

#[derive (Debug)]
pub struct Url<'a> {
    _host_name: &'a str,
}

impl <'a>Url<'a> {
    pub fn new(host_name: &'a str) -> Url<'a> {
        Url { _host_name: host_name }
    }
}

impl <'a>UrlFrontier<'a> {
    pub fn new() -> UrlFrontier<'a> {
        let urls_to_prioritize: Vec<<Option<Url<'a>>> = Vec::with_capacity(100);
        UrlFrontier {
            _urls_to_prioritize: urls_to_prioritize,

        }
    }

    pub fn push_url(&mut self, url: Url<'a>) {
        self._urls_to_prioritize.push(Some(url));
    }

    pub fn prioritize_urls(&mut self) {
        for opt_url in &self._urls_to_prioritize {
            let Some(url) = opt_url else {
                continue;
            };

        }
    }
}
