use axum::handler::Handler;
use futures::future::{ready, Ready};

fn main() {
    fn sniff<T, H>(_: H)
    where
        H: Handler<T>,
    {
    }

    fn staticf(_: String) -> Ready<&'static str> {
        ready("")
    }
    // OK
    sniff(|_: String| ready(""));

    // OK
    sniff(staticf);

    // ERR, these do not compile
    // sniff::<String, fn(String) -> Ready<&'static str>>(|_: String| ready(""));
    // sniff::<String, fn(String) -> Ready<&'static str>>(staticf);

    // but, all of this does:
    trait MyHandler<T> {}

    impl<T, F> MyHandler<T> for F where F: FnOnce(T) -> () {}

    fn sniff_mine<T, H>(_: H)
    where
        H: MyHandler<T>,
    {
    }

    // mirror `staticf`
    fn staticmine(_: String) {}

    sniff_mine(|_: String| {});
    sniff_mine(staticmine);
    sniff_mine::<String, fn(String) -> ()>(|_: String| {});
    sniff_mine::<String, fn(String) -> ()>(staticmine);

    // Resources:
    //
    // [1] axum::handler::Handler: https://docs.rs/axum/0.5.15/axum/handler/trait.Handler.html
    // [2] Macro source of impl Handler for FnOnce(Tuple1)
    //     ```
    //     impl<F, Fut, B, Res, $($ty,)*> Handler<($($ty,)*), B> for F
    //     where
    //         F: FnOnce($($ty,)*) -> Fut + Clone + Send + 'static,
    //         Fut: Future<Output = Res> + Send,
    //         B: Send + 'static,
    //         Res: IntoResponse,
    //         $( $ty: FromRequest<B> + Send,)*
    //     {
    //         type Future = Pin<Box<dyn Future<Output = Response> + Send>>;
    //
    //         fn call(self, req: Request<B>) -> Self::Future {
    //             Box::pin(async move {
    //                 let mut req = RequestParts::new(req);
    //
    //                 $(
    //                     let $ty = match $ty::from_request(&mut req).await {
    //                         Ok(value) => value,
    //                         Err(rejection) => return rejection.into_response(),
    //                     };
    //                 )*
    //
    //                 let res = self($($ty,)*).await;
    //
    //                 res.into_response()
    //             })
    //         }
    //     }
    //     ```
    // [3] A playground attempt at mimicking the axum API and most of the types involved:
    //      https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ec4fb0662f3f0effd016a72a4f5542af
    //
    //      Note, `IntoHandler` is the parallel of
    //         ```
    //         this does not compile
    //         sniff::<String, fn(String) -> Ready<&'static str>>(|_: String| ready(""));
    //         ```
}
