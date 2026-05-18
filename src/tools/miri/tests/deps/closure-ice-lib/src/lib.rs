/// Async closure in an external crate — required for the ICE (decoder.rs is only called for
/// defs loaded from .rmeta files, not local crate defs).
pub fn line_sink() -> impl futures::Sink<(), Error = std::io::Error> {
    futures::sink::unfold((), async |(), ()| Ok::<_, std::io::Error>(()))
}
