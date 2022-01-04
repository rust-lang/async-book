> **🌐 This is the korean translation of the book "Asynchronous Programming in Rust"**
>
> *[🔗 Click here to read the original book online!](https://rust-lang.github.io/async-book/)*
>
> you can also find out other translations [here](https://rust-lang.github.io/async-book/12_appendix/01_translations.html)

# async-book
Rust와 비동기 프로그래밍

## 사용 전 필수 사항
async book은 [`mdbook`]으로 제작되었습니다. 시작하기에 앞서 다음 명령어로 mdbook을 설치해 주세요.

```
cargo install mdbook
cargo install mdbook-linkcheck
```

[`mdbook`]: https://github.com/rust-lang/mdBook

## 배포
`mdbook build` 명령을 실행하면 `book/` 폴더 아래에 완성된 책이 생성됩니다.
```
mdbook build
```

## 개발환경
작성 도중 완성된 결과를 빠르게 확인하고 싶다면, `mdbook serve`로 로컬 웹 서버를 실행해 변경된 내용을 실시간으로 확인할 수 있습니다.
```
mdbook serve
```

## 기여하기
이 책은 [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)의 한국어 번역본입니다. 한국어 번역에 기여하고 싶으시다면 번역을 수정/개선한 부분을 PR로 보내주세요.