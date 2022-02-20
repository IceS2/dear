pub(crate) struct Selector<S>
where
    S: SelectorImp,
{
    query: Query,
    selector: S
}
