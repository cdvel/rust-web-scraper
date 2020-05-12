# Rust: Web scraper

A simple asynchronous web scraper written in Rust, based on [kadekillary/scraping-with-rust](https://kadekillary.work/post/webscraping-rust/)


## Sample output
```
*. Hacker news headlines

1. Gilbert Strang's a 2020 Vision of Linear Algebra
score:		30 points
hnuser:		organicfigs
age:		1 hour ago
url:		https://ocw.mit.edu/resources/res-18-010-a-2020-vision-of-linear-algebra-spring-2020/index.htm

2. Thank you for helping us increase our bandwidth
score:		403 points
hnuser:		edward
age:		8 hours ago
url:		https://blog.archive.org/2020/05/11/thank-you-for-helping-us-increase-our-bandwidth/

3. Webrecorder: Make an interactive copy of any web page that you browse
score:		356 points
hnuser:		pcr910303
age:		13 hours ago
url:		https://webrecorder.io/

4. Playing Games on a 60s Computer
score:		58 points
hnuser:		souterrain
age:		4 hours ago
url:		https://www.youtube.com/watch?v=L743MjJthHY

5. John Peel Sessions
score:		149 points
hnuser:		termau
age:		9 hours ago
url:		https://davestrickson.blogspot.com/2020/05/john-peel-sessions.html
```


## Dependencies

- [reqwest](https://github.com/seanmonstar/reqwest)
- [tokio](https://github.com/tokio-rs/tokio)
- [select](https://github.com/utkarshkukreti/select.rs)
- [scraper](https://github.com/causal-agent/scraper)


## Resources

- https://kadekillary.work/post/webscraping-rust/
- https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html
- https://doc.rust-lang.org/book/
